// Project Euler 829 - Integral Fusion
// For each n from 2 to 31, compute n!! (double factorial), find its "shape"
// (binary tree structure from balanced splitting), then find the smallest
// number with the same shape and same number of prime factors.
//
// Optimizations:
// 1. Integer shape IDs instead of String comparison
// 2. Stack-allocated factorization in best_divisor
// 3. Custom open-addressing hash for cache (C-style with key=0 sentinel)
// 4. HashMap for small pair interning table

use std::collections::HashMap;

const NN: usize = 31;
const PRIMES: [u64; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
const NPRIMES: usize = 11;

#[inline]
fn isqrt_ull(n: u64) -> u64 {
    if n == 0 { return 0; }
    let x = (n as f64).sqrt() as u64;
    let mut r = x;
    while (r + 1) as u128 * (r + 1) as u128 <= n as u128 { r += 1; }
    while (r as u128) * (r as u128) > n as u128 { r -= 1; }
    r
}

fn best_divisor(n: u64) -> u64 {
    let mut ps = [0u64; 16];
    let mut es = [0i32; 16];
    let mut nf = 0;
    {
        let mut m = n;
        for &p in &PRIMES {
            if (p as u128) * (p as u128) > m as u128 { break; }
            if m % p == 0 {
                let mut e = 0;
                while m % p == 0 { m /= p; e += 1; }
                ps[nf] = p;
                es[nf] = e;
                nf += 1;
            }
        }
        if m > 1 { ps[nf] = m; es[nf] = 1; nf += 1; }
    }
    let sqrt_n = isqrt_ull(n);
    let mut best = 1u64;

    // Precompute max_remaining[i] = max divisor using only factors from index i..nf
    let mut max_rem = [1u64; 17];
    for i in (0..nf).rev() {
        let mut pw = 1u64;
        for _ in 0..es[i] {
            pw = pw.saturating_mul(ps[i]);
        }
        max_rem[i] = max_rem[i + 1].saturating_mul(pw);
    }

    #[inline]
    fn dfs(idx: usize, nf: usize, ps: &[u64; 16], es: &[i32; 16],
           cur: u64, sqrt_n: u64, best: &mut u64, max_rem: &[u64; 17]) {
        if idx == nf {
            if cur <= sqrt_n && cur > *best { *best = cur; }
            return;
        }
        // Prune: if cur * max_remaining <= best, no point continuing
        if (cur as u128) * (max_rem[idx] as u128) <= *best as u128 { return; }
        let p = ps[idx];
        let e = es[idx];
        let mut mul = 1u64;
        for _ in 0..=e {
            let next = (cur as u128) * (mul as u128);
            if next > sqrt_n as u128 { break; }
            dfs(idx + 1, nf, ps, es, next as u64, sqrt_n, best, max_rem);
            if mul as u128 * p as u128 > sqrt_n as u128 { break; }
            mul *= p;
        }
    }

    dfs(0, nf, &ps, &es, 1, sqrt_n, &mut best, &max_rem);
    best
}

#[inline]
fn is_small_prime(n: u64) -> bool {
    matches!(n, 2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 | 31)
}

fn count_prime_factors(mut n: u64) -> i32 {
    let mut total = 0;
    for &p in &PRIMES {
        while n % p == 0 { n /= p; total += 1; }
    }
    if n > 1 { total += 1; }
    total
}

// C-style open-addressing hash table (key=0 sentinel)
// val stores shape_id + 1 (so 0 = empty)
const LEAF_ID: u32 = 0;

struct CacheOA {
    keys: Vec<u64>,
    vals: Vec<u32>,
    mask: usize,
}

impl CacheOA {
    fn new(bits: usize) -> Self {
        let size = 1usize << bits;
        CacheOA {
            keys: vec![0u64; size],
            vals: vec![0u32; size],
            mask: size - 1,
        }
    }

    #[inline(always)]
    fn hash64(x: u64) -> usize {
        let mut x = x;
        x ^= x >> 33;
        x = x.wrapping_mul(0xff51afd7ed558ccd);
        x ^= x >> 33;
        x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
        x ^= x >> 33;
        x as usize
    }

    #[inline]
    fn get(&self, key: u64) -> u32 {
        let mut idx = Self::hash64(key) & self.mask;
        loop {
            unsafe {
                let k = *self.keys.get_unchecked(idx);
                if k == 0 { return u32::MAX; }
                if k == key {
                    let v = *self.vals.get_unchecked(idx);
                    return if v == 0 { u32::MAX } else { v - 1 };
                }
            }
            idx = (idx + 1) & self.mask;
        }
    }

    #[inline]
    fn insert(&mut self, key: u64, val: u32) {
        let stored = val + 1;
        let mut idx = Self::hash64(key) & self.mask;
        loop {
            unsafe {
                let k = *self.keys.get_unchecked(idx);
                if k == 0 || k == key {
                    *self.keys.get_unchecked_mut(idx) = key;
                    *self.vals.get_unchecked_mut(idx) = stored;
                    return;
                }
            }
            idx = (idx + 1) & self.mask;
        }
    }
}

struct ShapeSystem {
    cache: CacheOA,
    pairs: HashMap<u64, u32>,
    next_id: u32,
}

impl ShapeSystem {
    fn new() -> Self {
        ShapeSystem {
            cache: CacheOA::new(21),
            pairs: HashMap::with_capacity(512),
            next_id: 1,
        }
    }

    fn shape_of(&mut self, n: u64) -> u32 {
        let cached = self.cache.get(n);
        if cached != u32::MAX {
            return cached;
        }

        let id = if is_small_prime(n) {
            LEAF_ID
        } else {
            let d = best_divisor(n);
            let left = self.shape_of(d);
            let right = self.shape_of(n / d);
            let pk = ((left as u64) << 32) | (right as u64);
            let next = self.next_id;
            let id = *self.pairs.entry(pk).or_insert(next);
            if id == next { self.next_id += 1; }
            id
        };

        self.cache.insert(n, id);
        id
    }
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

    let mut sys = ShapeSystem::new();
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
        let target_shape = sys.shape_of(ndf);

        let mut best_res = ndf;

        fn search(k: i32, min_pi: usize, cur: u64, target_shape: u32,
                  pows: &[[u64; 64]; NPRIMES], best_res: &mut u64,
                  sys: &mut ShapeSystem) {
            if k == 0 {
                let s = sys.shape_of(cur);
                if s == target_shape && cur < *best_res {
                    *best_res = cur;
                }
                return;
            }
            for pi in min_pi..NPRIMES {
                if cur as u128 * pows[pi][k as usize] as u128 > *best_res as u128 { break; }
                search(k - 1, pi, cur * PRIMES[pi], target_shape, pows, best_res, sys);
            }
        }

        search(k, 0, 1, target_shape, &pows, &mut best_res, &mut sys);
        ans += best_res;
    }

    println!("{}", ans);
}
