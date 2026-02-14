// Project Euler 339: Peredur fab Efrawg
fn main() {
    let n = 10_000usize;
    let k = 2 * n;

    let mut e = vec![0.0f64; k + 1];
    e[1] = 1.0;
    e[2] = 1.0;

    let mut cp = vec![0.0f64; n];
    let mut dp = vec![0.0f64; n];
    let mut x = vec![0.0f64; n];

    for kk in 3..=k {
        let ni = (kk - 1) / 2;
        let inv_k = 1.0 / kk as f64;

        cp[0] = -inv_k;
        dp[0] = 1.0 - inv_k;

        for i in 1..ni {
            let ai = (i + 1) as f64 * inv_k - 1.0;
            let ci = -((i + 1) as f64) * inv_k;
            let m = 1.0 - ai * cp[i - 1];
            let inv_m = 1.0 / m;
            cp[i] = ci * inv_m;
            dp[i] = (-ai * dp[i - 1]) * inv_m;
        }

        x[ni - 1] = dp[ni - 1];
        for i in (0..ni - 1).rev() {
            x[i] = dp[i] - cp[i] * x[i + 1];
        }

        let xn = x[ni - 1];
        let idx = (kk / 2) * 2 - 1;
        e[kk] = xn * kk as f64 + (1.0 - xn) * e[idx];
    }

    let ans = (e[k] + e[k - 3]) / 2.0;
    println!("{:.6}", ans);
}
