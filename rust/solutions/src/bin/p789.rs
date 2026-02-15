// Project Euler 789 - Minimal Pairing Modulo p
//
// Partition {1..p-1} into (p-1)/2 pairs (a_i, b_i) minimizing sum of (a_i*b_i mod p).
// The cost product = product of all (a_i*b_i mod p). All optimal pairings share the
// same cost product.
//
// Key insight: the cost product is congruent to (p-1)! = -1 (mod p) by Wilson's theorem.
// We search for a number congruent to -1 (mod p) that is a product of small primes
// with minimal sum of (prime-1). This is done by meet-in-the-middle: enumerate products
// of small primes with bounded cost, then for each product A find if there exists a
// product B such that A*B = -1 (mod p), i.e. B = (-1/A) mod p.
//
// The answer is the raw product A*B (NOT reduced mod p), which fits in u64.

use std::collections::HashMap;

const N_VAL: u64 = 2_000_000_011;

fn powmod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as u128 * base as u128 % m as u128) as u64; }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: u64, m: u64) -> u64 {
    powmod(a, m - 2, m)
}

fn helper(min_index: usize, cost: i32, prod: u64, cost_bound: i32, primes: &[u64], hm: &mut HashMap<u64, i32>) {
    let entry = hm.entry(prod).or_insert(cost);
    if cost < *entry { *entry = cost; }

    for i in min_index..primes.len() {
        let p = primes[i];
        let pc = p as i32;
        if cost + pc - 1 > cost_bound { return; }
        if prod > 10_000_000_000 / p { return; }
        helper(i, cost + pc - 1, prod * p, cost_bound, primes, hm);
    }
}

fn main() {
    let primes: Vec<u64> = vec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97];

    let mut ans: Option<u64> = None;
    let mut cost_bound = 1i32;

    while ans.is_none() {
        let mut hm: HashMap<u64, i32> = HashMap::new();

        helper(0, 0, 1, cost_bound, &primes, &mut hm);

        let entries: Vec<(u64, i32)> = hm.iter().map(|(&k, &v)| (k, v)).collect();

        let mut min_cost = i32::MAX;

        for &(prod, c1) in &entries {
            // We need prod * inv = -1 (mod N), so inv = mod_inv(prod, N) * (N-1) mod N
            // Equivalently inv = mod_inv(N - (prod % N), N) when prod % N != 0
            let prod_mod = prod % N_VAL;
            if prod_mod == 0 { continue; }
            let neg_prod_mod = N_VAL - prod_mod;
            let inv = mod_inv(neg_prod_mod, N_VAL);
            if let Some(&c2) = hm.get(&inv) {
                let total = c1 + c2;
                if total < min_cost {
                    min_cost = total;
                    // The answer is the raw product, NOT reduced mod N
                    ans = Some(prod * inv);
                }
            }
        }

        cost_bound *= 2;
    }

    println!("{}", ans.unwrap());
}
