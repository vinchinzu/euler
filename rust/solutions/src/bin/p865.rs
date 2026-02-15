// Project Euler 865 - Triplicate Numbers
// T(10^4) mod 998244353 via recurrence DP

const MOD: i64 = 998244353;
const LIMIT: usize = 3334;

fn power(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut res: i64 = 1;
    base = base.rem_euclid(modulus);
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
    let n = 10000;
    let limit = n / 3;

    let inv10 = power(10, MOD - 2, MOD);

    let mut dp = vec![0i64; LIMIT + 1];
    let mut prim = vec![0i64; LIMIT + 1];
    let mut v = vec![0i64; LIMIT + 1];

    dp[0] = 1;
    v[0] = 1;

    for m in 1..=limit {
        // 1. Compute dp[m]
        let mut sum_dp: i64 = 0;
        for c in 0..m {
            let rem = (m - 1) - c;
            let mut conv_v_v: i64 = 0;
            for a in 0..=rem {
                let b = rem - a;
                conv_v_v = (conv_v_v + v[a] as i128 * v[b] as i128 % MOD as i128) as i64 % MOD;
            }
            sum_dp = (sum_dp + dp[c] as i128 * conv_v_v as i128 % MOD as i128) as i64 % MOD;
        }
        dp[m] = 10 * sum_dp % MOD;

        // 2. Compute prim[m]
        let mut sum_prim_dp: i64 = 0;
        for k in 1..m {
            sum_prim_dp = (sum_prim_dp + prim[k] as i128 * dp[m - k] as i128 % MOD as i128) as i64 % MOD;
        }
        prim[m] = (dp[m] - sum_prim_dp + MOD) % MOD;

        // 3. Compute v[m]
        let mut sum_v: i64 = 0;
        for k in 1..=m {
            let p_val = prim[k] as i128 * 9 % MOD as i128 * inv10 as i128 % MOD as i128;
            sum_v = (sum_v as i128 + p_val * v[m - k] as i128 % MOD as i128) as i64 % MOD;
        }
        v[m] = sum_v;
    }

    let mut total: i64 = 0;
    for m in 1..=limit {
        let term = dp[m] as i128 * 9 % MOD as i128 * inv10 as i128 % MOD as i128;
        total = (total as i128 + term) as i64 % MOD;
    }

    println!("{}", total);
}
