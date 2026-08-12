// Project Euler 801 - x^y = y^x (mod n)
// Sieve primes in [A, A+B], factor p-1, compute multiplicative function

use rayon::prelude::*;

#[inline]
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    // modulus = 993353399 < 2^32 → u64 mul is safe
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn sieve_odds(limit: usize) -> Vec<usize> {
    let n_odds = (limit + 1) / 2;
    let mut is_prime = vec![true; n_odds];
    is_prime[0] = false;
    let mut i = 1usize;
    while {
        let p = 2 * i + 1;
        p * p <= limit
    } {
        if is_prime[i] {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                is_prime[j] = false;
                j += p;
            }
        }
        i += 1;
    }
    let mut primes = Vec::with_capacity(n_odds / 5);
    primes.push(2);
    for i in 1..n_odds {
        if is_prime[i] {
            let p = 2 * i + 1;
            if p <= limit {
                primes.push(p);
            }
        }
    }
    primes
}

fn main() {
    let a: u64 = 10_000_000_000_000_000; // 10^16
    let b: usize = 1_000_000;
    let m: u64 = 993_353_399;
    let l = ((a + b as u64) as f64).sqrt() as usize + 1;

    let primes = sieve_odds(l);

    // factor_lists[i] = small prime factors of (A+i)
    let mut factor_count = vec![0u16; b + 1];
    let mut factor_lists: Vec<Vec<u32>> = vec![Vec::new(); b + 1];

    for &p in &primes {
        let rem = (a % p as u64) as usize;
        let start = if rem == 0 { 0 } else { p - rem };
        let mut i = start;
        let p32 = p as u32;
        while i <= b {
            if (p as u64) < a + i as u64 {
                factor_count[i] += 1;
                factor_lists[i].push(p32);
            }
            i += p;
        }
    }

    let ans: u64 = (1..=b)
        .into_par_iter()
        .filter(|&i| factor_count[i] == 0)
        .map(|i| {
            let n = a + i as u64 - 1;
            let mut temp = n;
            let mut res: u64 = 1;

            for &p in &factor_lists[i - 1] {
                let p64 = p as u64;
                if temp % p64 != 0 {
                    continue;
                }
                let mut e: u64 = 0;
                while temp % p64 == 0 {
                    temp /= p64;
                    e += 1;
                }
                let term = (mod_pow(p64, 3 * e, m) + mod_pow(p64, 3 * e - 1, m) + m
                    - mod_pow(p64, 2 * e - 1, m))
                    % m;
                res = res * term % m;
            }

            if temp > 1 {
                let p = temp;
                let term =
                    (mod_pow(p, 3, m) + mod_pow(p, 2, m) + m - mod_pow(p, 1, m)) % m;
                res = res * term % m;
            }

            (mod_pow(n, 2, m) + res) % m
        })
        .sum::<u64>()
        % m;

    println!("{}", ans);
}
