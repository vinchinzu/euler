// Project Euler 69: Totient maximum
// n/phi(n) is maximized when n is a product of the smallest primes (primorial).

use euler_utils::is_prime;

fn main() {
    let limit = 1_000_000u64;
    let mut result = 1u64;
    let mut p = 2u64;

    while result * p <= limit {
        result *= p;
        p += 1;
        while !is_prime(p) { p += 1; }
    }

    println!("{result}");
}
