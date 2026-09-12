// Project Euler 717 - Summation of a Modular Formula
//
// For each odd prime p <= 10^7, compute g(p) using modular exponentiation
// with Wieferich quotient-like calculation.

use rayon::prelude::*;

// m*m fits u64 (mod p or p-1, p <= 1e7).
fn pow_mod_u64(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

// m = p^2 ~ 1e14, so m*m does not fit u64.
fn pow_mod_wide(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result: u64 = 1;
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

fn main() {
    let n: usize = 10_000_000;

    // Sieve of Eratosthenes
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut primes = Vec::with_capacity(n / 10);
    for p in (3..=n).step_by(2) {
        if is_prime[p] {
            primes.push(p as u64);
        }
    }

    let ans: i64 = primes
        .par_iter()
        .map(|&pu| {
            let pm1 = pu - 1;
            let p2 = pu * pu;
            let exp_pm1 = pow_mod_u64(2, pu, pm1);
            let k = pow_mod_u64(2, exp_pm1 + pu - 2, pu);
            let t2p = pow_mod_wide(2, pu, p2);
            let num = (k as u128 * t2p as u128 % p2 as u128) as u64;
            ((num / pu) % pu) as i64
        })
        .sum();

    println!("{}", ans);
}
