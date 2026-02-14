// Project Euler 117: Red, Green, and Blue Tiles
// DP: each position can extend grey, red(2), green(3), blue(4) tiles.

fn main() {
    let mut dp = [0i64; 51];
    dp[0] = 1;
    for i in 1..=50 {
        dp[i] = dp[i - 1];
        if i >= 2 { dp[i] += dp[i - 2]; }
        if i >= 3 { dp[i] += dp[i - 3]; }
        if i >= 4 { dp[i] += dp[i - 4]; }
    }
    println!("{}", dp[50]);
}
