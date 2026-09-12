// Project Euler 255: Rounded Square Roots
use rayon::prelude::*;

#[inline(always)]
fn ceil_div(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

#[inline(always)]
fn bound(n: i64, low: i64, high: i64) -> i64 {
    n.clamp(low, high)
}

#[inline(always)]
fn sum_iterations(l: i64, h: i64, x: i64, k: i64) -> i64 {
    let x2l = bound(x * (x - 1) + 1, l, h);
    let x2h = bound(x * (x + 1) + 1, l, h);
    sum_remaining_iterations(l, x2l, x, k)
        + (x2h - x2l) * k
        + sum_remaining_iterations(x2h, h, x, k)
}

#[inline(always)]
fn sum_remaining_iterations(mut l: i64, h: i64, x: i64, k: i64) -> i64 {
    let mut total = 0;
    while l < h {
        let next_x = (x + ceil_div(l, x)) / 2;
        let next_l = (((next_x * 2 + 1 - x) * x) + 1).min(h);
        total += sum_iterations(l, next_l, next_x, k + 1);
        l = next_l;
    }
    total
}

/// Top-level remaining walk: next_x increases by 1 and range width is 2*x
/// after the first (possibly partial) interval. Parallelize those independent
/// sum_iterations calls.
fn par_sum_remaining(mut l: i64, h: i64, x: i64, k: i64) -> i64 {
    if l >= h {
        return 0;
    }
    let next_x0 = (x + ceil_div(l, x)) / 2;
    let next_l0 = (((next_x0 * 2 + 1 - x) * x) + 1).min(h);
    let first = sum_iterations(l, next_l0, next_x0, k + 1);
    l = next_l0;
    if l >= h {
        return first;
    }

    let step = 2 * x;
    let n_full = ((h - l) / step) as usize;
    let x_start = next_x0 + 1;
    let rest: i64 = (0..n_full)
        .into_par_iter()
        .with_min_len(4096)
        .map(|i| {
            let i = i as i64;
            let li = l + i * step;
            sum_iterations(li, li + step, x_start + i, k + 1)
        })
        .sum();

    l += n_full as i64 * step;
    let last = if l < h {
        let nx = (x + ceil_div(l, x)) / 2;
        sum_iterations(l, h, nx, k + 1)
    } else {
        0
    };
    first + rest + last
}

fn main() {
    let l = 10_000_000_000_000i64;
    let h = 100_000_000_000_000i64;
    let x0 = 7_000_000i64;
    let k = 1i64;

    let x2l = bound(x0 * (x0 - 1) + 1, l, h);
    let x2h = bound(x0 * (x0 + 1) + 1, l, h);
    let total = par_sum_remaining(l, x2l, x0, k)
        + (x2h - x2l) * k
        + par_sum_remaining(x2h, h, x0, k);
    let result = total as f64 / (h - l) as f64;
    println!("{:.10}", result);
}
