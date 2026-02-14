// Project Euler 358: Cyclic Numbers
//
// Find the cyclic number with first 11 digits "00000000137" and last 5 digits "56789".
// Return the sum of all its digits.

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
                    // Compute digit sum via long division and verify full reptend
                    let mut digit_sum: i64 = 0;
                    let mut n: i64 = 1;
                    let mut full_reptend = true;

                    for i in 1..p {
                        n *= 10;
                        let digit = n / p;
                        digit_sum += digit;
                        n %= p;

                        if n == 1 && i < p - 1 {
                            full_reptend = false;
                            break;
                        }
                    }

                    if full_reptend && n == 1 {
                        println!("{}", digit_sum);
                        return;
                    }
                }
            }
        }
        p += 100000;
    }
}
