// Project Euler 613 - Pythagorean Ant
// Probability of exiting through hypotenuse, Simpson's rule integration

use std::f64::consts::PI;

fn simpson(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let n = if n % 2 == 1 { n + 1 } else { n };
    let h = (b - a) / n as f64;
    let mut result = f(a) + f(b);
    for i in 1..n {
        let coeff = if i % 2 == 1 { 4.0 } else { 2.0 };
        result += coeff * f(a + i as f64 * h);
    }
    result * h / 3.0
}

fn main() {
    let a_leg = 30.0_f64;
    let b_leg = 40.0_f64;

    let ans = (PI / 2.0
        + simpson(|x| 1.0 - b_leg * x.tan() / a_leg, 0.0, (a_leg / b_leg).atan(), 10000)
        + simpson(|x| 1.0 - a_leg * x.tan() / b_leg, 0.0, (b_leg / a_leg).atan(), 10000))
        / (2.0 * PI);

    println!("{:.10}", ans);
}
