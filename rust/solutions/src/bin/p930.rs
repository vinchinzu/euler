// Project Euler Problem 930 - Bowls and Balls
// G(N, M) = sum F(n, m) for n=2..N, m=2..M
// Uses DP over cosine-based state representation.
// Optimized: u64 packed states with FxHashMap instead of Vec<i32> keys.

use fxhash::FxHashMap;
use std::f64::consts::PI;

#[inline]
fn pack_state(counts: &[u8], k_sum: usize, num_unique: usize) -> u64 {
    let mut state = k_sum as u64;
    for i in 0..num_unique {
        state = state * 13 + counts[i] as u64;
    }
    state
}

#[inline]
fn unpack_state(mut state: u64, counts: &mut [u8], num_unique: usize) -> usize {
    for i in (0..num_unique).rev() {
        counts[i] = (state % 13) as u8;
        state /= 13;
    }
    state as usize
}

fn main() {
    const N_LIM: usize = 12;
    const M_LIM: usize = 12;

    let mut total_g: f64 = 0.0;

    for n in 2..=N_LIM {
        let mut cos_vals = vec![0.0f64; n];
        for k in 0..n {
            let val = (2.0 * PI * k as f64 / n as f64).cos();
            cos_vals[k] = (val * 1e13).round() / 1e13;
        }

        let mut unique_vals: Vec<f64> = Vec::new();
        let mut k_to_idx = vec![0usize; n];
        for k in 0..n {
            let mut found = false;
            for (u, &uv) in unique_vals.iter().enumerate() {
                if (cos_vals[k] - uv).abs() < 1e-12 {
                    k_to_idx[k] = u;
                    found = true;
                    break;
                }
            }
            if !found {
                k_to_idx[k] = unique_vals.len();
                unique_vals.push(cos_vals[k]);
            }
        }
        let num_unique = unique_vals.len();

        let mut idx_to_val = vec![0.0f64; num_unique];
        for k in 0..n {
            idx_to_val[k_to_idx[k]] = (2.0 * PI * k as f64 / n as f64).cos();
        }

        let mut dp: FxHashMap<u64, f64> = FxHashMap::with_capacity_and_hasher(65536, Default::default());
        let init_counts = vec![0u8; num_unique];
        let init_state = pack_state(&init_counts, 0, num_unique);
        dp.insert(init_state, 1.0);

        let mut counts_buf = vec![0u8; num_unique];

        for m in 2..=M_LIM {
            let mut new_dp: FxHashMap<u64, f64> = FxHashMap::with_capacity_and_hasher(dp.len() * 2, Default::default());

            for (&state, &ways) in &dp {
                let k_sum = unpack_state(state, &mut counts_buf, num_unique);

                for k in 0..n {
                    counts_buf[k_to_idx[k]] += 1;
                    let new_k_sum = (k_sum + k) % n;
                    let new_state = pack_state(&counts_buf, new_k_sum, num_unique);
                    *new_dp.entry(new_state).or_insert(0.0) += ways;
                    counts_buf[k_to_idx[k]] -= 1;
                }
            }

            dp = new_dp;

            let mut f_val: f64 = 0.0;
            for (&state, &ways) in &dp {
                let k_sum = unpack_state(state, &mut counts_buf, num_unique);

                let mut sum_cos: f64 = 0.0;
                for i in 0..num_unique {
                    sum_cos += counts_buf[i] as f64 * idx_to_val[i];
                }
                let cos_sum_k = (2.0 * PI * k_sum as f64 / n as f64).cos();
                let lambda = (sum_cos + cos_sum_k) / m as f64;

                if (1.0 - lambda).abs() < 1e-9 {
                    continue;
                }

                f_val += ways / (1.0 - lambda);
            }

            total_g += f_val;
        }
    }

    println!("{:.12e}", total_g);
}
