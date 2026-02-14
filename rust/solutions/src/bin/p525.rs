// Project Euler 525 - Rolling Ellipse
//
// Compute C(a,b) = length of path traced by center of ellipse rolling on x-axis.
// Answer: C(1,4) + C(3,4).

use std::f64::consts::PI;

const L: usize = 1_000_000;

fn c_val(a: f64, b: f64) -> f64 {
    let mut prev_perim_x = 0.0f64;
    let mut prev_perim_y = b;
    let mut prev_x = 0.0f64;
    let mut prev_y = b;
    let mut c = 0.0f64;

    let step = PI / 2.0 / L as f64;

    for i in 1..=L {
        let theta = step * i as f64;
        let sin_val = theta.sin();
        let cos_val = theta.cos();
        let perim_x = a * sin_val;
        let perim_y = b * cos_val;
        let r = perim_x.hypot(perim_y);
        let alpha = (perim_x / perim_y).atan() + (a * cos_val / (b * sin_val)).atan();
        let x = r * alpha.cos();
        let y = r * alpha.sin();

        let dx_perim = perim_x - prev_perim_x;
        let dy_perim = perim_y - prev_perim_y;
        let arc = dx_perim.hypot(dy_perim);

        c += (x - prev_x + arc).hypot(y - prev_y);

        prev_perim_x = perim_x;
        prev_perim_y = perim_y;
        prev_x = x;
        prev_y = y;
    }

    4.0 * c
}

fn main() {
    let ans = c_val(1.0, 4.0) + c_val(3.0, 4.0);
    println!("{:.8}", ans);
}
