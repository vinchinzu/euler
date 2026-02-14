// Project Euler 285: Pythagorean Odds
fn area(r: f64) -> f64 {
    (std::f64::consts::FRAC_PI_4 - (1.0 / r).asin()) * r * r - ((r * r - 1.0).sqrt() - 1.0)
}

fn main() {
    let n = 100_000;
    let mut ans = 0.0;

    for k in 1..=n {
        let kf = k as f64;
        let mut a = area(kf + 0.5);
        if k > 1 {
            a -= area(kf - 0.5);
        }
        ans += kf * a / (kf * kf);
    }

    println!("{:.5}", ans);
}
