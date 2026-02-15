// Project Euler 761 - Runner and Swimmer
// Find critical speed V for runner around N-sided pool.

fn main() {
    let n = 6;
    let t = std::f64::consts::PI / n as f64;
    let mut k = 0i32;

    while (k as f64 * t).sin() - (k as f64 + n as f64) * t.tan() * (k as f64 * t).cos() < 0.0 {
        k += 1;
    }
    k -= 1;

    let numerator = 2.0 * (k as f64 * t).sin();
    let denominator = (k as f64 + n as f64) * t.tan();
    let a = (k as f64 * t + (numerator / denominator - (k as f64 * t).cos()).acos()) / 2.0;

    let ans = 1.0 / a.cos();
    println!("{:.8}", ans);
}
