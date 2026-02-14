// Project Euler 124 - Ordered radicals
//
// Compute rad(n) for 1..100000 via sieve, sort by (rad, n), return the 10000th element.

use euler_utils::sieve;

const LIMIT: usize = 100_000;
const TARGET_K: usize = 10_000;

fn main() {
    let is_prime = sieve(LIMIT);

    // Compute radicals via sieve
    let mut rad = vec![1u64; LIMIT + 1];
    for p in 2..=LIMIT {
        if is_prime[p] {
            for m in (p..=LIMIT).step_by(p) {
                rad[m] *= p as u64;
            }
        }
    }

    // Build pairs (rad, n) and sort
    let mut pairs: Vec<(u64, usize)> = (1..=LIMIT).map(|n| (rad[n], n)).collect();
    pairs.sort();

    println!("{}", pairs[TARGET_K - 1].1);
}
