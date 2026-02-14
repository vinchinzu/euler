// Project Euler 219: Skew-cost Coding
fn main() {
    let big_n: i64 = 1_000_000_000;
    let c0: usize = 1;
    let c1: usize = 4;

    let mut dp = vec![0i64; 200];
    dp[0] = 1;
    let mut dp_len = 1usize;

    let mut n: i64 = 1;
    let mut ans: i64 = 0;

    while n < big_n {
        let mut num_codes = dp[dp_len - 1];
        if num_codes > big_n - n {
            num_codes = big_n - n;
        }
        n += num_codes;
        ans += num_codes * ((dp_len - 1) as i64 + c0 as i64 + c1 as i64);

        let mut next = 0i64;
        if dp_len >= c0 { next += dp[dp_len - c0]; }
        if dp_len >= c1 { next += dp[dp_len - c1]; }
        dp[dp_len] = next;
        dp_len += 1;
    }

    println!("{ans}");
}
