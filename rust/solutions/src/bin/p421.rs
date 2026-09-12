// Project Euler 421 - Prime factors of n^15+1
// For each prime p <= K, contribution is p * (number of n in [1,N] with p | n^15+1).

use rayon::prelude::*;

/// Modular exponentiation with pure u64 arithmetic.
/// SAFETY: modulus m <= 1e8, so (m-1)*(m-1) fits in u64.
#[inline(always)]
fn mod_pow_u64(mut base: u64, mut exp: u64, m: u64) -> u64 {
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

#[inline(always)]
fn contrib(p: u64, n: u64) -> i64 {
    let p_minus_1 = p - 1;
    let m3 = p_minus_1 % 3 == 0;
    let m5 = p_minus_1 % 5 == 0;
    let g_val = match (m3, m5) {
        (false, false) => 1,
        (true, false) => 3,
        (false, true) => 5,
        (true, true) => 15,
    };

    if g_val == 1 {
        // Only root of unity is 1.
        return p as i64 * ((n + 1) / p) as i64;
    }

    // Find a primitive g_val-th root of unity mod p.
    let exp = p_minus_1 / g_val;
    let mut nth_root = 1u64;
    for g in 2..p {
        let cand = mod_pow_u64(g, exp, p);
        if cand == 1 {
            continue;
        }
        if g_val == 3 || g_val == 5 {
            nth_root = cand;
            break;
        } else {
            // g_val == 15
            // cand != 1 already checked.
            // Check if cand has order 15: cand^3 != 1 and cand^5 != 1
            let c2 = cand * cand % p;
            let c3 = c2 * cand % p;
            if c3 == 1 {
                continue;
            }
            let c5 = c3 * c2 % p;
            if c5 == 1 {
                continue;
            }
            nth_root = cand;
            break;
        }
    }

    let mut sum = 0i64;
    let mut root = 1u64;
    for _ in 0..g_val {
        sum += p as i64 * ((n + root) / p) as i64;
        root = root * nth_root % p;
    }
    sum
}

fn main() {
    let n: u64 = 100_000_000_000; // 10^11
    let k: usize = 100_000_000; // 10^8

    // Fast odd-only bit sieve (6.25 MB total bitmap)
    let n_odds = k / 2;
    let num_words = (n_odds + 63) / 64;
    let mut composite_bits = vec![0u64; num_words];
    composite_bits[0] |= 1; // 1 is not prime

    let sqrt_k = (k as f64).sqrt() as usize;
    for i in 1..=(sqrt_k / 2) {
        if (composite_bits[i >> 6] & (1u64 << (i & 63))) == 0 {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                composite_bits[j >> 6] |= 1u64 << (j & 63);
                j += p;
            }
        }
    }

    let mut primes: Vec<u32> = Vec::with_capacity(5_761_455);
    primes.push(2u32);
    for i in 1..n_odds {
        if (composite_bits[i >> 6] & (1u64 << (i & 63))) == 0 {
            primes.push((2 * i + 1) as u32);
        }
    }
    drop(composite_bits);

    let ans: i64 = primes.into_par_iter().map(|p| contrib(p as u64, n)).sum();
    println!("{}", ans);
}
