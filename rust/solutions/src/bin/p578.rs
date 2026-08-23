// Project Euler 578 - Integers with Decreasing Prime Powers
//
// Count integers n <= N whose prime exponents are non-increasing.
// Powerful prefix via DFS; squarefree tails via Möbius + min-prime recurrence.
// Fast paths: prefix Q(x)/π(x), packed FxHash memos, rayon over heavy branches.

use fxhash::FxHashMap;
use rayon::prelude::*;

const NN: i64 = 10_000_000_000_000; // 10^13
const PAR_THRESH: i64 = 10_000_000;

struct Ctx {
    primes: Vec<u32>,
    prefix_mu: Vec<i32>,
    prefix_sf: Vec<i32>,
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
            q: FxHashMap::with_capacity_and_hasher(2_048, Default::default()),
            sf: FxHashMap::with_capacity_and_hasher(16_384, Default::default()),
            dpp: FxHashMap::with_capacity_and_hasher(4_096, Default::default()),
        }
    }

    fn new_large() -> Self {
        Self {
            q: FxHashMap::with_capacity_and_hasher(1 << 15, Default::default()),
            sf: FxHashMap::with_capacity_and_hasher(1 << 18, Default::default()),
            dpp: FxHashMap::with_capacity_and_hasher(1 << 16, Default::default()),
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
    let mut prefix_sf = vec![0i32; limit + 1];
    let mut pi = vec![0i32; limit + 1];
    let mut s_mu = 0i32;
    let mut s_sf = 0i32;
    let mut s_pi = 0i32;
    for i in 1..=limit {
        s_mu += mu[i] as i32;
        prefix_mu[i] = s_mu;
        if mu[i] != 0 {
            s_sf += 1;
        }
        prefix_sf[i] = s_sf;
        if lp[i] == i as u32 && i >= 2 {
            s_pi += 1;
        }
        pi[i] = s_pi;
    }

    Ctx {
        primes,
        prefix_mu,
        prefix_sf,
        pi,
    }
}

#[inline]
fn squarefree_upto(x: i64, ctx: &Ctx, memos: &mut Memos) -> i64 {
    if x <= 0 {
        return 0;
    }
    let slen = ctx.prefix_sf.len();
    if (x as usize) < slen {
        // SAFETY: x >= 1 and x < prefix_sf.len()
        return unsafe { *ctx.prefix_sf.get_unchecked(x as usize) } as i64;
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
        // SAFETY: 1 <= i <= j <= r < prefix_mu.len()
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
    if x == 1 || start_idx == 0 {
        return squarefree_upto(x, ctx, memos);
    }
    let nprimes = ctx.primes.len();
    if start_idx >= nprimes {
        return 1;
    }
    // SAFETY: start_idx < primes.len()
    let p0 = unsafe { *ctx.primes.get_unchecked(start_idx) } as i64;
    if p0 > x {
        return 1;
    }

    let pi_len = ctx.pi.len();
    let s = isqrt(x);
    if (x as usize) < pi_len && p0 > s {
        // 1 and the primes in [p0, x]
        // SAFETY: x < pi.len()
        let pix = unsafe { *ctx.pi.get_unchecked(x as usize) } as i64;
        return 1 + pix - start_idx as i64;
    }

    let key = sf_key(x, start_idx);
    if let Some(&cached) = memos.sf.get(&key) {
        return cached;
    }

    let mut total = squarefree_upto(x, ctx, memos);

    // Recurse only for p_i <= sqrt(x); each p in (sqrt(x), min(x, p_{start}))]
    // contributes S(x/p, i+1) = 1.
    let idx_s = if s > 0 && (s as usize) < pi_len {
        // SAFETY: s < pi.len()
        unsafe { *ctx.pi.get_unchecked(s as usize) as usize }
    } else {
        0
    };
    let cap = if (x as usize) < pi_len {
        // SAFETY: x < pi.len()
        (unsafe { *ctx.pi.get_unchecked(x as usize) } as usize).min(start_idx)
    } else {
        start_idx
    };
    if cap > idx_s {
        total -= (cap - idx_s) as i64;
    }
    let rec_end = cap.min(idx_s);
    for i in 0..rec_end {
        // SAFETY: i < idx_s <= π(s) <= nprimes, and primes[i] <= s so x/p >= s >= 1
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
    if max_exp <= 1 {
        return squarefree_min_prime(limit, start_idx, ctx, memos);
    }

    let key = dpp_key(limit, start_idx, max_exp);
    if let Some(&cached) = memos.dpp.get(&key) {
        return cached;
    }

    let mut res = squarefree_min_prime(limit, start_idx, ctx, memos);
    let s = isqrt(limit);
    let nprimes = ctx.primes.len();
    let end = ctx.primes.partition_point(|&p| (p as i64) <= s);
    let end = end.min(nprimes);

    for i in start_idx..end {
        // SAFETY: i < end <= primes.len()
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

    let mut heavy: Vec<(i64, usize, i32)> = Vec::new();
    let mut has_light = false;
    for i in start_idx..end {
        // SAFETY: i < end <= primes.len()
        let p = unsafe { *ctx.primes.get_unchecked(i) } as i64;
        let mut pe = p * p;
        let mut e = 2i32;
        while e <= max_exp {
            let nlim = limit / pe;
            if nlim >= PAR_THRESH {
                heavy.push((nlim, i + 1, e));
            } else {
                has_light = true;
            }
            e += 1;
            if pe > limit / p {
                break;
            }
            pe *= p;
        }
    }

    if heavy.is_empty() {
        let mut memos = Memos::new_large();
        return count_dpp_seq(limit, start_idx, max_exp, ctx, &mut memos);
    }

    let (light_and_sf, heavy_sum) = rayon::join(
        || {
            let mut memos = Memos::new_large();
            let mut sum = squarefree_min_prime(limit, start_idx, ctx, &mut memos);
            if has_light {
                for i in start_idx..end {
                    let p = unsafe { *ctx.primes.get_unchecked(i) } as i64;
                    let mut pe = p * p;
                    let mut e = 2i32;
                    while e <= max_exp {
                        let nlim = limit / pe;
                        if nlim < PAR_THRESH {
                            sum += count_dpp_seq(nlim, i + 1, e, ctx, &mut memos);
                        }
                        e += 1;
                        if pe > limit / p {
                            break;
                        }
                        pe *= p;
                    }
                }
            }
            sum
        },
        || {
            heavy
                .into_par_iter()
                .map(|(nlim, idx, e)| count_dpp_par(nlim, idx, e, ctx))
                .sum::<i64>()
        },
    );

    light_and_sf + heavy_sum
}

fn main() {
    let sieve_limit = isqrt(NN) as usize + 1000;
    let ctx = sieve_primes_and_mobius(sieve_limit);
    let max_e = (NN as f64).log2() as i32 + 1;
    let ans = count_dpp_par(NN, 0, max_e, &ctx);
    println!("{}", ans);
}
