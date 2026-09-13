// Project Euler 787 - Bezout's Game
// Vec-based memoization instead of HashMap, odd-sum via M(n)+M(n/2).

const N: i64 = 1_000_000_000;

fn main() {
    let l = (N as f64).sqrt() as i64 + 1;
    let sieve_limit = l as usize;

    let mut mobius = vec![1i8; sieve_limit + 1];
    let mut is_prime = vec![true; sieve_limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=sieve_limit {
        if is_prime[i] {
            for j in (2 * i..=sieve_limit).step_by(i) {
                is_prime[j] = false;
            }
            let i2 = i * i;
            if i2 <= sieve_limit {
                for j in (i2..=sieve_limit).step_by(i2) {
                    mobius[j] = 0;
                }
            }
            for j in (i..=sieve_limit).step_by(i) {
                mobius[j] = -mobius[j];
            }
        }
    }

    let mut mertens_small = vec![0i64; sieve_limit + 1];
    for i in 1..=sieve_limit {
        mertens_small[i] = mertens_small[i - 1] + mobius[i] as i64;
    }

    let cache_size = 2 * l as usize + 10;
    let mut mertens_cache = vec![-1000000000i64; cache_size];
    let mut sum_odd_cache = vec![-1000000000i64; cache_size];

    fn get_cache_idx(n: i64, l: i64) -> usize {
        if n <= l {
            return n as usize;
        }
        (l + (N / n)) as usize
    }

    fn mertens_fn(
        n: i64,
        l: i64,
        sieve_limit: usize,
        mertens_small: &[i64],
        cache: &mut [i64],
    ) -> i64 {
        if n <= sieve_limit as i64 {
            return mertens_small[n as usize];
        }
        
        let idx = get_cache_idx(n, l);
        if cache[idx] > -1000000000 {
            return cache[idx];
        }

        let sqrtn = (n as f64).sqrt() as i64;
        let mut result: i64 = 1;

        for k in 2..=n / (sqrtn + 1) {
            result -= mertens_fn(n / k, l, sieve_limit, mertens_small, cache);
        }

        for q in 1..=sqrtn {
            let kmin_base = n / (q + 1) + 1;
            let kmax = n / q;
            let kmin = kmin_base.max(n / (sqrtn + 1) + 1);
            if kmax >= kmin {
                result -= (kmax - kmin + 1) * mertens_fn(q, l, sieve_limit, mertens_small, cache);
            }
        }

        cache[idx] = result;
        result
    }

    fn sum_odd_fn(
        n: i64,
        l: i64,
        sieve_limit: usize,
        mertens_small: &[i64],
        sum_odd_small: &[i64],
        mertens_cache: &mut [i64],
        sum_odd_cache: &mut [i64],
    ) -> i64 {
        if n <= sieve_limit as i64 {
            return sum_odd_small[n as usize];
        }
        
        let idx = get_cache_idx(n, l);
        if sum_odd_cache[idx] > -1000000000 {
            return sum_odd_cache[idx];
        }

        let m = mertens_fn(n, l, sieve_limit, mertens_small, mertens_cache);
        let s = sum_odd_fn(n / 2, l, sieve_limit, mertens_small, sum_odd_small, mertens_cache, sum_odd_cache);
        let val = m + s;
        sum_odd_cache[idx] = val;
        val
    }

    let mut sum_odd_small = vec![0i64; sieve_limit + 1];
    for i in 1..=sieve_limit {
        sum_odd_small[i] = sum_odd_small[i - 1] + if i & 1 == 1 { mobius[i] as i64 } else { 0 };
    }

    fn tr(n: i64) -> i64 {
        n * (n + 1) / 2
    }

    let mut ans: i128 = 0;

    let mut g = 1i64;
    while g <= N / l {
        let t = N / g;
        ans += mobius[g as usize] as i128 * (tr(t) / 2) as i128;
        g += 2;
    }

    for t in 1..l {
        let upper = N / t;
        let lower = N / (t + 1);

        let s_upper = sum_odd_fn(upper, l, sieve_limit, &mertens_small, &sum_odd_small, &mut mertens_cache, &mut sum_odd_cache);
        let s_lower = sum_odd_fn(lower, l, sieve_limit, &mertens_small, &sum_odd_small, &mut mertens_cache, &mut sum_odd_cache);
        let coeff = s_upper - s_lower;

        ans += coeff as i128 * (tr(t) / 2) as i128;
    }

    println!("{}", ans as i64);
}

