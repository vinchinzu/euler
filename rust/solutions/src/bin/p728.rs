// Project Euler 728 - Circle of Coins
//
// Sum 2^{(i-1)*g} * mult(i) for g=1..N/i, using Euler totient sieve.

use rayon::prelude::*;

const N: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result: u64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: u64, m: u64) -> u64 {
    let mut t: i64 = 0;
    let mut new_t: i64 = 1;
    let mut r: i64 = m as i64;
    let mut new_r: i64 = (a % m) as i64;
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if t < 0 {
        t += m as i64;
    }
    t as u64
}

fn contrib(i: usize, l: usize, phi: &[u64], pow2s: &[u64]) -> u64 {
    let mut res: u64 = 0;
    if i > 1 && i < l {
        let base = pow2s[i - 1];
        let max_g = N / i;
        if base == 1 {
            res = max_g as u64 % MOD;
        } else {
            let numerator = (pow_mod(base, max_g as u64 + 1, MOD) + MOD - 1) % MOD;
            let denominator = mod_inv((base + MOD - 1) % MOD, MOD);
            res = (numerator * denominator % MOD + MOD - 1) % MOD;
        }
    } else {
        for g in 1..=N / i {
            let idx = (i - 1) * g;
            if idx <= N {
                res = (res + pow2s[idx]) % MOD;
            } else {
                res = (res + pow_mod(2, idx as u64, MOD)) % MOD;
            }
        }
    }

    let phi_i = phi[i];
    let multiplier = if i == 1 || i % 2 == 0 {
        2 * phi_i % MOD
    } else {
        3 * phi_i / 2 % MOD
    };
    res * multiplier % MOD
}

fn main() {
    let mut phi = vec![0u64; N + 1];
    for i in 0..=N {
        phi[i] = i as u64;
    }
    for i in 2..=N {
        if phi[i] == i as u64 {
            let mut j = i;
            while j <= N {
                phi[j] -= phi[j] / i as u64;
                j += i;
            }
        }
    }

    let mut pow2s = vec![0u64; N + 1];
    pow2s[0] = 1;
    for i in 1..=N {
        pow2s[i] = pow2s[i - 1] * 2 % MOD;
    }

    let l = N / 30;
    let ans: u64 = (1..N + 1)
        .into_par_iter()
        .map(|i| contrib(i, l, &phi, &pow2s))
        .sum();

    println!("{}", ans % MOD);
}
