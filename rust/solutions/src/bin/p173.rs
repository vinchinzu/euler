// Project Euler 173: Square laminae using up to 1 million tiles.

fn main() {
    let limit: i64 = 1_000_000;
    let mut total: i64 = 0;
    let max_k = ((limit as f64 / 4.0).sqrt()) as i64;

    for k in 1..=max_k {
        let max_m = limit / (4 * k) - k;
        if max_m >= 1 {
            total += max_m;
        }
    }

    println!("{}", total);
}
