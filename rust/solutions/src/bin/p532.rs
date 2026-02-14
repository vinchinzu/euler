// Project Euler 532 - Robots on a Sphere
//
// Binary search for smallest n such that line_length(n, R) >= 1000,
// then compute total combined distance.

use std::f64::consts::PI;

const STEPS: usize = 10_000;

fn line_length(num_robots: usize, r: f64) -> f64 {
    let lam = 2.0 * PI / num_robots as f64;
    let sin_lam = lam.sin();
    let cos_lam = lam.cos();
    let one_minus_cos_lam = 1.0 - cos_lam;
    let start = r.acos();
    let end = PI / 2.0;
    let h = (end - start) / STEPS as f64;

    // Simpson's rule
    let mut sum = 0.0f64;
    for i in 0..=STEPS {
        let t = start + h * i as f64;
        let sin_t = t.sin();
        let cos_t = t.cos();
        let dlong = sin_lam * cos_t;
        let dlat = sin_t * cos_t * one_minus_cos_lam;
        let val = if dlat.abs() < 1e-30 {
            0.0
        } else {
            dlat.hypot(dlong) / dlat
        };

        let weight = if i == 0 || i == STEPS { 1.0 }
            else if i % 2 == 1 { 4.0 }
            else { 2.0 };
        sum += weight * val;
    }

    sum * h / 3.0
}

fn main() {
    let n_target = 1000;
    let r = 0.999f64;

    let mut low = 1usize;
    let mut high = 1 << 30;
    while low + 1 < high {
        let mid = low + (high - low) / 2;
        if line_length(mid, r) >= n_target as f64 {
            high = mid;
        } else {
            low = mid;
        }
    }

    let ans = line_length(high, r) * high as f64;
    println!("{:.2}", ans);
}
