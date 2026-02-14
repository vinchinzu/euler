use std::f64::consts::PI;

const ITERATIONS: i32 = 10;

fn circle_area(k: f64) -> f64 {
    if k <= 0.0 { return 0.0; }
    let r = 1.0 / k;
    PI * r * r
}

fn recurse_area(k1: f64, k2: f64, k3: f64, depth: i32) -> f64 {
    if depth == 0 { return 0.0; }

    let sum_val = k1 + k2 + k3;
    let root_term = (k1 * k2 + k1 * k3 + k2 * k3).sqrt();
    let k_new = sum_val + 2.0 * root_term;

    let area_new = circle_area(k_new);
    area_new
        + recurse_area(k_new, k1, k2, depth - 1)
        + recurse_area(k_new, k1, k3, depth - 1)
        + recurse_area(k_new, k2, k3, depth - 1)
}

fn main() {
    let k_large = -1.0;
    let k_small = 1.0 + 2.0 / 3.0f64.sqrt();
    let area_large = PI;
    let small_area = PI * (1.0 / k_small) * (1.0 / k_small);

    let peripheral_extra = 3.0 * recurse_area(k_large, k_small, k_small, ITERATIONS);
    let central_extra = recurse_area(k_small, k_small, k_small, ITERATIONS);
    let extra_area = peripheral_extra + central_extra;
    let total_area = 3.0 * small_area + extra_area;
    let uncovered_fraction = 1.0 - (total_area / area_large);

    println!("{:.8}", uncovered_fraction);
}
