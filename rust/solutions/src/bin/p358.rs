// Project Euler 358: Cyclic Numbers
//
// Find the cyclic number with first 11 digits "00000000137" and last 5 digits "56789".
// Return the sum of all its digits.
//
// Key insight: For a full reptend prime p, the digit sum of the cyclic number
// (the repeating block of 1/p) is exactly 9*(p-1)/2. This avoids the O(p) long
// division loop entirely. We just need to verify the prime is full reptend by
// checking that 10 is a primitive root mod p.

use euler_utils::miller_rabin;

fn mod_inv(a: i64, m: i64) -> i64 {
    let (mut old_r, mut r) = (a, m);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        let tmp = r;
        r = old_r - q * r;
        old_r = tmp;
        let tmp = s;
        s = old_s - q * s;
        old_s = tmp;
    }
    ((old_s % m) + m) % m
}

/// Modular exponentiation: base^exp mod m, using u128 to avoid overflow
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

/// Trial division factorization. p-1 is at most ~730M so factors are small.
fn factor(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

/// Check if 10 is a primitive root mod p (i.e., ord(10, p) = p-1).
/// This means p is a full reptend prime.
/// We check that 10^((p-1)/q) != 1 (mod p) for each prime factor q of p-1.
fn is_full_reptend(p: u64) -> bool {
    let pm1 = p - 1;
    let factors = factor(pm1);
    for &q in &factors {
        if pow_mod(10, pm1 / q, p) == 1 {
            return false;
        }
    }
    true
}

fn main() {
    let lower = 100_000_000_000i64 / 138 + 1;
    let upper = 100_000_000_000i64 / 137;

    let inv = mod_inv(56789, 100000);
    let target_remainder = (99999i64 * inv) % 100000;

    let start = lower + (target_remainder - lower % 100000 + 100000) % 100000;

    let mut p = start;
    while p <= upper {
        if miller_rabin(p as u64) {
            // Verify first digits more precisely
            let first_digits = 10_000_000_000_000i64 / p;
            if first_digits >= 13700 && first_digits <= 13799 {
                // Verify last digits
                if (56789i64 * p + 1) % 100000 == 0 {
                    // Check full reptend via primitive root test
                    if is_full_reptend(p as u64) {
                        // For a full reptend prime, digit sum = 9*(p-1)/2
                        let digit_sum = 9i64 * (p - 1) / 2;
                        println!("{}", digit_sum);
                        return;
                    }
                }
            }
        }
        p += 100000;
    }
}
