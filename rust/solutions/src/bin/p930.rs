// Project Euler Problem 930 - Bowls and Balls
// G(N, M) = sum F(n, m) for n=2..N, m=2..M
// Uses DP over cosine-based state representation.
// Output: scientific format with 12 significant digits.

use std::collections::HashMap;
use std::f64::consts::PI;

fn main() {
    const N_LIM: usize = 12;
    const M_LIM: usize = 12;

    let mut total_g: f64 = 0.0;

    for n in 2..=N_LIM {
        // Compute unique cosines for this n
        let mut cos_vals = vec![0.0f64; n];
        for k in 0..n {
            let val = (2.0 * PI * k as f64 / n as f64).cos();
            cos_vals[k] = (val * 1e13).round() / 1e13;
        }

        // Find unique values and map k -> idx
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

        // Exact cos values for computation
        let mut idx_to_val = vec![0.0f64; num_unique];
        for k in 0..n {
            idx_to_val[k_to_idx[k]] = (2.0 * PI * k as f64 / n as f64).cos();
        }

        // State: (counts for each unique cos, k_sum mod n) -> ways
        // Use HashMap with Vec<i32> as key
        type State = (Vec<i32>, usize); // (counts, k_sum)
        let mut dp: HashMap<State, f64> = HashMap::new();

        // Initial state: empty
        let init_counts = vec![0i32; num_unique];
        dp.insert((init_counts, 0), 1.0);

        for m in 2..=M_LIM {
            // Transition: add one variable (ball)
            let old_entries: Vec<(State, f64)> = dp.drain().collect();
            let mut new_dp: HashMap<State, f64> = HashMap::new();

            for ((counts, k_sum), ways) in &old_entries {
                for k in 0..n {
                    let mut new_counts = counts.clone();
                    new_counts[k_to_idx[k]] += 1;
                    let new_k_sum = (k_sum + k) % n;
                    let key = (new_counts, new_k_sum);
                    *new_dp.entry(key).or_insert(0.0) += ways;
                }
            }

            dp = new_dp;

            // Compute F(n, m)
            let mut f_val: f64 = 0.0;
            for ((counts, k_sum), ways) in &dp {
                let mut sum_cos: f64 = 0.0;
                for i in 0..num_unique {
                    sum_cos += counts[i] as f64 * idx_to_val[i];
                }
                let cos_sum_k = (2.0 * PI * *k_sum as f64 / n as f64).cos();
                let lambda = (sum_cos + cos_sum_k) / m as f64;

                if (1.0 - lambda).abs() < 1e-9 {
                    continue;
                }

                f_val += ways / (1.0 - lambda);
            }

            total_g += f_val;
        }
    }

    // Format: scientific with 12 significant digits after decimal point
    println!("{:.12e}", total_g);
}
