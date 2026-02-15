use std::f64::consts::PI;

fn surface_area(a: f64, b: f64) -> f64 {
    if a >= b {
        let e = (1.0 - (b / a).powi(2)).sqrt();
        PI * (2.0 * a * a + b * b / e * ((1.0 + e) / (1.0 - e)).ln())
    } else {
        let e = (1.0 - (a / b).powi(2)).sqrt();
        2.0 * PI * (a * a + a * b / e * e.asin())
    }
}

fn main() {
    let a = 3.0f64;
    let b = 1.0f64;
    let t = 1.0f64;

    let ans = surface_area(a, b) * t
        + a * a * b * surface_area(1.0 / a, 1.0 / b) * t * t
        + 4.0 * PI / 3.0 * t * t * t;

    println!("{:.8}", ans);
}
