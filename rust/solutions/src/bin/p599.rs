// Project Euler 599 - Distinct Colorings of a Rubik's Cube
//
// Count the number of distinct colorings of a 2x2x2 Rubik's cube using N=10 colors.

fn comb(n: i64, k: i64) -> i64 {
    if k < 0 || k > n { return 0; }
    let k = k.min(n - k);
    let mut result: i64 = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn main() {
    let n: i64 = 10;

    let num_multicolored_corners = n * (n - 1) * (n - 2) / 3 + n * (n - 1);
    let num_corners = num_multicolored_corners + n;

    let ans = comb(num_corners + 7, 8)
        + 2 * comb(num_multicolored_corners + 7, 8);

    println!("{}", ans);
}
