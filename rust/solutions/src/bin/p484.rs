// Project Euler 484: Arithmetic derivative
// sum_{k=2..N} gcd(k, k') via Dirichlet convolution on powerful numbers.
// f = g * μ^{-1} on prime powers; only squareful d contribute.

use rayon::prelude::*;

const N: i64 = 5_000_000_000_000_000; // 5 * 10^15
/// Nested parallel DFS only while remaining limit is this large.
const PAR_N: i64 = 100_000_000_000;

fn primes_upto(n: usize) -> Vec<u32> {
    if n < 2 {
        return Vec::new();
    }
    // Odd-only sieve: index i <-> 2i+1.
    let size = n / 2 + 1;
    let mut comp = vec![0u8; size];
    comp[0] = 1;
    let lim = n.isqrt();
    let mut p = 3usize;
    while p <= lim {
        if comp[p / 2] == 0 {
            let mut j = p * p / 2;
            while j < size {
                // SAFETY: j < size by loop bound
                unsafe { *comp.get_unchecked_mut(j) = 1 };
                j += p;
            }
        }
        p += 2;
    }
    let mut primes = Vec::with_capacity(n / 12);
    primes.push(2);
    let last = if n % 2 == 0 { size - 1 } else { size };
    for i in 1..last {
        if unsafe { *comp.get_unchecked(i) } == 0 {
            primes.push((2 * i + 1) as u32);
        }
    }
    primes
}

#[inline]
fn icbrt(n: i64) -> i64 {
    if n < 8 {
        return if n >= 1 { 1 } else { 0 };
    }
    let mut x = (n as f64).cbrt() as i64;
    loop {
        let x3 = x.saturating_mul(x).saturating_mul(x);
        if x3 > n {
            x -= 1;
            continue;
        }
        let y = x + 1;
        let y3 = y.saturating_mul(y).saturating_mul(y);
        if y3 <= n {
            x = y;
            continue;
        }
        return x;
    }
}

/// sum_{p = primes[lo..hi)} (p-1) * floor(l0 / p^2).  p>2 so f(p^2)=p-1.
fn leaf_e2_sum(lo: usize, hi: usize, l0: i64, primes: &[u32], pref: &[i64]) -> i64 {
    if lo >= hi {
        return 0;
    }
    if hi - lo < 1024 {
        let mut res = 0i64;
        for i in lo..hi {
            let p = unsafe { *primes.get_unchecked(i) } as i64;
            res += (p - 1) * (l0 / (p * p));
        }
        return res;
    }
    let mut res = 0i64;
    let mut i = lo;
    while i < hi {
        let p = unsafe { *primes.get_unchecked(i) } as i64;
        let v = l0 / (p * p);
        if v == 0 {
            break;
        }
        let pmax = ((l0 / v) as u64).isqrt() as u32;
        let j = i + primes[i..hi].partition_point(|&x| x <= pmax);
        res += v * unsafe { *pref.get_unchecked(j) - *pref.get_unchecked(i) };
        i = j;
    }
    res
}

/// Contribution of powerful d that include primes[i] as the least prime, d | ... <= l0.
fn process_prime(i: usize, l0: i64, primes: &[u32], pref: &[i64], par: bool) -> i64 {
    let p = unsafe { *primes.get_unchecked(i) } as i64;
    let q = p * p;
    let mut l = l0 / q;
    if l == 0 {
        return 0;
    }
    let next_q = if i + 1 < primes.len() {
        let np = unsafe { *primes.get_unchecked(i + 1) } as i64;
        np * np
    } else {
        i64::MAX
    };

    // g(p^a); e is a mod p, with 0 meaning p | a.
    let mut g = 1i64;
    let mut e = 1i64;
    let mut res = 0i64;
    while l > 0 {
        let gp = g;
        e += 1;
        if e != 1 {
            if e == p {
                g *= q;
                e = 0;
            } else {
                g *= p;
            }
            let c = g - gp;
            res += c * l;
            if l >= next_q {
                res += c * dfs(i + 1, l, primes, pref, par && l >= PAR_N);
            }
        }
        l /= p;
    }
    res
}

fn dfs(i0: usize, l0: i64, primes: &[u32], pref: &[i64], par: bool) -> i64 {
    if i0 >= primes.len() {
        return 0;
    }
    let p0 = unsafe { *primes.get_unchecked(i0) } as i64;
    if p0 > l0 / p0 {
        return 0;
    }

    let p_hi = (l0 as u64).isqrt() as i64;
    let hi = i0 + primes[i0..].partition_point(|&p| (p as i64) <= p_hi);

    let rec_p = (p_hi as u64).isqrt() as i64;
    let rec_end = i0 + primes[i0..hi].partition_point(|&p| (p as i64) <= rec_p);

    let cub_p = icbrt(l0);
    let mid_end = i0 + primes[i0..hi].partition_point(|&p| (p as i64) <= cub_p);

    if par && rec_end > i0 + 1 {
        let (rec, rest) = rayon::join(
            || {
                (i0..rec_end)
                    .into_par_iter()
                    .with_min_len(1)
                    .map(|i| process_prime(i, l0, primes, pref, true))
                    .sum::<i64>()
            },
            || {
                let mut s = 0i64;
                for i in rec_end..mid_end {
                    s += process_prime(i, l0, primes, pref, false);
                }
                s + leaf_e2_sum(mid_end, hi, l0, primes, pref)
            },
        );
        rec + rest
    } else {
        let mut res = 0i64;
        for i in i0..mid_end {
            res += process_prime(i, l0, primes, pref, false);
        }
        res + leaf_e2_sum(mid_end, hi, l0, primes, pref)
    }
}

fn main() {
    let limit = (N as u64).isqrt() as usize;
    let primes = primes_upto(limit);
    let mut pref = vec![0i64; primes.len() + 1];
    for i in 0..primes.len() {
        pref[i + 1] = pref[i] + primes[i] as i64 - 1;
    }

    // sum_{k<=N} g(k) = N + sum_{d>1 powerful} f(d) floor(N/d); drop g(1)=1.
    let ans = N - 1 + dfs(0, N, &primes, &pref, true);
    println!("{}", ans);
}
