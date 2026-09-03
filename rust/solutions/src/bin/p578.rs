// Project Euler 578 - Integers with Decreasing Prime Powers
//
// Count integers n <= N whose prime exponents are non-increasing.
// Powerful prefix via DFS; squarefree tails via Möbius + min-prime recurrence.
// Fast paths:
// 1. Prefix squarefree counts prefix_sf_k[x][k] precomputed for k < 8 up to sqrt(N).
// 2. Analytic evaluation of DPP for p > N^(1/3) (all 1s) and N^(1/4) < p <= N^(1/3).
// 3. Parallel recursive evaluation via Rayon with split light/heavy branches.

use fxhash::FxHashMap;
use rayon::prelude::*;

const NN: i64 = 10_000_000_000_000; // 10^13
const PAR_THRESH: i64 = 20_000_000;
const K_PRE: usize = 8;
const K_SHIFT: usize = 3;

struct Ctx {
    primes: Vec<u32>,
    prefix_mu: Vec<i32>,
    prefix_sf_k: Vec<i32>, // interleaved: [limit + 1][8]
    pi: Vec<i32>,
}

struct Memos {
    q: FxHashMap<i64, i64>,
    sf: FxHashMap<u64, i64>,
    dpp: FxHashMap<(u64, u32), i64>,
}

impl Memos {
    fn new() -> Self {
        Self {
            q: FxHashMap::with_capacity_and_hasher(512, Default::default()),
            sf: FxHashMap::with_capacity_and_hasher(2048, Default::default()),
            dpp: FxHashMap::with_capacity_and_hasher(1024, Default::default()),
        }
    }

    fn new_large() -> Self {
        Self {
            q: FxHashMap::with_capacity_and_hasher(1024, Default::default()),
            sf: FxHashMap::with_capacity_and_hasher(4096, Default::default()),
            dpp: FxHashMap::with_capacity_and_hasher(2048, Default::default()),
        }
    }
}

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut r = (n as f64).sqrt() as i64;
    let rr = r * r;
    if rr > n {
        r -= 1;
    } else if n - rr > 2 * r {
        r += 1;
    }
    r
}

#[inline(always)]
fn icbrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut r = (n as f64).cbrt() as i64;
    while (r + 1).saturating_mul(r + 1).saturating_mul(r + 1) <= n {
        r += 1;
    }
    while r.saturating_mul(r).saturating_mul(r) > n {
        r -= 1;
    }
    r
}

#[inline(always)]
fn i4thrt(n: i64) -> i64 {
    isqrt(isqrt(n))
}

#[inline(always)]
fn sf_key(x: i64, start_idx: usize) -> u64 {
    ((x as u64) << 20) | start_idx as u64
}

#[inline(always)]
fn dpp_key(limit: i64, start_idx: usize, max_exp: i32) -> (u64, u32) {
    (limit as u64, ((start_idx as u32) << 8) | max_exp as u32)
}

fn sieve_primes_and_mobius(limit: usize) -> Ctx {
    let mut lp = vec![0u32; limit + 1];
    let mut mu = vec![0i8; limit + 1];
    let mut primes = Vec::new();
    mu[1] = 1;

    for i in 2..=limit {
        if lp[i] == 0 {
            lp[i] = i as u32;
            primes.push(i as u32);
            mu[i] = -1;
        }
        for &p in &primes {
            let ip = i * p as usize;
            if ip > limit {
                break;
            }
            lp[ip] = p;
            if p == lp[i] {
                mu[ip] = 0;
                break;
            }
            mu[ip] = -mu[i];
        }
    }

    let mut prefix_mu = vec![0i32; limit + 1];
    let mut pi = vec![0i32; limit + 1];
    let mut s_mu = 0i32;
    let mut s_pi = 0i32;
    for i in 1..=limit {
        s_mu += mu[i] as i32;
        prefix_mu[i] = s_mu;
        if lp[i] == i as u32 && i >= 2 {
            s_pi += 1;
        }
        pi[i] = s_pi;
    }

    let pre_primes = [2, 3, 5, 7, 11, 13, 17];
    let mut prefix_sf_k = vec![0i32; (limit + 1) << K_SHIFT];
    let mut sums = [0i32; K_PRE];
    for i in 1..=limit {
        if mu[i] != 0 {
            sums[0] += 1;
            let mut k = 1;
            while k < K_PRE {
                if i % pre_primes[k - 1] == 0 {
                    break;
                }
                sums[k] += 1;
                k += 1;
            }
        }
        let base = i << K_SHIFT;
        unsafe {
            let dst = prefix_sf_k.as_mut_ptr().add(base) as *mut [i32; K_PRE];
            *dst = sums;
        }
    }

    Ctx {
        primes,
        prefix_mu,
        prefix_sf_k,
        pi,
    }
}

#[inline]
fn squarefree_upto(x: i64, ctx: &Ctx, memos: &mut Memos) -> i64 {
    if x <= 0 {
        return 0;
    }
    let slen = ctx.pi.len();
    if (x as usize) < slen {
        return unsafe { *ctx.prefix_sf_k.get_unchecked((x as usize) << K_SHIFT) } as i64;
    }
    if let Some(&cached) = memos.q.get(&x) {
        return cached;
    }

    let mut r = isqrt(x);
    if r as usize >= slen {
        r = slen as i64 - 1;
    }
    let mut res: i64 = 0;
    let mut i: i64 = 1;
    let prefix_mu = &ctx.prefix_mu;
    while i <= r {
        let t = x / (i * i);
        let mut j = isqrt(x / t);
        if j > r {
            j = r;
        }
        let mu_sum = unsafe {
            *prefix_mu.get_unchecked(j as usize) - *prefix_mu.get_unchecked((i - 1) as usize)
        };
        res += t * mu_sum as i64;
        i = j + 1;
    }
    memos.q.insert(x, res);
    res
}

fn squarefree_min_prime(x: i64, start_idx: usize, ctx: &Ctx, memos: &mut Memos) -> i64 {
    if x <= 0 {
        return 0;
    }
    let pi_len = ctx.pi.len();
    if (x as usize) < pi_len && start_idx < K_PRE {
        return unsafe {
            *ctx.prefix_sf_k
                .get_unchecked(((x as usize) << K_SHIFT) | start_idx)
        } as i64;
    }
    if start_idx == 0 {
        return squarefree_upto(x, ctx, memos);
    }

    let nprimes = ctx.primes.len();
    if start_idx >= nprimes {
        return 1;
    }
    let p0 = unsafe { *ctx.primes.get_unchecked(start_idx) } as i64;
    if p0 > x {
        return 1;
    }

    let s = isqrt(x);
    if (x as usize) < pi_len && p0 > s {
        let pix = unsafe { *ctx.pi.get_unchecked(x as usize) } as i64;
        return 1 + pix - start_idx as i64;
    }
    let key = sf_key(x, start_idx);
    if let Some(&cached) = memos.sf.get(&key) {
        return cached;
    }
    let mut total = squarefree_upto(x, ctx, memos);
    let idx_s = if s > 0 && (s as usize) < pi_len {
        unsafe { *ctx.pi.get_unchecked(s as usize) as usize }
    } else {
        0
    };
    let cap = if (x as usize) < pi_len {
        (unsafe { *ctx.pi.get_unchecked(x as usize) } as usize).min(start_idx)
    } else {
        start_idx
    };
    if cap > idx_s {
        total -= (cap - idx_s) as i64;
    }
    let rec_end = cap.min(idx_s);
    for i in 0..rec_end {
        let p = unsafe { *ctx.primes.get_unchecked(i) } as i64;
        total -= squarefree_min_prime(x / p, i + 1, ctx, memos);
    }
    memos.sf.insert(key, total);
    total
}

fn count_dpp_seq(
    limit: i64,
    start_idx: usize,
    max_exp: i32,
    ctx: &Ctx,
    memos: &mut Memos,
) -> i64 {
    if limit <= 0 {
        return 0;
    }
    if limit == 1 {
        return 1;
    }
    let nprimes = ctx.primes.len();
    if start_idx >= nprimes {
        return 1;
    }
    let p0 = unsafe { *ctx.primes.get_unchecked(start_idx) } as i64;
    if limit < p0 {
        return 1;
    }

    let pi_len = ctx.pi.len();
    let s = isqrt(limit);
    if (limit as usize) < pi_len && p0 > s {
        let pix = unsafe { *ctx.pi.get_unchecked(limit as usize) } as i64;
        return 1 + pix - start_idx as i64;
    }

    if max_exp <= 1 {
        return squarefree_min_prime(limit, start_idx, ctx, memos);
    }
    let key = dpp_key(limit, start_idx, max_exp);
    if let Some(&cached) = memos.dpp.get(&key) {
        return cached;
    }
    let mut res = squarefree_min_prime(limit, start_idx, ctx, memos);
    let end = ctx.primes.partition_point(|&p| (p as i64) <= s).min(nprimes);

    let c = icbrt(limit);
    let cutoff3 = ctx.primes.partition_point(|&p| (p as i64) <= c).min(nprimes);

    let f = i4thrt(limit);
    let cutoff4 = ctx.primes.partition_point(|&p| (p as i64) <= f).min(nprimes);

    let rec_end = end.min(cutoff4);
    for i in start_idx..rec_end {
        let p = unsafe { *ctx.primes.get_unchecked(i) } as i64;
        let mut pe = p * p;
        let mut e = 2i32;
        while e <= max_exp {
            res += count_dpp_seq(limit / pe, i + 1, e, ctx, memos);
            e += 1;
            if pe > limit / p {
                break;
            }
            pe *= p;
        }
    }

    let mid_start = cutoff4.max(start_idx);
    let mid_end = cutoff3.min(end);
    if mid_end > mid_start {
        for i in mid_start..mid_end {
            let p = unsafe { *ctx.primes.get_unchecked(i) } as i64;
            let nlim2 = limit / (p * p);
            let next_p = if i + 1 < nprimes {
                unsafe { *ctx.primes.get_unchecked(i + 1) as i64 }
            } else {
                i64::MAX
            };
            if nlim2 < next_p {
                res += 1;
            } else if (nlim2 as usize) < pi_len {
                res += 1 + (unsafe { *ctx.pi.get_unchecked(nlim2 as usize) } as i64) - (i + 1) as i64;
            } else {
                res += squarefree_min_prime(nlim2, i + 1, ctx, memos);
            }

            let mut pe = p * p * p;
            let mut e = 3;
            while e <= max_exp && pe <= limit {
                res += 1;
                if pe > limit / p {
                    break;
                }
                pe *= p;
                e += 1;
            }
        }
    }

    let tail_start = cutoff3.max(start_idx);
    if end > tail_start {
        res += (end - tail_start) as i64;
    }

    memos.dpp.insert(key, res);
    res
}

fn count_dpp_par(limit: i64, start_idx: usize, max_exp: i32, ctx: &Ctx) -> i64 {
    if limit <= 1 {
        return if limit == 1 { 1 } else { 0 };
    }
    if max_exp <= 1 || limit < PAR_THRESH {
        let mut memos = Memos::new();
        return count_dpp_seq(limit, start_idx, max_exp, ctx, &mut memos);
    }
    let s = isqrt(limit);
    let end = ctx.primes.partition_point(|&p| (p as i64) <= s);
    let c = icbrt(limit);
    let cutoff3 = ctx.primes.partition_point(|&p| (p as i64) <= c);
    let f = i4thrt(limit);
    let cutoff4 = ctx.primes.partition_point(|&p| (p as i64) <= f);

    let mut heavy = Vec::new();
    let mut light = Vec::new();
    let rec_end = end.min(cutoff4);
    for i in start_idx..rec_end {
        let p = unsafe { *ctx.primes.get_unchecked(i) } as i64;
        let mut pe = p * p;
        let mut e = 2i32;
        while e <= max_exp {
            let nlim = limit / pe;
            if nlim >= PAR_THRESH {
                heavy.push((nlim, i + 1, e));
            } else {
                light.push((nlim, i + 1, e));
            }
            e += 1;
            if pe > limit / p {
                break;
            }
            pe *= p;
        }
    }

    let (sf_val, (light_sum, heavy_sum)) = rayon::join(
        || {
            let mut memos = Memos::new_large();
            squarefree_min_prime(limit, start_idx, ctx, &mut memos)
        },
        || {
            rayon::join(
                || {
                    if light.len() > 16 {
                        light
                            .par_iter()
                            .map(|&(nlim, idx, e)| {
                                let mut memos = Memos::new();
                                count_dpp_seq(nlim, idx, e, ctx, &mut memos)
                            })
                            .sum::<i64>()
                    } else {
                        let mut memos = Memos::new();
                        let mut sum = 0;
                        for &(nlim, idx, e) in &light {
                            sum += count_dpp_seq(nlim, idx, e, ctx, &mut memos);
                        }
                        sum
                    }
                },
                || {
                    heavy
                        .into_par_iter()
                        .map(|(nlim, idx, e)| count_dpp_par(nlim, idx, e, ctx))
                        .sum::<i64>()
                },
            )
        },
    );

    let mid_start = cutoff4.max(start_idx);
    let mid_end = cutoff3.min(end);
    let mut mid_sum = 0i64;
    if mid_end > mid_start {
        let nprimes = ctx.primes.len();
        let pi_len = ctx.pi.len();
        for i in mid_start..mid_end {
            let p = unsafe { *ctx.primes.get_unchecked(i) } as i64;
            let nlim2 = limit / (p * p);
            let next_p = if i + 1 < nprimes {
                unsafe { *ctx.primes.get_unchecked(i + 1) as i64 }
            } else {
                i64::MAX
            };
            if nlim2 < next_p {
                mid_sum += 1;
            } else if (nlim2 as usize) < pi_len {
                mid_sum += 1 + (unsafe { *ctx.pi.get_unchecked(nlim2 as usize) } as i64) - (i + 1) as i64;
            } else {
                let mut memos = Memos::new();
                mid_sum += squarefree_min_prime(nlim2, i + 1, ctx, &mut memos);
            }
            let mut pe = p * p * p;
            let mut e = 3;
            while e <= max_exp && pe <= limit {
                mid_sum += 1;
                if pe > limit / p {
                    break;
                }
                pe *= p;
                e += 1;
            }
        }
    }
    let tail_start = cutoff3.max(start_idx);
    let tail_ones = if end > tail_start {
        (end - tail_start) as i64
    } else {
        0
    };

    sf_val + light_sum + heavy_sum + mid_sum + tail_ones
}

fn main() {
    let sieve_limit = isqrt(NN) as usize + 1000;
    let ctx = sieve_primes_and_mobius(sieve_limit);
    let max_e = (NN as f64).log2() as i32 + 1;
    let ans = count_dpp_par(NN, 0, max_e, &ctx);
    println!("{}", ans);
}

