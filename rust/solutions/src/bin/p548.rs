// Project Euler 548 - Gozinta Chains
//
// g(n) = number of gozinta chains for n. Find the sum of all n <= 10^16 such that g(n) = n.
// Enumerate exponent signatures, compute g, and check if g matches n.

use euler_utils::number::factor;
use fxhash::FxHashMap;
use rayon::prelude::*;

const MAX_PRIMES: usize = 20;
const PRIMES: [u64; MAX_PRIMES] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
];
const N_LIMIT: u64 = 10_000_000_000_000_000; // 10^16

#[inline(always)]
fn encode_exps(es: &[i32]) -> u128 {
    let mut key = es.len() as u128;
    for &e in es {
        key = (key << 6) | (e as u128);
    }
    key
}

fn g_func(es: &[i32], cache: &mut FxHashMap<u128, i64>) -> i64 {
    let n = es.len();
    if n == 0 {
        return 1;
    }
    // g(p^e) = 2^{e-1}
    if n == 1 {
        return 1i64 << (es[0] - 1);
    }

    let key = encode_exps(es);
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let mut result: i64 = 0;
    let n_subsets = 1usize << n;

    for subset in 1..n_subsets {
        let mut fs = [0i32; MAX_PRIMES];
        let mut fn_ = 0usize;
        for i in 0..n {
            // SAFETY: i < es.len(); fn_ < n <= MAX_PRIMES
            let e = unsafe { *es.get_unchecked(i) } - ((subset >> i) & 1) as i32;
            if e > 0 {
                unsafe {
                    *fs.get_unchecked_mut(fn_) = e;
                }
                fn_ += 1;
            }
        }
        fs[..fn_].sort_unstable_by(|a, b| b.cmp(a));

        let term = if fn_ == 0 {
            1
        } else {
            2 * g_func(&fs[..fn_], cache)
        };
        if subset.count_ones() & 1 == 0 {
            result -= term;
        } else {
            result += term;
        }
    }

    cache.insert(key, result);
    result
}

fn has_exponents(n: u64, es: &[i32]) -> bool {
    let fac = factor(n);
    if fac.len() != es.len() {
        return false;
    }
    let k = fac.len();
    let mut exp = [0i32; MAX_PRIMES];
    for i in 0..k {
        exp[i] = fac[i].1 as i32;
    }
    exp[..k].sort_unstable_by(|a, b| b.cmp(a));
    exp[..k] == es[..]
}

#[inline(always)]
fn ilog2(n: u64) -> i32 {
    63 - n.leading_zeros() as i32
}

fn helper(
    es: &mut [i32],
    ne: usize,
    n: u64,
    g_cache: &mut FxHashMap<u128, i64>,
    candidates: &mut Vec<(u64, [i32; MAX_PRIMES], u8)>,
) {
    if ne > 0 {
        let g_val = g_func(&es[..ne], g_cache);
        if g_val > 0 && (g_val as u64) <= N_LIMIT {
            let mut packed = [0i32; MAX_PRIMES];
            packed[..ne].copy_from_slice(&es[..ne]);
            candidates.push((g_val as u64, packed, ne as u8));
        }
    }

    if ne >= MAX_PRIMES {
        return;
    }
    let max_c = if ne > 0 { es[ne - 1] } else { ilog2(N_LIMIT) };
    let p = PRIMES[ne];
    let mut p_pow = 1u64;
    for c in 1..=max_c {
        p_pow = match p_pow.checked_mul(p) {
            Some(v) => v,
            None => break,
        };
        let new_n = match n.checked_mul(p_pow) {
            Some(v) => v,
            None => break,
        };
        if new_n > N_LIMIT {
            break;
        }
        es[ne] = c;
        helper(es, ne + 1, new_n, g_cache, candidates);
    }
}

fn main() {
    let mut g_cache = FxHashMap::with_capacity_and_hasher(1 << 16, Default::default());
    let mut es = [0i32; MAX_PRIMES];
    let mut candidates = Vec::with_capacity(5_000);

    helper(&mut es, 0, 1, &mut g_cache, &mut candidates);

    let rest: u64 = candidates
        .par_iter()
        .map(|&(g, ref packed, ne)| {
            if has_exponents(g, &packed[..ne as usize]) {
                g
            } else {
                0
            }
        })
        .sum();

    println!("{}", rest + 1);
}
