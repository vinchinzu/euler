// Project Euler 704 - Factors of Two in Binomial Coefficients
//
// Sum F(n) for n = 1..N where N = 10^16, using iterative counting of
// even numbers at each scale.

fn main() {
    let n: i64 = 10_000_000_000_000_000; // 10^16
    let mut ans: i64 = 0;

    let mut current_n = n;
    while current_n > 1 {
        let mut start: i64 = 2;
        while start <= current_n {
            let count = (current_n - start) / 2 + 1;
            ans += count;
            start *= 2;
        }
        current_n = (current_n - 1) / 2;
    }

    println!("{}", ans);
}
