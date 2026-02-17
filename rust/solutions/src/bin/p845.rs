// Project Euler Problem 845: Prime Digit Sum
// Compute D(10^16), the 10^16-th positive integer with prime digit sum

use num::BigUint;
use num_traits::{One, Zero};

fn sieve_is_prime(limit: usize) -> Vec<bool> {
    if limit < 2 {
        return vec![false; limit + 1];
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrt = (limit as f64).sqrt() as usize;
    for p in 2..=sqrt {
        if is_prime[p] {
            let start = p * p;
            let step = p;
            for i in (start..=limit).step_by(step) {
                is_prime[i] = false;
            }
        }
    }
    is_prime
}

fn build_digit_sum_counts(max_len: usize) -> Vec<Vec<BigUint>> {
    let max_sum = 9 * max_len;
    let mut counts = vec![vec![BigUint::zero(); max_sum + 1]; max_len + 1];
    counts[0][0] = BigUint::one();

    for l in 1..=max_len {
        for s in 0..=(9 * l) {
            let mut total = BigUint::zero();
            for d in 0..=9 {
                if s >= d {
                    total += &counts[l - 1][s - d];
                }
            }
            counts[l][s] = total;
        }
    }
    counts
}

fn count_len_prime_digit_sum(length: usize, counts: &[Vec<BigUint>], is_prime: &[bool]) -> BigUint {
    if length == 0 {
        return BigUint::zero();
    }
    if length == 1 {
        return BigUint::from((1..=9).filter(|&d| is_prime[d]).count() as u64);
    }

    let mut total = BigUint::zero();
    let rem = length - 1;
    for first in 1..=9 {
        for tail_sum in 0..=(9 * rem) {
            if is_prime[first + tail_sum] {
                total += &counts[rem][tail_sum];
            }
        }
    }
    total
}

fn kth_len_number(
    length: usize,
    mut k: BigUint,
    counts: &[Vec<BigUint>],
    is_prime: &[bool],
) -> u64 {
    let mut digits = Vec::new();
    let mut prefix_sum = 0;

    for pos in 0..length {
        let rem = length - pos - 1;
        let start = if pos == 0 { 1 } else { 0 };

        for d in start..=9 {
            let mut cnt = BigUint::zero();
            let base = prefix_sum + d;
            for tail_sum in 0..=(9 * rem) {
                if is_prime[base as usize + tail_sum] {
                    cnt += &counts[rem][tail_sum];
                }
            }

            if k > cnt {
                k -= cnt;
            } else {
                digits.push(d);
                prefix_sum += d;
                break;
            }
        }
    }

    digits.iter().fold(0u64, |acc, &d| acc * 10 + d)
}

fn d(n: u64) -> u64 {
    let max_len = 25;
    let is_prime = sieve_is_prime(9 * max_len);
    let counts = build_digit_sum_counts(max_len);

    let mut remaining = BigUint::from(n);
    for length in 1..=max_len {
        let cnt_len = count_len_prime_digit_sum(length, &counts, &is_prime);
        if remaining > cnt_len {
            remaining -= cnt_len;
        } else {
            return kth_len_number(length, remaining, &counts, &is_prime);
        }
    }

    panic!("max_len too small");
}

fn main() {
    // Test: D(61) = 157
    assert_eq!(d(61), 157);

    // Test: D(10^8) = 403539364
    assert_eq!(d(100_000_000), 403539364);

    println!("{}", d(10_000_000_000_000_000u64));
}
