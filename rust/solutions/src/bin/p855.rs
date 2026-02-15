// Project Euler 855 - S(5,8) in scientific notation

unsafe extern "C" {
    fn lgamma(x: f64) -> f64;
}

fn ln_gamma(x: f64) -> f64 {
    unsafe { lgamma(x) }
}

fn main() {
    let a = 5.0_f64;
    let b = 8.0_f64;
    let ln10 = 10.0_f64.ln();

    let log10_fact_a = ln_gamma(a + 1.0) / ln10;
    let log10_fact_b = ln_gamma(b + 1.0) / ln10;
    let log10_fact_ab = ln_gamma(a * b + 1.0) / ln10;

    let log10_val = b * log10_fact_a + a * log10_fact_b - 2.0 * log10_fact_ab;
    let val = 10.0_f64.powf(log10_val);
    println!("{:.10e}", val);
}
