// Project Euler 249 - Prime Subset Sums
// Find number of subsets of primes < 5000 with prime sum, mod 10^16.

use euler_utils::sieve;

const N: usize = 5000;
const M: u64 = 10_000_000_000_000_000;

fn main() {
    // Generate primes up to N
    let is_prime_n = sieve(N);
    let mut primes = Vec::new();
    let mut total_sum: usize = 0;
    for i in 2..=N {
        if is_prime_n[i] {
            primes.push(i);
            total_sum += i;
        }
    }

    // DP: dp[i] = number of subsets with sum i, mod M
    let mut dp = vec![0u64; total_sum + 1];
    dp[0] = 1;

    let mut current_sum: usize = 0;
    for &p in &primes {
        current_sum += p;
        for i in (p..=current_sum).rev() {
            dp[i] += dp[i - p];
            if dp[i] >= M {
                dp[i] -= M;
            }
        }
    }

    // Sieve primes up to total_sum
    let is_prime_sum = sieve(total_sum);

    // Sum dp values at prime indices
    let mut ans: u64 = 0;
    for i in 2..=total_sum {
        if is_prime_sum[i] {
            ans += dp[i];
            if ans >= M {
                ans -= M;
            }
        }
    }

    println!("{}", ans);
}
