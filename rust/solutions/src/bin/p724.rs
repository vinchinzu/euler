// Project Euler 724 - Drone Delivery
//
// Harmonic sum computation: sum_{i=1}^{N} H(i)/i, then multiply by N.

fn main() {
    let n: i64 = 100_000_000;
    let mut h: f64 = 0.0;
    let mut ans: f64 = 0.0;
    for i in 1..=n {
        h += 1.0 / i as f64;
        ans += h / i as f64;
    }
    println!("{}", (ans * n as f64) as i64);
}
