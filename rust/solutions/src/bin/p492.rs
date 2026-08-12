// Project Euler 492: Exploding sequence
//
// a_1 = 1, a_{n+1} = 6a_n^2 + 10a_n + 3
// Find sum of a_N (mod p) for all primes X <= p <= X+Y.
// Uses 2x2 matrix exponentiation + segmented sieve.

use rayon::prelude::*;

type U = u64;

#[derive(Clone, Copy)]
struct Mat2([U; 4]);

#[inline]
fn mat_mul(a: Mat2, b: Mat2, p: U) -> Mat2 {
    Mat2([
        ((a.0[0] as u128 * b.0[0] as u128 + a.0[1] as u128 * b.0[2] as u128) % p as u128) as U,
        ((a.0[0] as u128 * b.0[1] as u128 + a.0[1] as u128 * b.0[3] as u128) % p as u128) as U,
        ((a.0[2] as u128 * b.0[0] as u128 + a.0[3] as u128 * b.0[2] as u128) % p as u128) as U,
        ((a.0[2] as u128 * b.0[1] as u128 + a.0[3] as u128 * b.0[3] as u128) % p as u128) as U,
    ])
}

#[inline]
fn mat_pow(m: Mat2, mut exp: U, p: U) -> Mat2 {
    let mut result = Mat2([1, 0, 0, 1]);
    let mut base = Mat2([
        m.0[0] % p,
        ((m.0[1] % p) + p) % p,
        ((m.0[2] % p) + p) % p,
        m.0[3] % p,
    ]);
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(result, base, p);
        }
        base = mat_mul(base, base, p);
        exp >>= 1;
    }
    result
}

#[inline]
fn mod_pow_local(mut base: U, mut exp: U, m: U) -> U {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as U;
        }
        base = (base as u128 * base as u128 % m as u128) as U;
        exp >>= 1;
    }
    result
}

fn main() {
    let n_val: U = 1_000_000_000_000_000; // 10^15
    let x: U = 1_000_000_000; // 10^9
    let y: U = 10_000_000; // 10^7

    // Sieve small primes (odd-only)
    let sqrt_limit = ((x + y) as f64).sqrt() as usize + 1;
    let n_odds = (sqrt_limit + 1) / 2;
    let mut odd = vec![true; n_odds];
    odd[0] = false;
    let mut i = 1usize;
    while {
        let p = 2 * i + 1;
        p * p <= sqrt_limit
    } {
        if odd[i] {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                odd[j] = false;
                j += p;
            }
        }
        i += 1;
    }
    let mut small_primes: Vec<U> = Vec::with_capacity(n_odds / 5);
    small_primes.push(2);
    for i in 1..n_odds {
        if odd[i] {
            let p = 2 * i + 1;
            if p <= sqrt_limit {
                small_primes.push(p as U);
            }
        }
    }

    // Segmented sieve for [X, X+Y]
    let mut is_prime = vec![true; y as usize + 1];
    for &p in &small_primes {
        let start = if p * p > x {
            (p * p - x) as usize
        } else {
            let rem = x % p;
            if rem == 0 {
                0
            } else {
                (p - rem) as usize
            }
        };
        let mut j = start;
        while j <= y as usize {
            is_prime[j] = false;
            j += p as usize;
        }
    }

    // Collect primes then process in parallel (heavy mat_pow work per prime)
    let primes: Vec<U> = (0..=y as usize)
        .filter(|&i| is_prime[i])
        .map(|i| x + i as U)
        .collect();

    let ans: U = primes
        .par_iter()
        .map(|&p| {
            // A = [[0, 1], [-1, 11]]
            let a = Mat2([0, 1, p - 1, 11 % p]);

            let test = mat_pow(a, p - 1, p);
            let period = if test.0[0] == 1 && test.0[1] == 0 && test.0[2] == 0 && test.0[3] == 1
            {
                p - 1
            } else {
                p + 1
            };

            let exp_val = mod_pow_local(2, n_val - 1, period);
            let mat = mat_pow(a, exp_val, p);

            let x_n =
                ((2u128 * mat.0[0] as u128 + 11u128 * mat.0[1] as u128) % p as u128) as U;
            let a_n = ((x_n + p - 5 % p) % p * mod_pow_local(6, p - 2, p)) % p;
            a_n
        })
        .sum();

    println!("{}", ans);
}
