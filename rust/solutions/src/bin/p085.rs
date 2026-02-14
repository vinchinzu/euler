// Project Euler 85: Counting rectangles
// Find area of grid closest to containing 2,000,000 rectangles.

fn main() {
    let target: i64 = 2_000_000;
    let mut closest_diff = target;
    let mut best_area = 0;

    for m in 1..=2000 {
        for n in 1..=m {
            let count = m * (m + 1) / 2 * (n * (n + 1) / 2);
            let diff = (count - target).abs();
            if diff < closest_diff {
                closest_diff = diff;
                best_area = m * n;
            }
            if count > target + closest_diff {
                break;
            }
        }
    }

    println!("{best_area}");
}
