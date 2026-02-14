// Project Euler 71: Ordered fractions
// Find the largest fraction n/d < 3/7 with d <= 1,000,000.
// For each d, the largest n with n/d < 3/7 is n = (3*d - 1) / 7.
// The best d has d % 7 != 0 and gives the tightest fraction.

fn main() {
    let limit = 1_000_000i64;
    let mut best_n = 0i64;
    let mut best_d = 1i64;

    for d in 1..=limit {
        let n = (3 * d - 1) / 7;
        // Compare n/d > best_n/best_d  <=>  n*best_d > best_n*d
        if n * best_d > best_n * d {
            best_n = n;
            best_d = d;
        }
    }

    println!("{best_n}");
}
