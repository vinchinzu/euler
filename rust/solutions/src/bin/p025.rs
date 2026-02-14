// Project Euler 025: 1000-digit Fibonacci Number
// Index of the first Fibonacci number with 1000 digits, using Binet's formula.

fn main() {
    let sqrt5: f64 = 5.0_f64.sqrt();
    let phi: f64 = (1.0 + sqrt5) / 2.0;
    let log10_phi = phi.log10();
    let log10_sqrt5 = sqrt5.log10();

    let digits = 1000;
    let x = ((digits - 1) as f64 + log10_sqrt5) / log10_phi;
    let n = x.ceil() as u64;

    println!("{n}");
}
