// Project Euler 286: Scoring Probabilities
// Find q such that P(exactly 20 points in 50 shots) = 0.02.

fn prob(q: f64, n: usize, k: usize) -> f64 {
    let mut dp = vec![vec![0.0f64; k + 1]; n + 1];
    dp[0][0] = 1.0;
    for i in 1..=n {
        for j in 0..=k {
            dp[i][j] = dp[i - 1][j] * (i as f64) / q;
            if j > 0 {
                dp[i][j] += dp[i - 1][j - 1] * (1.0 - (i as f64) / q);
            }
        }
    }
    dp[n][k]
}

fn main() {
    let n = 50;
    let k = 20;
    let r = 0.02;

    let mut lo = n as f64;
    let mut hi = 1e10;

    for _ in 0..200 {
        let mid = (lo + hi) / 2.0;
        if prob(mid, n, k) < r {
            hi = mid;
        } else {
            lo = mid;
        }
    }

    println!("{:.10}", lo);
}
