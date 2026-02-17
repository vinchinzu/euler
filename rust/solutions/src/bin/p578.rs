// Project Euler 578 - Integers with Decreasing Prime Powers
//
// Count integers n <= N where exponents in prime factorization are non-increasing.
// Optimized: Uses Möbius function for squarefree counting, only recurses for exponents >= 2.

use std::collections::HashMap;

const NN: i64 = 10_000_000_000_000; // 10^13

fn sieve_primes_and_mobius(limit: usize) -> (Vec<i64>, Vec<i32>) {
    let mut lp = vec![0u32; limit + 1];
    let mut mu = vec![0i8; limit + 1];
    let mut primes = Vec::new();
    mu[1] = 1;

    for i in 2..=limit {
        if lp[i] == 0 {
            lp[i] = i as u32;
            primes.push(i as i64);
            mu[i] = -1;
        }
        for &p in &primes {
            let ip = i * p as usize;
            if ip > limit {
                break;
            }
            lp[ip] = p as u32;
            if p as u32 == lp[i] {
                mu[ip] = 0;
                break;
            }
            mu[ip] = -mu[i];
        }
    }

    // Prefix sum of mu
    let mut prefix_mu = vec![0i32; limit + 1];
    let mut s = 0i32;
    for i in 1..=limit {
        s += mu[i] as i32;
        prefix_mu[i] = s;
    }

    (primes, prefix_mu)
}

// Count squarefree integers <= x (including 1)
fn squarefree_upto(x: i64, prefix_mu: &[i32]) -> i64 {
    if x <= 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }
    let r = (x as f64).sqrt() as i64;
    let r = r.min(prefix_mu.len() as i64 - 1);

    let mut res: i64 = 0;
    let mut i: i64 = 1;
    while i <= r {
        let t = x / (i * i);
        let j = (x as f64 / t as f64).sqrt() as i64;
        let j = j.min(r);
        let mu_sum = prefix_mu[j as usize] - prefix_mu[(i - 1) as usize];
        res += t * mu_sum as i64;
        i = j + 1;
    }
    res
}

// Count squarefree n <= x with all prime factors >= primes[start_idx]
// Uses memoization since this is called many times
fn squarefree_min_prime(
    x: i64,
    start_idx: usize,
    primes: &[i64],
    prefix_mu: &[i32],
    memo: &mut HashMap<(i64, usize), i64>,
) -> i64 {
    if x <= 0 {
        return 0;
    }
    if x == 1 || start_idx == 0 {
        return squarefree_upto(x, prefix_mu);
    }
    if start_idx >= primes.len() || primes[start_idx] > x {
        return 1;
    }

    let key = (x, start_idx);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut total = squarefree_upto(x, prefix_mu);
    for i in 0..start_idx {
        let p = primes[i];
        if p > x {
            break;
        }
        total -= squarefree_min_prime(x / p, i + 1, primes, prefix_mu, memo);
    }

    memo.insert(key, total);
    total
}

// Count DPP integers with product <= limit, using primes[start_idx..] with max_exp
fn count_dpp(
    limit: i64,
    start_idx: usize,
    max_exp: i32,
    primes: &[i64],
    prefix_mu: &[i32],
    memo_sf: &mut HashMap<(i64, usize), i64>,
    memo_dpp: &mut HashMap<(i64, usize, i32), i64>,
) -> i64 {
    if limit <= 0 {
        return 0;
    }
    if limit == 1 {
        return 1;
    }
    if max_exp <= 1 {
        return squarefree_min_prime(limit, start_idx, primes, prefix_mu, memo_sf);
    }

    let key = (limit, start_idx, max_exp);
    if let Some(&cached) = memo_dpp.get(&key) {
        return cached;
    }

    // Start with all squarefree numbers (exponent 1 for all)
    let mut res = squarefree_min_prime(limit, start_idx, primes, prefix_mu, memo_sf);

    // Add numbers with at least one prime having exponent >= 2
    for i in start_idx..primes.len() {
        let p = primes[i];
        if p * p > limit {
            break;
        }
        let mut pe = p * p; // Start from exponent 2
        let mut e = 2i32;
        while e <= max_exp && pe <= limit {
            res += count_dpp(limit / pe, i + 1, e, primes, prefix_mu, memo_sf, memo_dpp);
            e += 1;
            if pe > limit / p {
                break;
            }
            pe *= p;
        }
    }

    memo_dpp.insert(key, res);
    res
}

fn main() {
    let sieve_limit = (NN as f64).sqrt() as usize + 1000;
    let (primes, prefix_mu) = sieve_primes_and_mobius(sieve_limit);

    let mut memo_sf: HashMap<(i64, usize), i64> = HashMap::new();
    let mut memo_dpp: HashMap<(i64, usize, i32), i64> = HashMap::new();

    let max_e = (NN as f64).log2() as i32 + 1;
    let ans = count_dpp(
        NN,
        0,
        max_e,
        &primes,
        &prefix_mu,
        &mut memo_sf,
        &mut memo_dpp,
    );
    println!("{}", ans);
}
