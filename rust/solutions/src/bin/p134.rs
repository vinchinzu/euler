// Project Euler 134 - Prime pair connection
// For consecutive primes (p1, p2) with 5 <= p1 <= 1,000,000:
// Find smallest S divisible by p2 that ends with digits of p1.

use euler_utils::{sieve, mod_inv};

const LIMIT_P1: usize = 1_000_000;
const SIEVE_LIMIT: usize = LIMIT_P1 + 200;

fn num_digits(mut n: usize) -> u32 {
    let mut d = 0u32;
    while n > 0 {
        d += 1;
        n /= 10;
    }
    d
}

fn main() {
    let is_prime = sieve(SIEVE_LIMIT);
    let primes: Vec<usize> = (2..=SIEVE_LIMIT).filter(|&i| is_prime[i]).collect();

    let mut total: u64 = 0;

    for i in 0..primes.len() - 1 {
        let p1 = primes[i];
        if p1 < 5 {
            continue;
        }
        if p1 > LIMIT_P1 {
            break;
        }

        let p2 = primes[i + 1] as u64;
        let k = num_digits(p1);

        let mut m_val: u64 = 1;
        for _ in 0..k {
            m_val *= 10;
        }

        let m_inv = mod_inv(m_val, p2).unwrap();
        // t = (-p1 mod p2) * m_inv mod p2
        let neg_p1_mod = (p2 - (p1 as u64 % p2)) % p2;
        let t = (neg_p1_mod as u128 * m_inv as u128 % p2 as u128) as u64;
        let s = p1 as u64 + m_val * t;

        total += s;
    }

    println!("{}", total);
}
