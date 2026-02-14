// Project Euler 386 - Antichain Counting
//
// N(n) is the max antichain size in divisor lattice of n.
// Sum of N(n) for n=1 to 10^8.

use euler_utils::sieve;

const N: u64 = 100_000_000;

fn count_combinations(exps: &[i32]) -> i64 {
    if exps.is_empty() {
        return 1;
    }

    let total: i32 = exps.iter().sum();
    let target = (total / 2) as usize;

    let mut dp = vec![0i64; target + 1];
    dp[0] = 1;

    for &e in exps {
        let mut new_dp = vec![0i64; target + 1];
        for s in 0..=target {
            if dp[s] > 0 {
                let max_k = e.min((target - s) as i32) as usize;
                for k in 0..=max_k {
                    new_dp[s + k] += dp[s];
                }
            }
        }
        dp = new_dp;
    }

    dp[target]
}

fn main() {
    let is_prime = sieve(N as usize);
    let primes: Vec<u64> = (2..=N as usize).filter(|&i| is_prime[i]).map(|i| i as u64).collect();

    let mut ans: i64 = 0;
    let mut exponents = Vec::with_capacity(30);

    fn helper(
        min_index: usize,
        exponents: &mut Vec<i32>,
        n: u64,
        primes: &[u64],
        ans: &mut i64,
    ) {
        *ans += count_combinations(exponents);

        for index in min_index..primes.len() {
            let p = primes[index];
            if n * p > N {
                break;
            }

            let mut prod = 1u64;
            let mut e = 1i32;
            loop {
                prod *= p;
                if n * prod > N {
                    break;
                }
                exponents.push(e);
                helper(index + 1, exponents, n * prod, primes, ans);
                exponents.pop();
                e += 1;
            }
        }
    }

    helper(0, &mut exponents, 1, &primes, &mut ans);
    println!("{}", ans);
}
