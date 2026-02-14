// Project Euler 357: Prime Generating Integers
//
// Sum of all n <= 10^8 such that for every divisor d of n, d + n/d is prime.
// Key: n+1 must be prime, n must be even (or 1), d=2 check filters most.

use euler_utils::sieve;

fn main() {
    const LIMIT: usize = 100_000_001;

    let is_prime = sieve(LIMIT);

    let mut total: u64 = 0;

    // n=1: 1+1=2 is prime
    if is_prime[2] {
        total += 1;
    }

    // For n > 1, n must be even (n+1 is odd prime)
    let mut p: usize = 3;
    while p <= LIMIT {
        if !is_prime[p] {
            p += 2;
            continue;
        }

        let n = p - 1;
        if n > 100_000_000 {
            break;
        }

        // Quick check: d=2, 2 + n/2 must be prime
        if !is_prime[2 + n / 2] {
            p += 2;
            continue;
        }

        if check_divisors(n, &is_prime) {
            total += n as u64;
        }

        p += 2;
    }

    println!("{}", total);
}

fn check_divisors(n: usize, is_prime: &[bool]) -> bool {
    let mut d: usize = 2;
    while d * d <= n {
        if n % d == 0 {
            let quotient = n / d;
            if !is_prime[d + quotient] {
                return false;
            }
        }
        d += 1;
    }
    true
}
