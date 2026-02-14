// Project Euler 576 - Irrational Jumps

use euler_utils::primes_up_to;

#[derive(Clone, Copy)]
struct JumpPos {
    prime_idx: usize,
    total_len: f64,
}

fn frac_part(x: f64) -> f64 {
    x - x.floor()
}

fn main() {
    let n = 100;
    let d = 0.00002;

    let primes = primes_up_to(n);
    let nprimes = primes.len();

    let mut all_pos: Vec<JumpPos> = Vec::new();

    for (pi, &p) in primes.iter().enumerate() {
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

        all_pos.extend_from_slice(&tmp);
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

    let mut min_len = vec![f64::MAX; nprimes];

    while end < total_count {
        let frac_end = frac_part(all_pos[end].total_len);
        let mut frac_start = frac_part(all_pos[start].total_len);

        while frac_end - frac_start > d {
            start += 1;
            frac_start = frac_part(all_pos[start].total_len);
        }

        for v in min_len.iter_mut() {
            *v = f64::MAX;
        }

        for idx in start..end {
            let pi = all_pos[idx].prime_idx;
            if all_pos[idx].total_len < min_len[pi] {
                min_len[pi] = all_pos[idx].total_len;
            }
        }

        let total: f64 = min_len.iter().filter(|&&v| v < 1e17).sum();
        if total > ans {
            ans = total;
        }
        end += 1;
    }

    println!("{:.4}", ans);
}
