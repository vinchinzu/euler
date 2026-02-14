// Project Euler 50: Consecutive prime sum
// Find the prime below one million that is the sum of the most consecutive primes.
use euler_utils::{sieve, primes_up_to};

fn main() {
    let limit = 1_000_000usize;
    let is_prime = sieve(limit);
    let primes = primes_up_to(limit);

    // Prefix sums
    let mut prefix = vec![0i64; primes.len() + 1];
    for (i, &p) in primes.iter().enumerate() {
        prefix[i + 1] = prefix[i] + p as i64;
    }

    let mut max_len = 0;
    let mut best = 0i64;

    for start in 0..primes.len() {
        if start + max_len >= primes.len() {
            break;
        }
        for finish in (start + max_len)..primes.len() {
            let s = prefix[finish + 1] - prefix[start];
            if s >= limit as i64 {
                break;
            }
            if is_prime[s as usize] {
                let length = finish - start + 1;
                if length > max_len {
                    max_len = length;
                    best = s;
                }
            }
        }
    }

    println!("{best}");
}
