fn main() {
    let n = 40usize;

    let mut fact_n: f64 = 1.0;
    for i in 2..=n {
        fact_n *= i as f64;
    }

    let mut ans = n as f64 / 2.0;

    for m in 0..(n / 2) {
        let mut dp = vec![vec![0.0f64; m + 2]; n + 1];
        dp[0][0] = 1.0;

        for p in 1..=n {
            for s in 1..=m {
                dp[p][s] = s as f64 * dp[p - 1][s.wrapping_sub(1).min(m + 1)]
                    + s as f64 * (if s + 1 <= m + 1 { dp[p - 1][s + 1] } else { 0.0 })
                    + 2.0 * s as f64 * dp[p - 1][s];
            }
        }

        ans -= dp[n][1] / fact_n;
    }

    println!("{:.6}", ans);
}
