// Project Euler Problem 807: Loops of Ropes
// Compute P(n) = A(2n-1, n) / (2n-1)!
// where A(m,k) is the Eulerian number

use num::BigInt;
use num_traits::{One, Zero};

fn central_eulerian(n: u64) -> BigInt {
    if n == 0 {
        return BigInt::zero();
    }
    let m = 2 * n - 1;
    let mut total: BigInt = BigInt::zero();

    for j in 0..n {
        let base = n - j;
        // C(2n, j) * (n-j)^m
        let comb = comb_big(2 * n, j);
        let pow_val = pow_big(base, m);
        let term = &comb * &pow_val;

        if j & 1 == 1 {
            total -= term;
        } else {
            total += term;
        }
    }

    total
}

fn comb_big(n: u64, k: u64) -> BigInt {
    if k > n {
        return BigInt::zero();
    }
    let k = k.min(n - k);
    let mut result = BigInt::one();
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn pow_big(base: u64, exp: u64) -> BigInt {
    let mut result = BigInt::one();
    let mut b = BigInt::from(base);
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 {
            result *= &b;
        }
        b = &b * &b;
        e >>= 1;
    }
    result
}

fn factorial_big(n: u64) -> BigInt {
    let mut result = BigInt::one();
    for i in 2..=n {
        result *= i;
    }
    result
}

fn p_rounded_str(n: u64, digits: usize) -> String {
    let num = central_eulerian(n);
    let den = factorial_big(2 * n - 1);

    // Scale by 10^digits and round
    let scale = pow_big(10, digits as u64);
    let scaled = &num * &scale;
    let q = &scaled / &den;
    let r = &scaled % &den;

    // Round half-up
    let two_r = &r * 2;
    let rounded_q = if two_r >= den { &q + BigInt::one() } else { q };

    if digits == 0 {
        return rounded_q.to_string();
    }

    let int_part = &rounded_q / &scale;
    let frac_part = &rounded_q % &scale;

    // Convert to string and pad with zeros
    let int_str = int_part.to_string();
    let frac_str = frac_part.to_string();
    let frac_padded = format!("{:0>width$}", frac_str, width = digits);

    format!("{}.{}", int_str, &frac_padded[..digits])
}

fn main() {
    // Test: P(3) = 11/20 = 0.55
    let num3 = central_eulerian(3);
    let den3 = factorial_big(5);
    assert_eq!(&num3 * 20, &den3 * 11);

    // Test: P(5) rounded to 10 digits = "0.4304177690"
    assert_eq!(p_rounded_str(5, 10), "0.4304177690");

    println!("{}", p_rounded_str(80, 10));
}
