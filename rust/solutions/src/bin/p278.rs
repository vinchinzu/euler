// Project Euler 278: Linear Combinations of Semiprimes
// For primes p < q < r < 5000: f(pq, pr, qr) = 2pqr - (pq + pr + qr)
// Sum over all such triples.

use euler_utils::*;

fn main() {
    let limit = 5000usize;
    let primes = primes_up_to(limit);

    let n = primes.len();
    // Precompute suffix sums
    let mut suffix = vec![0i64; n + 1];
    for i in (0..n).rev() {
        suffix[i] = suffix[i + 1] + primes[i] as i64;
    }

    let mut ans: i128 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let p = primes[i] as i64;
            let q = primes[j] as i64;
            let sum_r = suffix[j + 1];
            let count_r = (n - j - 1) as i64;
            if count_r <= 0 { continue; }

            // sum_{k>j} [2pqr - pq - pr - qr]
            // = (2pq - p - q) * sum_r - pq * count_r
            let contrib = (2 * p * q - p - q) as i128 * sum_r as i128
                        - (p * q) as i128 * count_r as i128;
            ans += contrib;
        }
    }

    println!("{}", ans);
}
