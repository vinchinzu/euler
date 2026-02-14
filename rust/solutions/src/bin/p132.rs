// Project Euler Problem 132: Large repunit factors.
//
// Sum the first 40 prime factors of R(10^9).
// A prime p divides R(10^9) iff 10^(10^9) ≡ 1 (mod 9p).

use euler_utils::{sieve, mod_pow};

const EXPONENT: u64 = 1_000_000_000;
const TARGET_COUNT: usize = 40;
const SIEVE_LIMIT: usize = 200_000;

fn main() {
    let is_p = sieve(SIEVE_LIMIT);

    let mut total: u64 = 0;
    let mut found = 0usize;

    for p in 2..=SIEVE_LIMIT {
        if !is_p[p] {
            continue;
        }
        if p == 2 || p == 5 {
            continue;
        }
        if found >= TARGET_COUNT {
            break;
        }

        let modulus = 9 * p as u64;
        if mod_pow(10, EXPONENT, modulus) == 1 {
            total += p as u64;
            found += 1;
        }
    }

    println!("{}", total);
}
