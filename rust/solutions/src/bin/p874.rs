// Project Euler 874 - Prime Score
// M(7000, p(7000)): max prime score with sum of indices divisible by k=7000.

const MAX_PRIMES: usize = 7002;
const SIEVE_LIMIT: usize = 100_000;

fn main() {
    // Sieve
    let mut is_prime = vec![true; SIEVE_LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=SIEVE_LIMIT {
        if is_prime[i] {
            let mut j = i * i;
            while j <= SIEVE_LIMIT { is_prime[j] = false; j += i; }
        }
    }
    let primes: Vec<i64> = (2..=SIEVE_LIMIT).filter(|&i| is_prime[i]).take(MAX_PRIMES).map(|i| i as i64).collect();

    let k = 7000usize;
    let n = primes[k]; // p(7000)
    let p_max = primes[k - 1];

    let current_sum_indices = n * (k as i64 - 1);
    let remainder = (current_sum_indices % k as i64) as usize;

    if remainder == 0 {
        println!("{}", n * p_max);
        return;
    }

    let target_r = remainder;

    // Precompute costs
    let mut cost = vec![0i64; target_r + 1];
    for d in 1..=target_r {
        cost[d] = primes[k - 1] - primes[k - 1 - d];
    }

    // DP
    let mut dp = vec![i64::MAX / 2; target_r + 1];
    dp[0] = 0;

    for w in 1..=target_r {
        let mut min_c = cost[w];
        for j in 1..=w / 2 {
            let c = dp[j] + dp[w - j];
            if c < min_c { min_c = c; }
        }
        dp[w] = min_c;
    }

    let max_score = n * p_max - dp[target_r];
    println!("{}", max_score);
}
