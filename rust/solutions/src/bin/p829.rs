// Project Euler 829 - Integral Fusion

const NN: usize = 31;
const PRIMES: [u64; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
const NPRIMES: usize = 11;

fn isqrt_ull(n: u64) -> u64 {
    let x = (n as f64).sqrt() as u64;
    let mut r = x;
    while (r + 1) as u128 * (r + 1) as u128 <= n as u128 { r += 1; }
    while (r as u128) * (r as u128) > n as u128 { r -= 1; }
    r
}

fn factorize(mut n: u64) -> Vec<(u64, i32)> {
    let mut result = Vec::new();
    for &p in &PRIMES {
        if (p as u128) * (p as u128) > n as u128 { break; }
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 { n /= p; e += 1; }
            result.push((p, e));
        }
    }
    if n > 1 { result.push((n, 1)); }
    result
}

fn best_divisor(n: u64) -> u64 {
    let factors = factorize(n);
    let sqrt_n = isqrt_ull(n);
    let mut best = 1u64;

    fn dfs(idx: usize, factors: &[(u64, i32)], cur: u64, sqrt_n: u64, best: &mut u64) {
        if idx == factors.len() {
            if cur <= sqrt_n && cur > *best { *best = cur; }
            return;
        }
        let (p, e) = factors[idx];
        let mut mul = 1u64;
        for _ in 0..=e {
            let next = (cur as u128) * (mul as u128);
            if next > sqrt_n as u128 { break; }
            dfs(idx + 1, factors, next as u64, sqrt_n, best);
            if mul as u128 * p as u128 > sqrt_n as u128 { break; }
            mul *= p;
        }
    }

    dfs(0, &factors, 1, sqrt_n, &mut best);
    best
}

fn is_small_prime(n: u64) -> bool {
    PRIMES.contains(&n)
}

fn count_prime_factors(mut n: u64) -> i32 {
    let mut total = 0;
    for &p in &PRIMES {
        while n % p == 0 { n /= p; total += 1; }
    }
    if n > 1 { total += 1; }
    total
}

use std::collections::HashMap;

fn shape_of(n: u64, cache: &mut HashMap<u64, String>) -> String {
    if let Some(s) = cache.get(&n) {
        return s.clone();
    }

    let s = if is_small_prime(n) {
        ".".to_string()
    } else {
        let d = best_divisor(n);
        let left = shape_of(d, cache);
        let right = shape_of(n / d, cache);
        format!("({}{})", left, right)
    };

    cache.insert(n, s.clone());
    s
}

fn main() {
    // Precompute prime powers
    let mut pows = [[0u64; 64]; NPRIMES];
    for i in 0..NPRIMES {
        pows[i][0] = 1;
        for e in 1..64 {
            let v = pows[i][e - 1] as u128 * PRIMES[i] as u128;
            pows[i][e] = if v > u64::MAX as u128 { u64::MAX } else { v as u64 };
        }
    }

    let mut cache: HashMap<u64, String> = HashMap::new();
    let mut ans: u64 = 0;

    for n in 2..=NN {
        // n!! (double factorial)
        let mut ndf: u64 = 1;
        let mut i = n as u64;
        while i > 0 {
            ndf = ndf.saturating_mul(i);
            if i < 2 { break; }
            i -= 2;
        }

        let k = count_prime_factors(ndf);
        let target = shape_of(ndf, &mut cache);

        let mut best_res = ndf;

        fn search(k: i32, min_pi: usize, cur: u64, target: &str, pows: &[[u64; 64]; NPRIMES],
                  best_res: &mut u64, cache: &mut HashMap<u64, String>) {
            if k == 0 {
                let s = shape_of(cur, cache);
                if s == target && cur < *best_res {
                    *best_res = cur;
                }
                return;
            }
            for pi in min_pi..NPRIMES {
                if cur as u128 * pows[pi][k as usize] as u128 > *best_res as u128 { break; }
                search(k - 1, pi, cur * PRIMES[pi], target, pows, best_res, cache);
            }
        }

        search(k, 0, 1, &target, &pows, &mut best_res, &mut cache);
        ans += best_res;
    }

    println!("{}", ans);
}
