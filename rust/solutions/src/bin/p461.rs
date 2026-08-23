// Project Euler 461: Almost Pi
use rayon::prelude::*;

fn main() {
    const N: usize = 10000;
    let pi = std::f64::consts::PI;

    let mut f = vec![0.0f64; 2 * N + 1];
    for i in 0..=2 * N {
        f[i] = (i as f64 / N as f64).exp() - 1.0;
    }

    // f is strictly increasing; k_lim is the first k with f[k] >= pi
    let mut k_lim = 1;
    while f[k_lim] < pi {
        k_lim += 1;
    }

    let mut count = 0usize;
    for k1 in 1..k_lim {
        let target = pi - f[k1];
        count += f[k1..k_lim].partition_point(|&x| x < target);
    }

    let mut pairs: Vec<(f64, i32, i32)> = Vec::with_capacity(count);
    for k1 in 1..k_lim {
        let fk1 = f[k1];
        let target = pi - fk1;
        let k1i = k1 as i32;
        let mut k2 = k1;
        while k2 < k_lim && f[k2] < target {
            pairs.push((fk1 + f[k2], k1i, k2 as i32));
            k2 += 1;
        }
    }

    pairs.par_sort_unstable_by(|a, b| a.0.total_cmp(&b.0));

    let mut left = 0usize;
    let mut right = pairs.len() - 1;
    let mut min_error = f64::INFINITY;
    let mut min_left = 0usize;
    let mut min_right = 0usize;

    while left <= right {
        let error = pairs[left].0 + pairs[right].0 - pi;
        if error.abs() < min_error {
            min_error = error.abs();
            min_left = left;
            min_right = right;
        }
        if error < 0.0 {
            left += 1;
        } else if right == 0 {
            break;
        } else {
            right -= 1;
        }
    }

    let (_, a1, b1) = pairs[min_left];
    let (_, a2, b2) = pairs[min_right];
    let ans = a1 as u64 * a1 as u64
        + b1 as u64 * b1 as u64
        + a2 as u64 * a2 as u64
        + b2 as u64 * b2 as u64;
    println!("{}", ans);
}
