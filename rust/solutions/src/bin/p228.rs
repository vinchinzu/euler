// Project Euler 228: Minkowski Sums
// Count distinct edge angles in Minkowski sum of regular polygons S_1864..S_1909.

use euler_utils::euler_phi;

fn main() {
    let l = 1864;
    let m = 1909;
    let mut count: u64 = 0;

    for q in 1..=m {
        // Check if any multiple of q is in [l, m]
        let first_multiple = ((l + q - 1) / q) * q;
        if first_multiple <= m {
            count += euler_phi(q);
        }
    }

    println!("{}", count);
}
