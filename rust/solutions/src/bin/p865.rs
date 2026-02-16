// Project Euler 865 - Triplicate Numbers
// T(10^4) mod 998244353 via recurrence DP
// Optimized: precompute conv_vv incrementally to reduce O(m^2) to O(m) per step

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
    let mut conv_vv = vec![0u64; LIMIT + 1]; // conv_vv[k] = sum_{a=0}^{k} v[a]*v[k-a]

    dp[0] = 1;
    v[0] = 1;

    // Initialize conv_vv[0] = v[0]*v[0] = 1
    conv_vv[0] = 1;

    for m in 1..=limit {
        // 1. Compute dp[m] = 10 * sum_{c=0}^{m-1} dp[c] * conv_vv[m-1-c]
        // conv_vv[0..m-1] are all precomputed from previous steps
        let mut sum_dp = 0u64;
        for c in 0..m {
            // SAFETY: c < m, m-1-c >= 0, all indices valid
            unsafe {
                sum_dp += *dp.get_unchecked(c) * *conv_vv.get_unchecked(m - 1 - c) % MOD;
            }
            if sum_dp >= MOD { sum_dp -= MOD; }
        }
        dp[m] = 10 * sum_dp % MOD;

        // 2. Compute prim[m] = dp[m] - sum_{k=1}^{m-1} prim[k]*dp[m-k]
        let mut sum_prim_dp = 0u64;
        for k in 1..m {
            unsafe {
                sum_prim_dp += *prim.get_unchecked(k) * *dp.get_unchecked(m - k) % MOD;
            }
            if sum_prim_dp >= MOD { sum_prim_dp -= MOD; }
        }
        prim[m] = (dp[m] + MOD - sum_prim_dp) % MOD;

        // 3. Compute v[m] = sum_{k=1}^{m} (9/10)*prim[k] * v[m-k]
        let mut sum_v = 0u64;
        for k in 1..=m {
            unsafe {
                let p_val = *prim.get_unchecked(k) * 9 % MOD * inv10 % MOD;
                sum_v += p_val * *v.get_unchecked(m - k) % MOD;
            }
            if sum_v >= MOD { sum_v -= MOD; }
        }
        v[m] = sum_v;

        // 4. Update conv_vv[m] = sum_{a=0}^{m} v[a]*v[m-a]
        let mut cv = 0u64;
        for a in 0..=m {
            unsafe {
                cv += *v.get_unchecked(a) * *v.get_unchecked(m - a) % MOD;
            }
            if cv >= MOD { cv -= MOD; }
        }
        conv_vv[m] = cv;
    }

    let mut total = 0u64;
    for m in 1..=limit {
        let term = dp[m] * 9 % MOD * inv10 % MOD;
        total = (total + term) % MOD;
    }

    println!("{}", total);
}
