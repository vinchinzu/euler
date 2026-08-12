// Project Euler 381: (prime-k) factorial
//
// Sum of S(p) for primes 5 <= p < 10^8.
// S(p) = sum_{k=1}^{5} (p-k)! mod p = (p-3)*inv(8,p) mod p by Wilson's theorem.

use rayon::prelude::*;

fn sieve_odds(limit: usize) -> Vec<usize> {
    // Bit sieve over odd numbers only: index i represents 2*i+1
    let n_odds = (limit + 1) / 2;
    let mut is_prime = vec![true; n_odds];
    is_prime[0] = false; // 1
    let mut i = 1usize; // start at 3
    while {
        let p = 2 * i + 1;
        p * p <= limit
    } {
        if is_prime[i] {
            let p = 2 * i + 1;
            // start at p*p, step 2p, map to odd indices
            let mut j = (p * p) / 2;
            while j < n_odds {
                is_prime[j] = false;
                j += p;
            }
        }
        i += 1;
    }
    let mut primes = Vec::with_capacity(n_odds / 5);
    primes.push(2);
    for i in 1..n_odds {
        if is_prime[i] {
            let p = 2 * i + 1;
            if p < limit {
                primes.push(p);
            }
        }
    }
    primes
}

fn main() {
    let limit = 100_000_000usize;
    let primes = sieve_odds(limit);

    // For odd p: inv(2)=(p+1)/2, inv(8)=inv(2)^3. Avoids extended gcd.
    let total: u64 = primes
        .par_iter()
        .filter(|&&p| p >= 5)
        .map(|&p| {
            let p64 = p as u64;
            let inv2 = (p64 + 1) / 2;
            let inv4 = inv2 * inv2 % p64;
            let inv8 = inv4 * inv2 % p64;
            ((p64 - 3) * inv8) % p64
        })
        .sum();

    println!("{}", total);
}
