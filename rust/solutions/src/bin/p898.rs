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

    let mid = n / 2;
    let n_a = mid;
    let n_b = n - mid;

    let mut sums_a: Vec<(f64, f64)> = vec![(0.0, 1.0)];

    for i in 0..n_a {
        let p = unsafe { *pairs.get_unchecked(i) };
        let w = ((1.0 - p) / p).ln();
        let v0 = 2.0 * w;
        let v2 = -v0;
        let prob0 = (1.0 - p) * (1.0 - p);
        let prob1 = 2.0 * p * (1.0 - p);
        let prob2 = p * p;

        let len = sums_a.len();
        let mut next = Vec::with_capacity(len * 3);
        for j in 0..len {
            let (sv, sp) = unsafe { *sums_a.get_unchecked(j) };
            next.push((sv + v0, sp * prob0));
            next.push((sv, sp * prob1));
            next.push((sv + v2, sp * prob2));
        }
        sums_a = next;
    }

    let mut sums_b: Vec<(f64, f64)> = vec![(0.0, 1.0)];

    for i in 0..n_b {
        let p = unsafe { *pairs.get_unchecked(n_a + i) };
        let w = ((1.0 - p) / p).ln();
        let v0 = 2.0 * w;
        let v2 = -v0;
        let prob0 = (1.0 - p) * (1.0 - p);
        let prob1 = 2.0 * p * (1.0 - p);
        let prob2 = p * p;

        let len = sums_b.len();
        let mut next = Vec::with_capacity(len * 3);
        for j in 0..len {
            let (sv, sp) = unsafe { *sums_b.get_unchecked(j) };
            next.push((sv + v0, sp * prob0));
            next.push((sv, sp * prob1));
            next.push((sv + v2, sp * prob2));
        }
        sums_b = next;
    }

    sums_b.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let sz_b = sums_b.len();

    let mut suffix_probs = vec![0.0f64; sz_b + 1];
    for i in (0..sz_b).rev() {
        let current_prob = unsafe { sums_b.get_unchecked(i).1 };
        let next_suffix = unsafe { *suffix_probs.get_unchecked(i + 1) };
        unsafe {
            *suffix_probs.get_unchecked_mut(i) = next_suffix + current_prob;
        }
    }

    let mut total_prob = 0.0f64;
    let eps = 1e-9;

    for i in 0..sums_a.len() {
        let (val_a, prob_a) = unsafe { *sums_a.get_unchecked(i) };
        let target = -val_a;

        let idx_start = sums_b.partition_point(|x| x.0 < target - eps);
        let idx_end = sums_b.partition_point(|x| x.0 <= target + eps);

        let prob_strict = unsafe { *suffix_probs.get_unchecked(idx_end) };
        total_prob += prob_a * prob_strict;

        if idx_end > idx_start {
            let prob_equal = unsafe {
                *suffix_probs.get_unchecked(idx_start) - *suffix_probs.get_unchecked(idx_end)
            };
            total_prob += prob_a * prob_equal * 0.5;
        }
    }

    println!("{:.10}", total_prob);
}
