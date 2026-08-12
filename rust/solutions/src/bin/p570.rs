// Project Euler 570 - Snowflakes
//
// GCD(2*4^{n-2} - 3^{n-2}, 7n+3) summed for n=3..10^7, times 6.

use rayon::prelude::*;

// m = 7n+3 <= 7e7+3; product of two residues fits in u64
#[inline]
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
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

#[inline]
fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    // Binary GCD
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            break;
        }
    }
    a << shift
}

fn main() {
    let n_max: u64 = 10_000_000;
    let ans: u64 = (3..=n_max)
        .into_par_iter()
        .map(|n| {
            let m = 7 * n + 3;
            let t1 = mod_pow(4, n - 2, m);
            let t2 = mod_pow(3, n - 2, m);
            let term = (2 * t1 % m + m - t2) % m;
            6 * gcd_u64(term, m)
        })
        .sum();
    println!("{}", ans);
}
