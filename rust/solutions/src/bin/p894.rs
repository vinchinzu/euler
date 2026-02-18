// Problem 894 - Spiral of Circles
//
// C_k is produced from C_{k-1} by the same scaling (factor s, 0 < s < 1) and
// rotation (angle theta) about the origin. radius(C_k) = s^k, centres follow
// a logarithmic spiral.
//
// Given C0 externally tangent to C1, C7, C8 (no overlaps), find total area of
// all curvilinear triangles between mutually tangent circles.
//
// Tangency condition: d * |1 - z^n| = 1 + s^n  where z = s * e^{i*theta}.
// Two equations from n=7, n=8 matching n=1 ratio give (s, theta).
//
// Two triangle classes T1(k)=(C_k, C_{k+1}, C_{k+8}), T2(k)=(C_k, C_{k+7}, C_{k+8}).
// Total area = (A1 + A2) / (1 - s^2).

fn h(s: f64, theta: f64, n: i32) -> f64 {
    // |1 - (s e^{i theta})^n|^2 / (1 + s^n)^2
    let nf = n as f64;
    let sn = s.powi(n);
    let num = 1.0 + sn * sn - 2.0 * sn * (nf * theta).cos();
    let den = (1.0 + sn) * (1.0 + sn);
    num / den
}

fn f(s: f64, theta: f64) -> (f64, f64) {
    let h1 = h(s, theta, 1);
    (h1 - h(s, theta, 7), h1 - h(s, theta, 8))
}

fn objective(s: f64, theta: f64) -> f64 {
    let (f1, f2) = f(s, theta);
    f1 * f1 + f2 * f2
}

fn find_initial_guess() -> (f64, f64) {
    let mut best = (0.9, 0.8);
    let mut best_val = f64::INFINITY;

    let (s_min, s_max, ds) = (0.75, 0.99, 0.002);
    let (t_min, t_max, dt) = (0.05, std::f64::consts::PI - 0.05, 0.01);

    let mut s = s_min;
    while s <= s_max + 1e-12 {
        let mut theta = t_min;
        while theta <= t_max + 1e-12 {
            let val = objective(s, theta);
            if val < best_val {
                best_val = val;
                best = (s, theta);
            }
            theta += dt;
        }
        s += ds;
    }

    best
}

fn newton_solve(s0: f64, theta0: f64) -> (f64, f64) {
    let mut s = s0;
    let mut theta = theta0;

    for _ in 0..60 {
        let (f1, f2) = f(s, theta);
        if f1.abs().max(f2.abs()) < 1e-15 {
            return (s, theta);
        }

        let ds = 1e-8_f64;
        let dt = 1e-8_f64;

        let (f1_sp, f2_sp) = f(s + ds, theta);
        let (f1_sm, f2_sm) = f(s - ds, theta);
        let (f1_tp, f2_tp) = f(s, theta + dt);
        let (f1_tm, f2_tm) = f(s, theta - dt);

        let a = (f1_sp - f1_sm) / (2.0 * ds); // df1/ds
        let b = (f1_tp - f1_tm) / (2.0 * dt); // df1/dtheta
        let c = (f2_sp - f2_sm) / (2.0 * ds); // df2/ds
        let d = (f2_tp - f2_tm) / (2.0 * dt); // df2/dtheta

        let det = a * d - b * c;
        if det == 0.0 || !det.is_finite() {
            s *= 0.999;
            theta *= 0.999;
            continue;
        }

        let delta_s = (d * f1 - b * f2) / det;
        let delta_t = (-c * f1 + a * f2) / det;

        let cur_obj = f1 * f1 + f2 * f2;
        let mut step = 1.0_f64;
        let mut improved = false;
        for _ in 0..40 {
            let ns = s - step * delta_s;
            let nt = theta - step * delta_t;
            if !(0.0 < ns && ns < 1.0 && 0.0 < nt && nt < std::f64::consts::PI) {
                step *= 0.5;
                continue;
            }
            let nobj = objective(ns, nt);
            if nobj < cur_obj {
                s = ns;
                theta = nt;
                improved = true;
                break;
            }
            step *= 0.5;
        }
        if !improved {
            break;
        }
    }

    (s, theta)
}

fn clamp(x: f64, lo: f64, hi: f64) -> f64 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

fn curvilinear_triangle_area(r1: f64, r2: f64, r3: f64) -> f64 {
    // Triangle of centres has side lengths r_i + r_j
    let a = r2 + r3; // opposite r1
    let b = r1 + r3; // opposite r2
    let c = r1 + r2; // opposite r3

    let p = 0.5 * (a + b + c);
    let tri_sq = p * (p - a) * (p - b) * (p - c);
    let tri_sq = tri_sq.max(0.0);
    let tri_area = tri_sq.sqrt();

    // Angles at centres via law of cosines
    let cos1 = (b * b + c * c - a * a) / (2.0 * b * c);
    let cos2 = (a * a + c * c - b * b) / (2.0 * a * c);
    let cos3 = (a * a + b * b - c * c) / (2.0 * a * b);
    let ang1 = clamp(cos1, -1.0, 1.0).acos();
    let ang2 = clamp(cos2, -1.0, 1.0).acos();
    let ang3 = clamp(cos3, -1.0, 1.0).acos();

    let sectors = 0.5 * (r1 * r1 * ang1 + r2 * r2 * ang2 + r3 * r3 * ang3);
    tri_area - sectors
}

fn main() {
    let (s0, t0) = find_initial_guess();
    let (s, theta) = newton_solve(s0, t0);

    // Sanity checks
    let (f1, f2) = f(s, theta);
    assert!(0.0 < s && s < 1.0);
    assert!(0.0 < theta && theta < std::f64::consts::PI);
    assert!(f1.abs() < 1e-12 && f2.abs() < 1e-12);

    // Base curvilinear triangle areas
    let s7 = s.powi(7);
    let s8 = s7 * s;
    let a0 = curvilinear_triangle_area(1.0, s, s8);
    let b0 = curvilinear_triangle_area(1.0, s7, s8);

    let total = (a0 + b0) / (1.0 - s * s);
    println!("{:.10}", total);
}
