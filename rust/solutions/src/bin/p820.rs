// Project Euler 820 - Nth digit of reciprocal
// d_n(1/k) via 10^{N-1} mod k

use rayon::prelude::*;

#[inline]
fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result: i64 = 1;
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

fn main() {
    let n: i64 = 10_000_000;
    let nu = n as usize;

    let mut pows = vec![0i64; nu + 1];

    // Upper half: independent pow_mod — parallelize
    let half = (n / 2) as usize;
    let upper: Vec<i64> = ((half + 1)..=nu)
        .into_par_iter()
        .map(|k| pow_mod(10, n - 1, k as i64))
        .collect();
    for (i, v) in upper.into_iter().enumerate() {
        pows[half + 1 + i] = v;
    }

    // Lower half: pows[k] = pows[2k] % k (and 10^{n-1} mod k for free via doubling)
    for k in (1..=half).rev() {
        pows[k] = pows[2 * k] % (k as i64);
    }

    let ans: i64 = (1..=nu)
        .into_par_iter()
        .map(|k| pows[k] * 10 / (k as i64))
        .sum();

    println!("{}", ans);
}
