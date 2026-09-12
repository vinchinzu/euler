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

fn sieve_odds(limit: usize) -> Vec<u32> {
    let n_odds = (limit + 1) / 2;
    let num_words = (n_odds + 63) / 64;
    let mut composite_bits = vec![0u64; num_words];
    composite_bits[0] |= 1; // 1 is not prime

    let sqrt_lim = (limit as f64).sqrt() as usize;
    for i in 1..=(sqrt_lim / 2) {
        if (composite_bits[i >> 6] & (1u64 << (i & 63))) == 0 {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                composite_bits[j >> 6] |= 1u64 << (j & 63);
                j += p;
            }
        }
    }
    let mut primes = Vec::with_capacity(5_800_000);
    primes.push(2u32);
    for w in 0..num_words {
        let mut bits = !composite_bits[w];
        if w == 0 {
            bits &= !1; // 1 is not prime
        }
        let base_idx = w * 64;
        while bits != 0 {
            let tz = bits.trailing_zeros() as usize;
            let idx = base_idx + tz;
            if idx < n_odds {
                let p = (2 * idx + 1) as u32;
                if (p as usize) <= limit {
                    primes.push(p);
                }
            }
            bits &= bits - 1;
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

    // Pass 1: count factors per element in [0, B]
    let mut counts = vec![0u16; b + 1];
    for &p in &primes {
        let p = p as usize;
        let rem = (a % p as u64) as usize;
        let start = if rem == 0 { 0 } else { p - rem };
        let mut i = start;
        while i <= b {
            counts[i] += 1;
            i += p;
        }
    }

    // Pass 2: CSR offsets
    let mut offsets = vec![0usize; b + 2];
    for i in 0..=b {
        offsets[i + 1] = offsets[i] + counts[i] as usize;
    }
    let total_factors = offsets[b + 1];
    let mut factors = vec![0u32; total_factors];
    let mut cursors = offsets.clone();

    // Pass 3: populate flat CSR factors
    for &p in &primes {
        let pu = p;
        let p = p as usize;
        let rem = (a % p as u64) as usize;
        let start = if rem == 0 { 0 } else { p - rem };
        let mut i = start;
        while i <= b {
            factors[cursors[i]] = pu;
            cursors[i] += 1;
            i += p;
        }
    }
    drop(cursors);

    // Only iterate over primes in [A+1, A+B] (counts[i] == 0)
    let prime_indices: Vec<usize> = (1..=b).filter(|&i| counts[i] == 0).collect();

    let ans: u64 = prime_indices
        .into_par_iter()
        .map(|i| {
            let n = a + i as u64 - 1;
            let mut temp = n;
            let mut res: u64 = 1;

            let start_idx = offsets[i - 1];
            let end_idx = offsets[i];
            for &p in &factors[start_idx..end_idx] {
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
