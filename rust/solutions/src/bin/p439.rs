// Project Euler 439: Sum of sum of divisors
// S(N) = sum_{i=1}^N sum_{j=1}^N sigma(i*j) mod 10^9.
// Uses Mobius function, hyperbola method, and precomputed sums.
// Uses L = N^{2/3} sieve limit to minimize expensive n_mu_sum recursion.
// Linear multiplicative sieve for sigma and mobius.
// Parallelized sigma_sum precomputation with rayon.

use rayon::prelude::*;

const NN: i64 = 100_000_000_000;
const MOD: i64 = 1_000_000_000;

#[inline(always)]
fn modd(x: i64) -> i64 {
    ((x % MOD) + MOD) % MOD
}

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

#[inline(always)]
fn tr(n: i64) -> i64 {
    let n = ((n % (2 * MOD)) + 2 * MOD) % (2 * MOD);
    n * (n + 1) / 2 % MOD
}

#[inline(always)]
fn sum_range(a: i64, b: i64) -> i64 {
    modd(tr(b) - tr(a - 1))
}

/// Compute sigma_sum(n) = sum_{i=1}^{n} sigma(i) mod MOD using hyperbola method
fn compute_sigma_sum(n: i64) -> i64 {
    let sqrt_n = isqrt(n);
    let mut result: i64 = 0;
    for d in 1..=sqrt_n {
        result = modd(result + (d % MOD) * ((n / d) % MOD));
    }
    for k in 1..=sqrt_n {
        let d_hi = n / k;
        if d_hi > sqrt_n {
            let d_lo = n / (k + 1) + 1;
            result = modd(result + sum_range(d_lo, d_hi) * (k % MOD));
        }
    }
    result
}

/// Get n_mu_sum(v) from prefix or cache
#[inline(always)]
fn get_n_mu_sum(v: i64, l: usize, n_mu_prefix: &[i64], n_mu_cache: &[i64]) -> i64 {
    if v <= l as i64 {
        unsafe { *n_mu_prefix.get_unchecked(v as usize) }
    } else {
        unsafe { *n_mu_cache.get_unchecked((NN / v) as usize) }
    }
}

/// Get sigma_sum(v) from prefix or cache
#[inline(always)]
fn get_sigma_sum(v: i64, l: usize, sigma_prefix: &[i64], sigma_cache: &[i64]) -> i64 {
    if v <= l as i64 {
        unsafe { *sigma_prefix.get_unchecked(v as usize) }
    } else {
        unsafe { *sigma_cache.get_unchecked((NN / v) as usize) }
    }
}

fn main() {
    let l = (NN as f64).powf(2.0 / 3.0) as usize + 100;
    let sqrt_n = isqrt(NN) as usize;

    // Linear sieve: compute mobius, sigma, using Euler's sieve.
    // For sigma, track the prime-power part: ppow[n] = p^k where p^k || n (p = spf[n]).
    // sigma_pp[n] = sigma(ppow[n]) = 1 + p + ... + p^k.
    // sigma[n] = sigma[n/ppow[n]] * sigma_pp[n] (multiplicativity).
    let mut mobius = vec![0i8; l + 1];
    let mut sigma = vec![0i64; l + 1];
    let mut spf = vec![0u32; l + 1];
    let mut ppow = vec![0i64; l + 1];
    let mut sigma_pp = vec![0i64; l + 1];
    let mut primes: Vec<usize> = Vec::with_capacity(l / 10);

    mobius[1] = 1;
    sigma[1] = 1;

    for i in 2..=l {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i);
            mobius[i] = -1;
            sigma[i] = (i + 1) as i64;
            ppow[i] = i as i64;
            sigma_pp[i] = (i + 1) as i64;
        }
        for &p in &primes {
            let ip = i * p;
            if ip > l {
                break;
            }
            spf[ip] = p as u32;
            if i % p == 0 {
                let new_ppow = ppow[i] * p as i64;
                ppow[ip] = new_ppow;
                sigma_pp[ip] = sigma_pp[i] + new_ppow;
                if ppow[i] == i as i64 {
                    sigma[ip] = sigma_pp[ip];
                } else {
                    sigma[ip] = (sigma[i] / sigma_pp[i]) * sigma_pp[ip];
                }
                mobius[ip] = 0;
                break;
            } else {
                ppow[ip] = p as i64;
                sigma_pp[ip] = (p + 1) as i64;
                sigma[ip] = sigma[i] * (p + 1) as i64;
                mobius[ip] = -mobius[i];
            }
        }
    }

    // Compute sigma and n*mu prefix sums
    let sigma_prefix = {
        let mut prefix = vec![0i64; l + 1];
        for i in 1..=l {
            prefix[i] = modd(prefix[i - 1] + sigma[i]);
        }
        prefix
    };

    let n_mu_prefix = {
        let mut prefix = vec![0i64; l + 1];
        for i in 1..=l {
            prefix[i] = modd(prefix[i - 1] + i as i64 * mobius[i] as i64);
        }
        prefix
    };

    // Free large sieve arrays
    drop(sigma);
    drop(ppow);
    drop(sigma_pp);
    drop(spf);

    let cache_size = sqrt_n + 2;
    let max_g = NN / (l as i64 + 1);

    // Precompute sigma_sum for large quotient values (parallel)
    let sigma_cache: Vec<i64> = {
        let indices: Vec<usize> = (1..=max_g as usize).collect();
        let results: Vec<(usize, i64)> = indices
            .par_iter()
            .map(|&g| (g, compute_sigma_sum(NN / g as i64)))
            .collect();
        let mut cache = vec![0i64; cache_size];
        for (g, val) in results {
            cache[g] = val;
        }
        cache
    };

    // Precompute n_mu_sum bottom-up (sequential)
    let n_mu_cache = {
        let mut cache = vec![0i64; cache_size];
        for g in (1..=max_g).rev() {
            let v = NN / g;
            let sv = isqrt(v);
            let mut result: i64 = 1;

            for d in 2..=sv {
                let sub = get_n_mu_sum(v / d, l, &n_mu_prefix, &cache);
                result = modd(result - sub * (d % MOD));
            }
            for k in 1..=sv {
                let d_hi = v / k;
                let d_lo = v / (k + 1);
                if d_hi > sv && d_lo >= sv {
                    let sub = get_n_mu_sum(k, l, &n_mu_prefix, &cache);
                    result = modd(result - sub * sum_range(d_lo + 1, d_hi));
                }
            }

            cache[g as usize] = result;
        }
        cache
    };

    let mut ans: i64 = 0;

    // Part 1: g = 1..l (direct)
    for g in 1..=l {
        if mobius[g] != 0 {
            let ss = get_sigma_sum(NN / g as i64, l, &sigma_prefix, &sigma_cache);
            let term = modd(mobius[g] as i64 * (g as i64 % MOD) % MOD * ss % MOD * ss % MOD);
            ans = modd(ans + term);
        }
    }

    // Part 2: quotient values (g > l)
    let mut q = 1i64;
    while q <= max_g {
        let g_hi = NN / q;
        let mut g_lo = NN / (q + 1);
        if g_lo < l as i64 {
            g_lo = l as i64;
        }
        if g_hi > g_lo {
            let ss = get_sigma_sum(q, l, &sigma_prefix, &sigma_cache);
            let mu_diff = modd(
                get_n_mu_sum(g_hi, l, &n_mu_prefix, &n_mu_cache)
                    - get_n_mu_sum(g_lo, l, &n_mu_prefix, &n_mu_cache),
            );
            let term = modd(mu_diff * ss % MOD * ss % MOD);
            ans = modd(ans + term);
        }
        q += 1;
    }

    println!("{}", modd(ans));
}
