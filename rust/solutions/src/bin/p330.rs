// Project Euler 330: Euler's Number
//
// a(n) is defined for all integers n as:
//   a(n) = 1 for n < 0
//   a(n) = sum_{i>=1} a(n-i)/i! for n >= 0
//
// Each a(n) = (A(n)*e + B(n)) / n! where A(n), B(n) are integers.
// Find (A(10^9) + B(10^9)) mod 77777777.
//
// Approach: Let h(n) = A(n) + B(n). The EGF H(x) = (1-e^x)/((1-x)*(2-e^x)).
// Define d(n) = h(n) - n*h(n-1) with h(0)=0, d(0)=0.
// d satisfies: d(n) = sum_{k=1}^n C(n,k)*d(n-k) - 1 for n >= 1.
// Key observations (mod prime p):
//   - d(n) mod p has period p-1
//   - h(n) mod p has period p*(p-1)
// Use CRT over prime factors of 77777777 = 7*11*73*101*137.

use euler_utils::{crt, BinomialMod};

fn compute_h_mod_p(p: i64, target_n: i64) -> i64 {
    let period_d = (p - 1) as usize; // d has period p-1

    // Precompute binomial coefficients mod p using euler_utils
    let binom = BinomialMod::new(period_d, p as u64);

    // Compute d(n) mod p for n = 0..period_d-1
    let mut d_vals = vec![0i64; period_d];
    // d(0) = 0 (already set)
    for n in 1..period_d {
        let mut s = 0i64;
        for k in 1..=n {
            s = (s + binom.choose(n, k) as i64 * d_vals[n - k]) % p;
        }
        d_vals[n] = ((s - 1) % p + p) % p;
    }

    // h(n) = d(n) + n * h(n-1), h(0) = 0
    // h has period p*(p-1)
    let period_h = (p * (p - 1)) as usize;

    let mut h_prev = 0i64; // h(0)
    let target_idx = (target_n % period_h as i64) as usize;

    if target_idx == 0 {
        return 0;
    }

    for n in 1..=target_idx {
        let d_n = d_vals[n % period_d];
        let h_n = (d_n + (n as i64 % p) * h_prev % p) % p;
        h_prev = h_n;
    }

    h_prev
}

fn main() {
    let n: i64 = 1_000_000_000;

    let primes = [7i64, 11, 73, 101, 137];
    let mut residues = Vec::with_capacity(5);

    for &p in &primes {
        residues.push(compute_h_mod_p(p, n));
    }

    let result = crt(&residues, &primes).unwrap().0;
    println!("{}", result);
}
