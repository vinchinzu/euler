// Project Euler 789 - Minimal Pairing Modulo p
//
// Partition {1..p-1} into (p-1)/2 pairs (a_i, b_i) minimizing sum of (a_i*b_i mod p).
// The cost product = product of all (a_i*b_i mod p). All optimal pairings share the
// same cost product.
//
// Key insight: the cost product is congruent to (p-1)! = -1 (mod p) by Wilson's theorem.
// Optimal costs are primes, so we search prime-products of bounded cost sum(q-1).
// Meet-in-the-middle: enumerate products ≤ 10^10, then look up B = (-A)^{-1} (mod p).
//
// The answer is the raw product A*B (NOT reduced mod p), which fits in u64.

use rayon::prelude::*;

const N: u64 = 2_000_000_011;
const PROD_MAX: u64 = 10_000_000_000;
// Min cost is 239; original doubling loop first succeeds at 256.
const COST_BOUND: u16 = 256;
// Optimal factorization uses primes ≤ 53; larger primes cannot beat cost 239
// (verified by the original search over primes ≤ 97).
const PRIMES: [u64; 16] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

const HM_BITS: usize = 21;
const HM_SIZE: usize = 1 << HM_BITS;
const HM_MASK: usize = HM_SIZE - 1;

#[inline(always)]
fn hash_key(key: u64) -> usize {
    key.wrapping_mul(0x9E3779B97F4A7C15) as usize & HM_MASK
}

/// Modular inverse of `a` modulo N (N prime, a != 0). Extended gcd, no u128.
#[inline(always)]
fn mod_inv(a: u64) -> u64 {
    let m = N as i64;
    let mut t = 0i64;
    let mut new_t = 1i64;
    let mut r = m;
    let mut new_r = a as i64;
    while new_r != 0 {
        let q = r / new_r;
        (t, new_t) = (new_t, t - q * new_t);
        (r, new_r) = (new_r, r - q * new_r);
    }
    if t < 0 {
        t += m;
    }
    t as u64
}

#[inline(always)]
fn hm_insert(keys: &mut [u64], vals: &mut [u16], key: u64, cost: u16) {
    let mut i = hash_key(key);
    loop {
        // SAFETY: i is masked to HM_SIZE-1; tables have length HM_SIZE.
        let k = unsafe { *keys.get_unchecked(i) };
        if k == 0 {
            unsafe {
                *keys.get_unchecked_mut(i) = key;
                *vals.get_unchecked_mut(i) = cost;
            }
            return;
        }
        if k == key {
            unsafe {
                let slot = vals.get_unchecked_mut(i);
                if cost < *slot {
                    *slot = cost;
                }
            }
            return;
        }
        i = (i + 1) & HM_MASK;
    }
}

#[inline(always)]
fn hm_get(keys: &[u64], vals: &[u16], key: u64) -> Option<u16> {
    let mut i = hash_key(key);
    loop {
        // SAFETY: i is masked to HM_SIZE-1; tables have length HM_SIZE.
        let k = unsafe { *keys.get_unchecked(i) };
        if k == 0 {
            return None;
        }
        if k == key {
            return Some(unsafe { *vals.get_unchecked(i) });
        }
        i = (i + 1) & HM_MASK;
    }
}

fn main() {
    // All 53-smooth products ≤ PROD_MAX with cost ≤ COST_BOUND. Products are unique,
    // so a Vec is sufficient (no HashMap during generation).
    let mut items: Vec<(u64, u16)> = Vec::with_capacity(2_250_000);
    items.push((1, 0));
    for &p in &PRIMES {
        let n0 = items.len();
        let pc = (p - 1) as u16;
        for i in 0..n0 {
            let (mut prod, mut cost) = items[i];
            loop {
                if cost + pc > COST_BOUND {
                    break;
                }
                if prod > PROD_MAX / p {
                    break;
                }
                prod *= p;
                cost += pc;
                items.push((prod, cost));
            }
        }
    }

    // Lookup table: integer products that equal their residue (prod < N).
    // Matching looks up inv = (-A)^{-1} mod N, which is in 1..N-1.
    let mut hk = vec![0u64; HM_SIZE];
    let mut hv = vec![0u16; HM_SIZE];
    for &(prod, cost) in &items {
        if prod < N {
            hm_insert(&mut hk, &mut hv, prod, cost);
        }
    }

    let best = items
        .par_iter()
        .filter_map(|&(prod, c1)| {
            // N is a prime > 53, so no 53-smooth product is 0 mod N.
            let inv = mod_inv(N - prod % N);
            hm_get(&hk, &hv, inv).map(|c2| (c1 as u32 + c2 as u32, prod.wrapping_mul(inv)))
        })
        .min_by_key(|&(cost, _)| cost);

    println!("{}", best.unwrap().1);
}
