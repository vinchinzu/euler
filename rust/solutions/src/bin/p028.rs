// Project Euler 028: Number Spiral Diagonals
// Sum of diagonals in a 1001x1001 number spiral.

fn main() {
    let n: i64 = 1001;
    let m = (n - 1) / 2;

    let sum_k2 = m * (m + 1) * (2 * m + 1) / 6;
    let sum_k = m * (m + 1) / 2;

    let total = 1 + 16 * sum_k2 + 4 * sum_k + 4 * m;

    println!("{total}");
}
