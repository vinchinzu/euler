// Project Euler 10: Sum of primes below 2 million
use euler_utils::sieve;

fn main() {
    let is_prime = sieve(2_000_000);
    let sum: u64 = (2..2_000_000)
        .filter(|&i| is_prime[i])
        .map(|i| i as u64)
        .sum();
    println!("{sum}");
}
