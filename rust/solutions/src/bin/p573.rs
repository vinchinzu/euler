// Project Euler 573 - Unfair Race
//
// E_N = sum_{k=1}^{N} C(N,k) * k^k * (N-k)^(N-k) / N^N
// Compute using log-space to avoid overflow.
// N = 1000000.

fn main() {
    let n: usize = 1_000_000;

    // Precompute log factorials
    let mut log_fact = vec![0.0f64; n + 1];
    for i in 1..=n {
        log_fact[i] = log_fact[i - 1] + (i as f64).ln();
    }

    // Use Kahan summation for better precision
    let mut ans: f64 = 1.0; // k = N case
    let mut comp: f64 = 0.0;

    for k in 1..n {
        let log_term = log_fact[n] - log_fact[k] - log_fact[n - k]
            + k as f64 * (k as f64).ln()
            + (n - k) as f64 * ((n - k) as f64).ln()
            - n as f64 * (n as f64).ln();
        let term = log_term.exp();
        let y = term - comp;
        let t = ans + y;
        comp = (t - ans) - y;
        ans = t;
    }

    println!("{:.4}", ans);
}
