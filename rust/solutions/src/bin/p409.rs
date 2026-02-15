// Project Euler 409: Nim Extreme
use euler_utils::mod_pow;

const MOD: u64 = 1_000_000_007;

fn main() {
    let n: usize = 10_000_000;
    let m = MOD;
    let pow2 = mod_pow(2, n as u64, m);

    let mut dp_km2: u64 = 1; // dp[0]
    let mut dp_km1: u64 = 0; // dp[1]
    let mut np: u64 = (pow2 + m - 1) % m; // num_positions[1]

    for k in 2..=n {
        let km1 = (k - 1) as u64 % m;
        let pow2_km1 = (pow2 + m - km1 % m) % m;
        let new_dp = (np + m - dp_km1 + m - (dp_km2 % m * (km1 % m) % m * pow2_km1 % m) % m) % m;

        np = np * ((pow2 + m - (k as u64) % m) % m) % m;
        dp_km2 = dp_km1;
        dp_km1 = new_dp;
    }

    let ans = (np + m - dp_km1) % m;
    println!("{ans}");
}
