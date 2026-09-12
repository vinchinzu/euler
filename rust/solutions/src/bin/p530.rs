// Project Euler 530 - GCD Sum
//
// Compute sum_{n=1}^{N} sum_{d|n} gcd(d, n/d) for N = 10^15.
// Uses Mobius function sieve and floor quotient summation.
// Parallelized with rayon using load-balanced work distribution.

use rayon::prelude::*;

const N: i64 = 1_000_000_000_000_000;

#[inline]
fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

#[inline]
fn icbrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).cbrt() as i64;
    while x > 0 && x * x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

#[inline]
fn sq(x: i64) -> i64 {
    x * x
}

/// sum_{i=1}^{n} floor(n/i) using the identity:
/// S = 2 * sum_{i=1}^{floor(sqrt(n))} floor(n/i) - floor(sqrt(n))^2
/// Four independent accumulators so the CPU can overlap idiv latency.
#[inline]
fn sum_floor_quotients(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let s = isqrt(n);
    let mut s0 = 0i64;
    let mut s1 = 0i64;
    let mut s2 = 0i64;
    let mut s3 = 0i64;
    let mut k = 1i64;
    while k + 3 <= s {
        s0 += n / k;
        s1 += n / (k + 1);
        s2 += n / (k + 2);
        s3 += n / (k + 3);
        k += 4;
    }
    while k <= s {
        s0 += n / k;
        k += 1;
    }
    2 * (s0 + s1 + s2 + s3) - s * s
}

/// Parallel version of sum_floor_quotients for large n.
fn sum_floor_quotients_par(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let s = isqrt(n);
    if s <= 100_000 {
        return sum_floor_quotients(n);
    }

    let chunk_size = 500_000i64;
    let n_chunks = ((s - 1) / chunk_size + 1) as usize;

    let partial: i64 = (0..n_chunks)
        .into_par_iter()
        .map(|c| {
            let lo = c as i64 * chunk_size + 1;
            let hi = std::cmp::min(lo + chunk_size - 1, s);
            let mut s0 = 0i64;
            let mut s1 = 0i64;
            let mut s2 = 0i64;
            let mut s3 = 0i64;
            let mut i = lo;
            while i + 3 <= hi {
                s0 += n / i;
                s1 += n / (i + 1);
                s2 += n / (i + 2);
                s3 += n / (i + 3);
                i += 4;
            }
            while i <= hi {
                s0 += n / i;
                i += 1;
            }
            s0 + s1 + s2 + s3
        })
        .sum();

    2 * partial - s * s
}

#[inline]
fn sum_powers_1(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    n * (n + 1) / 2
}

fn mobius_sieve_par(n: usize) -> Vec<i8> {
    let slimit = {
        let mut s = (n as f64).sqrt() as usize;
        while s * s > n {
            s -= 1;
        }
        while (s + 1) * (s + 1) <= n {
            s += 1;
        }
        s
    };
    let mut is_prime = vec![true; slimit + 1];
    is_prime[0] = false;
    if slimit >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i <= slimit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= slimit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<u32> = (2..=slimit)
        .filter(|&p| is_prime[p])
        .map(|p| p as u32)
        .collect();

    let mut mu = vec![0i8; n + 1];
    mu[1] = 1;
    if n < 2 {
        return mu;
    }

    let chunk = 262_144usize;
    mu[2..].par_chunks_mut(chunk).enumerate().for_each(|(ci, sl)| {
        let lo = 2 + ci * chunk;
        let hi = lo + sl.len() - 1;
        for x in sl.iter_mut() {
            *x = 1;
        }
        let mut rem = vec![0u32; sl.len()];
        for i in 0..sl.len() {
            rem[i] = (lo + i) as u32;
        }
        for &p in &primes {
            let pu = p as usize;
            let mut m = ((lo + pu - 1) / pu) * pu;
            while m <= hi {
                let i = m - lo;
                sl[i] = -sl[i];
                rem[i] /= p;
                while rem[i] % p == 0 {
                    rem[i] /= p;
                    sl[i] = 0;
                }
                m += pu;
            }
        }
        for i in 0..sl.len() {
            if rem[i] > 1 && sl[i] != 0 {
                sl[i] = -sl[i];
            }
        }
    });
    mu
}

fn main() {
    let l = icbrt(N);
    let sqrt_n = isqrt(N) as usize;

    let mobius = mobius_sieve_par(sqrt_n);

    // Precompute small[]
    let small: Vec<i64> = {
        let mut s = vec![0i64; (l + 2) as usize];
        for i in 1..=l {
            s[i as usize] = sum_floor_quotients(i);
        }
        s
    };

    // Precompute big[] with parallel computation.
    let big: Vec<i64> = {
        let mut b = vec![0i64; (l + 2) as usize];
        let l_usize = l as usize;

        let inner_threshold = 3usize;

        for i in 1..=std::cmp::min(inner_threshold, l_usize) {
            b[i] = sum_floor_quotients_par(N / sq(i as i64));
        }

        if l_usize > inner_threshold {
            let start = inner_threshold + 1;
            let num_groups = rayon::current_num_threads() * 8;
            let total_harmonic: f64 = (start..=l_usize).map(|i| 1.0 / i as f64).sum();
            let target_per_group = total_harmonic / num_groups as f64;

            let mut groups: Vec<(usize, usize)> = Vec::with_capacity(num_groups);
            let mut grp_start = start;
            let mut cumulative = 0.0f64;
            for i in start..=l_usize {
                cumulative += 1.0 / i as f64;
                if cumulative >= target_per_group && groups.len() < num_groups - 1 {
                    groups.push((grp_start, i));
                    grp_start = i + 1;
                    cumulative = 0.0;
                }
            }
            if grp_start <= l_usize {
                groups.push((grp_start, l_usize));
            }

            let results: Vec<Vec<(usize, i64)>> = groups
                .into_par_iter()
                .map(|(lo, hi)| {
                    let mut local = Vec::with_capacity(hi - lo + 1);
                    for i in lo..=hi {
                        local.push((i, sum_floor_quotients(N / sq(i as i64))));
                    }
                    local
                })
                .collect();

            for group in results {
                for (i, val) in group {
                    b[i] = val;
                }
            }
        }

        b
    };

    let h_max = isqrt(N);
    let h_max_us = h_max as usize;

    let ans: i64 = (1..h_max_us + 1)
        .into_par_iter()
        .with_min_len(64)
        .filter(|&h| unsafe { *mobius.get_unchecked(h) != 0 })
        .map(|h| {
            let h = h as i64;
            let mu_h = unsafe { *mobius.get_unchecked(h as usize) } as i64;
            let n_h = N / sq(h);
            let l_local = icbrt(n_h) / 10 + 1;
            let sqrt_n_over_l = isqrt(n_h / l_local);

            let mut local_ans = 0i64;

            for g in 1..=sqrt_n_over_l {
                let gh = g * h;
                let term = if gh <= l {
                    unsafe { *big.get_unchecked(gh as usize) }
                } else {
                    let idx = n_h / sq(g);
                    if idx <= l {
                        unsafe { *small.get_unchecked(idx as usize) }
                    } else {
                        sum_floor_quotients(idx)
                    }
                };
                local_ans += mu_h * term * g;
            }

            for q in 1..l_local {
                let sqrt_n_q = isqrt(n_h / q);
                let sqrt_n_q1 = isqrt(n_h / (q + 1));
                let small_q = if q <= l {
                    unsafe { *small.get_unchecked(q as usize) }
                } else {
                    sum_floor_quotients(q)
                };
                local_ans += mu_h * small_q * (sum_powers_1(sqrt_n_q) - sum_powers_1(sqrt_n_q1));
            }

            local_ans
        })
        .sum();

    println!("{ans}");
}
