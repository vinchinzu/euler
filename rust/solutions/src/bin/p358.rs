// Project Euler 358: Cyclic Numbers
//
// Find the cyclic number with first 11 digits "00000000137" and last 5 digits "56789".
// Return the sum of all its digits.
//
// Key insight: For a full reptend prime p, the digit sum of the cyclic number
// (the repeating block of 1/p) is exactly 9*(p-1)/2. This avoids the O(p) long
// division loop entirely. We just need to verify the prime is full reptend by
// checking that 10 is a primitive root mod p.

use euler_utils::{factor, miller_rabin, mod_inv, mod_pow};

/// Check if 10 is a primitive root mod p (i.e., ord(10, p) = p-1).
/// This means p is a full reptend prime.
/// We check that 10^((p-1)/q) != 1 (mod p) for each prime factor q of p-1.
fn is_full_reptend(p: u64) -> bool {
    let pm1 = p - 1;
    let factors = factor(pm1);
    for (q, _) in &factors {
        if mod_pow(10, pm1 / q, p) == 1 {
            return false;
        }
    }
    true
}

fn main() {
    let lower = 100_000_000_000i64 / 138 + 1;
    let upper = 100_000_000_000i64 / 137;

    let inv = mod_inv(56789u64, 100000).unwrap() as i64;
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
