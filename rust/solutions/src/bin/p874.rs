// Project Euler 874 - Prime Score
// M(7000, p(7000)): max prime score with sum of indices divisible by k=7000.

const MAX_PRIMES: usize = 7002;
const SIEVE_LIMIT: usize = 100_000;

fn main() {
    // Sieve
    let mut is_prime = vec![true; SIEVE_LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrt_limit = (SIEVE_LIMIT as f64).sqrt() as usize + 1;
    for i in 2..=sqrt_limit {
        if is_prime[i] {
            let mut j = i * i;
            // SAFETY: loop condition ensures j <= SIEVE_LIMIT < is_prime.len()
            while j <= SIEVE_LIMIT { 
                unsafe { *is_prime.get_unchecked_mut(j) = false; }
                j += i;
            }
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

    // DP with inline cost computation
    let mut dp = vec![i64::MAX / 2; target_r + 1];
    dp[0] = 0;

    for w in 1..=target_r {
        // SAFETY: w <= target_r < k-1, so k-1 and k-1-w are valid indices in primes
        let cost_w = unsafe { 
            *primes.get_unchecked(k - 1) - *primes.get_unchecked(k - 1 - w)
        };
        let mut min_c = cost_w;
        for j in 1..=w / 2 {
            // SAFETY: j in [1, w/2] and w-j in [w/2, w-1], both < target_r+1 = dp.len()
            let c = unsafe { *dp.get_unchecked(j) + *dp.get_unchecked(w - j) };
            if c < min_c { min_c = c; }
        }
        dp[w] = min_c;
    }

    let max_score = n * p_max - dp[target_r];
    println!("{}", max_score);
}
