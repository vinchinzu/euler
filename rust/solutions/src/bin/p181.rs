// Project Euler 181: Grouping objects of two different colours
fn main() {
    const BLACK: usize = 60;
    const WHITE: usize = 40;
    let mut dp = vec![vec![0i64; WHITE + 1]; BLACK + 1];
    dp[0][0] = 1;

    for b in 0..=BLACK {
        for w in 0..=WHITE {
            if b == 0 && w == 0 { continue; }
            for i in b..=BLACK {
                for j in w..=WHITE {
                    dp[i][j] += dp[i - b][j - w];
                }
            }
        }
    }

    println!("{}", dp[BLACK][WHITE]);
}
