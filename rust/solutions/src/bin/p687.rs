// Project Euler 687 - Shuffling Cards
// DP with memoization: probability that number of "perfect" ranks is prime.

use std::collections::HashMap;

const N_RANKS: usize = 13;
const K_CARDS: usize = 4;

fn is_prime_small(n: usize) -> bool {
    matches!(n, 2 | 3 | 5 | 7 | 11 | 13)
}

fn make_key(r0: i32, r1: i32, r2: i32, r3: i32, r4: i32, i: i32, u: i32, p: i32) -> u64 {
    let mut k = r0 as u64;
    k = k * 14 + r1 as u64;
    k = k * 14 + r2 as u64;
    k = k * 14 + r3 as u64;
    k = k * 14 + r4 as u64;
    k = k * 5 + i as u64;
    k = k * 53 + u as u64;
    k = k * 14 + p as u64;
    k
}

fn helper(
    r: [i32; 5], i: i32, u: i32, p: i32,
    cache: &mut HashMap<u64, f64>,
) -> f64 {
    if p == r[0] && u == 0 && i == 0 {
        return if is_prime_small(p as usize) { 1.0 } else { 0.0 };
    }

    let key = make_key(r[0], r[1], r[2], r[3], r[4], i, u, p);
    if let Some(&val) = cache.get(&key) {
        return val;
    }

    let mut res = 0.0;

    // Case: next card is from imperfect ranks
    if u > 0 {
        res += u as f64 * helper(r, 0, u - 1, p, cache);
    }

    // Case: next card has same rank as top (category j == i)
    for j in 1..=4i32 {
        if r[j as usize] > 0 && j == i {
            let mut nr = r;
            nr[j as usize] -= 1;
            res += j as f64 * helper(nr, 0, u + j - 1, p - 1, cache);
        }
    }

    // Case: next card has different rank
    for j in 1..=4i32 {
        if r[j as usize] > 0 {
            let mut nr = r;
            nr[j as usize] -= 1;
            nr[(j - 1) as usize] += 1;
            let mult = j * if j == i { r[j as usize] - 1 } else { r[j as usize] };
            if mult > 0 {
                res += mult as f64 * helper(nr, j - 1, u, p, cache);
            }
        }
    }

    cache.insert(key, res);
    res
}

fn main() {
    let mut cache = HashMap::new();
    let r = [0i32, 0, 0, 0, N_RANKS as i32];
    let result = helper(r, 0, 0, N_RANKS as i32, &mut cache);

    // Divide by 52!
    let mut total_perm = 1.0f64;
    for i in 1..=(N_RANKS * K_CARDS) {
        total_perm *= i as f64;
    }

    let ans = result / total_perm;
    println!("{:.10}", ans);
}
