// Project Euler 576 - Irrational Jumps

use euler_utils::primes_up_to;
use rayon::prelude::*;

#[derive(Clone, Copy)]
struct JumpPos {
    prime_idx: usize,
    total_len: f64,
}

#[inline]
fn frac_part(x: f64) -> f64 {
    x - x.floor()
}

fn main() {
    let n = 100;
    let d = 0.00002;

    let primes = primes_up_to(n);
    let nprimes = primes.len();

    // Independent per-prime generation
    let per_prime: Vec<Vec<JumpPos>> = primes
        .par_iter()
        .enumerate()
        .map(|(pi, &p)| {
            let sqrt_inv_p = (1.0 / p as f64).sqrt();
            let mut tmp: Vec<JumpPos> = Vec::new();
            let mut i = 0usize;
            loop {
                tmp.push(JumpPos {
                    prime_idx: pi,
                    total_len: i as f64 * sqrt_inv_p,
                });
                i += 1;
                if i > 1 && (i & (i + 1)) == 0 {
                    let mut sorted: Vec<f64> = tmp.iter().map(|j| frac_part(j.total_len)).collect();
                    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let all_within = sorted.windows(2).all(|w| w[1] - w[0] <= d);
                    if all_within {
                        break;
                    }
                }
            }
            tmp
        })
        .collect();

    let mut all_pos: Vec<JumpPos> = Vec::new();
    for v in per_prime {
        all_pos.extend_from_slice(&v);
    }

    all_pos.sort_by(|a, b| {
        frac_part(a.total_len)
            .partial_cmp(&frac_part(b.total_len))
            .unwrap()
    });

    let total_count = all_pos.len();
    let mut ans: f64 = 0.0;
    let mut start = 0;
    let mut end = nprimes;

    // Incremental min tracking: only update on slide, not full rescan
    // Fall back to efficient rescan with fewer zeros
    let mut min_len = vec![f64::MAX; nprimes];
    let mut active = vec![0u32; nprimes]; // generation stamp
    let mut stamp = 1u32;

    while end < total_count {
        let frac_end = frac_part(all_pos[end].total_len);
        let mut frac_start = frac_part(all_pos[start].total_len);

        while frac_end - frac_start > d {
            start += 1;
            frac_start = frac_part(all_pos[start].total_len);
        }

        stamp = stamp.wrapping_add(1);
        if stamp == 0 {
            active.fill(0);
            stamp = 1;
        }

        let mut total = 0.0f64;
        for idx in start..end {
            let pi = all_pos[idx].prime_idx;
            let len = all_pos[idx].total_len;
            if active[pi] != stamp {
                active[pi] = stamp;
                min_len[pi] = len;
                total += len;
            } else if len < min_len[pi] {
                total += len - min_len[pi];
                min_len[pi] = len;
            }
        }

        if total > ans {
            ans = total;
        }
        end += 1;
    }

    println!("{:.4}", ans);
}
