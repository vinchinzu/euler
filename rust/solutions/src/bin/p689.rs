// Project Euler 689 - Binary Series
// Enumerate all 2^22 subsets, use erf approximation for tail probability.

use std::f64::consts::PI;

fn my_erf(z: f64) -> f64 {
    if z > 5.0 { return 1.0; }
    if z < -5.0 { return -1.0; }
    let az = z.abs();
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;
    let t = 1.0 / (1.0 + p * az);
    let val = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-z * z).exp();
    if z >= 0.0 { val } else { -val }
}

fn main() {
    let l = 22;
    let n = 0.5;

    let mut inv_sq = [0.0f64; 23];
    for i in 1..=l {
        inv_sq[i] = 1.0 / (i as f64 * i as f64);
    }

    let pi2_6 = PI * PI / 6.0;
    let pi4_90 = PI * PI * PI * PI / 90.0;
    let mut mean = pi2_6;
    let mut var = pi4_90;
    for i in 1..=l {
        mean -= inv_sq[i];
        var -= 1.0 / (i as f64 * i as f64 * i as f64 * i as f64);
    }
    mean /= 2.0;
    let stddev = (var / 2.0).sqrt();

    let mut ans = 0.0;
    let total = 1u32 << l;
    for subset in 0..total {
        let mut sum_val = 0.0;
        let mut s = subset;
        while s != 0 {
            let bit = s.trailing_zeros() as usize;
            sum_val += inv_sq[bit + 1];
            s &= s - 1;
        }
        ans += (1.0 - my_erf((n - mean - sum_val) / stddev)) / 2.0;
    }
    ans /= total as f64;

    println!("{:.8}", ans);
}
