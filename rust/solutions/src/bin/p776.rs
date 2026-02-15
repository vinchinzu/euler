// Project Euler 776 - Digit Sum Division
// Digit DP: f(n, s) returns (count, sum) of numbers <= n with digit sum s.

use std::collections::HashMap;

fn main() {
    let n: i64 = 1_234_567_890_123_456_789;
    let max_digit_sum = 9 * 19; // 171

    let mut memo: HashMap<(i64, i32), (i64, f64)> = HashMap::new();

    fn f(n: i64, s: i32, memo: &mut HashMap<(i64, i32), (i64, f64)>) -> (i64, f64) {
        if n == 0 && s == 0 { return (1, 0.0); }
        if n <= 0 || s < 0 { return (0, 0.0); }

        if let Some(&cached) = memo.get(&(n, s)) {
            return cached;
        }

        let mut count = 0i64;
        let mut total_sum = 0.0f64;

        for d in 0..10i64 {
            let n_prime = n / 10 - if d > n % 10 { 1 } else { 0 };
            if s - d as i32 >= 0 {
                let (sub_count, sub_sum) = f(n_prime, s - d as i32, memo);
                count += sub_count;
                total_sum += sub_sum * 10.0 + d as f64 * sub_count as f64;
            }
        }

        memo.insert((n, s), (count, total_sum));
        (count, total_sum)
    }

    let mut ans = 0.0f64;
    for s in 1..max_digit_sum {
        let (count, total_sum) = f(n, s, &mut memo);
        if count > 0 {
            ans += total_sum / s as f64;
        }
    }

    // Format as scientific notation
    let formatted = format!("{:.12e}", ans);
    // Remove '+' from exponent
    let cleaned: String = formatted.chars().filter(|&c| c != '+').collect();
    println!("{}", cleaned);
}
