// Project Euler 399: Square-free Fibonacci
// F_n is square-free iff n is not a multiple of p*z(p) for any prime p
// (Wall's conjecture: z(p^2) = p*z(p)). Find the 10^8-th such F_n.

use rayon::prelude::*;

const N: usize = 100_000_000;
const L: usize = 200_000_000;
const MOD: u64 = 10_000_000_000_000_000;

fn main() {
    let pmax = prime_upper_bound(L as u64) as usize;
    let (spf, primes) = linear_sieve(pmax + 1);

    let mut steps: Vec<usize> = primes
        .par_iter()
        .with_min_len(64)
        .filter_map(|&p| {
            if p as usize > pmax {
                return None;
            }
            let pu = p as u64;
            let z = rank_apparition(pu, &spf);
            let m = pu * z;
            if m < L as u64 {
                Some(m as usize)
            } else {
                None
            }
        })
        .collect();

    steps.sort_unstable();
    steps.dedup();
    let steps = prune_multiples(&steps);

    let bits = mark_sqf(&steps);
    let index = nth_set_bit(&bits, N);
    let last16 = fib_mod_u128(index as u64, MOD);

    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let log_value = index as f64 * phi.log10() - 0.5 * 5.0_f64.log10();
    let mut exponent = log_value.floor() as i64;
    let mut mantissa = 10.0_f64.powf(log_value - exponent as f64);
    if mantissa >= 9.95 {
        mantissa /= 10.0;
        exponent += 1;
    }

    println!("{},{:.1}e{}", last16, mantissa, exponent);
}

/// Largest p that can satisfy p*z(p) < nmax, using p <= F_k and p*k < nmax.
fn prime_upper_bound(nmax: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut best = 0u64;
    for k in 1..200u64 {
        let t = a + b;
        a = b;
        b = t;
        let cand = a.min(nmax / k);
        if cand > best {
            best = cand;
        }
        if k > 60 && nmax / k <= best {
            break;
        }
    }
    best
}

fn linear_sieve(limit: usize) -> (Vec<u32>, Vec<u32>) {
    let mut spf = vec![0u32; limit + 1];
    let mut primes = Vec::with_capacity(limit / 10);
    for i in 2..=limit {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
        }
        for &p in &primes {
            let v = i * p as usize;
            if v > limit || p > spf[i] {
                break;
            }
            spf[v] = p;
        }
    }
    (spf, primes)
}

/// z(p): smallest n>0 with p | F_n. For p≠5, z(p) | (p - (5/p)).
fn rank_apparition(p: u64, spf: &[u32]) -> u64 {
    if p == 5 {
        return 5;
    }
    // (5/p) = (p/5) since 5 ≡ 1 (mod 4)
    let mut m = if matches!(p % 5, 1 | 4) { p - 1 } else { p + 1 };

    let mut fac = [0u32; 16];
    let mut nf = 0usize;
    let mut t = m;
    while t > 1 {
        // SAFETY: t starts at p±1 <= spf.len()-1 and strictly decreases
        let q = unsafe { *spf.get_unchecked(t as usize) };
        fac[nf] = q;
        nf += 1;
        let q64 = q as u64;
        while t % q64 == 0 {
            t /= q64;
        }
    }
    for i in 0..nf {
        let q = fac[i] as u64;
        while m % q == 0 {
            let m2 = m / q;
            if fib_mod_small(m2, p) == 0 {
                m = m2;
            } else {
                break;
            }
        }
    }
    m
}

/// F_n mod p with p^2 fitting in u64 (p < ~6e6).
#[inline(always)]
fn fib_mod_small(n: u64, p: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    if n == 0 {
        return 0;
    }
    let mut bit = 1u64 << (63 - n.leading_zeros());
    while bit != 0 {
        let two_b = b << 1;
        let two_b = if two_b >= p { two_b - p } else { two_b };
        let t = if two_b >= a { two_b - a } else { two_b + p - a };
        let c = (a * t) % p;
        let d = (a * a + b * b) % p;
        if n & bit != 0 {
            a = d;
            b = c + d;
            if b >= p {
                b -= p;
            }
        } else {
            a = c;
            b = d;
        }
        bit >>= 1;
    }
    a
}

fn prune_multiples(steps: &[usize]) -> Vec<usize> {
    let mut out = Vec::with_capacity(steps.len());
    for &s in steps {
        if out.iter().any(|&k| s % k == 0) {
            continue;
        }
        out.push(s);
    }
    out
}

fn mark_sqf(steps: &[usize]) -> Vec<u64> {
    let nwords = (L + 63) / 64;
    // 192-bit repeating pattern: clear indices divisible by 6.
    let mut pat = [0u64; 3];
    for i in 0..192 {
        if i % 6 != 0 {
            pat[i / 64] |= 1u64 << (i % 64);
        }
    }
    let mut bits = Vec::with_capacity(nwords);
    for w in 0..nwords {
        bits.push(pat[w % 3]);
    }
    let rem = L & 63;
    if rem != 0 {
        bits[nwords - 1] &= (1u64 << rem) - 1;
    }

    for &step in steps {
        if step <= 6 {
            continue; // 6 already baked in; nothing smaller survives pruning
        }
        let mut i = step;
        while i < L {
            // SAFETY: i < L => i>>6 < nwords
            unsafe {
                *bits.get_unchecked_mut(i >> 6) &= !(1u64 << (i & 63));
            }
            i += step;
        }
    }
    bits
}

fn nth_set_bit(bits: &[u64], n: usize) -> usize {
    let mut rem = n;
    for (wi, &word) in bits.iter().enumerate() {
        let c = word.count_ones() as usize;
        if rem > c {
            rem -= c;
            continue;
        }
        let mut x = word;
        while rem > 0 {
            let b = x.trailing_zeros() as usize;
            x &= x - 1;
            rem -= 1;
            if rem == 0 {
                return (wi << 6) + b;
            }
        }
    }
    panic!("not enough square-free indices");
}

fn fib_mod_u128(n: u64, m: u64) -> u64 {
    let mul = |a: u64, b: u64| ((a as u128 * b as u128) % m as u128) as u64;
    let mut a = 0u64;
    let mut b = 1u64;
    if n == 0 {
        return 0;
    }
    let mut bit = 1u64 << (63 - n.leading_zeros());
    while bit != 0 {
        let two_b_minus_a = (2 * b % m + m - a % m) % m;
        let c = mul(a, two_b_minus_a);
        let d = (mul(a, a) + mul(b, b)) % m;
        if n & bit != 0 {
            a = d;
            b = (c + d) % m;
        } else {
            a = c;
            b = d;
        }
        bit >>= 1;
    }
    a
}
