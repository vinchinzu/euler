// Project Euler 100: Arranged probability
// Find blue discs b where b*(b-1)/(n*(n-1)) = 1/2, n > 10^12.
// Recurrence from Pell equation.

fn main() {
    let target: i64 = 1_000_000_000_000;
    let mut b: i64 = 15;
    let mut n: i64 = 21;

    while n <= target {
        let b_next = 3 * b + 2 * n - 2;
        let n_next = 4 * b + 3 * n - 3;
        b = b_next;
        n = n_next;
    }

    println!("{b}");
}
