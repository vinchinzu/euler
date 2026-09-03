// Project Euler 439: Sum of sum of divisors
// S(N) = sum_{i=1}^N sum_{j=1}^N sigma(i*j) mod 10^9.
//
// Optimized using:
// 1. Multiplicative sieve with compact u32 types and memory reuse.
// 2. Parallel prefix sums with Rayon chunks.
// 3. Parallel sigma_cache precomputation with u128 accumulation (no inner loop modulo).
// 4. Parallel n_mu_cache precomputation using dyadic layers with u128 accumulation.
// 5. Parallel chunked reduction for Part 1 using Rayon.
// 6. Fast closed-form range sum without division instructions.
// 7. Tuned sieve limit L (~6M) to fit within CPU L3 cache and minimize DRAM stalls.

use rayon::prelude::*;

const NN: i64 = 100_000_000_000;
const MOD: i64 = 1_000_000_000;
const MOD_U64: u64 = 1_000_000_000;

#[inline(always)]
fn modd(x: i64) -> i64 {
    let rem = x % MOD;
    if rem < 0 { rem + MOD } else { rem }
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

/// Compute sum_{i=a}^b i mod MOD without 64-bit hardware division
#[inline(always)]
fn sum_range(a: i64, b: i64) -> u64 {
    let cnt = (b - a + 1) as u64;
    let sum = (a + b) as u64;
    let prod = if cnt & 1 == 0 {
        ((cnt / 2) % MOD_U64) * (sum % MOD_U64)
    } else {
        (cnt % MOD_U64) * ((sum / 2) % MOD_U64)
    };
    prod % MOD_U64
}

/// Compute sigma_sum(n) = sum_{i=1}^{n} sigma(i) mod MOD using hyperbola method
fn compute_sigma_sum(n: i64) -> i64 {
    let sqrt_n = isqrt(n);
    let mut sum: u128 = 0;
    for d in 1..=sqrt_n {
        let q = (n / d) as u64 % MOD_U64;
        sum += (d as u64 * q) as u128;
    }
    for k in 1..=sqrt_n {
        let d_hi = n / k;
        if d_hi <= sqrt_n {
            break;
        }
        let d_lo = n / (k + 1) + 1;
        let range_sum = sum_range(d_lo, d_hi);
        sum += (range_sum * k as u64) as u128;
    }
    (sum % (MOD_U64 as u128)) as i64
}

fn solve() -> i64 {
    // Tuned sieve threshold: balances cache-friendly sieve against hyperbola cache size
    let l = 6_000_000;
    let sqrt_n = isqrt(NN) as usize;

    // Linear sieve: compute mobius and sigma using Euler's linear sieve with u32
    let mut mobius = vec![0i8; l + 1];
    let mut sigma = vec![0u32; l + 1];
    let mut ppow = vec![0u32; l + 1];
    let mut sigma_pp = vec![0u32; l + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(l / 12);

    mobius[1] = 1;
    sigma[1] = 1;

    for i in 2..=l {
        let i_u32 = i as u32;
        if ppow[i] == 0 {
            primes.push(i_u32);
            mobius[i] = -1;
            sigma[i] = i_u32 + 1;
            ppow[i] = i_u32;
            sigma_pp[i] = i_u32 + 1;
        }
        let cur_ppow = ppow[i];
        let cur_sigma = sigma[i];
        let cur_sigma_pp = sigma_pp[i];
        let cur_mob = mobius[i];

        for &p in &primes {
            let ip = i * p as usize;
            if ip > l {
                break;
            }
            if i % (p as usize) == 0 {
                let new_ppow = cur_ppow * p;
                ppow[ip] = new_ppow;
                let new_sigma_pp = cur_sigma_pp + new_ppow;
                sigma_pp[ip] = new_sigma_pp;
                if cur_ppow == i_u32 {
                    sigma[ip] = new_sigma_pp;
                } else {
                    sigma[ip] = (cur_sigma / cur_sigma_pp) * new_sigma_pp;
                }
                mobius[ip] = 0;
                break;
            } else {
                ppow[ip] = p;
                sigma_pp[ip] = p + 1;
                sigma[ip] = cur_sigma * (p + 1);
                mobius[ip] = -cur_mob;
            }
        }
    }

    // Parallel in-place prefix sums for sigma
    let num_chunks = 64;
    let chunk_size = (l + num_chunks) / num_chunks;

    let sigma_block_sums: Vec<u32> = sigma[1..]
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut s: u64 = 0;
            for &x in chunk {
                s += x as u64;
            }
            (s % MOD_U64) as u32
        })
        .collect();

    let mut sigma_offsets = vec![0u32; sigma_block_sums.len()];
    let mut cur: u64 = 0;
    for i in 0..sigma_block_sums.len() {
        sigma_offsets[i] = cur as u32;
        cur = (cur + sigma_block_sums[i] as u64) % MOD_U64;
    }

    sigma[1..]
        .par_chunks_mut(chunk_size)
        .zip(sigma_offsets.into_par_iter())
        .for_each(|(chunk, offset)| {
            let mut cur = offset as u64;
            for x in chunk {
                cur += *x as u64;
                if cur >= MOD_U64 {
                    cur -= MOD_U64;
                }
                *x = cur as u32;
            }
        });

    let sigma_prefix = sigma;
    drop(sigma_pp);

    // Reuse ppow memory for n_mu_prefix
    let mut n_mu_prefix = ppow;
    n_mu_prefix[0] = 0;

    let n_mu_block_sums: Vec<i64> = mobius[1..]
        .par_chunks(chunk_size)
        .enumerate()
        .map(|(chunk_idx, chunk)| {
            let start_i = chunk_idx * chunk_size + 1;
            let mut s: i64 = 0;
            for (idx, &m) in chunk.iter().enumerate() {
                if m == 1 {
                    s += (start_i + idx) as i64;
                } else if m == -1 {
                    s -= (start_i + idx) as i64;
                }
            }
            s
        })
        .collect();

    let mut n_mu_offsets = vec![0i64; n_mu_block_sums.len()];
    let mut cur_mu: i64 = 0;
    for i in 0..n_mu_block_sums.len() {
        n_mu_offsets[i] = cur_mu;
        cur_mu = modd(cur_mu + n_mu_block_sums[i]);
    }

    n_mu_prefix[1..]
        .par_chunks_mut(chunk_size)
        .enumerate()
        .zip(n_mu_offsets.into_par_iter())
        .for_each(|((chunk_idx, chunk), offset)| {
            let start_i = chunk_idx * chunk_size + 1;
            let mut cur = offset;
            for (idx, x) in chunk.iter_mut().enumerate() {
                let i = start_i + idx;
                let m = unsafe { *mobius.get_unchecked(i) };
                if m == 1 {
                    cur += i as i64;
                    if cur >= MOD {
                        cur -= MOD;
                    }
                } else if m == -1 {
                    cur -= i as i64;
                    if cur < 0 {
                        cur += MOD;
                    }
                }
                *x = cur as u32;
            }
        });

    let cache_size = sqrt_n + 2;
    let max_g = (NN / (l as i64 + 1)) as usize;

    // Precompute sigma_sum for large quotient values in parallel
    let sigma_cache: Vec<i64> = {
        let indices: Vec<usize> = (1..=max_g).collect();
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

    // Precompute n_mu_sum using dyadic layers in parallel
    let mut n_mu_cache = vec![0i64; cache_size];
    let mut cur_hi = max_g;
    while cur_hi >= 1 {
        let cur_lo = (cur_hi / 2) + 1;
        let layer_results: Vec<(usize, i64)> = (cur_lo..=cur_hi)
            .into_par_iter()
            .map(|g| {
                let v = NN / g as i64;
                let sv = isqrt(v);
                let mut neg: u128 = 0;

                for d in 2..=sv {
                    let q = v / d;
                    let sub = if q <= l as i64 {
                        unsafe { *n_mu_prefix.get_unchecked(q as usize) as u64 }
                    } else {
                        // Safe: NN / q >= 2*g > cur_hi, which has already been computed in previous layers
                        unsafe { *n_mu_cache.get_unchecked((NN / q) as usize) as u64 }
                    };
                    neg += (sub * d as u64) as u128;
                }
                for k in 1..=sv {
                    let d_hi = v / k;
                    if d_hi <= sv {
                        break;
                    }
                    let d_lo = v / (k + 1);
                    let sub = unsafe { *n_mu_prefix.get_unchecked(k as usize) as u64 };
                    let range_sum = sum_range(d_lo + 1, d_hi);
                    neg += (sub * range_sum) as u128;
                }
                let rem = (neg % (MOD_U64 as u128)) as i64;
                let result = if rem <= 1 { 1 - rem } else { 1 - rem + MOD };
                (g, result)
            })
            .collect();

        for (g, res) in layer_results {
            n_mu_cache[g] = res;
        }
        cur_hi = cur_lo - 1;
    }

    // Part 1: parallel reduction over g = 1..l with Rayon chunks
    let chunk_size = 65536;
    let ans_part1: i64 = mobius[1..]
        .par_chunks(chunk_size)
        .enumerate()
        .map(|(chunk_idx, chunk)| {
            let mut local_ans: i64 = 0;
            let start_g = chunk_idx * chunk_size + 1;
            for (idx, &m) in chunk.iter().enumerate() {
                if m != 0 {
                    let g = start_g + idx;
                    let q = NN / g as i64;
                    let ss = if q <= l as i64 {
                        unsafe { *sigma_prefix.get_unchecked(q as usize) as i64 }
                    } else {
                        unsafe { *sigma_cache.get_unchecked((NN / q) as usize) }
                    };
                    let term = ((g as u64 % MOD_U64) * ss as u64 % MOD_U64 * ss as u64) % MOD_U64;
                    if m == 1 {
                        local_ans += term as i64;
                        if local_ans >= MOD {
                            local_ans -= MOD;
                        }
                    } else {
                        local_ans -= term as i64;
                        if local_ans < 0 {
                            local_ans += MOD;
                        }
                    }
                }
            }
            local_ans
        })
        .reduce(|| 0i64, |a, b| (a + b) % MOD);

    // Part 2: quotient values (g > l)
    let mut ans_part2: i64 = 0;
    let mut q = 1i64;
    while q <= max_g as i64 {
        let g_hi = NN / q;
        let mut g_lo = NN / (q + 1);
        if g_lo < l as i64 {
            g_lo = l as i64;
        }
        if g_hi > g_lo {
            let ss = if q <= l as i64 {
                unsafe { *sigma_prefix.get_unchecked(q as usize) as i64 }
            } else {
                unsafe { *sigma_cache.get_unchecked((NN / q) as usize) }
            };
            let mu_hi = if g_hi <= l as i64 {
                unsafe { *n_mu_prefix.get_unchecked(g_hi as usize) as i64 }
            } else {
                unsafe { *n_mu_cache.get_unchecked((NN / g_hi) as usize) }
            };
            let mu_lo = if g_lo <= l as i64 {
                unsafe { *n_mu_prefix.get_unchecked(g_lo as usize) as i64 }
            } else {
                unsafe { *n_mu_cache.get_unchecked((NN / g_lo) as usize) }
            };
            let mu_diff = modd(mu_hi - mu_lo);
            let term = modd(mu_diff * ss % MOD * ss % MOD);
            ans_part2 = modd(ans_part2 + term);
        }
        q += 1;
    }

    modd(ans_part1 + ans_part2)
}

fn main() {
    let ans = solve();
    println!("{}", ans);
}
