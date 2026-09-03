// Project Euler 540 - Counting Primitive Pythagorean Triples
//
// Count primitive Pythagorean triples with hypotenuse <= N = pi * 10^15.
// Uses Euler's totient sieve for small m, inclusion-exclusion for large m.
// Optimized: stack prime-factor list; rayon over m.

use rayon::prelude::*;

const N: i64 = 3_141_592_653_589_793;

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let x = (n as f64).sqrt() as i64;
    x - (x * x > n) as i64
}


#[inline(always)]
fn fill_prime_factors(mut m: usize, limit: i64, ff: &[u16], out: &mut [i64; 8]) -> usize {
    let mut n = 0usize;
    if m % 2 == 0 {
        out[0] = 2;
        n = 1;
        m >>= m.trailing_zeros();
    }
    while m > 1 {
        let spf = unsafe { *ff.get_unchecked(m / 2) } as usize;
        if spf == 0 {
            if (m as i64) <= limit {
                out[n] = m as i64;
                n += 1;
            }
            break;
        }
        if (spf as i64) > limit {
            break;
        }
        out[n] = spf as i64;
        n += 1;
        m /= spf;
        while m % spf == 0 {
            m /= spf;
        }
    }
    n
}

#[inline(always)]
fn count_rec(limit: i64, factors: &[i64], idx: usize, cur: i64) -> i64 {
    let mut sum = limit / cur;
    for i in idx..factors.len() {
        let next = cur * factors[i];
        if next > limit {
            break;
        }
        sum -= count_rec(limit, factors, i + 1, next);
    }
    sum
}

#[inline(always)]
fn num_relatively_prime(m: usize, limit: i64, ff: &[u16]) -> i64 {
    if limit <= 0 {
        return 0;
    }
    let mut factors = [0i64; 8];
    let nf = fill_prime_factors(m, limit, ff, &mut factors);
    if nf == 0 {
        return limit;
    }
    if nf == 1 {
        return limit - limit / factors[0];
    }
    if nf == 2 {
        let f0 = factors[0];
        let f1 = factors[1];
        let p = f0 * f1;
        return limit - limit / f0 - limit / f1 + if p <= limit { limit / p } else { 0 };
    }
    if nf == 3 {
        let f0 = factors[0];
        let f1 = factors[1];
        let f2 = factors[2];
        let mut ans = limit - limit / f0 - limit / f1 - limit / f2;
        let p01 = f0 * f1;
        if p01 <= limit {
            ans += limit / p01;
            let p02 = f0 * f2;
            if p02 <= limit {
                ans += limit / p02;
                let p12 = f1 * f2;
                if p12 <= limit {
                    ans += limit / p12;
                    let p012 = p01 * f2;
                    if p012 <= limit {
                        ans -= limit / p012;
                    }
                }
            }
        }
        return ans;
    }
    count_rec(limit, &factors[..nf], 0, 1)
}

fn compute_small(l: i64) -> i64 {
    const K: usize = 250_000;
    let mut phi = vec![0u32; K + 1];
    let mut primes = Vec::with_capacity(25_000);
    phi[1] = 1;
    for i in 2..=K {
        if phi[i] == 0 {
            primes.push(i);
            phi[i] = (i - 1) as u32;
        }
        for &p in &primes {
            let ip = i * p;
            if ip > K {
                break;
            }
            if i % p == 0 {
                phi[ip] = phi[i] * p as u32;
                break;
            } else {
                phi[ip] = phi[i] * (p - 1) as u32;
            }
        }
    }
    let mut pref_small = vec![0i64; K + 1];
    for i in 1..=K {
        pref_small[i] = pref_small[i - 1] + phi[i] as i64;
    }

    let mut memo = [0i64; 256];
    fn get_phi(n: i64, l: i64, pref: &[i64], memo: &mut [i64; 256]) -> i64 {
        if (n as usize) < pref.len() {
            return pref[n as usize];
        }
        let d = (l / n) as usize;
        if memo[d] != 0 {
            return memo[d];
        }
        let mut sum = 0i64;
        let mut m = 2i64;
        while m <= n {
            let q = n / m;
            let next_m = n / q;
            sum += (next_m - m + 1) * get_phi(q, l, pref, memo);
            m = next_m + 1;
        }
        let res = n * (n + 1) / 2 - sum;
        memo[d] = res;
        res
    }

    let mut fast_small = 0i64;
    let mut cur = l;
    while cur > 0 {
        fast_small += get_phi(cur, l, &pref_small, &mut memo);
        cur /= 2;
    }
    (fast_small - 1) / 2
}

fn sieve_ff(sqrt_n: usize) -> Vec<u16> {
    let limit_prime = isqrt(sqrt_n as i64) as usize;
    let mut is_p = vec![true; limit_prime + 1];
    let mut odd_primes = Vec::new();
    for i in 3..=limit_prime {
        if i % 2 == 1 && is_p[i] {
            odd_primes.push(i);
            let mut j = i * i;
            while j <= limit_prime {
                is_p[j] = false;
                j += 2 * i;
            }
        }
    }

    let num_odds = sqrt_n / 2 + 1;
    let mut ff = vec![0u16; num_odds];
    const CHUNK_SIZE: usize = 65536;

    ff.par_chunks_mut(CHUNK_SIZE)
        .enumerate()
        .for_each(|(chunk_idx, chunk)| {
            let start_idx = chunk_idx * CHUNK_SIZE;
            let low = 2 * start_idx + 1;
            let chunk_len = chunk.len();
            let high = low + 2 * (chunk_len - 1);

            for &p in &odd_primes {
                if p * p > high {
                    break;
                }
                let mut k = (low + p - 1) / p;
                if k < p {
                    k = p;
                }
                if k % 2 == 0 {
                    k += 1;
                }
                let first = k * p;
                let mut local_idx = (first - low) / 2;
                while local_idx < chunk_len {
                    unsafe {
                        let ptr = chunk.get_unchecked_mut(local_idx);
                        if *ptr == 0 {
                            *ptr = p as u16;
                        }
                    }
                    local_idx += p;
                }
            }
        });
    ff
}

fn main() {
    let l = isqrt(N / 2);
    let sqrt_n = isqrt(N) as usize;
    let m_max = isqrt(N);

    let (sum_small, ff) = rayon::join(|| compute_small(l), || sieve_ff(sqrt_n));

    let start_m = l as usize + 1;
    let end_m = m_max as usize + 1;
    let sum_large: i64 = (start_m..end_m)
        .into_par_iter()
        .map(|m| {
            let m_i64 = m as i64;
            let limit = isqrt(N - m_i64 * m_i64) >> (m & 1);
            num_relatively_prime(m, limit, &ff)
        })
        .sum();

    let ans = sum_small + sum_large;

    println!("{ans}");
}
