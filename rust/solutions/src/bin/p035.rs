// Project Euler 035: Circular Primes
// Count circular primes below one million.

use euler_utils::sieve;

fn main() {
    const LIMIT: usize = 1_000_000;
    let is_prime = sieve(LIMIT);

    let count = (2..LIMIT)
        .filter(|&n| is_prime[n] && is_circular_prime(n, &is_prime))
        .count();

    println!("{count}");
}

fn is_circular_prime(n: usize, is_prime: &[bool]) -> bool {
    // Multi-digit primes with even digits or 5 can't be circular
    if n >= 10 {
        let mut tmp = n;
        while tmp > 0 {
            let d = tmp % 10;
            if d == 0 || d == 2 || d == 4 || d == 5 || d == 6 || d == 8 {
                return false;
            }
            tmp /= 10;
        }
    }

    let len = if n == 0 { 1 } else { (n as f64).log10() as u32 + 1 };
    let pow10 = 10usize.pow(len - 1);
    let mut rotated = n;

    for _ in 0..len {
        let first = rotated / pow10;
        rotated = (rotated % pow10) * 10 + first;
        if rotated >= is_prime.len() || !is_prime[rotated] {
            return false;
        }
    }
    true
}
