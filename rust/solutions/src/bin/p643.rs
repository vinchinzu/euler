// Project Euler 643 - 2-Friendly
// Count pairs 1 <= p < q <= N with gcd(p,q) a power of 2.
// Evaluates sum_{t >= 1} (S(N / 2^t) - 1) mod MOD where S(m) = sum_{k=1}^m phi(k) mod MOD.

use rayon::prelude::*;

const MOD: i64 = 1_000_000_007;
const MOD_U32: u32 = 1_000_000_007;

fn main() {
    let n_val: i64 = 100_000_000_000; // 10^11
    let v_size: usize = 5_000_000;

    let mut small = vec![0u32; v_size + 1];

    // Precompute small primes up to sqrt(v_size)
    let sqrt_v = (v_size as f64).sqrt() as usize;
    let mut is_p = vec![true; sqrt_v + 1];
    let mut primes_small = Vec::new();
    for i in 2..=sqrt_v {
        if is_p[i] {
            primes_small.push(i as u32);
            let mut j = i * i;
            while j <= sqrt_v {
                is_p[j] = false;
                j += i;
            }
        }
    }

    // Parallel block sieve for phi
    const CHUNK_SIZE: usize = 65536;
    let num_chunks = (v_size + CHUNK_SIZE - 1) / CHUNK_SIZE;
    let mut chunk_sums = vec![0u32; num_chunks];

    small[1..=v_size]
        .par_chunks_mut(CHUNK_SIZE)
        .zip(chunk_sums.par_iter_mut())
        .enumerate()
        .for_each(|(chunk_idx, (chunk, chunk_sum))| {
            let base = 1 + chunk_idx * CHUNK_SIZE;
            let len = chunk.len();
            let mut rem = vec![0u32; len];
            for i in 0..len {
                chunk[i] = (base + i) as u32;
                rem[i] = (base + i) as u32;
            }

            // Handle p = 2 separately using bit shifts and trailing zeros
            let start2 = if base % 2 == 0 { 0 } else { 1 };
            let mut j = start2;
            while j < len {
                chunk[j] >>= 1;
                rem[j] >>= rem[j].trailing_zeros();
                j += 2;
            }

            for &p in &primes_small[1..] {
                let pu = p as usize;
                let start = if base % pu == 0 { 0 } else { pu - (base % pu) };
                let mut j = start;
                while j < len {
                    chunk[j] -= chunk[j] / p;
                    let mut r = rem[j] / p;
                    while r % p == 0 {
                        r /= p;
                    }
                    rem[j] = r;
                    j += pu;
                }
            }

            let mut acc = 0u32;
            for i in 0..len {
                if rem[i] > 1 {
                    chunk[i] -= chunk[i] / rem[i];
                }
                acc += chunk[i];
                if acc >= MOD_U32 {
                    acc -= MOD_U32;
                }
                chunk[i] = acc;
            }
            *chunk_sum = acc;
        });

    // Parallel prefix sum across chunks
    let mut base_offsets = vec![0u32; num_chunks];
    let mut curr_base = 0u32;
    for c in 0..num_chunks {
        base_offsets[c] = curr_base;
        curr_base += chunk_sums[c];
        if curr_base >= MOD_U32 {
            curr_base -= MOD_U32;
        }
    }

    small[1..=v_size]
        .par_chunks_mut(CHUNK_SIZE)
        .zip(base_offsets.par_iter())
        .for_each(|(chunk, &offset)| {
            if offset > 0 {
                for x in chunk.iter_mut() {
                    *x += offset;
                    if *x >= MOD_U32 {
                        *x -= MOD_U32;
                    }
                }
            }
        });

    const INV2: i64 = 500_000_004;

    // Flat array for large[i] = S(N/i) for even i <= limit
    let limit = (n_val / (v_size as i64 + 1)) as usize + 1;
    let mut large = vec![0u32; limit + 1];

    let mut layer_high = limit;
    while layer_high >= 2 {
        let layer_low = (layer_high / 2) + 1;
        let results: Vec<(usize, u32)> = (layer_low..=layer_high)
            .into_par_iter()
            .filter(|&i| i % 2 == 0)
            .filter_map(|i| {
                let m = n_val / i as i64;
                if m <= v_size as i64 {
                    return None;
                }
                let mut result = (m % MOD) * ((m + 1) % MOD) % MOD * INV2 % MOD;

                let sqrt_m = (m as f64).sqrt() as i64;
                let d_threshold = m / (sqrt_m + 1);
                let mut d: i64 = 2;
                let mut sum_terms: u128 = 0;
                while d <= d_threshold {
                    let q = m / d;
                    let d_max = m / q;
                    let s_q = if q <= v_size as i64 {
                        small[q as usize] as u64
                    } else {
                        let idx = (n_val / q) as usize;
                        large[idx] as u64
                    };
                    sum_terms += (d_max - d + 1) as u128 * s_q as u128;
                    d = d_max + 1;
                }

                let mut prev_d = m;
                let max_q = (m / (d - 1)) as usize;
                for q in 1..max_q {
                    let next_d = m / (q as i64 + 1);
                    sum_terms += (prev_d - next_d) as u128 * small[q] as u128;
                    prev_d = next_d;
                }
                if max_q >= 1 {
                    sum_terms += (prev_d - (d - 1)) as u128 * small[max_q] as u128;
                }

                result = (result - (sum_terms % MOD as u128) as i64).rem_euclid(MOD);

                Some((i, (result % MOD) as u32))
            })
            .collect();

        for (i, val) in results {
            large[i] = val;
        }

        layer_high = layer_high / 2;
    }

    // S(m) lookup
    let get_s = |m: i64| -> i64 {
        if m <= v_size as i64 {
            small[m as usize] as i64
        } else {
            let idx = (n_val / m) as usize;
            large[idx] as i64
        }
    };

    // Compute the answer
    let mut ans = 0i64;
    let mut t = 1u32;
    while (1i64 << t) <= n_val {
        let lim = n_val >> t;
        let s_lim = get_s(lim);
        ans = (ans + s_lim - 1 + MOD) % MOD;
        t += 1;
    }

    println!("{}", ans % MOD);
}
