// Project Euler 127 - abc-hits
//
// Find sum of c for all abc-hits below 120000.

use euler_utils::gcd;

const LIMIT: usize = 120_000;

fn main() {
    let mut rad = vec![1u64; LIMIT];

    // Sieve primes and build radicals
    let mut is_prime = vec![true; LIMIT];
    is_prime[0] = false;
    if LIMIT > 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i < LIMIT {
        if is_prime[i] {
            let mut j = i * i;
            while j < LIMIT {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    for p in 2..LIMIT {
        if is_prime[p] {
            let mut m = p;
            while m < LIMIT {
                rad[m] *= p as u64;
                m += p;
            }
        }
    }

    // Sort (rad, num) pairs for numbers 1..LIMIT-1
    let mut pairs: Vec<(u64, usize)> = (1..LIMIT).map(|n| (rad[n], n)).collect();
    pairs.sort();

    let sorted_rads: Vec<u64> = pairs.iter().map(|&(r, _)| r).collect();
    let sorted_nums: Vec<usize> = pairs.iter().map(|&(_, n)| n).collect();

    let mut sum_c: i64 = 0;

    for c in 3..LIMIT {
        let rad_c = rad[c];
        if rad_c == c as u64 {
            continue;
        }

        let max_rad_a = (c as u64 - 1) / rad_c;
        if max_rad_a == 0 {
            continue;
        }

        // Binary search: upper bound
        let limit_idx = sorted_rads.partition_point(|&r| r <= max_rad_a);
        let limit_a = c / 2;

        for i in 0..limit_idx {
            let a = sorted_nums[i];
            if a >= limit_a || a >= c {
                continue;
            }
            if gcd(a as u64, c as u64) != 1 {
                continue;
            }

            let b = c - a;
            if a >= b {
                continue;
            }
            if rad[a] * rad[b] * rad_c >= c as u64 {
                continue;
            }

            sum_c += c as i64;
        }
    }

    println!("{}", sum_c);
}
