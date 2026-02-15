// Project Euler 229 - Four Representations using Squares
//
// Count n <= 2*10^9 representable as a^2+b^2, a^2+2b^2, a^2+3b^2, a^2+7b^2
// simultaneously (a,b > 0 for each). Process in chunks; parallelize across chunks.
//
// Chunk size tuned so per-chunk bitmap arrays fit comfortably in L2 cache,
// reducing memory-bandwidth contention when multiple cores run simultaneously.

use rayon::prelude::*;

fn main() {
    let n_limit: u64 = 2_000_000_000;
    let chunk: u64 = 5_000_000; // smaller chunks for better cache behavior
    let ks = [1u64, 2, 3, 7];
    let n_chunks = ((n_limit - 1) / chunk + 1) as usize;

    let total: u64 = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let base = ci as u64 * chunk + 1;
            let hi = std::cmp::min(base + chunk - 1, n_limit);
            let len = (hi - base + 1) as usize;
            let bytes = (len + 7) / 8;

            let mut combined = vec![0xFFu8; bytes];
            if len % 8 != 0 {
                combined[bytes - 1] = (1u8 << (len % 8)) - 1;
            }

            for &k in &ks {
                let mut temp = vec![0u8; bytes];

                let mut b = 1u64;
                while k * b * b < hi {
                    let kb2 = k * b * b;
                    let min_n = if kb2 + 1 < base { base } else { kb2 + 1 };
                    if min_n > hi {
                        break;
                    }

                    let diff_lo = min_n - kb2;
                    let mut a_lo = (diff_lo as f64).sqrt() as u64;
                    if a_lo * a_lo < diff_lo {
                        a_lo += 1;
                    }
                    if a_lo < 1 {
                        a_lo = 1;
                    }

                    let diff_hi = hi - kb2;
                    let mut a_hi = (diff_hi as f64).sqrt() as u64;
                    while a_hi * a_hi > diff_hi {
                        a_hi -= 1;
                    }

                    for a in a_lo..=a_hi {
                        let n = a * a + kb2;
                        let idx = (n - base) as usize;
                        temp[idx >> 3] |= 1 << (idx & 7);
                    }

                    b += 1;
                }

                for i in 0..bytes {
                    combined[i] &= temp[i];
                }
            }

            let mut count = 0u64;
            for &byte in &combined {
                count += byte.count_ones() as u64;
            }
            count
        })
        .sum();

    println!("{}", total);
}
