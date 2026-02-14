// Project Euler 528 - Constrained Sums
//
// S(n, k, b) via inclusion-exclusion with nCr mod 10^9+7 for large n, small k.
// Answer: sum_{k=10}^{15} S(10^k, k, k) mod 10^9+7.

use euler_utils::mod_pow;

const MOD: u64 = 1_000_000_007;

fn ncr_mod(n: i64, k: i32) -> u64 {
    if k < 0 || (k as i64) > n { return 0; }
    if k == 0 { return 1; }

    let mut num = 1u64;
    for i in 0..k {
        num = (num as u128 * (((n - i as i64) % MOD as i64 + MOD as i64) % MOD as i64) as u128 % MOD as u128) as u64;
    }

    let mut den = 1u64;
    for i in 1..=k as u64 {
        den = (den as u128 * i as u128 % MOD as u128) as u64;
    }

    (num as u128 * mod_pow(den, MOD - 2, MOD) as u128 % MOD as u128) as u64
}

fn ipow(base: i64, exp: i32) -> i64 {
    let mut result = 1i64;
    for _ in 0..exp {
        result = result.checked_mul(base).unwrap_or(-1);
        if result < 0 { return -1; } // overflow sentinel
    }
    result
}

fn s_func(n: i64, k: i32, b: i32) -> u64 {
    let mut result = 0u64;

    for subset in 0..(1 << k) {
        let mut d: i64 = 0;
        let mut bits = 0i32;
        let mut overflow = false;

        for i in 0..k {
            if subset & (1 << i) != 0 {
                let pw = ipow(b as i64, i + 1);
                if pw < 0 { overflow = true; break; }
                d += pw + 1;
                if d < 0 { overflow = true; break; }
                bits += 1;
            }
        }

        if overflow || d > n + k as i64 { continue; }

        let rem = n - d + k as i64;
        if rem < 0 { continue; }

        let term = ncr_mod(rem, k);
        if bits % 2 == 0 {
            result = (result + term) % MOD;
        } else {
            result = (result + MOD - term % MOD) % MOD;
        }
    }

    result
}

fn main() {
    let mut ans = 0u64;

    for k in 10..=15i32 {
        let mut n = 1i64;
        for _ in 0..k { n *= 10; }
        ans = (ans + s_func(n, k, k)) % MOD;
    }

    println!("{ans}");
}
