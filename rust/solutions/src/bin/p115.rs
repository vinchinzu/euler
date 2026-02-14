// Project Euler 115: Counting Block Combinations II
// Find smallest n where f(50,n) > 1,000,000.

fn count_arrangements(min_len: usize, length: usize) -> i64 {
    let mut dp = vec![0i64; length + 1];
    dp[0] = 1;
    for i in 1..=length {
        dp[i] = dp[i - 1]; // grey square
        for bl in min_len..=i {
            if i == bl {
                dp[i] += 1;
            } else if i - bl >= 1 {
                dp[i] += dp[i - bl - 1]; // grey separator
            }
        }
    }
    dp[length]
}

fn main() {
    let min_len = 50;
    let target = 1_000_000i64;
    for n in min_len..500 {
        if count_arrangements(min_len, n) > target {
            println!("{n}");
            return;
        }
    }
}
