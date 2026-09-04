// Project Euler 423 - Consecutive die throws
// C(n) = number of outcomes of n die throws where consecutive identical pairs <= pi(n).

use rayon::prelude::*;

const N: usize = 50_000_000;
const K: u64 = 6;
const MOD: u64 = 1_000_000_007;
const BARRETT_M: u64 = 18446743944;

#[inline(always)]
fn fast_mod(x: u64) -> u64 {
    let q = ((x as u128 * BARRETT_M as u128) >> 64) as u64;
    let mut r = x - q * MOD;
    if r >= MOD {
        r -= MOD;
    }
    r
}

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    fast_mod(a * b)
}

#[inline(always)]
fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mul(res, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    res
}

const CHUNK_SIZE: usize = 32768;

fn fill_inverses_chunk(start: u32, chunk: &mut [u32]) {
    let count = chunk.len();
    if count == 0 {
        return;
    }
    let mut prod = start as u64;
    chunk[0] = prod as u32;
    for i in 1..count {
        prod = mul(prod, (start + i as u32) as u64);
        chunk[i] = prod as u32;
    }

    let mut inv = pow_mod(chunk[count - 1] as u64, MOD - 2);

    for i in (1..count).rev() {
        let prev = chunk[i - 1] as u64;
        chunk[i] = mul(inv, prev) as u32;
        inv = mul(inv, (start + i as u32) as u64);
    }
    chunk[0] = inv as u32;
}

fn main() {
    // 1. Parallel Segmented Sieve of odd numbers up to N
    const NUM_ODDS: usize = (N + 1) / 2;
    const NUM_WORDS: usize = (NUM_ODDS + 63) / 64;
    let limit = ((N as f64).sqrt() as usize - 1) / 2;

    // Small primes up to sqrt(N)
    let mut base_sieve = vec![false; limit + 1];
    let mut small_primes = Vec::new();
    for k in 1..=limit {
        if !base_sieve[k] {
            let p = 2 * k + 1;
            small_primes.push(p);
            let mut j = 2 * k * (k + 1);
            while j <= limit {
                base_sieve[j] = true;
                j += p;
            }
        }
    }

    const SIEVE_CHUNK_WORDS: usize = 4096; // 32 KB per chunk
    let mut sieve = vec![0u64; NUM_WORDS];
    sieve[0] = 1; // 1 is not prime (k = 0)

    sieve
        .par_chunks_mut(SIEVE_CHUNK_WORDS)
        .enumerate()
        .for_each(|(chunk_idx, chunk)| {
            let chunk_odd_start = chunk_idx * SIEVE_CHUNK_WORDS * 64;
            let chunk_odd_end = (chunk_odd_start + chunk.len() * 64).min(NUM_ODDS);
            for &p in &small_primes {
                let idx0 = (p * p - 1) / 2;
                if idx0 >= chunk_odd_end {
                    continue;
                }
                let start = if chunk_odd_start <= idx0 {
                    idx0
                } else {
                    let rem = (chunk_odd_start - idx0) % p;
                    if rem == 0 {
                        chunk_odd_start
                    } else {
                        chunk_odd_start + (p - rem)
                    }
                };
                let mut cur = start;
                while cur < chunk_odd_end {
                    let local = cur - chunk_odd_start;
                    chunk[local >> 6] |= 1u64 << (local & 63);
                    cur += p;
                }
            }
        });

    // 2. Precompute prime inverses in parallel: pi(50_000_000) = 3_001_134
    const NUM_PRIMES: usize = 3_001_134;
    let mut inv_pi = vec![0u32; NUM_PRIMES + 2];
    inv_pi[1..=NUM_PRIMES + 1]
        .par_chunks_mut(CHUNK_SIZE)
        .enumerate()
        .for_each(|(chunk_idx, chunk)| {
            let start = 1 + chunk_idx * CHUNK_SIZE;
            fill_inverses_chunk(start as u32, chunk);
        });

    // 3. Precompute composite inverses in parallel with Rayon
    const NUM_COMPOSITES: usize = N - 1 - NUM_PRIMES; // 46_998_865
    let mut inv_comp = vec![0u32; NUM_COMPOSITES + 1];
    inv_comp[1..]
        .par_chunks_mut(CHUNK_SIZE)
        .enumerate()
        .for_each(|(chunk_idx, chunk)| {
            let start = 1 + chunk_idx * CHUNK_SIZE;
            fill_inverses_chunk(start as u32, chunk);
        });

    // 4. Parallel Main loop across 64 chunks
    let mut primes_per_word = Vec::with_capacity(NUM_WORDS);
    primes_per_word.push(((!sieve[0]) & !1u64).count_ones() as usize);
    for w in 1..NUM_WORDS {
        primes_per_word.push((!sieve[w]).count_ones() as usize);
    }

    let num_chunks = 64;
    let words_per_chunk = (NUM_WORDS + num_chunks - 1) / num_chunks;
    let mut chunk_meta = Vec::with_capacity(num_chunks);
    let mut w_start = 0;
    let mut cur_pi = 1; // pi(2) = 1
    let mut cur_comp = 0; // comp(2) = 0

    for _ in 0..num_chunks {
        let w_end = (w_start + words_per_chunk).min(NUM_WORDS);
        if w_start >= w_end {
            break;
        }
        chunk_meta.push((w_start, w_end, cur_pi, cur_comp));
        for w in w_start..w_end {
            let p_cnt = primes_per_word[w];
            cur_pi += p_cnt;
            let total_in_w = if w == 0 { 63 * 2 } else { 64 * 2 };
            cur_comp += total_in_w - p_cnt;
        }
        w_start = w_end;
    }

    let chunk_results: Vec<(usize, u64, u64, u64)> = chunk_meta
        .into_par_iter()
        .map(|(w_start, w_end, pi_start, comp_start)| {
            let mut f: u64 = 0;
            let mut r: u64 = 1;
            let mut sum_f: u64 = 0;
            let mut pi_idx = pi_start + 1;
            let mut comp_idx = comp_start + 1;
            let mut l: usize = 0;

            for w in w_start..w_end {
                let word = sieve[w];
                let b_start = if w == 0 { 1 } else { 0 };
                for b in b_start..64 {
                    let n = ((w * 64 + b) * 2 + 1) as u64;
                    let is_prime = (word & (1u64 << b)) == 0;
                    if is_prime {
                        let ratio = mul(n - 1, unsafe { *inv_pi.get_unchecked(pi_idx) } as u64);
                        pi_idx += 1;
                        let r_new = mul(r, ratio);
                        f = fast_mod(6 * f + MOD - r + r_new);
                        r = r_new;
                    } else {
                        let inv = unsafe { *inv_comp.get_unchecked(comp_idx) } as u64;
                        comp_idx += 1;
                        let ratio = mul(5 * (n - 1), inv);
                        let r_new = mul(r, ratio);
                        f = fast_mod(6 * f + MOD - r);
                        r = r_new;
                    }
                    sum_f += f;

                    let inv = unsafe { *inv_comp.get_unchecked(comp_idx) } as u64;
                    comp_idx += 1;
                    let ratio = mul(5 * n, inv);
                    let r_new = mul(r, ratio);
                    f = fast_mod(6 * f + MOD - r);
                    r = r_new;
                    sum_f += f;

                    l += 2;
                }
            }
            (l, fast_mod(sum_f), f, r)
        })
        .collect();

    const INV5: u64 = 400_000_003;
    let mut state_f: u64 = 6;
    let mut state_r: u64 = 1;
    let mut total_sum_f: u64 = 7; // n=1: f=1; n=2: f=6

    for (l, alpha, beta, lambda_tot) in chunk_results {
        let pwr6_l = pow_mod(6, l as u64);
        let sigma = mul(fast_mod(6 * pwr6_l + MOD - 6), INV5);

        let chunk_s = fast_mod(mul(sigma, state_f) + mul(alpha, state_r));
        total_sum_f += chunk_s;

        let next_f = fast_mod(mul(pwr6_l, state_f) + mul(beta, state_r));
        let next_r = mul(lambda_tot, state_r);
        state_f = next_f;
        state_r = next_r;
    }

    let ans = mul(K, fast_mod(total_sum_f));
    println!("{}", ans);
}

