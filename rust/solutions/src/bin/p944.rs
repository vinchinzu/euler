// Project Euler 944 - Sum of Elevisors
// S(n) = sum contributions from divisor pairs.
// Uses sqrt decomposition.

fn power(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut res: i64 = 1;
    base %= modulus;
    if base < 0 {
        base += modulus;
    }
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    res
}

fn main() {
    const MOD: i64 = 1234567891;
    const MOD_EXP: i64 = MOD - 1;

    let n: i64 = 100_000_000_000_000; // 10^14

    let mut s = (n as f64).sqrt() as i64;
    while s * s > n {
        s -= 1;
    }
    while (s + 1) * (s + 1) <= n {
        s += 1;
    }

    let n_mod_exp = n % MOD_EXP;
    let mut sum1: i64 = 0;

    // Part 1: d = 2 to s
    for d in 2..=s {
        let n_div_d = n / d;
        // C(floor(n/d)) = floor(n/d) * (floor(n/d)+1) / 2 mod MOD
        let nd_mod = n_div_d % MOD;
        let nd1_mod = (n_div_d + 1) % MOD;
        let c_val = if nd_mod % 2 == 0 {
            ((nd_mod / 2) as i128 * nd1_mod as i128 % MOD as i128) as i64
        } else {
            (nd_mod as i128 * (nd1_mod / 2) as i128 % MOD as i128) as i64
        };

        // 2^(n-d) mod MOD
        let exp = (n_mod_exp - d % MOD_EXP + MOD_EXP) % MOD_EXP;
        let power_val = power(2, exp, MOD);

        let term = (power_val as i128 * c_val as i128 % MOD as i128) as i64;
        sum1 = (sum1 + term) % MOD;
    }

    // Part 2: d from s+1 to n, transformed to sum over k
    let k_max = n / (s + 1);

    // C(k_max) * 2^(n-s)
    let km_mod = k_max % MOD;
    let km1_mod = (k_max + 1) % MOD;
    let c_k_max = if km_mod % 2 == 0 {
        ((km_mod / 2) as i128 * km1_mod as i128 % MOD as i128) as i64
    } else {
        (km_mod as i128 * (km1_mod / 2) as i128 % MOD as i128) as i64
    };

    let mut exp_s = (n - s) % MOD_EXP;
    if exp_s < 0 {
        exp_s += MOD_EXP;
    }
    let term1 = (c_k_max as i128 * power(2, exp_s, MOD) as i128 % MOD as i128) as i64;

    // sum k * 2^(n - floor(n/k)) for k=1..k_max
    let mut sum_k: i64 = 0;
    for k in 1..=k_max {
        let n_div_k = n / k;
        let mut exp = (n - n_div_k) % MOD_EXP;
        if exp < 0 {
            exp += MOD_EXP;
        }
        let pv = power(2, exp, MOD);
        let tk = ((k % MOD) as i128 * pv as i128 % MOD as i128) as i64;
        sum_k = (sum_k + tk) % MOD;
    }

    let sum2 = (term1 - sum_k % MOD + MOD) % MOD;
    let result = (sum1 + sum2) % MOD;

    println!("{}", result);
}
