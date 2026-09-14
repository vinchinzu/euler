use rayon::prelude::*;

fn fibonacci(k: usize) -> i64 {
    if k <= 2 { return 1; }
    let (mut a, mut b) = (1i64, 1i64);
    for _ in 2..k {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn c_func(n: i64, a_val: f64, b_val: f64) -> f64 {
    let max_budget = 80.0 * a_val.max(b_val);
    let max_i = (max_budget / a_val) as usize + 2;
    let max_j = (max_budget / b_val) as usize + 2;
    let mut costs = Vec::with_capacity(max_i * max_j);

    for i in 0..=max_i {
        let ia = i as f64 * a_val;
        if ia > max_budget { break; }
        for j in 0..=max_j {
            let c = ia + j as f64 * b_val;
            if c > max_budget { break; }
            costs.push(c);
        }
    }

    costs.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut write_idx = 0;
    for read_idx in 0..costs.len() {
        if write_idx == 0 || costs[read_idx] - costs[write_idx - 1] > 1e-12 {
            costs[write_idx] = costs[read_idx];
            write_idx += 1;
        }
    }
    costs.truncate(write_idx);

    let mut f_vals = vec![0i64; costs.len()];
    let eps = 1e-9;

    for idx in 0..costs.len() {
        let c = costs[idx];

        let target_a = c - a_val + eps;
        let pos_a = costs[..idx + 1].partition_point(|&x| x <= target_a);
        let fa = if pos_a > 0 { f_vals[pos_a - 1] } else { 0 };

        let target_b = c - b_val + eps;
        let pos_b = costs[..idx + 1].partition_point(|&x| x <= target_b);
        let fb = if pos_b > 0 { f_vals[pos_b - 1] } else { 0 };

        let f_c = 1 + fa + fb;

        if f_c >= n {
            return c;
        }

        f_vals[idx] = f_c;
    }

    costs[costs.len() - 1]
}

fn main() {
    let n: i64 = 1_000_000_000_000;

    let total: f64 = (1..=30).into_par_iter().map(|k| {
        let a = (k as f64).sqrt();
        let b = (fibonacci(k) as f64).sqrt();
        c_func(n, a, b)
    }).sum();

    println!("{:.8}", total);
}
