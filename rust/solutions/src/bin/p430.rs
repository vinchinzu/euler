// Project Euler 430: Range flips
fn main() {
    let n: f64 = 1e10;
    let m: i32 = 4000;
    let n2 = n * n;
    let mut ans: f64 = 0.0;

    for k in 1..=(n as i64 / 2) {
        let km1 = (k - 1) as f64;
        let nmk = n - k as f64;
        let p_k = (km1 * km1 + nmk * nmk) / n2;
        let term = (2.0 * p_k - 1.0).powi(m);
        if term.abs() < 1e-15 { break; }
        ans += term;
    }
    ans += n / 2.0;
    println!("{:.2}", ans);
}
