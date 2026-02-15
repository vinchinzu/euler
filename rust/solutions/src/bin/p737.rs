// Project Euler 737 - Coin Loops
//
// Simulate coin placement on unit circle and count loops.

const NLOOPS: i32 = 2020;

fn main() {
    let mut x: f64 = 1.0;
    let mut y: f64 = 0.0;
    let mut last_cy: f64 = 0.0;
    let mut num_loops = 0;

    let mut k: i64 = 2;
    loop {
        let r2 = x * x + y * y;
        let l = (1.0 / r2 - 0.25).sqrt();
        let cx = x / 2.0 - y * l;
        let cy = y / 2.0 + x * l;
        x += (cx - x) / k as f64;
        y += (cy - y) / k as f64;

        if cy > 0.0 && last_cy < 0.0 {
            num_loops += 1;
        }
        last_cy = cy;

        if num_loops == NLOOPS {
            println!("{}", k);
            break;
        }
        k += 1;
    }
}
