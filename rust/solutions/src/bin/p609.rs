// Project Euler 609 - Pi sequences
// Iterated prime-counting sequences, product of bin counts

use rayon::prelude::*;

const NLIMIT: usize = 100_000_000;
const MOD: u64 = 1_000_000_007;

fn main() {
    // Odd-only bit sieve: index i <-> number 2*i+1
    let n_odds = (NLIMIT + 1) / 2;
    let mut is_odd_prime = vec![true; n_odds];
    is_odd_prime[0] = false; // 1
    let mut i = 1usize;
    while {
        let p = 2 * i + 1;
        p * p <= NLIMIT
    } {
        if is_odd_prime[i] {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                is_odd_prime[j] = false;
                j += p;
            }
        }
        i += 1;
    }

    // Compact is_prime as bool for O(1) chain checks (needed for all n in chains)
    let mut is_prime = vec![false; NLIMIT + 1];
    is_prime[2] = true;
    for i in 1..n_odds {
        if is_odd_prime[i] {
            let p = 2 * i + 1;
            if p <= NLIMIT {
                is_prime[p] = true;
            }
        }
    }
    drop(is_odd_prime);

    let mut pi = vec![0u32; NLIMIT + 1];
    let mut count = 0u32;
    for i in 0..=NLIMIT {
        if is_prime[i] {
            count += 1;
        }
        pi[i] = count;
    }

    let mut max_len = 0;
    let mut n = NLIMIT;
    while n > 0 {
        max_len += 1;
        n = pi[n] as usize;
    }

    // Collect primes
    let primes: Vec<usize> = (2..=NLIMIT).filter(|&i| is_prime[i]).collect();

    // Parallel accumulation of ps bins (max_len is small ~15)
    let ps = primes
        .par_windows(2)
        .map(|w| {
            let p = w[0];
            let cnt = w[1] - 1 - p;
            let mut local = vec![0u64; max_len + 2];
            let mut n = pi[p] as usize;
            let mut c = 0usize;
            while n > 0 {
                if !is_prime[n] {
                    c += 1;
                }
                local[c] += 1;
                local[c + 1] += cnt as u64;
                n = pi[n] as usize;
            }
            local
        })
        .reduce(
            || vec![0u64; max_len + 2],
            |mut a, b| {
                for i in 0..a.len() {
                    a[i] += b[i];
                }
                a
            },
        );

    // Last prime: range from last prime to NLIMIT
    let mut ps = ps;
    if let Some(&p) = primes.last() {
        let cnt = NLIMIT - p;
        let mut n = pi[p] as usize;
        let mut c = 0usize;
        while n > 0 {
            if !is_prime[n] {
                c += 1;
            }
            ps[c] += 1;
            ps[c + 1] += cnt as u64;
            n = pi[n] as usize;
        }
    }

    let mut ans = 1u64;
    for &v in &ps {
        if v != 0 {
            ans = (ans as u128 * (v % MOD) as u128 % MOD as u128) as u64;
        }
    }

    println!("{}", ans);
}
