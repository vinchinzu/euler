// Project Euler 695 - Random Rectangles
// Closed-form: (24*ln((3+sqrt(5))/4) + 22*sqrt(5) - 41) / 144

fn main() {
    let s5 = 5.0f64.sqrt();
    let ans = (24.0 * ((3.0 + s5) / 4.0).ln() + 22.0 * s5 - 41.0) / 144.0;
    println!("{:.10}", ans);
}
