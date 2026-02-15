// Project Euler 664 - An Infinite Game of Tag
// Sum score = sum d^N * phi^(4-d), answer = floor(log_phi(total_score)).

fn log_sum(log_a: f64, log_b: f64) -> f64 {
    if log_a == f64::NEG_INFINITY { return log_b; }
    if log_b == f64::NEG_INFINITY { return log_a; }
    if log_a >= log_b { log_a + (log_b - log_a).exp().ln_1p() }
    else { log_b + (log_a - log_b).exp().ln_1p() }
}

fn main() {
    let n = 1_234_567u64;
    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let log_phi = phi.ln();
    let mut log_sum_val = f64::NEG_INFINITY;
    let mut d = 1u64;
    loop {
        let log_val = n as f64 * (d as f64).ln() - (d as f64 - 4.0) * log_phi;
        let new_log_sum = log_sum(log_sum_val, log_val);
        if (log_sum_val - new_log_sum).abs() < 1e-10 { break; }
        log_sum_val = new_log_sum;
        d += 1;
    }
    log_sum_val /= log_phi;
    println!("{}", log_sum_val as i64);
}
