// Project Euler 226: A Scoop of Blancmange

fn main() {
    let mut low_x = 0.0f64;
    let mut high_x = 0.5f64;
    let mut high_y = 0.5f64;
    let mut blancmange_area = 0.0f64;

    let feq = |a: f64, b: f64| (a - b).abs() < 1e-13;

    while !feq(low_x, high_x) {
        let mid_x = (low_x + high_x) / 2.0;
        let mut mid_y = 0.0f64;
        let mut pow_val = 1.0f64;
        for _ in 0..50 {
            mid_y += (mid_x - (pow_val * mid_x).round() / pow_val).abs();
            pow_val *= 2.0;
        }

        let circle_y = 0.5 - (0.0625 - (0.25 - mid_x) * (0.25 - mid_x)).sqrt();

        if mid_y < circle_y {
            low_x = mid_x;
        } else {
            blancmange_area += ((high_x - mid_x) * (high_y + mid_y) +
                               (high_x - mid_x) * (high_x - mid_x)) / 2.0;
            high_x = mid_x;
            high_y = mid_y;
        }
    }

    let trapezoid_area = (0.5 - high_x) * (0.5 + high_y) / 2.0;
    let segment_area = (4.0 * high_x - 1.0).acos() / 32.0 - (0.5 - high_y) / 8.0;
    let ans = blancmange_area - trapezoid_area + segment_area;

    println!("{:.8}", ans);
}
