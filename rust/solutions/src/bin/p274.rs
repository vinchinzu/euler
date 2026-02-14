// Project Euler 274: Divisibility Multipliers
//
// For each prime p <= 10^7 with gcd(p, 10) = 1, find the modular inverse
// of 10 mod p. Sum all such inverses.

use euler_utils::{sieve, mod_inv};

fn main() {
    let limit = 10_000_000usize;
    let is_prime = sieve(limit);

    let mut ans: u64 = 0;
    for p in 2..=limit {
        if is_prime[p] && p != 2 && p != 5 {
            ans += mod_inv(10, p as u64).unwrap();
        }
    }

    println!("{}", ans);
}
