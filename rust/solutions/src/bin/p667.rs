// Project Euler 667 - Moving Pentagon
// Maximum area pentagon through L-shaped corridor via ternary search.

use std::f64::consts::PI;

fn corridor_ratio_at(sofa_a: f64, sofa_b: f64, t: f64) -> f64 {
    let dx = (sofa_a + sofa_b - t).cos();
    let dy = t.sin() + (sofa_a + sofa_b - t).sin();
    let ox = (sofa_a - t).cos() / (2.0 * sofa_a.cos());
    let oy = (sofa_a + t).sin() / (2.0 * sofa_a.cos());
    (oy / (dy - oy) - ox / (dx - ox)) / (1.0 / (dy - oy) - 1.0 / (dx - ox))
}

fn ternary_search_ratio(sofa_a: f64, sofa_b: f64, mut left: f64, mut right: f64) -> f64 {
    let eps = 1e-12;
    while right - left > eps {
        let m1 = left + (right - left) / 3.0;
        let m2 = right - (right - left) / 3.0;
        if corridor_ratio_at(sofa_a, sofa_b, m1) < corridor_ratio_at(sofa_a, sofa_b, m2) {
            left = m1;
        } else {
            right = m2;
        }
    }
    corridor_ratio_at(sofa_a, sofa_b, (left + right) / 2.0)
}

fn max_area(a: f64) -> f64 {
    let b = (1.0 / (4.0 * a.cos())).acos();
    let start_ratio = 0.5 * a.tan() + (b - a).sin();
    let middle_ratio = ternary_search_ratio(a, b, 0.0, PI / 2.0);
    let s_denom = start_ratio.max(middle_ratio);
    let s = 1.0 / s_denom;
    s * s * (a.tan() / 4.0 + b.tan() / (8.0 * a.cos() * a.cos()))
}

fn main() {
    let (mut left, mut right) = (0.0, PI / 3.0);
    let eps = 1e-12;
    while right - left > eps {
        let m1 = left + (right - left) / 3.0;
        let m2 = right - (right - left) / 3.0;
        if max_area(m1) < max_area(m2) { left = m1; } else { right = m2; }
    }
    println!("{:.10}", max_area((left + right) / 2.0));
}
