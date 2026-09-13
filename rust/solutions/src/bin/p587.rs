// Project Euler 587 - Concave Triangle
//
// Find the minimum n such that the L-section area divided by
// a ray with slope 1/n is less than 0.1% of the total L-section area.

fn f(n: i32) -> f64 {
    let nf = n as f64;
    let sqrt_2n = (nf * 2.0).sqrt();
    let y = 1.0 / (nf + sqrt_2n + 1.0);
    (1.0 - (nf - 1.0) * y - (1.0 - nf * y).asin()) * 0.5
}

fn main() {
    const THRESHOLD: f64 = 0.001 * (1.0 - std::f64::consts::FRAC_PI_4);
    
    let mut lo = 1;
    let mut hi = 10000;
    
    while lo < hi {
        let mid = (lo + hi) / 2;
        if f(mid) < THRESHOLD {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    println!("{}", lo);
}
