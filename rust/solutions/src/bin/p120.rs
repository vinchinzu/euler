// Project Euler 120: Square Remainders
// For each a in [3,1000], max remainder of (a-1)^n + (a+1)^n mod a^2.

use euler_utils::gcd;

fn main() {
    let sum: i64 = (3..=1000)
        .map(|a: i64| {
            let d = gcd(4u64, a as u64) as i64;
            let m = 2 + d * ((a - 3) / d);
            a * m
        })
        .sum();
    println!("{sum}");
}
