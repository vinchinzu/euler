// Project Euler 589 - Pooh-sticks Marathon
//
// Two sticks dropped simultaneously. Transit time uniform integer in [n, m].
// Retrieval takes K=5 seconds. Find S(100).

fn solve_linear(a: &[f64], b: &[f64], n: usize) -> Vec<f64> {
    let mut aug = vec![0.0f64; n * (n + 1)];
    for i in 0..n {
        for j in 0..n {
            aug[i * (n + 1) + j] = a[i * n + j];
        }
        aug[i * (n + 1) + n] = b[i];
    }

    // Forward elimination with partial pivoting
    for col in 0..n {
        let mut max_row = col;
        let mut max_val = aug[col * (n + 1) + col].abs();
        for row in col + 1..n {
            let v = aug[row * (n + 1) + col].abs();
            if v > max_val {
                max_val = v;
                max_row = row;
            }
        }
        if max_row != col {
            for j in col..=n {
                let tmp = aug[col * (n + 1) + j];
                aug[col * (n + 1) + j] = aug[max_row * (n + 1) + j];
                aug[max_row * (n + 1) + j] = tmp;
            }
        }
        let pivot = aug[col * (n + 1) + col];
        for row in col + 1..n {
            let factor = aug[row * (n + 1) + col] / pivot;
            for j in col..=n {
                aug[row * (n + 1) + j] -= factor * aug[col * (n + 1) + j];
            }
        }
    }

    // Back substitution
    let mut x = vec![0.0f64; n];
    for i in (0..n).rev() {
        x[i] = aug[i * (n + 1) + n];
        for j in i + 1..n {
            x[i] -= aug[i * (n + 1) + j] * x[j];
        }
        x[i] /= aug[i * (n + 1) + i];
    }
    x
}

fn compute_e(m: i32, n: i32, k: i32) -> f64 {
    let span = (m - n + 1) as f64;
    let size = (m + k + 1) as usize;
    let mult = 1.0 / span;

    let mut mat = vec![0.0f64; size * size];
    let mut t_vec = vec![0.0f64; size];

    for d in 0..size {
        mat[d * size + d] = 1.0;
        for t1 in n..=m {
            if t1 < d as i32 - k {
                t_vec[d] += t1 as f64 * mult;
            } else {
                for t2 in (d as i32 + n)..=(d as i32 + m) {
                    let abs_diff = (t1 - t2).unsigned_abs() as usize;
                    if abs_diff < size {
                        mat[d * size + abs_diff] -= mult * mult;
                    }
                    let min_val = t1.min(t2) as f64;
                    t_vec[d] += (min_val + k as f64) * mult * mult;
                }
            }
        }
    }

    let x = solve_linear(&mat, &t_vec, size);
    x[0]
}

fn main() {
    let n_max = 100;
    let k = 5;
    let mut ans = 0.0f64;

    for m in 2..=n_max {
        for n in 1..m {
            ans += compute_e(m, n, k);
        }
    }

    println!("{:.2}", ans);
}
