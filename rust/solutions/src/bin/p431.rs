use std::f64::consts::PI;

const R_VAL: f64 = 6.0;

fn integrand(theta: f64, x: f64) -> f64 {
    let beta = (x * theta.sin() / R_VAL).asin();
    let pq = x * theta.cos() + R_VAL * beta.cos();
    pq * pq * pq
}

fn integrate_simpson(x: f64, a: f64, b: f64, n: i32) -> f64 {
    let h = (b - a) / n as f64;
    let mut result = integrand(a, x) + integrand(b, x);
    for i in 1..n {
        let t = a + i as f64 * h;
        if i % 2 == 0 {
            result += 2.0 * integrand(t, x);
        } else {
            result += 4.0 * integrand(t, x);
        }
    }
    result * h / 3.0
}

fn v(x: f64, alpha_rad: f64) -> f64 {
    integrate_simpson(x, 0.0, 2.0 * PI, 1000) * alpha_rad.tan() / 3.0
}

fn is_square(n: f64) -> bool {
    let root = n.sqrt().round() as i64;
    (root as f64 * root as f64 - n).abs() < 1e-9
}

fn main() {
    let alpha_rad = 40.0 * PI / 180.0;

    let lower = v(0.0, alpha_rad);
    let higher = v(R_VAL, alpha_rad);
    let mut ans = 0.0f64;

    for v_val in 1..=(higher as i64) {
        if v_val as f64 > lower && is_square(v_val as f64) {
            let mut low = 0.0f64;
            let mut high = R_VAL;
            while (high - low).abs() > 1e-12 {
                let mid = (low + high) / 2.0;
                if v(mid, alpha_rad) < v_val as f64 {
                    low = mid;
                } else {
                    high = mid;
                }
            }
            ans += low;
        }
    }

    println!("{:.9}", ans);
}
