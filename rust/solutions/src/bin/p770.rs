// Project Euler 770 - Deliberate Strategy
// f(n,n) ~ 2 - 2/(1+sqrt(pi*n)), solve for n >= (2/R-1)^-2 / pi.

fn main() {
    let r: f64 = 1.9999;
    let val = 2.0 / r - 1.0;
    let n = 1.0 / (val * val) / std::f64::consts::PI;
    let ans = n.ceil() as i64;
    println!("{}", ans);
}
