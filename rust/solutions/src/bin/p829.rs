// Project Euler 829 - Integral Fusion
// For each n from 2 to 31, compute n!! (double factorial), find its "shape"
// (binary tree structure from balanced splitting), then find the smallest
// number with the same shape and same number of prime factors.
//
// Optimizations:
// 1. Integer shape IDs instead of String comparison
// 2. Meet-in-the-middle largest-divisor-≤√n (hot path of shape matching)
// 3. Ω-prune: reject a split if leaf counts of the target subtrees disagree
// 4. Custom open-addressing hash for cache (C-style with key=0 sentinel)
// 5. Rayon over n=2..31 and prime-prefix DFS (thread-local shape caches)

use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

const NN: usize = 31;
const PRIMES: [u64; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
const NPRIMES: usize = 11;
// Parallel-split while remaining Ω is large and the next prime can still be small.
const SPLIT_K: i32 = 10;
const SPLIT_PI: usize = 4;
const DIV_CAP: usize = 8192;

#[inline(always)]
fn isqrt_ull(n: u64) -> u64 {
    n.isqrt()
}

fn factor_small(n: u64, ps: &mut [u64; 16], es: &mut [i32; 16]) -> usize {
    let mut nf = 0;
    let mut m = n;
    for &p in &PRIMES {
        if (p as u128) * (p as u128) > m as u128 {
            break;
        }
        if m % p == 0 {
            let mut e = 0;
            while m % p == 0 {
                m /= p;
                e += 1;
            }
            ps[nf] = p;
            es[nf] = e;
            nf += 1;
        }
    }
    if m > 1 {
        ps[nf] = m;
        es[nf] = 1;
        nf += 1;
    }
    nf
}

fn best_divisor_dfs(ps: &[u64; 16], es: &[i32; 16], nf: usize, sqrt_n: u64) -> u64 {
    let mut max_rem = [1u64; 17];
    for i in (0..nf).rev() {
        let mut pw = 1u64;
        for _ in 0..es[i] {
            pw = pw.saturating_mul(ps[i]);
        }
        max_rem[i] = max_rem[i + 1].saturating_mul(pw);
    }

    // Greedy largest-prime-first seed so max_remaining prunes immediately.
    let mut best = {
        let mut rem = sqrt_n;
        let mut d = 1u64;
        for i in (0..nf).rev() {
            let p = ps[i];
            for _ in 0..es[i] {
                if rem < p {
                    break;
                }
                d *= p;
                rem /= p;
            }
        }
        d
    };

    fn dfs(
        idx: usize,
        nf: usize,
        ps: &[u64; 16],
        es: &[i32; 16],
        cur: u64,
        sqrt_n: u64,
        best: &mut u64,
        max_rem: &[u64; 17],
    ) {
        if idx == nf {
            if cur <= sqrt_n && cur > *best {
                *best = cur;
            }
            return;
        }
        if (cur as u128) * (max_rem[idx] as u128) <= *best as u128 {
            return;
        }
        let p = ps[idx];
        let e = es[idx];
        let mut mul = 1u64;
        for _ in 0..=e {
            let next = (cur as u128) * (mul as u128);
            if next > sqrt_n as u128 {
                break;
            }
            dfs(idx + 1, nf, ps, es, next as u64, sqrt_n, best, max_rem);
            if mul as u128 * p as u128 > sqrt_n as u128 {
                break;
            }
            mul *= p;
        }
    }

    dfs(0, nf, ps, es, 1, sqrt_n, &mut best, &max_rem);
    best
}

fn gen_divs(
    idx: usize,
    end: usize,
    cur: u64,
    limit: u64,
    ps: &[u64; 16],
    es: &[i32; 16],
    out: &mut Vec<u64>,
) -> bool {
    if idx == end {
        if out.len() >= DIV_CAP {
            return false;
        }
        out.push(cur);
        return true;
    }
    let p = ps[idx];
    let e = es[idx];
    let mut pw = 1u64;
    for _ in 0..=e {
        let next = cur as u128 * pw as u128;
        if next > limit as u128 {
            break;
        }
        if !gen_divs(idx + 1, end, next as u64, limit, ps, es, out) {
            return false;
        }
        if pw as u128 * p as u128 > limit as u128 {
            break;
        }
        pw *= p;
    }
    true
}

fn best_divisor(n: u64, buf_a: &mut Vec<u64>, buf_b: &mut Vec<u64>) -> u64 {
    let mut ps = [0u64; 16];
    let mut es = [0i32; 16];
    let nf = factor_small(n, &mut ps, &mut es);
    if nf == 0 {
        return 1;
    }
    let sqrt_n = isqrt_ull(n);
    if nf == 1 {
        let p = ps[0];
        let mut pw = 1u64;
        for _ in 0..es[0] / 2 {
            pw *= p;
        }
        return pw;
    }
    if nf == 2 {
        let (p, q) = (ps[0], ps[1]);
        let (ea, eb) = (es[0], es[1]);
        let mut best = 1u64;
        let mut pp = 1u64;
        for _i in 0..=ea {
            let mut qq = 1u64;
            for _j in 0..=eb {
                let prod = pp as u128 * qq as u128;
                if prod > sqrt_n as u128 {
                    break;
                }
                let d = prod as u64;
                if d > best {
                    best = d;
                }
                if qq as u128 * q as u128 > sqrt_n as u128 {
                    break;
                }
                qq *= q;
            }
            if pp as u128 * p as u128 > sqrt_n as u128 {
                break;
            }
            pp *= p;
        }
        return best;
    }

    // Balance the two MITM halves by divisor-count.
    let mut tau = [1u32; 17];
    for i in 0..nf {
        tau[i + 1] = tau[i].saturating_mul(es[i] as u32 + 1);
    }
    let tot = tau[nf];
    let mut mid = nf / 2;
    let mut best_diff = u32::MAX;
    for i in 1..nf {
        let l = tau[i];
        let r = tot / l;
        let d = if l > r { l - r } else { r - l };
        if d < best_diff {
            best_diff = d;
            mid = i;
        }
    }

    buf_a.clear();
    buf_b.clear();
    if !gen_divs(0, mid, 1, sqrt_n, &ps, &es, buf_a)
        || !gen_divs(mid, nf, 1, sqrt_n, &ps, &es, buf_b)
    {
        return best_divisor_dfs(&ps, &es, nf, sqrt_n);
    }

    let left = buf_a.as_mut_slice();
    let right = buf_b.as_mut_slice();
    let nleft = left.len();
    let nright = right.len();
    if nleft == 0 || nright == 0 {
        return 1;
    }

    let mut best = 1u64;
    if nleft.saturating_mul(nright) <= 256 {
        for &a in left.iter() {
            for &b in right.iter() {
                let prod = a as u128 * b as u128;
                if prod <= sqrt_n as u128 {
                    let d = prod as u64;
                    if d > best {
                        best = d;
                    }
                }
            }
        }
        return best;
    }

    left.sort_unstable();
    right.sort_unstable();
    let mut j = nright as i32 - 1;
    for &a in left.iter() {
        while j >= 0 && a as u128 * right[j as usize] as u128 > sqrt_n as u128 {
            j -= 1;
        }
        if j < 0 {
            break;
        }
        let d = a * right[j as usize];
        if d > best {
            best = d;
        }
    }
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
    children: Vec<(u32, u32)>,
    nleaves: Vec<i32>,
    buf_a: Vec<u64>,
    buf_b: Vec<u64>,
}

impl ShapeSystem {
    fn new() -> Self {
        ShapeSystem {
            cache: CacheOA::new(18),
            pairs: HashMap::with_capacity(512),
            next_id: 1,
            children: vec![(0, 0)],
            nleaves: vec![1],
            buf_a: Vec::with_capacity(DIV_CAP),
            buf_b: Vec::with_capacity(DIV_CAP),
        }
    }

    fn intern_pair(&mut self, left: u32, right: u32) -> u32 {
        let pk = ((left as u64) << 32) | (right as u64);
        let next = self.next_id;
        let id = *self.pairs.entry(pk).or_insert(next);
        if id == next {
            self.next_id += 1;
            self.children.push((left, right));
            self.nleaves
                .push(self.nleaves[left as usize] + self.nleaves[right as usize]);
        }
        id
    }

    fn shape_of(&mut self, n: u64) -> u32 {
        let cached = self.cache.get(n);
        if cached != u32::MAX {
            return cached;
        }

        let id = if is_small_prime(n) {
            LEAF_ID
        } else {
            let d = best_divisor(n, &mut self.buf_a, &mut self.buf_b);
            let left = self.shape_of(d);
            let right = self.shape_of(n / d);
            self.intern_pair(left, right)
        };

        self.cache.insert(n, id);
        id
    }

    // True iff T(n) has the same interned shape as `tid`.
    // After the first split, reject on Ω(left)/Ω(right) before recursing.
    fn matches(&mut self, n: u64, tid: u32) -> bool {
        let cached = self.cache.get(n);
        if cached != u32::MAX {
            return cached == tid;
        }
        if tid == LEAF_ID {
            return is_small_prime(n);
        }
        if is_small_prime(n) {
            return false;
        }
        let d = best_divisor(n, &mut self.buf_a, &mut self.buf_b);
        let (tl, tr) = self.children[tid as usize];
        if count_prime_factors(d) != self.nleaves[tl as usize] {
            return false;
        }
        let nd = n / d;
        if count_prime_factors(nd) != self.nleaves[tr as usize] {
            return false;
        }
        if !self.matches(d, tl) || !self.matches(nd, tr) {
            return false;
        }
        self.cache.insert(n, tid);
        true
    }
}

thread_local! {
    static SYS: RefCell<ShapeSystem> = RefCell::new(ShapeSystem::new());
}

fn search_seq(
    k: i32,
    min_pi: usize,
    cur: u64,
    target_shape: u32,
    pows: &[[u64; 64]; NPRIMES],
    best: &AtomicU64,
    sys: &mut ShapeSystem,
) {
    if k == 0 {
        if cur >= best.load(Ordering::Relaxed) {
            return;
        }
        if sys.matches(cur, target_shape) {
            best.fetch_min(cur, Ordering::Relaxed);
        }
        return;
    }
    for pi in min_pi..NPRIMES {
        let bound = best.load(Ordering::Relaxed);
        if (cur as u128) * (pows[pi][k as usize] as u128) > bound as u128 {
            break;
        }
        search_seq(k - 1, pi, cur * PRIMES[pi], target_shape, pows, best, sys);
    }
}

fn fork_pis(
    pis: &[usize],
    k: i32,
    cur: u64,
    ndf: u64,
    pows: &[[u64; 64]; NPRIMES],
    best: &AtomicU64,
) {
    match pis.len() {
        0 => {}
        1 => {
            let pi = pis[0];
            search_par(k - 1, pi, cur * PRIMES[pi], ndf, pows, best);
        }
        n => {
            let mid = n / 2;
            rayon::join(
                || fork_pis(&pis[..mid], k, cur, ndf, pows, best),
                || fork_pis(&pis[mid..], k, cur, ndf, pows, best),
            );
        }
    }
}

fn search_par(
    k: i32,
    min_pi: usize,
    cur: u64,
    ndf: u64,
    pows: &[[u64; 64]; NPRIMES],
    best: &AtomicU64,
) {
    if k <= SPLIT_K || min_pi >= SPLIT_PI {
        SYS.with(|cell| {
            let mut sys = cell.borrow_mut();
            let target = sys.shape_of(ndf);
            search_seq(k, min_pi, cur, target, pows, best, &mut sys);
        });
        return;
    }

    let bound = best.load(Ordering::Relaxed);
    let mut pis = [0usize; NPRIMES];
    let mut npi = 0usize;
    for pi in min_pi..NPRIMES {
        if (cur as u128) * (pows[pi][k as usize] as u128) > bound as u128 {
            break;
        }
        pis[npi] = pi;
        npi += 1;
    }
    fork_pis(&pis[..npi], k, cur, ndf, pows, best);
}

fn m_of_n(n: usize, pows: &[[u64; 64]; NPRIMES]) -> u64 {
    let mut ndf: u64 = 1;
    let mut i = n as u64;
    while i > 0 {
        ndf = ndf.saturating_mul(i);
        if i < 2 { break; }
        i -= 2;
    }

    let k = count_prime_factors(ndf);
    let best = AtomicU64::new(ndf);
    search_par(k, 0, 1, ndf, pows, &best);
    best.load(Ordering::Relaxed)
}

fn main() {
    let mut pows = [[0u64; 64]; NPRIMES];
    for i in 0..NPRIMES {
        pows[i][0] = 1;
        for e in 1..64 {
            let v = pows[i][e - 1] as u128 * PRIMES[i] as u128;
            pows[i][e] = if v > u64::MAX as u128 { u64::MAX } else { v as u64 };
        }
    }

    debug_assert_eq!(m_of_n(9, &pows), 72);

    let ans: u64 = (2..NN + 1).into_par_iter().rev().map(|n| m_of_n(n, &pows)).sum();
    println!("{}", ans);
}
