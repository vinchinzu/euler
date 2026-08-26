// Project Euler 379 - Least common multiple count
// Uses Mobius sieve + hyperbola method for D(n) and T(m).
// Outer d is parallel; T/D stay sequential to avoid nested-rayon contention.

use rayon::prelude::*;

/// Prefix of D(n)=sum_{k<=n} floor(n/k) for n < SMALL_LIM (fits L3 as u32).
const SMALL_LIM: usize = 10_000_000;

fn isqrt(n: u64) -> u64 {
    n.isqrt()
}

fn icbrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = (n as f64).cbrt() as u64;
    while x > 0 && x.saturating_mul(x).saturating_mul(x) > n {
        x -= 1;
    }
    loop {
        let nxt = x + 1;
        match nxt.checked_mul(nxt).and_then(|s| s.checked_mul(nxt)) {
            Some(c) if c <= n => x = nxt,
            _ => break,
        }
    }
    x
}

fn make_d_prefix(lim: usize) -> Vec<u32> {
    // Linear sieve for tau(n), replacing the harmonic-series divisor sieve.
    let mut least_prime = vec![0u32; lim];
    let mut exponent = vec![0u8; lim];
    let mut tau = vec![0u16; lim];
    let mut primes = Vec::with_capacity(lim / 10);
    tau[1] = 1;
    for i in 2..lim {
        if least_prime[i] == 0 {
            least_prime[i] = i as u32;
            exponent[i] = 1;
            tau[i] = 2;
            primes.push(i as u32);
        }
        for &p in &primes {
            let x = i * p as usize;
            if x >= lim {
                break;
            }
            least_prime[x] = p;
            if p == least_prime[i] {
                let old_factor = exponent[i] as u16 + 1;
                exponent[x] = exponent[i] + 1;
                tau[x] = tau[i] / old_factor * (old_factor + 1);
                break;
            }
            exponent[x] = 1;
            tau[x] = tau[i] * 2;
        }
    }
    drop(least_prime);
    drop(exponent);
    drop(primes);

    let mut d = vec![0u32; lim];
    let mut acc = 0u32;
    for i in 1..lim {
        acc += tau[i] as u32;
        d[i] = acc;
    }
    d
}

/// D(n) for n that fits in u32. Independent accumulators keep IDIV in flight.
#[inline(always)]
fn d_func_u32(n: u32) -> u64 {
    let sq = n.isqrt();
    let mut s0 = 0u64;
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    let mut s3 = 0u64;
    let mut k = 1u32;
    while k + 3 <= sq {
        s0 += (n / k) as u64;
        s1 += (n / (k + 1)) as u64;
        s2 += (n / (k + 2)) as u64;
        s3 += (n / (k + 3)) as u64;
        k += 4;
    }
    while k <= sq {
        s0 += (n / k) as u64;
        k += 1;
    }
    let sq64 = sq as u64;
    2 * (s0 + s1 + s2 + s3) - sq64 * sq64
}

/// D(n) = sum_{k=1}^{n} floor(n/k)
#[inline(always)]
fn d_func(n: u64, small: &[u32]) -> u64 {
    if n == 0 {
        return 0;
    }
    if (n as usize) < small.len() {
        // SAFETY: n < small.len()
        return unsafe { *small.get_unchecked(n as usize) } as u64;
    }
    if n <= u32::MAX as u64 {
        return d_func_u32(n as u32);
    }
    let sq = isqrt(n);
    let mut s0 = 0u64;
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    let mut s3 = 0u64;
    let mut k = 1u64;
    while k + 3 <= sq {
        s0 += n / k;
        s1 += n / (k + 1);
        s2 += n / (k + 2);
        s3 += n / (k + 3);
        k += 4;
    }
    while k <= sq {
        s0 += n / k;
        k += 1;
    }
    2 * (s0 + s1 + s2 + s3) - sq * sq
}

/// T(m) = number of ordered triples (a,b,c) with a*b*c <= m
/// Sequential on purpose: nested par_iter contends with the outer d pool.
fn t_func(m: u64, small: &[u32]) -> i64 {
    if m == 0 {
        return 0;
    }

    let cbrt_m = icbrt(m);
    let mut total: u64 = 0;
    for a in 1..=cbrt_m {
        total += d_func(m / a, small);
    }

    let mut a = cbrt_m + 1;
    while a <= m {
        let v = m / a;
        let a_max = m / v;
        total += d_func(v, small) * (a_max - a + 1);
        a = a_max + 1;
    }

    total as i64
}

fn main() {
    let n_big: u64 = 1_000_000_000_000;
    let l = isqrt(n_big) as usize;

    let dprefix = make_d_prefix(SMALL_LIM);

    // Sieve Mobius function using linear sieve
    let mut mobius = vec![0i8; l + 1];
    let mut is_p = vec![true; l + 1];
    let mut primes = Vec::with_capacity(l / 10 + 1);
    mobius[1] = 1;

    for i in 2..=l {
        if is_p[i] {
            primes.push(i);
            mobius[i] = -1;
        }
        for &p in &primes {
            let ip = i as u64 * p as u64;
            if ip > l as u64 {
                break;
            }
            is_p[ip as usize] = false;
            if i % p == 0 {
                mobius[ip as usize] = 0;
                break;
            }
            mobius[ip as usize] = -mobius[i];
        }
    }

    // Parallel summation over squarefree d. Cost of T(n/d²) falls with d;
    // reverse so expensive (small d) items are stolen first.
    let mut nonzero_d: Vec<usize> = (1..=l).filter(|&d| mobius[d] != 0).collect();
    nonzero_d.reverse();
    let ans_parallel: i64 = nonzero_d
        .par_iter()
        .map(|&d| {
            // SAFETY: d in 1..=l, mobius.len() = l+1
            let mu = unsafe { *mobius.get_unchecked(d) } as i64;
            let d64 = d as u64;
            mu * t_func(n_big / (d64 * d64), &dprefix)
        })
        .sum();
    let mut ans = ans_parallel;
    ans += n_big as i64;
    ans /= 2;

    println!("{}", ans);
}
