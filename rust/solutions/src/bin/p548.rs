// Project Euler 548 - Gozinta Chains
//
// g(n) = number of gozinta chains for n. Find the sum of all n <= 10^16 such that g(n) = n.
// Enumerate exponent signatures, compute g, and check if g matches n.

use std::collections::HashMap;

const MAX_PRIMES: usize = 20;
const PRIMES: [i64; MAX_PRIMES] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
const N_LIMIT: i64 = 10_000_000_000_000_000; // 10^16

fn encode_exps(es: &[i32]) -> i64 {
    let mut key = es.len() as i64;
    for &e in es {
        key = key * 60 + e as i64;
    }
    key
}

fn g_func(es: &[i32], cache: &mut HashMap<i64, i64>) -> i64 {
    let n = es.len();
    if n == 0 { return 1; }

    let key = encode_exps(es);
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let mut result: i64 = 0;
    let n_subsets = 1usize << n;

    for subset in 1..n_subsets {
        let mut fs = Vec::new();
        for i in 0..n {
            let e = es[i] - ((subset >> i) & 1) as i32;
            if e > 0 { fs.push(e); }
        }
        fs.sort_unstable_by(|a, b| b.cmp(a));

        let bit_count = subset.count_ones() as i32;
        let parity = if bit_count % 2 == 0 { 1i64 } else { -1 };

        if fs.is_empty() {
            result -= parity;
        } else {
            result -= parity * 2 * g_func(&fs, cache);
        }
    }

    cache.insert(key, result);
    result
}

fn has_exponents(mut n: i64, es: &[i32]) -> bool {
    let mut es_copy: Vec<i32> = es.to_vec();

    let mut factor: i64 = 2;
    while factor * factor <= n && !es_copy.is_empty() {
        let mut e = 0;
        while n % factor == 0 {
            n /= factor;
            e += 1;
        }
        if e > 0 {
            if let Some(pos) = es_copy.iter().position(|&x| x == e) {
                es_copy.swap_remove(pos);
            } else {
                return false;
            }
        }
        factor += 1;
    }
    if n > 1 {
        if let Some(pos) = es_copy.iter().position(|&x| x == 1) {
            es_copy.swap_remove(pos);
        } else {
            return false;
        }
    }
    es_copy.is_empty()
}

fn ipow(base: i64, exp: i32) -> i64 {
    let mut result: i64 = 1;
    for _ in 0..exp {
        if result > 1_000_000_000_000_000_00 / (base + 1) {
            return 1_000_000_000_000_000_00; // overflow guard
        }
        result *= base;
    }
    result
}

fn ilog2(mut n: i64) -> i32 {
    let mut r = 0;
    while n > 1 { n >>= 1; r += 1; }
    r
}

fn helper(
    es: &mut Vec<i32>,
    n: i64,
    ans: &mut i64,
    g_cache: &mut HashMap<i64, i64>,
) {
    let ne = es.len();
    if ne > 0 {
        let mut sorted: Vec<i32> = es.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));

        let g_val = g_func(&sorted, g_cache);
        if g_val > 0 && g_val <= N_LIMIT && has_exponents(g_val, &sorted) {
            *ans += g_val;
        }
    }

    let max_c = if ne > 0 { es[ne - 1] } else { ilog2(N_LIMIT) };
    if ne >= MAX_PRIMES { return; }

    for c in 1..=max_c {
        let new_n = n * ipow(PRIMES[ne], c);
        if new_n > N_LIMIT { break; }
        es.push(c);
        helper(es, new_n, ans, g_cache);
        es.pop();
    }
}

fn main() {
    let mut ans: i64 = 0;
    let mut g_cache: HashMap<i64, i64> = HashMap::new();
    let mut es: Vec<i32> = Vec::new();

    helper(&mut es, 1, &mut ans, &mut g_cache);

    println!("{ans}");
}
