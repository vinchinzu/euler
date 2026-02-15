// Project Euler 765 - Gambling Probability
// Find probability using log-space arithmetic for large binomial coefficients.

fn main() {
    let n = 1000;
    let k: f64 = 1e12;
    let p: f64 = 0.6;

    let mut log_fact = vec![0.0f64; n + 1];
    for i in 1..=n {
        log_fact[i] = log_fact[i - 1] + (i as f64).ln();
    }

    fn log_sum(log_x: f64, log_y: f64) -> f64 {
        if log_x == f64::NEG_INFINITY { return log_y; }
        if log_y == f64::NEG_INFINITY { return log_x; }
        if log_x > log_y {
            log_x + (1.0 + (log_y - log_x).exp()).ln()
        } else {
            log_y + (1.0 + (log_x - log_y).exp()).ln()
        }
    }

    fn log_diff(log_x: f64, log_y: f64) -> f64 {
        log_x + (1.0 - (log_y - log_x).exp()).ln()
    }

    let mut max_count = n as f64 * 2.0_f64.ln() - k.ln();
    let mut ans = f64::NEG_INFINITY;

    let mut ki = 0;
    loop {
        let count = log_fact[n] - log_fact[ki] - log_fact[n - ki];
        let term = count.min(max_count) + (n - ki) as f64 * p.ln() + ki as f64 * (1.0 - p).ln();
        ans = log_sum(ans, term);
        if count > max_count {
            ans = ans.exp();
            break;
        }
        max_count = log_diff(max_count, count);
        ki += 1;
    }

    println!("{:.10}", ans);
}
