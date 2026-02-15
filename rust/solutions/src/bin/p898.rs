// Project Euler 898
// Claire Voyant coin problem.
// Meet-in-the-middle: split 25 pairs into two halves,
// enumerate all 3^half (value, prob) combos for each,
// then use sorted merge to compute P(sum > 0) + 0.5 * P(sum == 0).

fn main() {
    let n = 25;
    let mut pairs = [0.0f64; 25];
    for k in 0..25 {
        pairs[k] = (25 + k) as f64 / 100.0;
    }

    let mid = n / 2; // 12
    let n_a = mid;
    let n_b = n - mid; // 13

    // Generate all sums for group A
    let mut sums_a: Vec<(f64, f64)> = vec![(0.0, 1.0)];

    for i in 0..n_a {
        let p = pairs[i];
        let w = ((1.0 - p) / p).ln();
        let vals = [2.0 * w, 0.0, -2.0 * w];
        let probs = [(1.0 - p) * (1.0 - p), 2.0 * p * (1.0 - p), p * p];

        let mut next = Vec::with_capacity(sums_a.len() * 3);
        for &(sv, sp) in &sums_a {
            for k in 0..3 {
                next.push((sv + vals[k], sp * probs[k]));
            }
        }
        sums_a = next;
    }

    // Generate all sums for group B
    let mut sums_b: Vec<(f64, f64)> = vec![(0.0, 1.0)];

    for i in 0..n_b {
        let p = pairs[n_a + i];
        let w = ((1.0 - p) / p).ln();
        let vals = [2.0 * w, 0.0, -2.0 * w];
        let probs = [(1.0 - p) * (1.0 - p), 2.0 * p * (1.0 - p), p * p];

        let mut next = Vec::with_capacity(sums_b.len() * 3);
        for &(sv, sp) in &sums_b {
            for k in 0..3 {
                next.push((sv + vals[k], sp * probs[k]));
            }
        }
        sums_b = next;
    }

    // Sort B by value
    sums_b.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let sz_b = sums_b.len();

    // Compute suffix probability sums for B
    let mut suffix_probs = vec![0.0f64; sz_b + 1];
    for i in (0..sz_b).rev() {
        suffix_probs[i] = suffix_probs[i + 1] + sums_b[i].1;
    }

    let mut total_prob = 0.0f64;
    let eps = 1e-9;

    for &(val_a, prob_a) in &sums_a {
        let target = -val_a;

        // Binary search for idx_start: first element >= target - eps
        let idx_start = sums_b.partition_point(|x| x.0 < target - eps);

        // Binary search for idx_end: first element > target + eps
        let idx_end = sums_b.partition_point(|x| x.0 <= target + eps);

        // Strictly greater: from idx_end onwards
        let prob_strict = suffix_probs[idx_end];
        total_prob += prob_a * prob_strict;

        // Equal
        if idx_end > idx_start {
            let prob_equal = suffix_probs[idx_start] - suffix_probs[idx_end];
            total_prob += prob_a * prob_equal * 0.5;
        }
    }

    println!("{:.10}", total_prob);
}
