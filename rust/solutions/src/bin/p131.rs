// Project Euler 131: Prime cube partnership
// Primes of the form p = 3a^2 + 3a + 1 (consecutive cube differences).
// Count how many such primes below 1,000,000.

use euler_utils::is_prime;

fn main() {
    let mut count = 0u32;
    for a in 1.. {
        let p = 3 * a * a + 3 * a + 1;
        if p >= 1_000_000 {
            break;
        }
        if is_prime(p as u64) {
            count += 1;
        }
    }
    println!("{count}");
}
