// Project Euler 250: 250250
use euler_utils::mod_pow;

fn main() {
    const N: usize = 250250;
    const K: usize = 250;
    const MOD: u64 = 10_000_000_000_000_000;

    let mut dp = vec![0u64; K];
    dp[0] = 1;

    for k in 1..=N {
        let val = mod_pow(k as u64, k as u64, K as u64) as usize;
        let mut new_dp = dp.clone();
        for i in 0..K {
            let new_i = (i + val) % K;
            new_dp[new_i] = (dp[i] + new_dp[new_i]) % MOD;
        }
        dp = new_dp;
    }

    let ans = (dp[0] + MOD - 1) % MOD;
    println!("{}", ans);
}
