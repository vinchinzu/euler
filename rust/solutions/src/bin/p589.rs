// Project Euler 589 - Pooh-sticks Marathon
//
// Two sticks dropped simultaneously. Transit time uniform integer in [n, m].
// Retrieval takes K=5 seconds. Find S(100).
//
// Matrix construction is O(size * span) closed-form range updates instead of
// O(size * span^2) t1/t2 loops. Integer counts until the 1/span^2 scale.

use rayon::prelude::*;

fn solve_linear(a: &[f64], b: &[f64], n: usize) -> Vec<f64> {
    let mut aug = vec![0.0f64; n * (n + 1)];
    for i in 0..n {
        for j in 0..n {
            aug[i * (n + 1) + j] = a[i * n + j];
        }
        aug[i * (n + 1) + n] = b[i];
    }

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

#[inline(always)]
fn add_range(diff: &mut [f64], lo: usize, hi: usize, size: usize, val: f64) {
    if lo >= size {
        return;
    }
    let hi = hi.min(size - 1);
    if lo > hi {
        return;
    }
    diff[lo] += val;
    diff[hi + 1] -= val;
}

/// Histogram of |t1 - t2| for t2 in [t2lo, t2hi], clipped to [0, size).
#[inline(always)]
fn add_abs_range(diff: &mut [f64], t1: i32, t2lo: i32, t2hi: i32, size: usize, val: f64) {
    let a = t1 - t2hi;
    let b = t1 - t2lo;
    if b <= 0 {
        add_range(diff, (-b) as usize, (-a) as usize, size, val);
    } else if a >= 0 {
        add_range(diff, a as usize, b as usize, size, val);
    } else {
        add_range(diff, 0, 0, size, val);
        add_range(diff, 1, (-a) as usize, size, val);
        if b > 0 {
            add_range(diff, 1, b as usize, size, val);
        }
    }
}

#[inline(always)]
fn sum_min(t1: i32, t2lo: i32, t2hi: i32) -> f64 {
    let span = (t2hi - t2lo + 1) as f64;
    if t1 <= t2lo {
        span * t1 as f64
    } else if t1 >= t2hi {
        span * (t2lo + t2hi) as f64 * 0.5
    } else {
        let nleft = (t1 - t2lo + 1) as f64;
        nleft * (t2lo + t1) as f64 * 0.5 + t1 as f64 * (t2hi - t1) as f64
    }
}

fn compute_e(m: i32, n: i32, k: i32) -> f64 {
    let span = (m - n + 1) as f64;
    let size = (m + k + 1) as usize;
    let mult = 1.0 / span;
    let mm = mult * mult;
    let span_i = t2_span(n, m);

    let mut mat = vec![0.0f64; size * size];
    let mut t_vec = vec![0.0f64; size];
    let mut diff = vec![0.0f64; size + 1];

    for d in 0..size {
        diff.fill(0.0);
        let t2lo = d as i32 + n;
        let t2hi = d as i32 + m;
        let t1_early_hi = (d as i32 - k - 1).min(m);
        if t1_early_hi >= n {
            let cnt = (t1_early_hi - n + 1) as f64;
            t_vec[d] += cnt * (n + t1_early_hi) as f64 * 0.5 * mult;
        }
        let t1_b_lo = (d as i32 - k).max(n);
        if t1_b_lo <= m {
            let n_t1 = (m - t1_b_lo + 1) as f64;
            t_vec[d] += n_t1 * span_i * k as f64 * mm;
            for t1 in t1_b_lo..=m {
                add_abs_range(&mut diff, t1, t2lo, t2hi, size, -mm);
                t_vec[d] += sum_min(t1, t2lo, t2hi) * mm;
            }
        }
        let mut run = 0.0;
        let row = d * size;
        for j in 0..size {
            run += diff[j];
            mat[row + j] = run;
        }
        mat[row + d] += 1.0;
    }

    let x = solve_linear(&mat, &t_vec, size);
    x[0]
}

#[inline(always)]
fn t2_span(n: i32, m: i32) -> f64 {
    (m - n + 1) as f64
}

fn main() {
    let n_max = 100;
    let k = 5;
    let ans: f64 = (2..=n_max)
        .into_par_iter()
        .map(|m| {
            let mut s = 0.0f64;
            for n in 1..m {
                s += compute_e(m, n, k);
            }
            s
        })
        .sum();

    println!("{:.2}", ans);
}
