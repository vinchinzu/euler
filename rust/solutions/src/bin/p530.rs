// Project Euler 530 - GCD Sum
//
// Compute sum_{n=1}^{N} sum_{d|n} gcd(d, n/d) for N = 10^15.
// Uses Mobius function sieve and floor quotient summation.
// Parallelized with rayon using load-balanced work distribution.

use rayon::prelude::*;

const N: i64 = 1_000_000_000_000_000;

#[inline]
fn isqrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

#[inline]
fn icbrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).cbrt() as i64;
    while x > 0 && x * x * x > n { x -= 1; }
    while (x + 1) * (x + 1) * (x + 1) <= n { x += 1; }
    x
}

#[inline]
fn sq(x: i64) -> i64 { x * x }

/// sum_{i=1}^{n} floor(n/i) using the identity:
/// S = 2 * sum_{i=1}^{floor(sqrt(n))} floor(n/i) - floor(sqrt(n))^2
/// Sequential version for small n.
#[inline]
fn sum_floor_quotients(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let s = isqrt(n);
    let mut result = 0i64;
    let mut i = 1i64;
    while i <= s {
        result += n / i;
        i += 1;
    }
    2 * result - s * s
}

/// Parallel version of sum_floor_quotients for large n.
/// Splits [1, sqrt(n)] into chunks and sums n/i in parallel.
fn sum_floor_quotients_par(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let s = isqrt(n);
    if s <= 100_000 { return sum_floor_quotients(n); }

    let chunk_size = 500_000i64;
    let n_chunks = ((s - 1) / chunk_size + 1) as usize;

    let partial: i64 = (0..n_chunks).into_par_iter().map(|c| {
        let lo = c as i64 * chunk_size + 1;
        let hi = std::cmp::min(lo + chunk_size - 1, s);
        let mut acc = 0i64;
        let mut i = lo;
        while i <= hi {
            acc += n / i;
            i += 1;
        }
        acc
    }).sum();

    2 * partial - s * s
}

#[inline]
fn sum_powers_1(n: i64) -> i64 {
    if n <= 0 { return 0; }
    n * (n + 1) / 2
}

fn main() {
    let l = icbrt(N);
    let sqrt_n = isqrt(N) as usize;

    // Pre-compute Mobius function using linear sieve - O(N).
    let mobius: Vec<i8> = {
        let n = sqrt_n;
        let mut mu = vec![0i8; n + 1];
        let mut spf = vec![0u32; n + 1];
        let mut primes = Vec::with_capacity(2_200_000);

        mu[1] = 1;

        for i in 2..=n {
            if spf[i] == 0 {
                spf[i] = i as u32;
                mu[i] = -1;
                primes.push(i);
            }
            for &p in &primes {
                let ip = i * p;
                if ip > n { break; }
                spf[ip] = p as u32;
                if i % p == 0 {
                    mu[ip] = 0;
                    break;
                } else {
                    mu[ip] = -mu[i];
                }
            }
        }
        mu
    };

    // Precompute small[]
    let small: Vec<i64> = {
        let mut s = vec![0i64; (l + 2) as usize];
        for i in 1..=l {
            s[i as usize] = sum_floor_quotients(i);
        }
        s
    };

    // Precompute big[] with parallel computation.
    // big[i] = sum_floor_quotients(N/i^2).
    // For small i, use sum_floor_quotients_par (parallelizes within each call).
    // For large i, use outer parallelism across i values.
    let big: Vec<i64> = {
        let mut b = vec![0i64; (l + 2) as usize];
        let l_usize = l as usize;

        // Threshold: below this, N/i^2 is large enough for inner parallelism.
        // N/i^2 has sqrt(N/i^2) = sqrt(N)/i iterations.
        // Inner parallelism beneficial when sqrt(N)/i > ~100K, i.e., i < sqrt(N)/100K ~ 316.
        let inner_threshold = 3usize; // Only parallelize the heaviest few

        // Use std::thread to overlap inner-parallel big[1..3] with outer-parallel big[4..L]
        // Since rayon uses a global thread pool, inner parallel calls will use available threads.
        // Process big[1..inner_threshold] with inner parallelism
        for i in 1..=std::cmp::min(inner_threshold, l_usize) {
            b[i] = sum_floor_quotients_par(N / sq(i as i64));
        }

        // Process remaining with outer parallelism using load-balanced groups
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

            let results: Vec<Vec<(usize, i64)>> = groups.into_par_iter()
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

    // Main sum: parallelize over h values
    let ans: i64 = (1..=h_max).into_par_iter()
        .filter(|&h| {
            unsafe { *mobius.get_unchecked(h as usize) != 0 }
        })
        .map(|h| {
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
