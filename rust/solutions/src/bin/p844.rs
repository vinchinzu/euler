// Project Euler 844 - k-Markov Numbers
// Compressed BFS for small k, polynomial summation for large k

const MOD: i64 = 1405695061;
const CUTOFF: i64 = 40000;

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    ones: i32,
    non_ones: Vec<i64>,
}

fn pow_mod(mut base: i128, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i128;
    let mm = m as i128;
    base %= mm;
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % mm; }
        base = base * base % mm;
        exp >>= 1;
    }
    r as i64
}

fn solve_compressed(k: i64, n: i64) -> i64 {
    let mut visited: HashSet<State> = HashSet::new();
    let mut found: HashSet<i64> = HashSet::new();
    let mut queue: VecDeque<State> = VecDeque::new();

    let init = State { ones: k as i32, non_ones: Vec::new() };
    visited.insert(init.clone());
    queue.push_back(init);
    found.insert(1);
    let mut found_sum: i64 = 1;

    while let Some(cur) = queue.pop_front() {
        let product: i128 = cur.non_ones.iter().fold(1i128, |acc, &x| acc * x as i128);

        // Try replacing a 1
        if cur.ones > 0 && k as i128 * product <= n as i128 + 1 {
            let val = (k as i128 * product - 1) as i64;
            if val <= n && val > 1 {
                let mut ns = State {
                    ones: cur.ones - 1,
                    non_ones: cur.non_ones.clone(),
                };
                ns.non_ones.push(val);
                ns.non_ones.sort();
                if visited.insert(ns.clone()) {
                    if found.insert(val) { found_sum = (found_sum + val) % MOD; }
                    queue.push_back(ns);
                }
            }
        }

        // Try replacing each non-one
        for i in 0..cur.non_ones.len() {
            let x = cur.non_ones[i];
            let p_others = product / x as i128;
            if k as i128 * p_others > n as i128 + x as i128 { continue; }
            let val = (k as i128 * p_others - x as i128) as i64;
            if val <= n && val > x {
                let mut ns = cur.clone();
                ns.non_ones[i] = val;
                ns.non_ones.sort();
                if visited.insert(ns.clone()) {
                    if found.insert(val) { found_sum = (found_sum + val) % MOD; }
                    queue.push_back(ns);
                }
            }
        }
    }

    found_sum % MOD
}

fn sum_pow(p: i32, n: i64) -> i64 {
    if n < 1 { return 0; }
    let nm = n % MOD;
    let inv2 = (MOD + 1) / 2;
    let inv6 = pow_mod(6, MOD - 2, MOD);
    let m = MOD as i128;

    match p {
        0 => nm,
        1 => (nm as i128 * ((nm + 1) % MOD) as i128 % m * inv2 as i128 % m) as i64,
        2 => (nm as i128 * ((nm + 1) % MOD) as i128 % m
              * ((2 * nm + 1) % MOD) as i128 % m * inv6 as i128 % m) as i64,
        3 => {
            let v = (nm as i128 * ((nm + 1) % MOD) as i128 % m * inv2 as i128 % m) as i64;
            (v as i128 * v as i128 % m) as i64
        }
        4 => {
            let t1 = nm as i128 * ((nm + 1) % MOD) as i128 % m;
            let t2 = (2 * nm + 1) % MOD;
            let t3 = ((3i128 * nm as i128 % m * nm as i128 % m + 3 * nm as i128 % m - 1 + m) % m) as i64;
            let inv30 = pow_mod(30, MOD - 2, MOD);
            (t1 * t2 as i128 % m * t3 as i128 % m * inv30 as i128 % m) as i64
        }
        _ => 0,
    }
}

fn poly_eval_sum(coeffs: &[i64], limit: i64) -> i64 {
    let mut total = 0i64;
    let m = MOD as i128;
    for (p, &c) in coeffs.iter().enumerate() {
        let sp = sum_pow(p as i32, limit);
        let cm = ((c as i128 % m) + m) % m;
        total = ((total as i128 + cm * sp as i128) % m) as i64;
    }
    total
}

fn poly_sum_range(coeffs: &[i64], start_k: i64, end_k: i64) -> i64 {
    if start_k > end_k { return 0; }
    let s_end = poly_eval_sum(coeffs, end_k);
    let s_start = poly_eval_sum(coeffs, start_k - 1);
    (s_end - s_start + MOD) % MOD
}

fn poly_eval_exact(coeffs: &[i64], k: i64) -> i128 {
    let mut res = 0i128;
    let mut kp = 1i128;
    for &c in coeffs {
        res += c as i128 * kp;
        kp *= k as i128;
    }
    res
}

fn find_limit(poly: &[i64], n: i64, start_k: i64, max_k: i64) -> i64 {
    if poly_eval_exact(poly, start_k) > n as i128 { return start_k - 1; }
    let mut low = start_k;
    let mut high = max_k;
    let mut ans = start_k - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if poly_eval_exact(poly, mid) <= n as i128 { ans = mid; low = mid + 1; }
        else { high = mid - 1; }
    }
    ans
}

fn main() {
    let k_val: i64 = 1_000_000_000_000_000_000;
    let n_val: i64 = 1_000_000_000_000_000_000;

    let mut total_sum: i64 = 0;

    // 1. Small k: BFS
    let limit_small = CUTOFF.min(k_val);
    for k in 3..=limit_small {
        let mk = solve_compressed(k, n_val);
        total_sum = (total_sum + mk) % MOD;
    }

    // 2. Large k: polynomial summation
    if k_val > CUTOFF {
        let polys: Vec<Vec<i64>> = vec![
            vec![1],
            vec![-1, 1],
            vec![-1, -1, 1],
            vec![1, -2, -1, 1],
        ];

        let start_k_large = CUTOFF + 1;

        for poly in &polys {
            let limit = find_limit(poly, n_val, start_k_large, k_val);
            if limit >= start_k_large {
                let term_sum = poly_sum_range(poly, start_k_large, limit);
                total_sum = (total_sum + term_sum) % MOD;
            }
        }
    }

    println!("{}", total_sum);
}
