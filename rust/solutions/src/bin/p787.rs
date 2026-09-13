// Project Euler 787 - Bezout's Game
// Vec-based memoization; odd-sum via M(n)+M(n/2).

const N: i64 = 1_000_000_000;

fn main() {
    let l = (N as f64).sqrt() as i64;
    let sieve_limit = (l + 1) as usize;

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
    let mut sum_odd_small = vec![0i64; sieve_limit + 1];
    for i in 1..=sieve_limit {
        mertens_small[i] = mertens_small[i - 1] + mobius[i] as i64;
        sum_odd_small[i] = sum_odd_small[i - 1] + if i & 1 == 1 { mobius[i] as i64 } else { 0 };
    }

    let cache_size = 2 * l as usize + 10;
    let mut mertens_cache = vec![0i64; cache_size];
    let mut sum_odd_cache = vec![0i64; cache_size];
    let mut mertens_seen = vec![false; cache_size];
    let mut sum_odd_seen = vec![false; cache_size];

    #[inline(always)]
    fn get_idx(n: i64, l: i64) -> usize {
        if n <= l {
            n as usize
        } else {
            (l + N / n) as usize
        }
    }

    fn mertens(n: i64, l: i64, small: &[i64], cache: &mut [i64], seen: &mut [bool]) -> i64 {
        if n <= l {
            return small[n as usize];
        }
        let idx = get_idx(n, l);
        if seen[idx] {
            return cache[idx];
        }

        let sqrtn = (n as f64).sqrt() as i64;
        let mut result: i64 = 1;

        for k in 2..=n / (sqrtn + 1) {
            result -= mertens(n / k, l, small, cache, seen);
        }

        for q in 1..=sqrtn {
            let kmax = n / q;
            let kmin = (n / (q + 1) + 1).max(n / (sqrtn + 1) + 1);
            if kmax >= kmin {
                result -= (kmax - kmin + 1) * mertens(q, l, small, cache, seen);
            }
        }

        cache[idx] = result;
        seen[idx] = true;
        result
    }

    fn sum_odd(
        n: i64,
        l: i64,
        mertens_small: &[i64],
        sum_odd_small: &[i64],
        mertens_cache: &mut [i64],
        mertens_seen: &mut [bool],
        sum_odd_cache: &mut [i64],
        sum_odd_seen: &mut [bool],
    ) -> i64 {
        if n <= l {
            return sum_odd_small[n as usize];
        }
        let idx = get_idx(n, l);
        if sum_odd_seen[idx] {
            return sum_odd_cache[idx];
        }

        let m = mertens(n, l, mertens_small, mertens_cache, mertens_seen);
        let s = sum_odd(n / 2, l, mertens_small, sum_odd_small, mertens_cache, mertens_seen, sum_odd_cache, sum_odd_seen);
        let val = m + s;
        sum_odd_cache[idx] = val;
        sum_odd_seen[idx] = true;
        val
    }

    #[inline(always)]
    fn tr(n: i64) -> i64 {
        n * (n + 1) / 2
    }

    let mut ans: i128 = 0;

    for g in (1..=N / l).step_by(2) {
        let t = N / g;
        ans += mobius[g as usize] as i128 * (tr(t) / 2) as i128;
    }

    for t in 1..l {
        let upper = N / t;
        let lower = N / (t + 1);

        let s_upper = sum_odd(upper, l, &mertens_small, &sum_odd_small, &mut mertens_cache, &mut mertens_seen, &mut sum_odd_cache, &mut sum_odd_seen);
        let s_lower = sum_odd(lower, l, &mertens_small, &sum_odd_small, &mut mertens_cache, &mut mertens_seen, &mut sum_odd_cache, &mut sum_odd_seen);

        ans += (s_upper - s_lower) as i128 * (tr(t) / 2) as i128;
    }

    println!("{}", ans as i64);
}

