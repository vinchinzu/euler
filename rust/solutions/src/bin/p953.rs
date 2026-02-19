// Project Euler Problem 953
// Factorisation Nim: S(10^14) mod 10^9+7
// Optimized: no i128, rayon on DFS phase, unchecked hot paths.

use rayon::prelude::*;

const N_VAL: i64 = 100_000_000_000_000;
const MOD: i64 = 1_000_000_007;
const INV6: i64 = 166_666_668; // modular inverse of 6 mod MOD
const LIMIT_PRIME: usize = 22_000_000;
const SMALL_M_LIMIT: usize = 100_000;

#[inline(always)]
fn s2_contribution(k: i64) -> i64 {
    let quot = N_VAL / k;
    let mut m = (quot as f64).sqrt() as i64;
    while (m + 1) * (m + 1) <= quot {
        m += 1;
    }
    while m * m > quot {
        m -= 1;
    }
    if m == 0 {
        return 0;
    }
    let mm = m % MOD;
    let s2 = mm * (mm + 1) % MOD * ((2 * mm + 1) % MOD) % MOD * INV6 % MOD;
    k % MOD * s2 % MOD
}

#[inline(always)]
fn is_prime_td(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut d = 5i64;
    while d * d <= n {
        if n % d == 0 || n % (d + 2) == 0 {
            return false;
        }
        d += 6;
    }
    true
}

fn dfs(
    idx: i32,
    current_m: i64,
    current_g: i32,
    q: i32,
    limit_m: i64,
    primes_small: &[i32],
    is_prime_sieve: &[bool],
) -> i64 {
    let mut local_sum: i64 = 0;
    let p = current_g ^ q;

    if p > q {
        let is_p = if (p as usize) <= LIMIT_PRIME {
            // SAFETY: p <= LIMIT_PRIME, sieve has LIMIT_PRIME+1 entries
            unsafe { *is_prime_sieve.get_unchecked(p as usize) }
        } else {
            is_prime_td(p as i64)
        };
        if is_p {
            // current_m <= limit_m = N/q^2, so cq = current_m*q <= N/q < N, fits i64
            let cq = current_m * q as i64;
            if p as i64 <= N_VAL / cq {
                local_sum = s2_contribution(cq * p as i64);
            }
        }
    }

    let mut i = idx;
    while i >= 0 {
        // SAFETY: i in 0..primes_small.len() guaranteed by caller
        let next_p = unsafe { *primes_small.get_unchecked(i as usize) };
        // current_m * next_p <= N/q < 10^14, fits i64
        let nm = current_m * next_p as i64;
        if nm <= limit_m {
            local_sum = (local_sum
                + dfs(
                    i - 1,
                    nm,
                    current_g ^ next_p,
                    q,
                    limit_m,
                    primes_small,
                    is_prime_sieve,
                ))
                % MOD;
        }
        i -= 1;
    }
    local_sum
}

fn main() {
    let mut is_prime_sieve = vec![true; LIMIT_PRIME + 1];
    is_prime_sieve[0] = false;
    is_prime_sieve[1] = false;
    {
        let mut i = 2;
        while i * i <= LIMIT_PRIME {
            if is_prime_sieve[i] {
                let mut j = i * i;
                while j <= LIMIT_PRIME {
                    is_prime_sieve[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let primes: Vec<i32> = (2..=LIMIT_PRIME)
        .filter(|&i| is_prime_sieve[i])
        .map(|i| i as i32)
        .collect();

    let mut lp = vec![0i32; SMALL_M_LIMIT + 1];
    let mut g_arr = vec![0i32; SMALL_M_LIMIT + 1];
    let mut max_p_arr = vec![0i32; SMALL_M_LIMIT + 1];
    let mut sq = vec![true; SMALL_M_LIMIT + 1];
    let mut pr: Vec<i32> = Vec::with_capacity(10000);

    for i in 2..=SMALL_M_LIMIT {
        if lp[i] == 0 {
            lp[i] = i as i32;
            pr.push(i as i32);
            g_arr[i] = i as i32;
            max_p_arr[i] = i as i32;
        }
        for pi in 0..pr.len() {
            let p = pr[pi];
            if p > lp[i] || (i as i64) * (p as i64) > SMALL_M_LIMIT as i64 {
                break;
            }
            let ip = i * p as usize;
            lp[ip] = p;
            max_p_arr[ip] = max_p_arr[i];
            if p == lp[i] {
                sq[ip] = false;
            } else {
                sq[ip] = sq[i];
            }
            g_arr[ip] = g_arr[i] ^ p;
        }
    }

    let mut total_sum: i64 = s2_contribution(1) % MOD;
    let max_q = ((N_VAL / 2) as f64).sqrt() as i64;

    // Split into DFS items (large limit_m) and direct iteration items (small limit_m)
    let mut dfs_items: Vec<(usize, i32)> = Vec::new();
    let mut direct_items: Vec<(usize, i32)> = Vec::new();

    for (qi, &q) in primes.iter().enumerate() {
        if (q as i64) > max_q {
            break;
        }
        let q_sq = q as i64 * q as i64;
        let limit_m = N_VAL / q_sq;
        if limit_m == 0 {
            break;
        }
        if limit_m <= SMALL_M_LIMIT as i64 {
            direct_items.push((qi, q));
        } else if 2 * q_sq <= N_VAL && qi > 0 {
            dfs_items.push((qi, q));
        }
    }

    // Phase 1: DFS items (parallel, heavy per-item)
    let dfs_sum: i64 = dfs_items
        .par_iter()
        .map(|&(qi, q)| {
            let limit_m = N_VAL / (q as i64 * q as i64);
            dfs(
                qi as i32 - 1,
                1,
                0,
                q,
                limit_m,
                &primes[..qi],
                &is_prime_sieve,
            ) % MOD
        })
        .reduce(|| 0i64, |a, b| (a + b) % MOD);
    total_sum = (total_sum + dfs_sum) % MOD;

    // Phase 2: Direct iteration items (parallel)
    let direct_sum: i64 = direct_items
        .par_iter()
        .map(|&(_qi, q)| {
            let q_i64 = q as i64;
            let limit_m = N_VAL / (q_i64 * q_i64);
            let mut local_sum: i64 = 0;

            for m in 2..=limit_m as usize {
                // SAFETY: m <= SMALL_M_LIMIT, arrays have SMALL_M_LIMIT+1 entries
                let sqm = unsafe { *sq.get_unchecked(m) };
                if !sqm {
                    continue;
                }
                let maxp = unsafe { *max_p_arr.get_unchecked(m) };
                if maxp >= q {
                    continue;
                }
                let p = unsafe { *g_arr.get_unchecked(m) } ^ q;
                if p <= q {
                    continue;
                }
                let is_p = if (p as usize) <= LIMIT_PRIME {
                    unsafe { *is_prime_sieve.get_unchecked(p as usize) }
                } else {
                    is_prime_td(p as i64)
                };
                if !is_p {
                    continue;
                }
                let mq = m as i64 * q_i64;
                if p as i64 <= N_VAL / mq {
                    local_sum = (local_sum + s2_contribution(mq * p as i64)) % MOD;
                }
            }

            local_sum % MOD
        })
        .reduce(|| 0i64, |a, b| (a + b) % MOD);

    total_sum = (total_sum + direct_sum) % MOD;

    println!("{}", total_sum);
}
