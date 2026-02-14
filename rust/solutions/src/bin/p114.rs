// Project Euler 114: Counting Block Combinations I
// Count ways to tile row of 50 with red blocks (min length 3) + grey.

fn main() {
    // dp[i] = ways to tile row of length i
    // ways(-1) = 1 (sentinel for block filling entire row + gap)
    let n = 50usize;
    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        dp[i] = dp[i - 1]; // grey square
        for k in 3..=i {
            // place red block of length k ending at position i
            if i == k {
                dp[i] += 1; // block fills entire prefix
            } else {
                dp[i] += dp[i - k - 1]; // grey separator before block
            }
        }
    }
    println!("{}", dp[n]);
}
