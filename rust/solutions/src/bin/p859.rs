// Project Euler 859 - Cookies game (partition DP)

const N: usize = 300;
const OFFSET: usize = 2000;
const MAX_VAL: usize = 4001;

fn main() {
    let mut g = vec![0i32; N + 1];
    for k in 1..=N {
        if k % 2 == 1 {
            let m = (k - 1) / 2;
            let val = 2 * g[m];
            g[k] = if val < 0 { 0 } else { val + 1 };
        } else {
            let m = (k - 2) / 2;
            let val = 2 * g[m];
            g[k] = if val > 0 { 0 } else { val - 1 };
        }
    }

    let mut dp = vec![vec![0i64; MAX_VAL]; N + 1];
    dp[0][OFFSET] = 1;

    for k in 1..=N {
        let g_val = g[k];
        for n in k..=N {
            if g_val >= 0 {
                for i in 0..(MAX_VAL as i32 - g_val) as usize {
                    if dp[n - k][i] > 0 {
                        dp[n][(i as i32 + g_val) as usize] += dp[n - k][i];
                    }
                }
            } else {
                for i in (-g_val) as usize..MAX_VAL {
                    if dp[n - k][i] > 0 {
                        dp[n][(i as i32 + g_val) as usize] += dp[n - k][i];
                    }
                }
            }
        }
    }

    let total: i64 = dp[N][..=OFFSET].iter().sum();
    println!("{}", total);
}
