// Project Euler 724 - Drone Delivery
//
// E(n) = n/2 * (H_n^2 + H_n^{(2)}).
// H_n and H_n^{(2)} via Euler–Maclaurin (n = 1e8 is far into the asymptotic).

fn main() {
    let n: f64 = 100_000_000.0;
    const GAMMA: f64 = 0.5772156649015328606;
    let inv = 1.0 / n;
    let inv2 = inv * inv;
    let inv3 = inv2 * inv;
    let inv4 = inv2 * inv2;
    let inv5 = inv3 * inv2;
    let inv6 = inv4 * inv2;
    let inv7 = inv5 * inv2;
    let h = n.ln() + GAMMA + 0.5 * inv - inv2 / 12.0 + inv4 / 120.0 - inv6 / 252.0;
    let h2 = std::f64::consts::PI * std::f64::consts::PI / 6.0 - inv + 0.5 * inv2
        - inv3 / 6.0
        + inv5 / 30.0
        - inv7 / 42.0;
    let e = 0.5 * n * (h * h + h2);
    println!("{}", e.round() as i64);
}
