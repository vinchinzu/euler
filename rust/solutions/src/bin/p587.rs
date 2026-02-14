// Project Euler 587 - Concave Triangle
//
// Find the minimum n such that the L-section area divided by
// a ray with slope 1/n is less than 0.1% of the total L-section area.

fn f(n: i32) -> f64 {
    let nf = n as f64;
    let y = 1.0 / (nf + (2.0 * nf).sqrt() + 1.0);
    (1.0 - (nf - 1.0) * y - (1.0 - nf * y).asin()) / 2.0
}

fn main() {
    let r = 0.001;
    let total_area = 1.0 - std::f64::consts::PI / 4.0;

    let mut ans = 0;
    while f(ans) >= r * total_area {
        ans += 1;
    }

    println!("{}", ans);
}
