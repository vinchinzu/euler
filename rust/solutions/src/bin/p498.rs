// Project Euler 498 - Remainder of polynomial division
// Coefficient of x^D in remainder when x^N is divided by (x-1)^K.
// Uses Lucas theorem for nCk mod prime.

use euler_utils::{mod_pow, mod_mul};

const MOD: u64 = 999999937;

fn ncr_small(n: u64, k: u64, m: u64) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    if k == 0 {
        return 1;
    }
    let mut num = 1u64;
    let mut den = 1u64;
    for i in 1..=k {
        num = mod_mul(num, (n - k + i) % m, m);
        den = mod_mul(den, i % m, m);
    }
    mod_mul(num, mod_pow(den, m - 2, m), m)
}

fn ncr_lucas(mut n: u64, mut k: u64, m: u64) -> u64 {
    if k > n {
        return 0;
    }
    let mut result = 1u64;
    while n > 0 || k > 0 {
        let ni = n % m;
        let ki = k % m;
        if ki > ni {
            return 0;
        }
        result = mod_mul(result, ncr_small(ni, ki, m), m);
        n /= m;
        k /= m;
    }
    result
}

fn main() {
    let n: u64 = 10_000_000_000_000; // 10^13
    let k: u64 = 1_000_000_000_000;  // 10^12
    let d: u64 = 10_000;             // 10^4

    let n1 = n - d - 1;
    let k1 = k - 1 - d;

    let mut coeff = ncr_lucas(n, d, MOD);
    coeff = mod_mul(coeff, ncr_lucas(n1, k1, MOD), MOD);

    println!("{}", coeff);
}
