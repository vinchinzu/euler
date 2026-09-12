// Project Euler 437 - Fibonacci primitive roots
// Sum of primes p <= 10^8 that have a Fibonacci primitive root.

use rayon::prelude::*;

const N: usize = 100_000_000;

/// Fast iterative Fibonacci pair (F_n, F_{n+1}) mod m.
/// Since m <= 1e8, a*a + b*b < 2e16 < 2^64, so reduction is deferred.
#[inline(always)]
fn fib_pair(n: u64, m: u64) -> (u64, u64) {
    if n == 0 {
        return (0, 1);
    }
    let mut a = 0u64;
    let mut b = 1u64;
    let bits = 64 - n.leading_zeros();
    for i in (0..bits).rev() {
        let two_b = if (b << 1) >= m { (b << 1) - m } else { b << 1 };
        let diff = if two_b >= a { two_b - a } else { two_b + m - a };
        let c = (a * diff) % m;
        let d = (a * a + b * b) % m;
        if (n >> i) & 1 == 1 {
            let next_b = if c + d >= m { c + d - m } else { c + d };
            a = d;
            b = next_b;
        } else {
            a = c;
            b = d;
        }
    }
    (a, b)
}

fn main() {
    // Sieve small primes up to 10,000 for factoring p - 1
    const SQRT_N: usize = 10_000;
    let mut is_small_prime = [true; SQRT_N + 1];
    is_small_prime[0] = false;
    is_small_prime[1] = false;
    let mut p = 2;
    while p * p <= SQRT_N {
        if is_small_prime[p] {
            let mut j = p * p;
            while j <= SQRT_N {
                is_small_prime[j] = false;
                j += p;
            }
        }
        p += 1;
    }
    // Small primes starting from 3
    let mut small_primes = Vec::with_capacity(1250);
    for i in 3..=SQRT_N {
        if is_small_prime[i] {
            small_primes.push(i as u32);
        }
    }

    // Fast odd-only bit sieve for primes <= 10^8 (6.25 MB)
    let n_odds = N / 2;
    let num_words = (n_odds + 63) / 64;
    let mut composite_bits = vec![0u64; num_words];
    composite_bits[0] |= 1; // 1 is not prime

    for i in 1..=(SQRT_N / 2) {
        if (composite_bits[i >> 6] & (1u64 << (i & 63))) == 0 {
            let pr = 2 * i + 1;
            let mut j = (pr * pr) / 2;
            while j < n_odds {
                composite_bits[j >> 6] |= 1u64 << (j & 63);
                j += pr;
            }
        }
    }

    // Collect candidate primes (p = 5 and primes with p % 10 in {1, 9})
    let mut candidates: Vec<u32> = Vec::with_capacity(3_000_000);
    candidates.push(5u32);
    for i in 1..n_odds {
        if (composite_bits[i >> 6] & (1u64 << (i & 63))) == 0 {
            let pr = (2 * i + 1) as u32;
            let mod10 = pr % 10;
            if mod10 == 1 || mod10 == 9 {
                candidates.push(pr);
            }
        }
    }
    drop(composite_bits);

    let sum: u64 = candidates
        .par_iter()
        .map(|&p| {
            if p == 5 {
                return 5u64;
            }
            let p64 = p as u64;

            // Factor p - 1: 2 is always a factor
            let mut n = (p - 1) >> 1;
            while n & 1 == 0 {
                n >>= 1;
            }
            let mut factors = [0u32; 16];
            factors[0] = 2;
            let mut nf = 1;

            for &q in &small_primes {
                if (q as u64) * (q as u64) > n as u64 {
                    break;
                }
                if n % q == 0 {
                    factors[nf] = q;
                    nf += 1;
                    n /= q;
                    while n % q == 0 {
                        n /= q;
                    }
                }
            }
            if n > 1 {
                factors[nf] = n;
                nf += 1;
            }

            for &q in &factors[..nf] {
                let (f, g) = fib_pair((p64 - 1) / q as u64, p64);
                if f == 0 && g == 1 {
                    return 0;
                }
            }

            p64
        })
        .sum();

    println!("{}", sum);
}
