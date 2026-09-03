// Project Euler Problem 953
// Factorisation Nim: S(10^14) mod 10^9+7
// Optimized with parity pruning, leaf unrolling, contiguous valid_m table,
// fast odd-only bit sieve, and rayon nested work-stealing.

use rayon::prelude::*;

const N_VAL: i64 = 100_000_000_000_000;
const MOD: i64 = 1_000_000_007;
const INV6: i64 = 166_666_668; // modular inverse of 6 mod MOD
const LIMIT_PRIME: usize = 7_100_000;
const SMALL_M_LIMIT: usize = 72_000;

#[inline(always)]
fn s2_contribution(k: i64) -> i64 {
    let k_mod = k % MOD;
    let quot = (N_VAL / k) as u64;
    if quot < 4 {
        return k_mod;
    }
    if quot < 9 {
        return (k_mod * 5) % MOD;
    }
    if quot < 16 {
        return (k_mod * 14) % MOD;
    }
    let m = quot.isqrt() as i64;
    let mm = m % MOD;
    let s2 = (mm * (mm + 1) % MOD) * (2 * mm + 1) % MOD * INV6 % MOD;
    (k_mod * s2) % MOD
}

/// Odd-only bit-packed sieve. Bit i represents n = 2*i+1.
fn sieve_odd_bits(limit: usize) -> (Vec<u64>, Vec<i32>) {
    let n_bits = limit / 2 + 1;
    let n_words = n_bits.div_ceil(64);
    let mut words = vec![u64::MAX; n_words];
    words[0] &= !1u64; // 1 is not prime
    let rem = n_bits % 64;
    if rem != 0 {
        words[n_words - 1] &= (1u64 << rem) - 1;
    }

    let sqrt_limit = limit.isqrt();
    let mut i = 1usize;
    while 2 * i + 1 <= sqrt_limit {
        if (words[i >> 6] >> (i & 63)) & 1 != 0 {
            let p = 2 * i + 1;
            let mut j = p * p / 2;
            while j < n_bits {
                words[j >> 6] &= !(1u64 << (j & 63));
                j += p;
            }
        }
        i += 1;
    }

    let mut primes = Vec::with_capacity(n_bits / 5);
    primes.push(2);
    for i in 1..n_bits {
        if (words[i >> 6] >> (i & 63)) & 1 != 0 {
            let p = 2 * i + 1;
            if p <= limit {
                primes.push(p as i32);
            }
        }
    }
    (words, primes)
}

#[inline(always)]
fn is_prime_odd_sieve(p: usize, words: &[u64]) -> bool {
    let i = p >> 1;
    // SAFETY: p is odd and <= LIMIT_PRIME
    unsafe { (*words.get_unchecked(i >> 6) >> (i & 63)) & 1 != 0 }
}

fn dfs(
    start: usize,
    current_m: i64,
    current_g: i32,
    q: i32,
    limit_m: i64,
    primes_small: &[i32],
    words: &[u64],
) -> i64 {
    let mut local_sum: i64 = 0;

    if (current_g & 1) == 0 {
        let p = current_g ^ q;
        if p > q && is_prime_odd_sieve(p as usize, words) {
            let cq = current_m * q as i64;
            if p as i64 <= N_VAL / cq {
                local_sum = s2_contribution(cq * p as i64);
            }
        }
    }

    let nsp = primes_small.len();
    let mut i = start;
    let is_odd_g = (current_g & 1) != 0;

    while i < nsp {
        let next_p = unsafe { *primes_small.get_unchecked(i) };
        let nm = current_m * next_p as i64;
        if nm > limit_m {
            break;
        }

        if !is_odd_g {
            if next_p > 2 {
                if i + 1 >= nsp {
                    break;
                }
                let next_p2 = unsafe { *primes_small.get_unchecked(i + 1) };
                if nm > limit_m / next_p2 as i64 {
                    break;
                }
            }
            local_sum += dfs(
                i + 1,
                nm,
                current_g ^ next_p,
                q,
                limit_m,
                primes_small,
                words,
            );
        } else {
            let can_have_children = if i + 1 < nsp {
                let next_p2 = unsafe { *primes_small.get_unchecked(i + 1) };
                nm <= limit_m / next_p2 as i64
            } else {
                false
            };

            if can_have_children {
                local_sum += dfs(
                    i + 1,
                    nm,
                    current_g ^ next_p,
                    q,
                    limit_m,
                    primes_small,
                    words,
                );
            } else {
                // All remaining next_p in this loop are leaves
                let n_over_cq = N_VAL / (current_m * q as i64);
                while i < nsp {
                    let next_p_leaf = unsafe { *primes_small.get_unchecked(i) };
                    let nm_leaf = current_m * next_p_leaf as i64;
                    if nm_leaf > limit_m {
                        break;
                    }
                    let p = (current_g ^ next_p_leaf) ^ q;
                    if p > q && (p as i64) * next_p_leaf as i64 <= n_over_cq && is_prime_odd_sieve(p as usize, words) {
                        local_sum += s2_contribution(nm_leaf * q as i64 * p as i64);
                    }
                    i += 1;
                }
                break;
            }
        }
        i += 1;
    }
    local_sum % MOD
}

#[derive(Clone, Copy)]
struct ValidM {
    m: u32,
    g: i32,
    max_p: i32,
}

fn main() {
    let (sieve_words, primes) = sieve_odd_bits(LIMIT_PRIME);

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

    let mut valid_m_list: Vec<ValidM> = Vec::with_capacity(25000);
    let mut valid_m_prefix_len = vec![0u32; SMALL_M_LIMIT + 1];

    for m in 2..=SMALL_M_LIMIT {
        if sq[m] && (g_arr[m] & 1 == 0) {
            valid_m_list.push(ValidM {
                m: m as u32,
                g: g_arr[m],
                max_p: max_p_arr[m],
            });
        }
        valid_m_prefix_len[m] = valid_m_list.len() as u32;
    }

    let mut total_sum: i64 = s2_contribution(1) % MOD;
    let max_q = ((N_VAL / 2) as u64).isqrt() as i64;

    // Split into DFS items (large limit_m) and direct iteration items (small limit_m)
    let mut dfs_items: Vec<(usize, i32)> = Vec::with_capacity(4000);
    let mut direct_items: Vec<(usize, i32)> = Vec::with_capacity(500_000);

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

    let (dfs_sum, direct_sum) = rayon::join(
        || {
            dfs_items
                .par_iter()
                .map(|&(qi, q)| {
                    let limit_m = N_VAL / (q as i64 * q as i64);
                    dfs(0, 1, 0, q, limit_m, &primes[..qi], &sieve_words)
                })
                .reduce(|| 0i64, |a, b| (a + b) % MOD)
        },
        || {
            direct_items
                .par_chunks(1024)
                .map(|chunk| {
                    let mut chunk_sum: i64 = 0;
                    for &(_qi, q) in chunk {
                        let q_i64 = q as i64;
                        let limit_m = (N_VAL / (q_i64 * q_i64)) as usize;
                        let n_valid = unsafe { *valid_m_prefix_len.get_unchecked(limit_m) } as usize;
                        let entries = unsafe { valid_m_list.get_unchecked(..n_valid) };

                        let n_over_q = N_VAL / q_i64;
                        for entry in entries {
                            if entry.max_p >= q {
                                continue;
                            }
                            let p = entry.g ^ q;
                            if p <= q {
                                continue;
                            }
                            let p_i64 = p as i64;
                            if p_i64 * entry.m as i64 > n_over_q {
                                continue;
                            }
                            if !is_prime_odd_sieve(p as usize, &sieve_words) {
                                continue;
                            }
                            chunk_sum += s2_contribution(entry.m as i64 * q_i64 * p_i64);
                        }
                    }
                    chunk_sum % MOD
                })
                .reduce(|| 0i64, |a, b| (a + b) % MOD)
        },
    );

    total_sum = (total_sum + dfs_sum + direct_sum) % MOD;

    println!("{}", total_sum);
}
