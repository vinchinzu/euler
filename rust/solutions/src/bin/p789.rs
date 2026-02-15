// Project Euler 789 - Minimal Pairing Product
// Bidirectional search over small-cost products of primes, meet-in-the-middle mod N.

use std::collections::HashMap;

const N_VAL: i64 = 2_000_000_011;

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    if base < 0 { base += m; }
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: i64, m: i64) -> i64 {
    powmod(a, m - 2, m)
}

fn main() {
    let primes = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97];

    let mut ans_prod: i64 = -1;
    let mut cost_bound = 1;

    while ans_prod < 0 {
        let mut hm: HashMap<i64, i32> = HashMap::new();

        fn helper(min_index: usize, cost: i32, prod: i64, cost_bound: i32, primes: &[i32], hm: &mut HashMap<i64, i32>) {
            let entry = hm.entry(prod).or_insert(cost);
            if cost < *entry { *entry = cost; }

            for i in min_index..primes.len() {
                let p = primes[i];
                if cost + p - 1 > cost_bound { return; }
                if prod > 10_000_000_000 / p as i64 { return; }
                helper(i, cost + p - 1, prod * p as i64, cost_bound, primes, hm);
            }
        }

        let primes_i32: Vec<i32> = primes.iter().map(|&p| p as i32).collect();
        helper(0, 0, 1, cost_bound, &primes_i32, &mut hm);

        let entries: Vec<(i64, i32)> = hm.iter().map(|(&k, &v)| (k, v)).collect();

        let mut min_cost = i32::MAX;
        let mut best_prod: i64 = -1;

        for &(prod, c1) in &entries {
            let inv = mod_inv(((-prod) % N_VAL + N_VAL) % N_VAL, N_VAL);
            if let Some(&c2) = hm.get(&inv) {
                let total = c1 + c2;
                if total < min_cost {
                    min_cost = total;
                    best_prod = (prod as i128 * inv as i128 % N_VAL as i128) as i64;
                }
            }
        }

        if best_prod >= 0 { ans_prod = best_prod; }
        cost_bound *= 2;
    }

    println!("{}", ans_prod);
}
