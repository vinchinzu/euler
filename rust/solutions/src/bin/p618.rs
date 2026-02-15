// Project Euler 618 - Numbers with a given prime factor sum
// DP over primes, sum dp[F(i)] for Fibonacci F(2)..F(24)

const LIMIT: usize = 46368;
const MOD: i64 = 1_000_000_000;

fn main() {
    let mut is_prime = vec![true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= LIMIT { if is_prime[i] { let mut j = i*i; while j <= LIMIT { is_prime[j] = false; j += i; } } i += 1; }

    let mut dp = vec![0i64; LIMIT + 1];
    dp[0] = 1;
    for p in 2..=LIMIT {
        if !is_prime[p] { continue; }
        for k in p..=LIMIT {
            dp[k] = (dp[k] + p as i64 * dp[k - p]) % MOD;
        }
    }

    let mut fib_prev = 0usize;
    let mut fib_curr = 1usize;
    let mut ans = 0i64;
    for i in 1..=24 {
        let tmp = fib_prev + fib_curr;
        fib_prev = fib_curr;
        fib_curr = tmp;
        if i >= 2 {
            ans = (ans + dp[fib_prev]) % MOD;
        }
    }

    println!("{}", ans);
}
