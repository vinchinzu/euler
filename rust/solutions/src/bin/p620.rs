// Project Euler 620 - Gears
// Triple loop over gear sizes with trigonometric computation

use std::f64::consts::PI;

fn main() {
    let n = 500;
    let mut ans: i64 = 0;
    for s in 5..n - 9 {
        for p in 5..n - s {
            for q in p + 1..=n - s - p {
                let a = (s + p) as f64;
                let b = (p + q) as f64 - 2.0 * PI;
                let c = (s + q) as f64;
                let alpha = ((a * a + b * b - c * c) / (2.0 * a * b)).acos();
                let beta = (a * alpha.sin() / c).asin();
                let g = (((s + q) as f64 * beta - (s + p) as f64 * alpha) / PI + (s + p) as f64) as i64;
                ans += g;
            }
        }
    }
    println!("{}", ans);
}
