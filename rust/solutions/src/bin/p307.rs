// Project Euler 307: Chip Defects
fn main() {
    let k: i64 = 20000;
    let n: i64 = 1_000_000;

    // Compute ln(P(0))
    let mut ln_p0: f64 = 0.0;
    for i in 0..k {
        ln_p0 += (1.0 - i as f64 / n as f64).ln();
    }

    // Find peak x
    let mut ln_px = ln_p0;
    let mut max_ln_px = ln_p0;
    let mut peak_x: i64 = 0;

    for x in 1..=(k / 2) {
        let num = (k - 2*x + 2) as f64 * (k - 2*x + 1) as f64;
        let den = 2.0 * x as f64 * (n - k + x) as f64;
        let ratio = num / den;
        ln_px += ratio.ln();
        if ln_px > max_ln_px {
            max_ln_px = ln_px;
            peak_x = x;
        }
        if ratio < 0.5 && x > peak_x + 100 { break; }
    }

    // Recompute all terms with log-sum-exp
    ln_px = ln_p0;
    let mut sum_scaled: f64 = (ln_p0 - max_ln_px).exp();

    for x in 1..=(k / 2) {
        let num = (k - 2*x + 2) as f64 * (k - 2*x + 1) as f64;
        let den = 2.0 * x as f64 * (n - k + x) as f64;
        ln_px += (num / den).ln();
        let contrib = (ln_px - max_ln_px).exp();
        sum_scaled += contrib;
        if x > peak_x + 500 && contrib < 1e-18 { break; }
    }

    let prob_no_3plus = max_ln_px.exp() * sum_scaled;
    let answer = 1.0 - prob_no_3plus;
    println!("{:.10}", answer);
}
