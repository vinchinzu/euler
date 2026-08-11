// Project Euler 421 - Prime factors of n^15+1
// For each prime p <= K, contribution is p * (number of n in [1,N] with p | n^15+1).
// Optimized: local u64 modpow (p <= 1e8 so products fit u64), fast order check,
// and rayon over the prime list.

use rayon::prelude::*;

#[inline(always)]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

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

/// True iff `x` has multiplicative order exactly `g_val` modulo `p`.
/// `g_val` is always in {1, 3, 5, 15} = divisors of 15.
#[inline(always)]
fn has_order(x: u64, g_val: u64, p: u64) -> bool {
    if mod_pow_u64(x, g_val, p) != 1 {
        return false;
    }
    // Reject proper divisors of g_val.
    if g_val % 3 == 0 && mod_pow_u64(x, g_val / 3, p) == 1 {
        return false;
    }
    if g_val % 5 == 0 && mod_pow_u64(x, g_val / 5, p) == 1 {
        return false;
    }
    true
}

#[inline(always)]
fn contrib(p: u64, n: u64, r: u64) -> i64 {
    let g_val = gcd(p - 1, r);
    if g_val == 1 {
        // Only root of unity is 1.
        return p as i64 * ((n + 1) / p) as i64;
    }

    // Find a primitive g_val-th root of unity mod p.
    let exp = (p - 1) / g_val;
    let mut nth_root = 1u64;
    for g in 2..p {
        let cand = mod_pow_u64(g, exp, p);
        if has_order(cand, g_val, p) {
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
    let r: u64 = 15;

    // Sieve
    let mut is_composite = vec![false; k + 1];
    is_composite[0] = true;
    if k >= 1 {
        is_composite[1] = true;
    }
    let mut i = 2usize;
    while i * i <= k {
        if !is_composite[i] {
            let mut j = i * i;
            while j <= k {
                is_composite[j] = true;
                j += i;
            }
        }
        i += 1;
    }

    let primes: Vec<u64> = (2..=k as u64)
        .filter(|&p| !is_composite[p as usize])
        .collect();

    let ans: i64 = primes.into_par_iter().map(|p| contrib(p, n, r)).sum();
    println!("{}", ans);
}
