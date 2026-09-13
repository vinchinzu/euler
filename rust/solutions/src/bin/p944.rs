// Project Euler 944 - Sum of Elevisors
// S(n) = sum contributions from divisor pairs.
// Uses sqrt decomposition.

use rayon::prelude::*;

const MOD: u64 = 1234567891;
const MOD_EXP: u64 = MOD - 1;

#[inline(always)]
fn power(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    res
}

#[inline(always)]
fn compute_c(n_val: u64) -> u64 {
    let n1 = (n_val + 1) % MOD;
    let result = if n_val & 1 == 0 {
        (n_val / 2) * n1
    } else {
        n_val * (n1 / 2)
    };
    result % MOD
}

fn main() {
    let n: u64 = 100_000_000_000_000;

    let mut s = (n as f64).sqrt() as u64;
    while s * s > n {
        s -= 1;
    }
    while (s + 1) * (s + 1) <= n {
        s += 1;
    }

    let n_mod_exp = n % MOD_EXP;

    let sum1: u64 = (2..=s)
        .into_par_iter()
        .map(|d| {
            let c_val = compute_c((n / d) % MOD);
            let exp = (n_mod_exp + MOD_EXP - d % MOD_EXP) % MOD_EXP;
            let power_val = power(2, exp);
            power_val * c_val % MOD
        })
        .sum::<u64>()
        % MOD;

    let k_max = n / (s + 1);
    let c_k_max = compute_c(k_max % MOD);
    let term1 = c_k_max * power(2, (n - s) % MOD_EXP) % MOD;

    let sum_k: u64 = (1..=k_max)
        .into_par_iter()
        .map(|k| {
            let exp = (n - n / k) % MOD_EXP;
            (k % MOD) * power(2, exp) % MOD
        })
        .sum::<u64>()
        % MOD;

    let sum2 = (term1 + MOD - sum_k) % MOD;
    let result = (sum1 + sum2) % MOD;

    println!("{}", result);
}
