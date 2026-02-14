// Project Euler 217: Balanced Numbers
use euler_utils::mod_pow;

fn main() {
    const MAX_N: usize = 47;
    const B: usize = 10;
    const MAX_SUM: usize = B * (MAX_N / 2 + 1);

    // M = 3^15
    let m: u64 = 14_348_907;

    // dp[i][j] = number of ways for i digits to sum to j (mod M)
    let mut dp = vec![vec![0u64; MAX_SUM + 1]; MAX_N / 2 + 2];
    dp[0][0] = 1;

    for i in 1..=MAX_N / 2 {
        for j in 0..MAX_SUM {
            let mut s = 0u64;
            for d in 0..B {
                if d <= j {
                    s += dp[i - 1][j - d];
                }
            }
            dp[i][j] = s % m;
        }
    }

    let mut ans: u64 = 0;

    for k in 1..=MAX_N {
        let half = k / 2;
        for i in 0..k {
            for d in 1..B {
                let mut mult = mod_pow(B as u64, i as u64, m) * d as u64 % m;
                if k % 2 == 1 && i != k / 2 {
                    mult = mult * B as u64 % m;
                }

                for s_val in 0..MAX_SUM {
                    let term;
                    if i < half {
                        let a = (dp[half][s_val] + m - dp[half - 1][s_val] % m) % m;
                        let b = if s_val >= d { dp[half - 1][s_val - d] } else { 0 };
                        term = a % m * (b % m) % m;
                    } else if k % 2 == 1 && i == half {
                        let sub = if k >= 2 { dp[half - 1][s_val] } else { 0 };
                        let a = (dp[half][s_val] + m - sub % m) % m;
                        let b = dp[half][s_val] % m;
                        term = a * b % m;
                    } else if i < k - 1 {
                        let a1 = if s_val >= d { dp[half - 1][s_val - d] } else { 0 };
                        let a2 = if k >= 4 && s_val >= d { dp[half - 2][s_val - d] } else { 0 };
                        let a = (a1 + m - a2 % m) % m;
                        let b = dp[half][s_val] % m;
                        term = a * b % m;
                    } else {
                        // i == k - 1
                        let a = if s_val >= d { dp[half - 1][s_val - d] } else { 0 };
                        let b = dp[half][s_val] % m;
                        term = (a % m) * b % m;
                    };

                    ans = (ans + mult % m * (term % m)) % m;
                }
            }
        }
    }

    println!("{}", ans);
}
