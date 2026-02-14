// Project Euler 234 - Semidivisible numbers
// Find sum of semidivisible numbers up to N = 999966663333.

use euler_utils::sieve;

const SIEVE_LIMIT: usize = 2_000_001;

fn main() {
    let n: i64 = 999_966_663_333;
    let is_prime = sieve(SIEVE_LIMIT);
    let primes: Vec<i64> = (2..SIEVE_LIMIT)
        .filter(|&i| is_prime[i])
        .map(|i| i as i64)
        .collect();

    let mut ans: i64 = 0;

    for i in 0..primes.len() - 1 {
        let prev_p = if i > 0 { primes[i - 1] } else { 0 };
        let p = primes[i];
        let next_p = primes[i + 1];

        let mut lo = prev_p * prev_p;
        if lo < 4 {
            lo = 4;
        }
        let mut hi = next_p * next_p;
        if hi > n {
            hi = n;
        }

        // Round up lo to nearest multiple of p
        let min_val = ((lo + p - 1) / p) * p;
        // Round down hi to nearest multiple of p
        let max_val = (hi / p) * p;

        if min_val > max_val {
            continue;
        }

        let count = (max_val - min_val) / p + 1;
        // Sum of arithmetic progression: count * (min_val + max_val) / 2
        ans += (max_val + min_val) / 2 * count;
        if (max_val + min_val) % 2 != 0 {
            ans += count / 2;
        }

        // Subtract numbers divisible by both p and prev_p, p*p, p*next_p
        let vals = [prev_p, p, next_p];
        for &v in &vals {
            let pq = p * v;
            if pq >= 4 && pq <= n && pq >= lo && pq <= hi {
                ans -= pq;
            }
        }
    }

    println!("{}", ans);
}
