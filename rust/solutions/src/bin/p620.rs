// Project Euler 620 - Gears
// Triple loop over gear sizes with trigonometric computation

use rayon::prelude::*;
use std::f64::consts::PI;

fn main() {
    let n = 500;
    let ans: i64 = (5..n - 9)
        .into_par_iter()
        .map(|s| {
            let mut local = 0i64;
            for p in 5..n - s {
                for q in p + 1..=n - s - p {
                    let a = (s + p) as f64;
                    let b = (p + q) as f64 - 2.0 * PI;
                    let c = (s + q) as f64;
                    let alpha = ((a * a + b * b - c * c) / (2.0 * a * b)).acos();
                    let beta = (a * alpha.sin() / c).asin();
                    let g = (((s + q) as f64 * beta - (s + p) as f64 * alpha) / PI + (s + p) as f64) as i64;
                    local += g;
                }
            }
            local
        })
        .sum();
    println!("{}", ans);
}
