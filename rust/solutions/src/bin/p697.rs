// Project Euler 697 - Randomly Decaying Sequence
// Binary search for log(c) in log-space.

use std::f64::consts::E;

fn log_sum(log_a: f64, log_b: f64) -> f64 {
    if log_a == f64::NEG_INFINITY { return log_b; }
    if log_b == f64::NEG_INFINITY { return log_a; }
    if log_a > log_b {
        log_a + (log_b - log_a).exp().ln_1p()
    } else {
        log_b + (log_a - log_b).exp().ln_1p()
    }
}

fn main() {
    let n: usize = 10_000_000;
    let r: f64 = 0.25;

    // Precompute log factorials
    let mut log_facts = vec![0.0f64; n + 1];
    for i in 1..=n {
        log_facts[i] = log_facts[i - 1] + (i as f64).ln();
    }

    let mut low = 0.0f64;
    let mut high = 2.0 * n as f64;

    while low + 1e-3 < high {
        let log_c = (low + high) / 2.0;
        let mut log_prob = f64::NEG_INFINITY;
        let log_log_c = log_c.ln();

        for k in (0..n).rev() {
            let new_log_prob = log_sum(
                log_prob,
                k as f64 * log_log_c - log_facts[k],
            );
            if (log_prob - new_log_prob).abs() < 1e-15 && k < n - 10 {
                break;
            }
            log_prob = new_log_prob;
        }

        if log_prob > r.ln() + log_c {
            low = log_c;
        } else {
            high = log_c;
        }
    }

    let ans = low * E.log10();
    println!("{:.2}", ans);
}
