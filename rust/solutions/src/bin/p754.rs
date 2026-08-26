// Project Euler 754 - Product of Gauss Factorials
// Π_{i=1}^N g(i) = Π_g ( g^{tr(⌊N/g⌋)} Π_{i=1}^{⌊N/g⌋} i! )^{μ(g)}
//
// - Segmented μ (primes ≤ √N only; no N-length SPF / prefix arrays)
// - Sparse prod_fact[q] for Dirichlet q, 32-thread + 8-wide prefix
// - Phase-1 pow_mod over g ≤ N/√N in rayon
// - Phase-2/3: 32 equal g-ranges, fused μ sieve + block products

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const PHI: u64 = MOD - 1;
const N: usize = 100_000_000;
const SEG: usize = 1 << 16;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    a * b % MOD
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul(r, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn tr(n: u64) -> u64 {
    n.wrapping_mul(n + 1) / 2
}

/// Linear sieve μ and odd+even primes up to `limit` (√N).
fn sieve_small(limit: usize) -> (Vec<i8>, Vec<u32>) {
    let mut mu = vec![0i8; limit + 1];
    let mut vis = vec![0u8; limit + 1];
    let mut primes = Vec::with_capacity(limit / 5);
    mu[1] = 1;
    for i in 2..=limit {
        if unsafe { *vis.get_unchecked(i) } == 0 {
            primes.push(i as u32);
            unsafe {
                *mu.get_unchecked_mut(i) = -1;
            }
        }
        let mu_i = unsafe { *mu.get_unchecked(i) };
        for &p in &primes {
            let p = p as usize;
            if p > limit / i {
                break;
            }
            let ip = i * p;
            unsafe {
                *vis.get_unchecked_mut(ip) = 1;
            }
            if i % p == 0 {
                break;
            }
            unsafe {
                *mu.get_unchecked_mut(ip) = -mu_i;
            }
        }
    }
    (mu, primes)
}

struct ProdFact {
    l: usize,
    small: Vec<u64>,
    large: Vec<u64>,
}

impl ProdFact {
    #[inline(always)]
    fn get(&self, q: usize) -> u64 {
        if q <= self.l {
            unsafe { *self.small.get_unchecked(q) }
        } else {
            unsafe { *self.large.get_unchecked(N / q) }
        }
    }
}

/// 8-wide step of relative factorial prefixes.
/// `rel` = (i-1)! / (lo-1)!, `pf` = Π_{k=lo}^{i-1} (k! / (lo-1)!).
#[inline(always)]
fn step8(rel: u64, pf: u64, i: u64) -> (u64, u64) {
    let r0 = i;
    let r1 = mul(r0, i + 1);
    let r2 = mul(r1, i + 2);
    let r3 = mul(r2, i + 3);
    let r4 = mul(r3, i + 4);
    let r5 = mul(r4, i + 5);
    let r6 = mul(r5, i + 6);
    let r7 = mul(r6, i + 7);
    let p01 = mul(r0, r1);
    let p23 = mul(r2, r3);
    let p45 = mul(r4, r5);
    let p67 = mul(r6, r7);
    let prod_r = mul(mul(p01, p23), mul(p45, p67));
    let rel2 = mul(rel, rel);
    let rel4 = mul(rel2, rel2);
    let rel8 = mul(rel4, rel4);
    (mul(rel, r7), mul(pf, mul(rel8, prod_r)))
}

fn needed_qs(l: usize) -> Vec<usize> {
    let mut v = Vec::with_capacity(4 * l);
    let mut g = 1usize;
    while g <= N {
        let q = N / g;
        v.push(q);
        g = N / q + 1;
    }
    v.sort_unstable();
    v.dedup();
    v
}

/// prod_fact[q] = Π_{i=1}^q i! at every Dirichlet q = ⌊N/g⌋.
fn compute_prod_fact(l: usize) -> ProdFact {
    let needed = needed_qs(l);
    let nq = needed.len();
    let nthreads = rayon::current_num_threads().max(1);
    let chunk = (N + nthreads - 1) / nthreads;

    let parts: Vec<(u64, u64, Vec<(usize, u64)>)> = (0..nthreads)
        .into_par_iter()
        .map(|t| {
            let lo = t * chunk + 1;
            let hi = ((t + 1) * chunk).min(N);
            if lo > hi {
                return (1, 1, Vec::new());
            }
            let s = needed.partition_point(|&x| x < lo);
            let e = needed.partition_point(|&x| x <= hi);
            let mut rel = 1u64;
            let mut local_pf = 1u64;
            let mut saves = Vec::with_capacity(e.saturating_sub(s));
            let mut i = lo;
            let mut qi = s;
            while i <= hi {
                let next_save = if qi < e {
                    unsafe { *needed.get_unchecked(qi) }
                } else {
                    hi + 1
                };
                while i + 7 < next_save {
                    let (nr, np) = step8(rel, local_pf, i as u64);
                    rel = nr;
                    local_pf = np;
                    i += 8;
                }
                let stop = next_save.min(hi);
                while i <= stop {
                    rel = mul(rel, i as u64);
                    local_pf = mul(local_pf, rel);
                    if qi < e && i == unsafe { *needed.get_unchecked(qi) } {
                        saves.push((qi, local_pf));
                        qi += 1;
                    }
                    i += 1;
                }
            }
            (rel, local_pf, saves)
        })
        .collect();

    let mut values = vec![0u64; nq];
    let mut fact_scale = 1u64;
    let mut pf_scale = 1u64;
    for t in 0..nthreads {
        let lo = t * chunk + 1;
        let hi = ((t + 1) * chunk).min(N);
        if lo > hi {
            continue;
        }
        for &(qi, local) in &parts[t].2 {
            let q = unsafe { *needed.get_unchecked(qi) };
            let e = (q - lo + 1) as u64;
            unsafe {
                *values.get_unchecked_mut(qi) = mul(mul(pf_scale, pow_mod(fact_scale, e)), local);
            }
        }
        let len = (hi - lo + 1) as u64;
        pf_scale = mul(mul(pf_scale, pow_mod(fact_scale, len)), parts[t].1);
        fact_scale = mul(fact_scale, parts[t].0);
    }

    let mut small = vec![0u64; l + 1];
    let mut large = vec![0u64; l + 1];
    for (qi, &q) in needed.iter().enumerate() {
        let v = values[qi];
        if q <= l {
            small[q] = v;
        } else {
            large[N / q] = v;
        }
    }
    ProdFact { l, small, large }
}

#[inline(always)]
fn absorb(m: i8, x: u64, pos: &mut u64, neg: &mut u64, sum: &mut i32) {
    if m == 1 {
        *pos = mul(*pos, x);
        *sum += 1;
    } else if m == -1 {
        *neg = mul(*neg, x);
        *sum -= 1;
    }
}

/// μ on [start, end] via primes ≤ √N, then Dirichlet g-products + prod_fact powers.
fn process_segment(
    start: usize,
    end: usize,
    primes: &[u32],
    pf: &ProdFact,
    mu: &mut [i8],
    rest: &mut [u32],
) -> (u64, u64) {
    let len = end - start + 1;
    mu[..len].fill(1);
    for i in 0..len {
        unsafe {
            *rest.get_unchecked_mut(i) = (start + i) as u32;
        }
    }

    for &p32 in primes {
        let p = p32 as usize;
        if p > end {
            break;
        }
        let rem = start % p;
        let mut m = if rem == 0 { start } else { start + p - rem };
        while m <= end {
            let idx = m - start;
            unsafe {
                let mv = *mu.get_unchecked(idx);
                if mv != 0 {
                    let r = rest.get_unchecked_mut(idx);
                    *r /= p32;
                    if *r % p32 == 0 {
                        *mu.get_unchecked_mut(idx) = 0;
                        while *r % p32 == 0 {
                            *r /= p32;
                        }
                    } else {
                        *mu.get_unchecked_mut(idx) = -mv;
                    }
                }
            }
            m += p;
        }
    }
    for i in 0..len {
        unsafe {
            if *mu.get_unchecked(i) != 0 && *rest.get_unchecked(i) > 1 {
                *mu.get_unchecked_mut(i) = -*mu.get_unchecked(i);
            }
        }
    }

    let mut pos = 1u64;
    let mut neg = 1u64;
    let mut g = start;
    while g <= end {
        let q = N / g;
        let g_hi = (N / q).min(end);
        let mut sub_pos = 1u64;
        let mut sub_neg = 1u64;
        let mut sum_mu = 0i32;
        let mut x = g;
        while x + 7 <= g_hi {
            unsafe {
                absorb(
                    *mu.get_unchecked(x - start),
                    x as u64,
                    &mut sub_pos,
                    &mut sub_neg,
                    &mut sum_mu,
                );
                absorb(
                    *mu.get_unchecked(x + 1 - start),
                    (x + 1) as u64,
                    &mut sub_pos,
                    &mut sub_neg,
                    &mut sum_mu,
                );
                absorb(
                    *mu.get_unchecked(x + 2 - start),
                    (x + 2) as u64,
                    &mut sub_pos,
                    &mut sub_neg,
                    &mut sum_mu,
                );
                absorb(
                    *mu.get_unchecked(x + 3 - start),
                    (x + 3) as u64,
                    &mut sub_pos,
                    &mut sub_neg,
                    &mut sum_mu,
                );
                absorb(
                    *mu.get_unchecked(x + 4 - start),
                    (x + 4) as u64,
                    &mut sub_pos,
                    &mut sub_neg,
                    &mut sum_mu,
                );
                absorb(
                    *mu.get_unchecked(x + 5 - start),
                    (x + 5) as u64,
                    &mut sub_pos,
                    &mut sub_neg,
                    &mut sum_mu,
                );
                absorb(
                    *mu.get_unchecked(x + 6 - start),
                    (x + 6) as u64,
                    &mut sub_pos,
                    &mut sub_neg,
                    &mut sum_mu,
                );
                absorb(
                    *mu.get_unchecked(x + 7 - start),
                    (x + 7) as u64,
                    &mut sub_pos,
                    &mut sub_neg,
                    &mut sum_mu,
                );
            }
            x += 8;
        }
        while x <= g_hi {
            unsafe {
                absorb(
                    *mu.get_unchecked(x - start),
                    x as u64,
                    &mut sub_pos,
                    &mut sub_neg,
                    &mut sum_mu,
                );
            }
            x += 1;
        }
        let exp = tr(q as u64) % PHI;
        pos = mul(pos, pow_mod(sub_pos, exp));
        neg = mul(neg, pow_mod(sub_neg, exp));
        if sum_mu > 0 {
            pos = mul(pos, pow_mod(pf.get(q), sum_mu as u64));
        } else if sum_mu < 0 {
            neg = mul(neg, pow_mod(pf.get(q), (-sum_mu) as u64));
        }
        g = g_hi + 1;
    }
    (pos, neg)
}

fn process_range(lo: usize, hi: usize, primes: &[u32], pf: &ProdFact) -> (u64, u64) {
    if lo > hi {
        return (1, 1);
    }
    let mut mu_buf = vec![0i8; SEG];
    let mut rest_buf = vec![0u32; SEG];
    let mut pos = 1u64;
    let mut neg = 1u64;
    let mut start = lo;
    while start <= hi {
        let end = (start + SEG - 1).min(hi);
        let (p, n) = process_segment(start, end, primes, pf, &mut mu_buf, &mut rest_buf);
        pos = mul(pos, p);
        neg = mul(neg, n);
        start = end + 1;
    }
    (pos, neg)
}

fn main() {
    let l = (N as u64).isqrt() as usize;
    let g_limit = N / l;

    let ((mu_small, primes), pf) = rayon::join(|| sieve_small(l.max(g_limit)), || compute_prod_fact(l));

    let p1 = (1..g_limit + 1)
        .into_par_iter()
        .with_min_len(64)
        .map(|g| {
            let m = unsafe { *mu_small.get_unchecked(g) };
            if m == 0 {
                return (1u64, 1u64);
            }
            let q = N / g;
            let v = mul(pow_mod(g as u64, tr(q as u64) % PHI), pf.get(q));
            if m == 1 { (v, 1) } else { (1, v) }
        })
        .reduce(|| (1, 1), |a, b| (mul(a.0, b.0), mul(a.1, b.1)));

    let lo0 = g_limit + 1;
    let nthreads = rayon::current_num_threads().max(1);
    let total = N - g_limit;
    let chunk = (total + nthreads - 1) / nthreads;
    let p2 = (0..nthreads)
        .into_par_iter()
        .map(|t| {
            let lo = lo0 + t * chunk;
            let hi = (lo0 + (t + 1) * chunk - 1).min(N);
            process_range(lo, hi, &primes, &pf)
        })
        .reduce(|| (1, 1), |a, b| (mul(a.0, b.0), mul(a.1, b.1)));

    let res_pos = mul(p1.0, p2.0);
    let res_neg = mul(p1.1, p2.1);
    let ans = mul(res_pos, pow_mod(res_neg, PHI - 1));
    println!("{}", ans);
}
