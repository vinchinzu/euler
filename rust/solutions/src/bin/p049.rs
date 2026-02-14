// Project Euler 49: Prime permutations
// Find the 3-term arithmetic sequence of 4-digit primes that are permutations of each other
// (other than the known 1487, 4817, 8147).
use euler_utils::sieve;

fn sorted_digits(n: u32) -> [u8; 4] {
    let mut d = [
        ((n / 1000) % 10) as u8,
        ((n / 100) % 10) as u8,
        ((n / 10) % 10) as u8,
        (n % 10) as u8,
    ];
    d.sort_unstable();
    d
}

fn main() {
    let is_prime = sieve(9999);
    let primes: Vec<u32> = (1000..10000).filter(|&i| is_prime[i as usize]).collect();

    for i in 0..primes.len() {
        for j in i + 1..primes.len() {
            if sorted_digits(primes[i]) != sorted_digits(primes[j]) {
                continue;
            }
            let diff = primes[j] - primes[i];
            let c = primes[j] + diff;
            if c >= 10000 {
                continue;
            }
            if !is_prime[c as usize] {
                continue;
            }
            if sorted_digits(primes[i]) != sorted_digits(c) {
                continue;
            }
            if primes[i] == 1487 {
                continue;
            }
            println!("{}{}{}", primes[i], primes[j], c);
            return;
        }
    }
}
