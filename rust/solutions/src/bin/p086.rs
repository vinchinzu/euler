// Project Euler 86: Cuboid route
// Find smallest M such that the number of cuboids with integer
// shortest surface path exceeds 1,000,000.

fn is_perfect_square(n: i64) -> bool {
    let root = (n as f64).sqrt() as i64;
    root * root == n || (root + 1) * (root + 1) == n
}

fn main() {
    let target = 1_000_000;
    let mut count = 0i64;

    for m in 1.. {
        for sum in 2..=2 * m {
            let dist_sq = (m as i64) * (m as i64) + (sum as i64) * (sum as i64);
            if is_perfect_square(dist_sq) {
                let min_b = 1i64.max(sum as i64 - m as i64);
                let max_b = sum as i64 / 2;
                if max_b >= min_b {
                    count += max_b - min_b + 1;
                }
            }
        }
        if count > target {
            println!("{m}");
            break;
        }
    }
}
