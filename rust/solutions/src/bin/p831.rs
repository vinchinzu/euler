// Project Euler Problem 831: Triple Product
// Compute first 10 base-7 digits of g(142857)

use num::BigInt;
use num_traits::{One, Zero};

const DEG: usize = 5;

fn poly_mul_trunc(a: &[BigInt], b: &[BigInt], deg: usize) -> Vec<BigInt> {
    let mut res = vec![BigInt::zero(); deg + 1];
    for (i, ai) in a.iter().enumerate() {
        if ai.is_zero() {
            continue;
        }
        for (j, bj) in b.iter().enumerate() {
            if bj.is_zero() {
                continue;
            }
            let k = i + j;
            if k <= deg {
                res[k] = &res[k] + ai * bj;
            }
        }
    }
    res
}

fn poly_pow_trunc(base: &[BigInt], exp: u64, deg: usize) -> Vec<BigInt> {
    let mut res = vec![BigInt::zero(); deg + 1];
    res[0] = BigInt::one();
    let mut b = base.to_vec();
    let mut e = exp;

    while e > 0 {
        if e & 1 == 1 {
            res = poly_mul_trunc(&res, &b, deg);
        }
        e >>= 1;
        if e > 0 {
            b = poly_mul_trunc(&b, &b, deg);
        }
    }
    res
}

fn coeff_c(m: u64) -> BigInt {
    // Q(x) = 1 + 3x + 5x^2 + 5x^3 + 3x^4 + x^5
    let q: Vec<BigInt> = vec![1, 3, 5, 5, 3, 1]
        .into_iter()
        .map(BigInt::from)
        .collect();
    let qm = poly_pow_trunc(&q, m, DEG);

    // (1+x)^5 coefficients
    let a: Vec<BigInt> = vec![1, 5, 10, 10, 5, 1]
        .into_iter()
        .map(BigInt::from)
        .collect();

    let aq = poly_mul_trunc(&a, &qm, DEG);
    aq[DEG].clone()
}

fn to_base7(n: &BigInt) -> String {
    if n.is_zero() {
        return "0".to_string();
    }
    let mut n = n.clone();
    let seven = BigInt::from(7);
    let mut digits = Vec::new();
    while n > BigInt::zero() {
        let digit = &n % &seven;
        digits.push(digit.to_string().parse::<u8>().unwrap());
        n /= &seven;
    }
    digits.reverse();
    digits.iter().map(|&d| (b'0' + d) as char).collect()
}

fn first_digits_base7_of_g(m: u64, k: usize) -> String {
    let c = coeff_c(m);
    let s = to_base7(&c);

    if s.len() >= k {
        s[..k].to_string()
    } else {
        let need = k - s.len();
        format!("{}{}", s, "0".repeat(need))
    }
}

fn main() {
    // Test: g(10) = 127278262644918
    let c10 = coeff_c(10);
    let seven_10 = BigInt::from(7).pow(10);
    let g10 = &seven_10 * &c10;
    assert_eq!(g10.to_string(), "127278262644918");

    // Test: First 5 digits of g(10) in base 7 = "35544"
    let first5 = first_digits_base7_of_g(10, 5);
    assert_eq!(first5, "35544");

    println!("{}", first_digits_base7_of_g(142857, 10));
}
