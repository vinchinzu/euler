// Project Euler 73: Counting fractions in a range
// Count fractions n/d strictly between 1/3 and 1/2 with d <= 12000 and gcd(n,d) = 1.

use euler_utils::gcd;

fn main() {
    let limit = 12_000u64;
    let mut count = 0u64;

    for d in 1..=limit {
        let n_min = d / 3 + 1;
        let n_max = (d - 1) / 2;
        for n in n_min..=n_max {
            if gcd(n, d) == 1 {
                count += 1;
            }
        }
    }

    println!("{count}");
}
