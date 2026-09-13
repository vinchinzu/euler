// Project Euler 855 - S(5,8) in scientific notation

unsafe extern "C" {
    fn lgamma(x: f64) -> f64;
}

#[inline(always)]
fn ln_gamma(x: f64) -> f64 {
    unsafe { lgamma(x) }
}

fn main() {
    const A: f64 = 5.0;
    const B: f64 = 8.0;
    const LN10: f64 = 2.302585092994045684;

    let log10_fact_a = ln_gamma(A + 1.0) / LN10;
    let log10_fact_b = ln_gamma(B + 1.0) / LN10;
    let log10_fact_ab = ln_gamma(A * B + 1.0) / LN10;

    let log10_val = B.mul_add(log10_fact_a, A.mul_add(log10_fact_b, -2.0 * log10_fact_ab));
    let val = 10.0_f64.powf(log10_val);
    println!("{:.10e}", val);
}
