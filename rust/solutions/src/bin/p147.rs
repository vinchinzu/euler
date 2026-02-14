// Project Euler 147: Rectangles in cross-hatched grids

fn upright(m: i64, n: i64) -> i64 {
    (m * (m + 1) / 2) * (n * (n + 1) / 2)
}

fn diagonal(m: i64, n: i64) -> i64 {
    let (m, n) = if m < n { (n, m) } else { (m, n) };
    n * ((2 * m - n) * (4 * n * n - 1) - 3) / 6
}

fn main() {
    let mut total: i64 = 0;
    for a in 1..=47 {
        for b in 1..=43 {
            total += upright(a, b) + diagonal(a, b);
        }
    }
    println!("{}", total);
}
