// Project Euler 267: Billionaire

fn main() {
    let n = 1000usize;
    let log_c = (1e9f64).ln();

    // Precompute log(C(N, w))
    let mut log_ncr = vec![0.0f64; n + 1];
    for i in 1..=n {
        log_ncr[i] = log_ncr[i - 1] + ((n - i + 1) as f64).ln() - (i as f64).ln();
    }

    let log_2n = n as f64 * 2.0f64.ln();
    let mut ans = 0.0f64;

    for w in (0..=n).rev() {
        let f = (3.0 * w as f64 / n as f64 - 1.0) / 2.0;
        if f <= 0.0 || f >= 1.0 { continue; }
        let log_e = (n - w) as f64 * (1.0 - f).ln() + w as f64 * (1.0 + 2.0 * f).ln();
        if log_e < log_c { break; }
        ans += (log_ncr[w] - log_2n).exp();
    }

    println!("{:.12}", ans);
}
