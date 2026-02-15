// Project Euler 787 - Bezout's Game
// Mertens function with hash-based memoization and sum_odd.

use std::collections::HashMap;

const N: i64 = 1_000_000_000;

fn main() {
    let l = (N as f64).sqrt() as i64 + 1;
    let sieve_limit = (l + 100) as usize;

    // Sieve Mobius
    let mut mobius = vec![1i8; sieve_limit + 1];
    let mut is_prime = vec![true; sieve_limit + 2];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=sieve_limit {
        if is_prime[i] {
            for j in (i..=sieve_limit).step_by(i) {
                if j > i { is_prime[j] = false; }
            }
            let mut j = (i as u64) * (i as u64);
            while j <= sieve_limit as u64 {
                mobius[j as usize] = 0;
                j += (i as u64) * (i as u64);
            }
            for j in (i..=sieve_limit).step_by(i) {
                mobius[j] = -mobius[j];
            }
        }
    }

    // Prefix sums
    let mut mertens_prefix = vec![0i64; sieve_limit + 1];
    let mut sum_odd_prefix = vec![0i64; sieve_limit + 1];
    for i in 1..=sieve_limit {
        mertens_prefix[i] = mertens_prefix[i - 1] + mobius[i] as i64;
        sum_odd_prefix[i] = sum_odd_prefix[i - 1] + if i & 1 == 1 { mobius[i] as i64 } else { 0 };
    }

    let mut mertens_cache: HashMap<i64, i64> = HashMap::new();
    let mut sum_odd_cache: HashMap<i64, i64> = HashMap::new();

    fn mertens_fn(
        n: i64,
        sieve_limit: usize,
        mertens_prefix: &[i64],
        cache: &mut HashMap<i64, i64>,
    ) -> i64 {
        if n <= sieve_limit as i64 { return mertens_prefix[n as usize]; }
        if let Some(&v) = cache.get(&n) { return v; }

        let sqrtn = (n as f64).sqrt() as i64;
        let mut result: i64 = 1;

        for k in 2..=n / (sqrtn + 1) {
            result -= mertens_fn(n / k, sieve_limit, mertens_prefix, cache);
        }

        for q in 1..=sqrtn {
            let kmin_base = n / (q + 1) + 1;
            let kmax = n / q;
            let kmin = kmin_base.max(n / (sqrtn + 1) + 1);
            if kmax >= kmin {
                result -= (kmax - kmin + 1) * mertens_fn(q, sieve_limit, mertens_prefix, cache);
            }
        }

        cache.insert(n, result);
        result
    }

    fn sum_odd_fn(
        n: i64,
        sieve_limit: usize,
        mertens_prefix: &[i64],
        sum_odd_prefix: &[i64],
        mertens_cache: &mut HashMap<i64, i64>,
        sum_odd_cache: &mut HashMap<i64, i64>,
    ) -> i64 {
        if n <= sieve_limit as i64 { return sum_odd_prefix[n as usize]; }
        if let Some(&v) = sum_odd_cache.get(&n) { return v; }

        let m = mertens_fn(n, sieve_limit, mertens_prefix, mertens_cache);
        let s = sum_odd_fn(n / 2, sieve_limit, mertens_prefix, sum_odd_prefix, mertens_cache, sum_odd_cache);
        let val = m + s;
        sum_odd_cache.insert(n, val);
        val
    }

    fn tr(n: i64) -> i64 { n * (n + 1) / 2 }

    let mut ans: i128 = 0;

    // Direct: g from 1 to N/L (odd g only)
    let mut g = 1i64;
    while g <= N / l {
        let t = N / g;
        ans += mobius[g as usize] as i128 * (tr(t) / 2) as i128;
        g += 2;
    }

    // Batch: for quotient t from 1 to L-1
    for t in 1..l {
        let upper = N / t;
        let lower = N / (t + 1);
        let s_upper = sum_odd_fn(upper, sieve_limit, &mertens_prefix, &sum_odd_prefix, &mut mertens_cache, &mut sum_odd_cache);
        let s_lower = sum_odd_fn(lower, sieve_limit, &mertens_prefix, &sum_odd_prefix, &mut mertens_cache, &mut sum_odd_cache);
        let coeff = s_upper - s_lower;
        ans += coeff as i128 * (tr(t) / 2) as i128;
    }

    println!("{}", ans as i64);
}
