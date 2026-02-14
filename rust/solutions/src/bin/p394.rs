// Project Euler 394: Eating Pie
// E(x) = (6*ln(x) + 2/x^3 + 7) / 9

fn main() {
    let n = 40.0_f64;
    let result = (6.0 * n.ln() + 2.0 / (n * n * n) + 7.0) / 9.0;
    println!("{:.10}", result);
}
