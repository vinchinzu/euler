use num_integer::Integer;

use super::primes::miller_rabin;
use super::modular::mod_mul;

/// Greatest common divisor (unsigned).
#[inline]
pub fn gcd(a: u64, b: u64) -> u64 {
    a.gcd(&b)
}

/// Greatest common divisor (signed i64).
#[inline]
pub fn gcd_i64(a: i64, b: i64) -> i64 {
    a.gcd(&b)
}

/// Greatest common divisor (signed i32).
#[inline]
pub fn gcd_i32(a: i32, b: i32) -> i32 {
    a.gcd(&b)
}

/// Least common multiple.
#[inline]
pub fn lcm(a: u64, b: u64) -> u64 {
    a.lcm(&b)
}

/// Pollard's Rho factorization (Brent variant with batch GCD).
/// Returns a non-trivial factor of n. n must be composite and > 1.
fn pollard_rho(n: u64) -> u64 {
    if n % 2 == 0 { return 2; }
    if n % 3 == 0 { return 3; }
    let mut c = 1u64;
    loop {
        let f = |x: u64| (mod_mul(x, x, n) + c) % n;
        let mut x = c + 1;
        let mut y = x;
        let mut d = 1u64;
        // Brent's variant with batch GCD
        let mut r = 1u64;
        let mut q = 1u64;
        let mut ys = 0u64;
        loop {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            let mut k = 0u64;
            let mut found = false;
            while k < r && !found {
                ys = y;
                let batch = r - k;
                let batch = if batch > 128 { 128 } else { batch };
                for _ in 0..batch {
                    y = f(y);
                    q = mod_mul(q, x.abs_diff(y), n);
                }
                d = gcd(q, n);
                k += batch;
                if d != 1 {
                    found = true;
                }
            }
            if found { break; }
            r *= 2;
        }
        if d == n {
            // Backtrack: retry one step at a time from ys
            loop {
                ys = f(ys);
                d = gcd(x.abs_diff(ys), n);
                if d != 1 { break; }
            }
        }
        if d != n {
            return d;
        }
        c += 1; // try different c
    }
}

/// Prime factorization of n. Returns sorted vec of (prime, exponent) pairs.
pub fn factor(n: u64) -> Vec<(u64, u32)> {
    if n <= 1 {
        return vec![];
    }
    let mut factors: Vec<u64> = Vec::new();
    let mut stack = vec![n];
    while let Some(mut m) = stack.pop() {
        // Strip small prime factors by trial division
        for &p in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
            while m % p == 0 {
                factors.push(p);
                m /= p;
            }
        }
        if m == 1 {
            continue;
        }
        if miller_rabin(m) {
            factors.push(m);
            continue;
        }
        // Composite: split with Pollard Rho
        let d = pollard_rho(m);
        stack.push(d);
        stack.push(m / d);
    }
    factors.sort_unstable();
    // Compress to (prime, exponent) pairs
    let mut result: Vec<(u64, u32)> = Vec::new();
    for p in factors {
        if let Some(last) = result.last_mut() {
            if last.0 == p {
                last.1 += 1;
                continue;
            }
        }
        result.push((p, 1));
    }
    result
}

/// Euler's totient function φ(n). Uses Pollard Rho factorization.
pub fn euler_phi(n: u64) -> u64 {
    if n <= 1 { return n; }
    let mut result = n;
    for &(p, _) in &factor(n) {
        result -= result / p;
    }
    result
}

/// Return all divisors of n in sorted order. Uses Pollard Rho factorization.
pub fn divisors(n: u64) -> Vec<u64> {
    if n == 0 { return vec![]; }
    let factors = factor(n);
    let mut divs = vec![1u64];
    for &(p, e) in &factors {
        let prev_len = divs.len();
        let mut pk = 1u64;
        for _ in 0..e {
            pk *= p;
            for j in 0..prev_len {
                divs.push(divs[j] * pk);
            }
        }
    }
    divs.sort_unstable();
    divs
}

/// Count of divisors of n. Uses Pollard Rho factorization.
pub fn divisor_count(n: u64) -> u64 {
    if n == 0 { return 0; }
    factor(n).iter().map(|&(_, e)| e as u64 + 1).product()
}

/// Sum of divisors of n. Uses Pollard Rho factorization.
pub fn divisor_sum(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut sum = 1u64;
    for &(p, e) in &factor(n) {
        // σ(p^e) = (p^(e+1) - 1) / (p - 1)
        let mut s = 1u64;
        let mut pk = 1u64;
        for _ in 0..e {
            pk *= p;
            s += pk;
        }
        sum *= s;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 13), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 7), 21);
    }

    #[test]
    fn test_euler_phi() {
        assert_eq!(euler_phi(1), 1);
        assert_eq!(euler_phi(10), 4);
        assert_eq!(euler_phi(12), 4);
        assert_eq!(euler_phi(7), 6);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divisors(7), vec![1, 7]);
    }

    #[test]
    fn test_divisor_count() {
        assert_eq!(divisor_count(12), 6);
        assert_eq!(divisor_count(1), 1);
        assert_eq!(divisor_count(7), 2);
    }

    #[test]
    fn test_divisor_sum() {
        assert_eq!(divisor_sum(12), 28); // 1+2+3+4+6+12
        assert_eq!(divisor_sum(1), 1);
    }

    #[test]
    fn test_factor_trivial() {
        assert_eq!(factor(0), vec![]);
        assert_eq!(factor(1), vec![]);
        assert_eq!(factor(2), vec![(2, 1)]);
        assert_eq!(factor(3), vec![(3, 1)]);
        assert_eq!(factor(4), vec![(2, 2)]);
    }

    #[test]
    fn test_factor_composites() {
        assert_eq!(factor(12), vec![(2, 2), (3, 1)]);
        assert_eq!(factor(60), vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(factor(360), vec![(2, 3), (3, 2), (5, 1)]);
        assert_eq!(factor(1024), vec![(2, 10)]);
    }

    #[test]
    fn test_factor_primes() {
        assert_eq!(factor(97), vec![(97, 1)]);
        assert_eq!(factor(104729), vec![(104729, 1)]);
        assert_eq!(factor(999999937), vec![(999999937, 1)]);
    }

    #[test]
    fn test_factor_semiprimes() {
        // 104729 * 999999937
        let n: u64 = 104729 * 999999937;
        let f = factor(n);
        assert_eq!(f, vec![(104729, 1), (999999937, 1)]);
    }

    #[test]
    fn test_factor_large() {
        // 2^63 - 25 = 9223372036854775783, which is prime
        let p = 9223372036854775783u64;
        assert_eq!(factor(p), vec![(p, 1)]);

        // Product of two moderate primes
        let a = 1000000007u64;
        let b = 1000000009u64;
        let n = a * b;
        let f = factor(n);
        assert_eq!(f, vec![(a, 1), (b, 1)]);
    }

    #[test]
    fn test_factor_powers() {
        // 7^8 = 5764801
        assert_eq!(factor(5764801), vec![(7, 8)]);
        // 2^3 * 7^2 = 392
        assert_eq!(factor(392), vec![(2, 3), (7, 2)]);
    }
}
