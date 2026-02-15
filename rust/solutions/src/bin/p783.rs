// Project Euler 783 - Urns Balls Black White
// Hypergeometric recurrence tracking E[C_t] and E[C_t^2] with f64.

fn main() {
    let n = 1_000_000;
    let k = 10;

    let mut mu: f64 = 0.0;
    let mut nu: f64 = 0.0;
    let mut total: f64 = 0.0;
    let kf = k as f64;
    let k2 = kf * kf;

    for t in 1..=n {
        let alpha = (n - t + 2) as f64;
        let m = kf * alpha;

        let p = mu + kf;
        let q = nu + 2.0 * kf * mu + k2;

        let eb2 = 2.0 / (alpha * (m - 1.0)) * (kf * (alpha - 2.0) * p + (2.0 * kf - 1.0) * q);
        total += eb2;

        if alpha > 2.0 {
            mu = p * (alpha - 2.0) / alpha;
            nu = (alpha - 2.0) / (alpha * (m - 1.0))
                * ((m - 2.0 * kf - 1.0) * q + 2.0 * kf * p);
        } else {
            mu = 0.0;
            nu = 0.0;
        }
    }

    println!("{}", total.round() as i64);
}
