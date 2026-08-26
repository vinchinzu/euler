// Project Euler 127 - abc-hits
// Sum of c for all abc-hits below 120000.

use euler_utils::gcd;
use rayon::prelude::*;

const LIMIT: usize = 120_000;

fn main() {
    let mut rad = vec![1u64; LIMIT];
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

    let mut pairs: Vec<(u64, usize)> = (1..LIMIT).map(|n| (rad[n], n)).collect();
    pairs.sort_unstable();

    let sorted_rads: Vec<u64> = pairs.iter().map(|&(r, _)| r).collect();
    let sorted_nums: Vec<usize> = pairs.iter().map(|&(_, n)| n).collect();

    let sum_c: i64 = (3..LIMIT)
        .into_par_iter()
        .map(|c| {
            let rad_c = rad[c];
            if rad_c == c as u64 {
                return 0;
            }
            let max_rad_a = (c as u64 - 1) / rad_c;
            if max_rad_a == 0 {
                return 0;
            }
            let limit_idx = sorted_rads.partition_point(|&r| r <= max_rad_a);
            let limit_a = c / 2;
            let mut local = 0i64;
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
                local += c as i64;
            }
            local
        })
        .sum();

    println!("{}", sum_c);
}
