// Project Euler 363: Bezier Curves

fn bezier_x(t: f64, v: f64) -> f64 {
    let u = 1.0 - t;
    u*u*u + 3.0*u*u*t + 3.0*u*t*t*v
}

fn bezier_y(t: f64, v: f64) -> f64 {
    let u = 1.0 - t;
    3.0*u*u*t*v + 3.0*u*t*t + t*t*t
}

fn bezier_dx(t: f64, v: f64) -> f64 {
    let u = 1.0 - t;
    6.0*u*t*(v - 1.0) - 3.0*v*t*t
}

fn bezier_dy(t: f64, v: f64) -> f64 {
    let u = 1.0 - t;
    3.0*v*u*u + 6.0*(1.0 - v)*u*t
}

fn adaptive_simpson(
    f: fn(f64, f64) -> f64, v: f64,
    a: f64, b: f64, eps: f64,
    whole: f64, fa: f64, fb: f64, fc: f64,
    depth: i32, max_depth: i32,
) -> f64 {
    if depth > max_depth { return whole; }
    let c = (a + b) / 2.0;
    let h = b - a;
    let d = (a + c) / 2.0;
    let e = (c + b) / 2.0;
    let fd = f(d, v);
    let fe = f(e, v);
    let left = (h / 12.0) * (fa + 4.0 * fd + fc);
    let right = (h / 12.0) * (fc + 4.0 * fe + fb);
    let total = left + right;
    if (total - whole).abs() < 15.0 * eps {
        return total + (total - whole) / 15.0;
    }
    adaptive_simpson(f, v, a, c, eps / 2.0, left, fa, fc, fd, depth + 1, max_depth) +
    adaptive_simpson(f, v, c, b, eps / 2.0, right, fc, fb, fe, depth + 1, max_depth)
}

fn integrate(f: fn(f64, f64) -> f64, v: f64, eps: f64) -> f64 {
    let fa = f(0.0, v);
    let fb = f(1.0, v);
    let fc = f(0.5, v);
    let whole = (1.0 / 6.0) * (fa + 4.0 * fc + fb);
    adaptive_simpson(f, v, 0.0, 1.0, eps, whole, fa, fb, fc, 0, 30)
}

fn area_integrand(t: f64, v: f64) -> f64 {
    bezier_x(t, v) * bezier_dy(t, v)
}

fn speed_func(t: f64, v: f64) -> f64 {
    let dx = bezier_dx(t, v);
    let dy = bezier_dy(t, v);
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let pi = std::f64::consts::PI;
    let target = pi / 4.0;
    let mut low = 0.1f64;
    let mut high = 2.0f64;

    for _ in 0..200 {
        let mid = (low + high) / 2.0;
        let area = integrate(area_integrand, mid, 1e-16);
        if area < target { low = mid; } else { high = mid; }
    }
    let v = (low + high) / 2.0;

    let l = integrate(speed_func, v, 1e-16);
    let quarter_arc = pi / 2.0;
    let pct = 100.0 * (l - quarter_arc) / quarter_arc;

    println!("{:.10}", pct);
}
