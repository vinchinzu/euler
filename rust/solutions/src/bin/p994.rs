// Problem 994: Counting Triangles
// T(1234*10^8, 2345*10^8) mod 10^9+7 with Du Jiao / totient prefix sums.
// Bottom-up Du Jiao; parallelize floor-block accumulation for large n.

use fxhash::FxHashMap;
use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const INV2: u64 = (MOD + 1) / 2;
const INV6: u64 = 166_666_668;

const SIEVE_LIMIT: usize = 50_000_000;
/// Parallelize Du Jiao block reduction when n is large enough.
const PAR_N_THRESHOLD: u64 = 5_000_000;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % MOD as u128) as u64
}

#[inline(always)]
fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}

#[inline(always)]
fn add_mod(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn c2_mod(x: u64) -> u64 {
    let x = x % MOD;
    mul(mul(x, (x + MOD - 1) % MOD), INV2)
}

#[inline(always)]
fn c3_mod(x: u64) -> u64 {
    let x = x % MOD;
    mul(mul(mul(x, (x + MOD - 1) % MOD), (x + MOD - 2) % MOD), INV6)
}

#[inline(always)]
fn p1(n: u64) -> u64 {
    let n = n % MOD;
    mul(mul(n, (n + 1) % MOD), INV2)
}

#[inline(always)]
fn p2(n: u64) -> u64 {
    let n = n % MOD;
    mul(mul(mul(n, (n + 1) % MOD), (2 * n + 1) % MOD), INV6)
}

#[inline(always)]
fn p3(n: u64) -> u64 {
    let s = p1(n);
    mul(s, s)
}

struct TotientPrefix {
    limit: u64,
    pref0: Vec<u32>,
    pref1: Vec<u32>,
    pref2: Vec<u32>,
    big: FxHashMap<u64, (u64, u64, u64)>,
}

impl TotientPrefix {
    fn new(limit: usize) -> Self {
        let (pref0, pref1, pref2) = Self::build(limit);
        Self {
            limit: limit as u64,
            pref0,
            pref1,
            pref2,
            big: FxHashMap::default(),
        }
    }

    fn build(limit: usize) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
        let mut phi = vec![0u32; limit + 1];
        let mut primes = Vec::with_capacity(limit / 10);
        let mut lp = vec![0u32; limit + 1];

        phi[1] = 1;
        for i in 2..=limit {
            if lp[i] == 0 {
                lp[i] = i as u32;
                primes.push(i as u32);
                phi[i] = (i - 1) as u32;
            }
            for &p in &primes {
                let ip = i as u64 * p as u64;
                if ip > limit as u64 || p > lp[i] {
                    break;
                }
                let j = ip as usize;
                lp[j] = p;
                if p == lp[i] {
                    phi[j] = phi[i].wrapping_mul(p);
                    break;
                } else {
                    phi[j] = phi[i].wrapping_mul(p - 1);
                }
            }
        }
        drop(lp);
        drop(primes);

        let mut pref0 = vec![0u32; limit + 1];
        let mut pref1 = vec![0u32; limit + 1];
        let mut pref2 = vec![0u32; limit + 1];
        let mut s0 = 0u64;
        let mut s1 = 0u64;
        let mut s2 = 0u64;
        for i in 1..=limit {
            let ph = phi[i] as u64;
            let im = (i as u64) % MOD;
            s0 += ph;
            if s0 >= MOD { s0 -= MOD; }
            s1 += mul(im, ph);
            if s1 >= MOD { s1 -= MOD; }
            s2 += mul(mul(im, im), ph);
            if s2 >= MOD { s2 -= MOD; }
            pref0[i] = s0 as u32;
            pref1[i] = s1 as u32;
            pref2[i] = s2 as u32;
        }
        (pref0, pref1, pref2)
    }

    #[inline(always)]
    fn prefix_at(&self, n: u64) -> (u64, u64, u64) {
        let i = n as usize;
        (self.pref0[i] as u64, self.pref1[i] as u64, self.pref2[i] as u64)
    }

    #[inline(always)]
    fn values(&self, n: u64) -> (u64, u64, u64) {
        if n <= self.limit {
            self.prefix_at(n)
        } else {
            self.big[&n]
        }
    }

    #[inline(always)]
    fn lookup_q(&self, q: u64) -> (u64, u64, u64) {
        if q <= self.limit {
            self.prefix_at(q)
        } else {
            self.big[&q]
        }
    }

    fn compute_one(&self, n: u64) -> (u64, u64, u64) {
        let mut f0 = p1(n);
        let mut f1 = p2(n);
        let mut f2 = p3(n);

        if n >= PAR_N_THRESHOLD {
            // Collect floor blocks, then parallel reduce contributions.
            let mut blocks: Vec<(u64, u64, u64)> = Vec::with_capacity(((n as f64).sqrt() as usize) * 2 + 16);
            let mut l = 2u64;
            while l <= n {
                let q = n / l;
                let r = n / q;
                blocks.push((l, r, q));
                l = r + 1;
            }

            let (d0, d1, d2) = blocks
                .par_iter()
                .map(|&(l, r, q)| {
                    let sum_0 = (r - l + 1) % MOD;
                    let sum_1 = sub_mod(p1(r), p1(l - 1));
                    let sum_2 = sub_mod(p2(r), p2(l - 1));
                    let (sub0, sub1, sub2) = self.lookup_q(q);
                    (mul(sum_0, sub0), mul(sum_1, sub1), mul(sum_2, sub2))
                })
                .reduce(
                    || (0u64, 0u64, 0u64),
                    |a, b| (add_mod(a.0, b.0), add_mod(a.1, b.1), add_mod(a.2, b.2)),
                );
            f0 = sub_mod(f0, d0);
            f1 = sub_mod(f1, d1);
            f2 = sub_mod(f2, d2);
        } else {
            let mut l = 2u64;
            while l <= n {
                let q = n / l;
                let r = n / q;
                let sum_0 = (r - l + 1) % MOD;
                let sum_1 = sub_mod(p1(r), p1(l - 1));
                let sum_2 = sub_mod(p2(r), p2(l - 1));
                let (sub0, sub1, sub2) = self.lookup_q(q);
                f0 = sub_mod(f0, mul(sum_0, sub0));
                f1 = sub_mod(f1, mul(sum_1, sub1));
                f2 = sub_mod(f2, mul(sum_2, sub2));
                l = r + 1;
            }
        }
        (f0, f1, f2)
    }

    fn precompute_for(&mut self, ns: &[u64]) {
        let limit = self.limit;
        let mut keys: Vec<u64> = Vec::with_capacity(4096);
        for &n in ns {
            if n <= limit {
                continue;
            }
            let mut l = 1u64;
            while l <= n {
                let q = n / l;
                if q > limit {
                    keys.push(q);
                } else {
                    break;
                }
                l = n / q + 1;
            }
        }
        keys.sort_unstable();
        keys.dedup();
        self.big.reserve(keys.len());

        for &n in &keys {
            let val = self.compute_one(n);
            self.big.insert(n, val);
        }
    }
}

fn nonconcurrent_candidate_count(m: u64, n: u64) -> u64 {
    let two_same_bottom = mul(
        mul(
            mul(
                mul(mul(m % MOD, (m + MOD - 1) % MOD), n % MOD),
                (n + MOD - 1) % MOD,
            ),
            (n + 1) % MOD,
        ),
        INV6,
    );
    let distinct_bottoms = mul(c3_mod(m), sub_mod(c3_mod(n + 2), n % MOD));
    add_mod(two_same_bottom, distinct_bottoms)
}

fn weighted_gcd_sum(m: u64, n: u64, tp: &TotientPrefix) -> u64 {
    let m1 = m - 1;
    let n1 = n - 1;
    let upper = m1.min(n1);
    let mut total = 0u64;

    let mut l = 1u64;
    while l <= upper {
        let qm = m1 / l;
        let qn = n1 / l;
        let r = (m1 / qm).min(n1 / qn).min(upper);

        let (r0, r1, r2) = tp.values(r);
        let (l0, l1, l2) = if l == 1 {
            (0, 0, 0)
        } else {
            tp.values(l - 1)
        };

        let s0 = sub_mod(r0, l0);
        let s1 = sub_mod(r1, l1);
        let s2 = sub_mod(r2, l2);

        let qm_mod = qm % MOD;
        let qn_mod = qn % MOD;

        let a0m = mul(qm_mod, m % MOD);
        let a1m = mul(mul((MOD - qm_mod) % MOD, (qm + 1) % MOD), INV2);
        let a0n = mul(qn_mod, n % MOD);
        let a1n = mul(mul((MOD - qn_mod) % MOD, (qn + 1) % MOD), INV2);

        let c0 = mul(a0m, a0n);
        let c1 = add_mod(mul(a0m, a1n), mul(a1m, a0n));
        let c2 = mul(a1m, a1n);

        total = add_mod(
            total,
            add_mod(add_mod(mul(c0, s0), mul(c1, s1)), mul(c2, s2)),
        );
        l = r + 1;
    }
    total
}

fn concurrent_triple_count(m: u64, n: u64, tp: &TotientPrefix) -> u64 {
    let gcd_part = weighted_gcd_sum(m, n, tp);
    let endpoint = mul(c2_mod(m), c2_mod(n));
    sub_mod(gcd_part, endpoint)
}

fn t_fn(m: u64, n: u64, tp: &TotientPrefix) -> u64 {
    sub_mod(
        nonconcurrent_candidate_count(m, n),
        concurrent_triple_count(m, n, tp),
    )
}

fn main() {
    let mut tp = TotientPrefix::new(SIEVE_LIMIT);
    let m = 1234 * 10u64.pow(8);
    let n = 2345 * 10u64.pow(8);
    tp.precompute_for(&[m - 1, n - 1]);
    println!("{}", t_fn(m, n, &tp));
}
