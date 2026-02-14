// Project Euler 381: (prime-k) factorial
//
// Sum of S(p) for primes 5 <= p < 10^8.
// S(p) = sum_{k=1}^{5} (p-k)! mod p = (p-3)*inv(8,p) mod p by Wilson's theorem.

use euler_utils::{sieve, mod_inv};

fn main() {
    let limit = 100_000_000usize;
    let is_prime = sieve(limit - 1);

    let mut total: u64 = 0;
    for p in 5..limit {
        if is_prime[p] {
            let p64 = p as u64;
            let inv8 = mod_inv(8, p64).unwrap();
            total += ((p64 - 3) * inv8) % p64;
        }
    }

    println!("{}", total);
}
