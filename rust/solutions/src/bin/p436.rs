// Project Euler 436: Unfair wager
fn main() {
    let e = std::f64::consts::E;
    let ans = (1.0 + 14.0 * e - 5.0 * e * e) / 4.0;
    println!("{:.10}", ans);
}
