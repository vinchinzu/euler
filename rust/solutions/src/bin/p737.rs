// Project Euler 737 - Coin Loops
//
// Simulate coin placement on unit circle and count loops.
// Optimized: reformulate to 2 independent sqrts + 1 division per iteration.
// The original needs 2 divisions + 1 sqrt. Trading one division for one sqrt
// is beneficial when the two sqrts can execute in parallel on the sqrt unit.

const NLOOPS: i32 = 2020;

fn main() {
    let mut x: f64 = 1.0;
    let mut y: f64 = 0.0;
    let mut last_cy_neg = false;
    let mut num_loops: i32 = 0;

    let mut k: i64 = 2;
    loop {
        let r2 = x * x + y * y;
        // Two independent sqrts (can pipeline on modern CPUs):
        let r = r2.sqrt();         // sqrt(x^2 + y^2)
        let s = (4.0 - r2).sqrt(); // sqrt(4 - x^2 - y^2)

        // These 4 products can all start once r and s are available
        let xr = x * r;
        let yr = y * r;
        let xs = x * s;
        let ys = y * s;

        // cy = (y*r + x*s) / (2*r), sign is same as y*r + x*s since r > 0
        let cy_sum = yr + xs;

        // Single division: 1/(2*k*r) merges both 1/r2 and 1/k from original
        let inv_2kr = 1.0 / (2.0 * k as f64 * r);

        // State update
        x -= (xr + ys) * inv_2kr;
        y += (xs - yr) * inv_2kr;

        // Zero-crossing detection (positive crossing)
        if cy_sum > 0.0 && last_cy_neg {
            num_loops += 1;
            if num_loops == NLOOPS {
                println!("{}", k);
                return;
            }
        }
        last_cy_neg = cy_sum < 0.0;

        k += 1;
    }
}
