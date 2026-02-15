// Project Euler 471: Triangle inscribed in ellipse

fn harmonic(n: i64) -> f64 {
    if n <= 0 { return 0.0; }
    let nd = n as f64;
    let gamma = 0.5772156649015328606;
    let ln_n = nd.ln();
    let inv_n = 1.0 / nd;
    let inv_n2 = inv_n * inv_n;

    ln_n + gamma
        + 0.5 * inv_n
        - inv_n2 / 12.0
        + inv_n2 * inv_n2 / 120.0
        - inv_n2 * inv_n2 * inv_n2 / 252.0
}

fn main() {
    let n: i64 = 100_000_000_000; // 10^11

    let log_term1 = (n as f64).log10() + (2.0 * n as f64 - 1.0).log10()
        + (3.0 * n as f64 + 4.0).log10() - 24.0f64.log10();

    let h_diff = harmonic(n) - harmonic(n / 2);
    let log_term2 = (n as f64).log10() + (n as f64 + 1.0).log10()
        + (2.0 * n as f64 + 1.0).log10() + h_diff.log10() - 6.0f64.log10();

    let diff_log = log_term2 - log_term1;
    let ratio = 10.0f64.powf(diff_log);
    let log_ans = log_term1 + (1.0 - ratio).log10();

    let exp_part = log_ans.floor() as i32;
    let mantissa = 10.0f64.powf(log_ans - exp_part as f64);

    println!("{:.9}e{}", mantissa, exp_part);
}
