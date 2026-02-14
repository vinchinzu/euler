// Project Euler 187 - Semiprimes
//
// Count composites n < 10^8 that are the product of exactly two primes (not necessarily distinct).

use euler_utils::primes_up_to;

const LIMIT: usize = 100_000_000;

fn main() {
    // Need primes up to LIMIT/2 for the second factor
    let primes = primes_up_to(LIMIT / 2);
    let nprimes = primes.len();

    let mut count: u64 = 0;
    for i in 0..nprimes {
        let p = primes[i] as u64;
        if p * p >= LIMIT as u64 {
            break;
        }
        let max_q = (LIMIT as u64 - 1) / p;
        // Binary search for largest prime <= max_q
        let mut lo = i;
        let mut hi = nprimes;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if primes[mid] as u64 <= max_q {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        // lo is now the first index > max_q, so count from i..lo
        if lo > i {
            count += (lo - i) as u64;
        }
    }

    println!("{}", count);
}
