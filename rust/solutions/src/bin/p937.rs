// Project Euler Problem 937 - Equiproduct Partition
//
// Key result: k! is in A iff parity is even, where
//   parity = TM(v_2(k!)) + sum_{inert prime q <= k} TM(v_q(k!))  (mod 2)
// TM(v) = popcount(v) mod 2. Inert primes: p % 8 in {5, 7}.
//
// Optimizations:
// 1. Parallel odd-only segmented sieve (16 KB L1d cache chunks).
// 2. Primes p > N/3: directly mark diff[p] ^= 1 during segmented sieve (zero conflicts, single multiple).
// 3. p=2 and Small primes (p <= 10000): parallel disjoint chunking over [1..N] (100% lock-free, zero atomics).
// 4. Valuation tracking via Legendre's formula and tzcnt (no buffer allocation, fast parity check).
// 5. Medium primes (p in (10000, N/3]): parallel across primes with dynamic load balancing, unrolled small multiples.
// 6. Parallel chunked scan: independent factorial prefix products and parity sums across chunks.

use rayon::prelude::*;

fn main() {
    const MOD: u64 = 1_000_000_007;
    const N: usize = 100_000_000;

    let mut diff = vec![0u8; N + 2];

    #[derive(Copy, Clone)]
    struct UnsafeDiff(*mut u8);
    unsafe impl Sync for UnsafeDiff {}
    unsafe impl Send for UnsafeDiff {}
    impl UnsafeDiff {
        #[inline(always)]
        unsafe fn xor(&self, idx: usize) {
            unsafe { *self.0.add(idx) ^= 1; }
        }
    }
    let raw_diff = UnsafeDiff(diff.as_mut_ptr());

    // 1. Base primes up to 10000
    let mut is_prime_base = [true; 10001];
    is_prime_base[0] = false;
    is_prime_base[1] = false;
    for i in 2..=100 {
        if is_prime_base[i] {
            let mut j = i * i;
            while j <= 10000 {
                is_prime_base[j] = false;
                j += i;
            }
        }
    }
    let mut base_primes = Vec::with_capacity(1250);
    for i in 3..=10000 {
        if is_prime_base[i] && (i & 1) != 0 {
            base_primes.push(i);
        }
    }

    // 2. Parallel segmented odd-only sieve
    let total_odds = (N + 1) / 2;
    const CHUNK_ODDS: usize = 131_072; // 16 KB
    let num_chunks = (total_odds + CHUNK_ODDS - 1) / CHUNK_ODDS;

    let (small_primes, medium_primes): (Vec<usize>, Vec<usize>) = {
        let results: Vec<(Vec<usize>, Vec<usize>)> = (0..num_chunks).into_par_iter().map(|c_idx| {
            let chunk_start = c_idx * CHUNK_ODDS;
            let chunk_end = ((c_idx + 1) * CHUNK_ODDS).min(total_odds);
            let chunk_len = chunk_end - chunk_start;

            let mut chunk_sieve = vec![0u8; (chunk_len + 7) / 8];
            for &p in &base_primes {
                let i0 = (p * p - 1) / 2;
                if i0 >= chunk_end {
                    break;
                }
                let start_idx = if i0 >= chunk_start {
                    i0
                } else {
                    let low_val = 2 * chunk_start + 1;
                    let rem = low_val % p;
                    let next_mult = if rem == 0 { low_val } else { low_val + (p - rem) };
                    let odd_mult = if next_mult % 2 == 0 { next_mult + p } else { next_mult };
                    (odd_mult - 1) / 2
                };
                let mut cur = start_idx - chunk_start;
                while cur < chunk_len {
                    chunk_sieve[cur >> 3] |= 1 << (cur & 7);
                    cur += p;
                }
            }

            let mut loc_small = Vec::new();
            let mut loc_med = Vec::new();

            for b in 0..chunk_sieve.len() {
                let byte = chunk_sieve[b];
                if byte == 0xFF {
                    continue;
                }
                for bit in 0..8 {
                    if byte & (1 << bit) == 0 {
                        let idx = chunk_start + b * 8 + bit;
                        if idx == 0 || idx >= total_odds {
                            continue;
                        }
                        let p = 2 * idx + 1;
                        if p > N {
                            break;
                        }
                        if (idx & 2) != 0 {
                            // p % 8 in {5, 7}
                            if p > N / 3 {
                                unsafe { raw_diff.xor(p); }
                            } else if p <= 10000 {
                                loc_small.push(p);
                            } else {
                                loc_med.push(p);
                            }
                        }
                    }
                }
            }

            (loc_small, loc_med)
        }).collect();

        let mut all_small = Vec::new();
        let mut all_med = Vec::new();
        for (s, m) in results {
            all_small.extend(s);
            all_med.extend(m);
        }
        (all_small, all_med)
    };

    // Process p=2 and small primes in parallel disjoint chunks (100% lock-free, zero atomics)
    let num_diff_chunks = 128;
    let diff_chunk_size = (N + num_diff_chunks - 1) / num_diff_chunks;

    (0..num_diff_chunks).into_par_iter().for_each(|c_idx| {
        let chunk_start = c_idx * diff_chunk_size + 1;
        let chunk_end = ((c_idx + 1) * diff_chunk_size).min(N);
        if chunk_start > chunk_end {
            return;
        }

        // 1. Process p=2 in this chunk
        let even_start = if chunk_start % 2 == 0 { chunk_start } else { chunk_start + 1 };
        if even_start <= chunk_end {
            let mut v = (even_start - 2) - (even_start - 2).count_ones() as usize;
            let mut m = even_start;
            while m <= chunk_end {
                let e = (m as u32).trailing_zeros() as usize;
                let old_tm = v.count_ones() & 1;
                v += e;
                let new_tm = v.count_ones() & 1;
                if old_tm != new_tm {
                    unsafe { raw_diff.xor(m); }
                }
                m += 2;
            }
        }

        // 2. Process all small primes in this chunk
        for &p in &small_primes {
            let num_mult = N / p;
            let k_start = (chunk_start + p - 1) / p;
            let k_end = (chunk_end / p).min(num_mult);
            if k_start > k_end {
                continue;
            }

            let prev = k_start - 1;
            let mut vp_prev = 0;
            let mut temp = prev;
            while temp > 0 {
                vp_prev += temp / p;
                temp /= p;
            }
            let mut v = (prev + vp_prev) as u32;
            let mut rem = prev % p;
            let mut m = k_start * p;

            for k in k_start..=k_end {
                rem += 1;
                if rem == p {
                    rem = 0;
                    let mut temp = k / p;
                    let mut e = 2;
                    while temp % p == 0 {
                        e += 1;
                        temp /= p;
                    }
                    let old_tm = v.count_ones() & 1;
                    v += e;
                    let new_tm = v.count_ones() & 1;
                    if old_tm != new_tm {
                        unsafe { raw_diff.xor(m); }
                    }
                } else {
                    v += 1;
                    if (v.trailing_zeros() & 1) == 0 {
                        unsafe { raw_diff.xor(m); }
                    }
                }
                m += p;
            }
        }
    });

    // Process medium primes in parallel without atomics, with dynamic load balancing
    medium_primes.par_iter().with_max_len(64).for_each(|&p| {
        if p > N / 4 {
            unsafe {
                raw_diff.xor(p);
                raw_diff.xor(3 * p);
            }
        } else if p > N / 5 {
            unsafe {
                raw_diff.xor(p);
                raw_diff.xor(3 * p);
                raw_diff.xor(4 * p);
            }
        } else if p > N / 7 {
            unsafe {
                raw_diff.xor(p);
                raw_diff.xor(3 * p);
                raw_diff.xor(4 * p);
                raw_diff.xor(5 * p);
            }
        } else {
            let num_mult = N / p;
            let mut m = p;
            for c in 1..=num_mult {
                if (c.trailing_zeros() & 1) == 0 {
                    unsafe { raw_diff.xor(m); }
                }
                m += p;
            }
        }
    });

    // Parallel chunked scan
    let num_chunks = 64;
    let chunk_size = (N + num_chunks - 1) / num_chunks;
    struct ChunkResult {
        s0: u64,
        s1: u64,
        f_end: u64,
        d_end: u64,
    }

    let chunks: Vec<ChunkResult> = (0..num_chunks).into_par_iter().map(|chunk_idx| {
        let start = chunk_idx * chunk_size + 1;
        let end = ((chunk_idx + 1) * chunk_size).min(N);
        if start > end {
            return ChunkResult { s0: 0, s1: 0, f_end: 1, d_end: 0 };
        }

        let mut f: u64 = 1;
        let mut d: u64 = 0;
        let mut s0: u64 = 0;
        let mut s1: u64 = 0;

        for k in start..=end {
            f = (f * (k as u64)) % MOD;
            d ^= unsafe { *diff.get_unchecked(k) } as u64;
            if d == 0 {
                s0 += f;
                if s0 >= MOD * 8 {
                    s0 %= MOD;
                }
            } else {
                s1 += f;
                if s1 >= MOD * 8 {
                    s1 %= MOD;
                }
            }
        }

        ChunkResult {
            s0: s0 % MOD,
            s1: s1 % MOD,
            f_end: f,
            d_end: d,
        }
    }).collect();

    let mut parity: u64 = 0;
    let mut factorial: u64 = 1;
    let mut total_sum: u64 = 0;

    for chunk in chunks {
        let chunk_s = if parity == 0 { chunk.s0 } else { chunk.s1 };
        total_sum = (total_sum + factorial * chunk_s) % MOD;
        factorial = (factorial * chunk.f_end) % MOD;
        parity ^= chunk.d_end;
    }

    println!("{}", total_sum);
}
