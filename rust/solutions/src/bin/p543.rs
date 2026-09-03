// Project Euler 543 - Prime-Sum Numbers
//
// S(n) uses prime counting function pi(n) on Fibonacci numbers.
// Parallel segmented bit sieve up to F_44 = 701408733, then compute S(F_k) for k=3..44.

use rayon::prelude::*;

fn main() {
    let max_fib: usize = 701_408_733;

    let mut fibs = [0i64; 45];
    fibs[1] = 1;
    for i in 2..=44 {
        fibs[i] = fibs[i - 1] + fibs[i - 2];
    }

    let sqrt_limit = (max_fib as f64).sqrt() as usize + 1;
    let mut is_prime = vec![true; sqrt_limit + 1];
    let mut base_primes = Vec::new();
    for p in 2..=sqrt_limit {
        if is_prime[p] {
            if p >= 11 {
                base_primes.push(p);
            }
            let mut mult = p * p;
            while mult <= sqrt_limit {
                is_prime[mult] = false;
                mult += p;
            }
        }
    }

    // Precompute wheel pattern of 105 u64 words (6720 bits = 64 * 105)
    // where odd numbers divisible by 3, 5, or 7 are 0.
    const PATTERN_WORDS: usize = 105;
    let mut pattern = [!0u64; PATTERN_WORDS];
    for (w, item) in pattern.iter_mut().enumerate() {
        for b in 0..64 {
            let odd = 2 * (w * 64 + b) + 1;
            if odd.is_multiple_of(3) || odd.is_multiple_of(5) || odd.is_multiple_of(7) {
                *item &= !(1u64 << b);
            }
        }
    }

    let max_odd_idx = (max_fib - 1) / 2;
    let total_odds = max_odd_idx + 1;

    const CHUNKS_PER_PATTERN: usize = 64;
    const CHUNK_WORDS: usize = PATTERN_WORDS * CHUNKS_PER_PATTERN;
    const CHUNK_BITS: usize = CHUNK_WORDS * 64;

    let num_chunks = total_odds.div_ceil(CHUNK_BITS);
    let total_words = num_chunks * CHUNK_WORDS;
    let mut sieve = vec![0u64; total_words];

    let chunk_counts: Vec<u32> = sieve
        .par_chunks_mut(CHUNK_WORDS)
        .enumerate()
        .map(|(c, chunk)| {
            for chunk_slice in chunk.as_chunks_mut::<PATTERN_WORDS>().0 {
                chunk_slice.copy_from_slice(&pattern);
            }

            let low = 2 * (c * CHUNK_BITS) + 1;
            let high = 2 * ((c + 1) * CHUNK_BITS - 1) + 1;

            if c == 0 {
                chunk[0] &= !1u64;
                chunk[0] |= (1u64 << 1) | (1u64 << 2) | (1u64 << 3);
            }

            for &p in &base_primes {
                if p * p > high {
                    break;
                }
                let start_m = if low <= p * p {
                    p * p
                } else {
                    let q = low.div_ceil(p);
                    if q.is_multiple_of(2) {
                        (q + 1) * p
                    } else {
                        q * p
                    }
                };

                let mut idx = (start_m - low) / 2;
                let step4 = p * 4;
                while idx + step4 <= CHUNK_BITS {
                    unsafe {
                        *chunk.get_unchecked_mut(idx >> 6) &= !(1u64 << (idx & 63));
                        let i1 = idx + p;
                        *chunk.get_unchecked_mut(i1 >> 6) &= !(1u64 << (i1 & 63));
                        let i2 = i1 + p;
                        *chunk.get_unchecked_mut(i2 >> 6) &= !(1u64 << (i2 & 63));
                        let i3 = i2 + p;
                        *chunk.get_unchecked_mut(i3 >> 6) &= !(1u64 << (i3 & 63));
                    }
                    idx += step4;
                }
                while idx < CHUNK_BITS {
                    unsafe {
                        *chunk.get_unchecked_mut(idx >> 6) &= !(1u64 << (idx & 63));
                    }
                    idx += p;
                }
            }

            chunk.iter().map(|w| w.count_ones()).sum()
        })
        .collect();

    let mut chunk_prefix = vec![0u64; num_chunks + 1];
    for i in 0..num_chunks {
        chunk_prefix[i + 1] = chunk_prefix[i] + chunk_counts[i] as u64;
    }

    let count_primes = |n: usize| -> i64 {
        if n < 2 {
            return 0;
        }
        if n == 2 {
            return 1;
        }
        let max_odd = if n.is_multiple_of(2) { n - 1 } else { n };
        let odd_idx = (max_odd - 1) / 2;
        let chunk_idx = odd_idx / CHUNK_BITS;
        let in_chunk_idx = odd_idx % CHUNK_BITS;

        let mut count = chunk_prefix[chunk_idx];
        let chunk_word_offset = chunk_idx * CHUNK_WORDS;
        let target_word = in_chunk_idx / 64;
        let target_bit = in_chunk_idx % 64;

        for w in 0..target_word {
            count += sieve[chunk_word_offset + w].count_ones() as u64;
        }

        let mask = if target_bit == 63 {
            !0u64
        } else {
            (1u64 << (target_bit + 1)) - 1
        };
        count += (sieve[chunk_word_offset + target_word] & mask).count_ones() as u64;

        (count + 1) as i64
    };

    let triangular = |n: i64| -> i64 { n * (n + 1) / 2 };

    let compute_s = |n: i64| -> i64 {
        let nu = n as usize;
        let mut result = count_primes(nu);
        if n >= 4 {
            result += n / 2 - 1;
            result += count_primes((n - 2) as usize) - 1;
            let half = n / 2;
            if half >= 3 {
                result += (n + 1) * (half - 2) - 2 * (triangular(half) - 3);
            }
        }
        result
    };

    let mut ans: i64 = 0;
    for &fib in &fibs[3..=44] {
        ans += compute_s(fib);
    }

    println!("{ans}");
}
