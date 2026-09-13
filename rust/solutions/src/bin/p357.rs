// Project Euler 357: Prime Generating Integers
//
// Sum of all n <= 10^8 such that for every divisor d of n, d + n/d is prime.
// Key: n+1 must be prime, n must be even (or 1), d=2 check filters most.

use rayon::prelude::*;

fn sieve_odds(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![false; limit + 1];
    if limit >= 2 {
        is_prime[2] = true;
    }
    let n_odds = (limit + 1) / 2;
    let mut odd = vec![true; n_odds];
    odd[0] = false;
    let mut i = 1usize;
    while {
        let p = 2 * i + 1;
        p * p <= limit
    } {
        if odd[i] {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                odd[j] = false;
                j += p;
            }
        }
        i += 1;
    }
    for i in 1..n_odds {
        if odd[i] {
            let p = 2 * i + 1;
            if p <= limit {
                is_prime[p] = true;
            }
        }
    }
    is_prime
}

fn check_divisors(n: usize, is_prime: &[bool]) -> bool {
    let mut d: usize = 2;
    while d * d <= n {
        if n % d == 0 {
            let quotient = n / d;
            if !unsafe { *is_prime.get_unchecked(d + quotient) } {
                return false;
            }
        }
        d += 1;
    }
    true
}

fn main() {
    const LIMIT: usize = 100_000_001;

    let is_prime = sieve_odds(LIMIT);

    let mut total: u64 = if is_prime[2] { 1 } else { 0 };

    // Odd primes p = n+1; n = p-1 even
    let sum: u64 = (1..=LIMIT / 2)
        .into_par_iter()
        .filter_map(|k| {
            let p = 2 * k + 1; // odd numbers 3,5,7,...
            if p > LIMIT || !is_prime[p] {
                return None;
            }
            let n = p - 1;
            if n > 100_000_000 {
                return None;
            }
            if !is_prime[2 + n / 2] {
                return None;
            }
            if check_divisors(n, &is_prime) {
                Some(n as u64)
            } else {
                None
            }
        })
        .sum();

    total += sum;
    println!("{}", total);
}
