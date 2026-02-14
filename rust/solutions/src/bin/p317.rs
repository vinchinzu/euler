use std::f64::consts::PI;

fn main() {
    let g = 9.81;
    let h0 = 100.0;
    let v0 = 20.0;

    let z_top = h0 + v0 * v0 / (2.0 * g);
    let volume = PI * v0 * v0 * z_top * z_top / g;

    println!("{:.4}", volume);
}
