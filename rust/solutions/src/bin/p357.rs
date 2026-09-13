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
    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 1..n_odds {
        let p = 2 * i + 1;
        if p > sqrt_limit {
            break;
        }
        if odd[i] {
            let mut j = (p * p) / 2;
            while j < n_odds {
                odd[j] = false;
                j += p;
            }
        }
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
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            let quotient = n / d;
            // SAFETY: d >= 3, quotient = n/d, d+quotient < n+sqrt(n) < LIMIT
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
    const MAX_N: usize = 100_000_000;

    let is_prime = sieve_odds(LIMIT);

    let sum: u64 = (1..=MAX_N / 2)
        .into_par_iter()
        .filter_map(|k| {
            let p = 2 * k + 1;
            if p > LIMIT {
                return None;
            }
            // SAFETY: p <= LIMIT, is_prime has size LIMIT+1
            if !unsafe { *is_prime.get_unchecked(p) } {
                return None;
            }
            let n = p - 1;
            let half_n = n / 2;
            // SAFETY: n <= MAX_N, so 2 + half_n <= 2 + MAX_N/2 < LIMIT
            if !unsafe { *is_prime.get_unchecked(2 + half_n) } {
                return None;
            }
            if check_divisors(n, &is_prime) {
                Some(n as u64)
            } else {
                None
            }
        })
        .sum();

    println!("{}", 1 + sum);
}
