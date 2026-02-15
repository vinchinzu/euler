// Project Euler 865 - Triplicate Numbers
// T(10^4) mod 998244353 via recurrence DP

const MOD: u64 = 998244353;
const LIMIT: usize = 3334;

fn power(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 { res = res * base % MOD; }
        base = base * base % MOD;
        exp >>= 1;
    }
    res
}

fn main() {
    let n = 10000;
    let limit = n / 3;

    let inv10 = power(10, MOD - 2);

    let mut dp = vec![0u64; LIMIT + 1];
    let mut prim = vec![0u64; LIMIT + 1];
    let mut v = vec![0u64; LIMIT + 1];

    dp[0] = 1;
    v[0] = 1;

    for m in 1..=limit {
        // 1. Compute dp[m]
        let mut sum_dp = 0u64;
        for c in 0..m {
            let rem = (m - 1) - c;
            let mut conv_v_v = 0u64;
            for a in 0..=rem {
                let b = rem - a;
                // SAFETY: a, b <= rem <= limit-1, arrays have size LIMIT+1
                unsafe {
                    conv_v_v += *v.get_unchecked(a) * *v.get_unchecked(b) % MOD;
                }
                if conv_v_v >= MOD { conv_v_v -= MOD; }
            }
            // SAFETY: c < m <= limit, array has size LIMIT+1
            unsafe {
                sum_dp += *dp.get_unchecked(c) * conv_v_v % MOD;
            }
            if sum_dp >= MOD { sum_dp -= MOD; }
        }
        dp[m] = 10 * sum_dp % MOD;

        // 2. Compute prim[m]
        let mut sum_prim_dp = 0u64;
        for k in 1..m {
            // SAFETY: k < m, m-k > 0, all indices valid
            unsafe {
                sum_prim_dp += *prim.get_unchecked(k) * *dp.get_unchecked(m - k) % MOD;
            }
            if sum_prim_dp >= MOD { sum_prim_dp -= MOD; }
        }
        prim[m] = (dp[m] + MOD - sum_prim_dp) % MOD;

        // 3. Compute v[m]
        let mut sum_v = 0u64;
        for k in 1..=m {
            // SAFETY: k <= m, m-k >= 0, all indices valid
            unsafe {
                let p_val = *prim.get_unchecked(k) * 9 % MOD * inv10 % MOD;
                sum_v += p_val * *v.get_unchecked(m - k) % MOD;
            }
            if sum_v >= MOD { sum_v -= MOD; }
        }
        v[m] = sum_v;
    }

    let mut total = 0u64;
    for m in 1..=limit {
        let term = dp[m] * 9 % MOD * inv10 % MOD;
        total = (total + term) % MOD;
    }

    println!("{}", total);
}
