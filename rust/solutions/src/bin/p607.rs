// Project Euler 607 - Marsh Crossing
// Snell's Law + binary search for minimum time path through marsh strips

use std::f64::consts::PI;

fn main() {
    let d = 100.0_f64;
    let l = 50.0_f64;
    let k = 5;
    let speeds = [10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 10.0];
    let num_borders = k + 2;

    let mut border_xs = vec![0.0; num_borders];
    for i in 0..=k {
        border_xs[i] = d / 2.0 - l / 2.0_f64.sqrt() + (l / k as f64) * 2.0_f64.sqrt() * i as f64;
    }
    border_xs[k + 1] = d;

    let sin45 = (PI / 4.0).sin();
    let cos45 = (PI / 4.0).cos();

    let trace = |alpha: f64| -> (f64, f64) {
        let mut a = alpha;
        let mut px = 0.0_f64;
        let mut py = 0.0_f64;
        let mut total_time = 0.0_f64;

        for i in 0..num_borders {
            let bx = border_xs[i];
            let a1 = sin45;
            let b1 = -cos45;
            let c1 = a1 * bx;
            let a2 = a.sin();
            let b2 = -a.cos();
            let c2 = a2 * px + b2 * py;
            let denom = a2 * b1 - b2 * a1;
            if denom.abs() < 1e-15 { break; }
            let nx = (c2 * b1 - b2 * c1) / denom;
            let ny = (c2 * a1 - a2 * c1) / (b2 * a1 - a2 * b1);
            let dist = ((nx - px).powi(2) + (ny - py).powi(2)).sqrt();
            total_time += dist / speeds[i];
            px = nx;
            py = ny;
            if i < num_borders - 1 {
                let sin_val = speeds[i + 1] * (PI / 4.0 + a).sin() / speeds[i];
                if sin_val.abs() > 1.0 { break; }
                a = sin_val.asin() - PI / 4.0;
            }
        }
        (py, total_time)
    };

    let mut low = 0.0_f64;
    let mut high = PI / 4.0;
    for _ in 0..200 {
        let mid = (low + high) / 2.0;
        let (fy, _) = trace(mid);
        if fy < 0.0 { low = mid; } else { high = mid; }
    }

    let alpha = (low + high) / 2.0;
    let (_, total_time) = trace(alpha);
    println!("{:.10}", total_time);
}
