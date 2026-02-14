// Project Euler 77: Prime summations
// Find the smallest number that can be written as a sum of primes in over 5000 ways.

use euler_utils::primes_up_to;

fn main() {
    let n_limit = 100usize;
    let primes = primes_up_to(n_limit);

    let mut ways = vec![0i64; n_limit + 1];
    ways[0] = 1;

    for &p in &primes {
        for s in p..=n_limit {
            ways[s] += ways[s - p];
        }
    }

    for i in 2..=n_limit {
        if ways[i] > 5000 {
            println!("{i}");
            return;
        }
    }
}
