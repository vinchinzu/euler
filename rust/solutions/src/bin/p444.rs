// Project Euler 444: The Roundtable Lottery
fn harmonic(n: i64) -> f64 {
    if n == 0 { return 0.0; }
    if n < 2000 {
        let mut s = 0.0;
        for i in 1..=n { s += 1.0 / i as f64; }
        return s;
    }
    let gamma = 0.57721566490153286060651209;
    let dn = n as f64;
    gamma + dn.ln() + 1.0 / (2.0 * dn) - 1.0 / (12.0 * dn * dn)
}

fn main() {
    let n: i64 = 100_000_000_000_000;
    let k = 20;

    let mut log_binom = 0.0f64;
    for i in 1..=k {
        log_binom += ((n + i) as f64).ln() - (i as f64).ln();
    }

    let h_diff = harmonic(n + k) - harmonic(k);
    let log_ans = log_binom + h_diff.ln();

    let log10_ans = log_ans / 10.0f64.ln();
    let mut exponent = log10_ans.floor() as i32;
    let mut mantissa = 10.0f64.powf(log10_ans - exponent as f64);

    if mantissa >= 10.0 {
        mantissa /= 10.0;
        exponent += 1;
    }

    println!("{:.9}e{}", mantissa, exponent);
}
