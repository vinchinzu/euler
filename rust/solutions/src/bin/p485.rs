// Project Euler 485 - Maximum number of divisors
//
// Parallel pair-divisor sieve for d(n), then parallel sliding-window maxima
// over independent endpoint ranges (K-element lookback per task).

use rayon::prelude::*;

const N: usize = 100_000_000;
const K: usize = 100_000;
const SQ: usize = 10_000; // isqrt(N); N is a square
const CHUNK: usize = 1 << 18; // 512 KiB of u16, one L2-sized segment
const QCAP: usize = 1 << 17; // 131072 > K, power of two for mask wrap
const QMASK: usize = QCAP - 1;

/// Sum of max(d[i-K+1..=i]) for i in i_lo..=i_hi. Requires i_lo >= K.
fn window_sum(divs: &[u16], i_lo: usize, i_hi: usize) -> u64 {
    let mut q_idx = vec![0u32; QCAP];
    let mut q_val = vec![0u16; QCAP];
    let mut head = 0usize;
    let mut tail = 0usize;
    let mut sum = 0u64;

    let start = i_lo - K + 1;

    for i in start..=i_hi {
        let di = unsafe { *divs.get_unchecked(i) };

        // Strictly decreasing values: pop back while last <= di.
        while head != tail {
            let last = (tail.wrapping_sub(1)) & QMASK;
            if unsafe { *q_val.get_unchecked(last) } > di {
                break;
            }
            tail = last;
        }

        unsafe {
            *q_idx.get_unchecked_mut(tail) = i as u32;
            *q_val.get_unchecked_mut(tail) = di;
        }
        tail = (tail + 1) & QMASK;

        if i >= i_lo {
            let left = (i - K + 1) as u32;
            while unsafe { *q_idx.get_unchecked(head) } < left {
                head = (head + 1) & QMASK;
            }
            sum += unsafe { *q_val.get_unchecked(head) } as u64;
        }
    }
    sum
}

fn main() {
    let mut d = vec![0u16; N + 1];

    // Segmented pair-divisor sieve: d[i*i] += 1, d[i*j] += 2 for i < j.
    d.par_chunks_mut(CHUNK).enumerate().for_each(|(ci, chunk)| {
        let lo = ci * CHUNK;
        let hi = lo + chunk.len();
        let start = lo.max(1);
        for i in 1..=SQ {
            let sqi = i * i;
            if sqi >= start && sqi < hi {
                // SAFETY: start <= sqi < hi maps into this chunk
                unsafe {
                    *chunk.get_unchecked_mut(sqi - lo) += 1;
                }
            }
            let j_lo = (i + 1).max(start.div_ceil(i));
            let j_hi = (hi - 1) / i;
            if j_lo <= j_hi {
                let mut idx = i * j_lo - lo;
                let end_idx = i * j_hi - lo;
                // SAFETY: i*j_lo >= start >= lo and i*j_hi <= hi-1
                unsafe {
                    while idx + 3 * i <= end_idx {
                        *chunk.get_unchecked_mut(idx) += 2;
                        *chunk.get_unchecked_mut(idx + i) += 2;
                        *chunk.get_unchecked_mut(idx + 2 * i) += 2;
                        *chunk.get_unchecked_mut(idx + 3 * i) += 2;
                        idx += 4 * i;
                    }
                    while idx <= end_idx {
                        *chunk.get_unchecked_mut(idx) += 2;
                        idx += i;
                    }
                }
            }
        }
    });

    // Independent sliding-window sums over endpoint partitions of [K, N].
    let n_windows = N - K + 1;
    let ntasks = rayon::current_num_threads().max(1) * 2;
    let sum: u64 = (0..ntasks)
        .into_par_iter()
        .map(|t| {
            let begin = t * n_windows / ntasks;
            let end = (t + 1) * n_windows / ntasks;
            if begin >= end {
                0
            } else {
                window_sum(&d, K + begin, K + end - 1)
            }
        })
        .sum();

    println!("{}", sum);
}
