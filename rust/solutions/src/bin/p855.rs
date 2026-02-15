// Project Euler 855 - S(5,8) in scientific notation

fn main() {
    let a = 5.0_f64;
    let b = 8.0_f64;
    let ln10 = 10.0_f64.ln();

    let log10_fact_a = (a + 1.0).lgamma().0 / ln10;
    let log10_fact_b = (b + 1.0).lgamma().0 / ln10;
    let log10_fact_ab = (a * b + 1.0).lgamma().0 / ln10;

    let log10_val = b * log10_fact_a + a * log10_fact_b - 2.0 * log10_fact_ab;
    let val = 10.0_f64.powf(log10_val);
    println!("{:.10e}", val);
}
