// Project Euler 027: Quadratic Primes
// Find a*b for the quadratic n^2 + a*n + b producing the most consecutive primes.

use euler_utils::is_prime;

fn main() {
    // b must be prime (since n=0 gives b)
    let primes_b: Vec<i64> = (2..=1000)
        .filter(|&p| is_prime(p as u64))
        .collect();

    let mut max_count = 0;
    let mut best_product: i64 = 0;

    for a in -999..=999i64 {
        for &b in &primes_b {
            let mut n = 0i64;
            loop {
                let val = n * n + a * n + b;
                if val < 2 || !is_prime(val as u64) {
                    break;
                }
                n += 1;
            }
            if n > max_count {
                max_count = n;
                best_product = a * b;
            }
        }
    }

    println!("{best_product}");
}
