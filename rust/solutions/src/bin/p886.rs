// Project Euler 886
// Permutation counting with coprimality and parity constraints on numbers up to 34.
// Parallel helper splits + adj-bitmask num_perms + compact prev-keyed memo.

#![allow(unsafe_op_in_unsafe_fn)]

use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};

const NN: usize = 34;
const MOD: i64 = 83_456_729;
const MOD32: i32 = 83_456_729;
const MODU: u64 = 83_456_729;
const NPREV: usize = 10;
const SPLIT: usize = 5;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

struct Ctx {
    cache: *mut AtomicI32,
    prod_val: i32,
    prods: [i32; NN + 1],
    prev_key: [usize; NN + 1],
    adj_mask: [u64; NN + 1],
    max_counts: [i32; NN + 1],
    live: [usize; NN + 1],
    n_live: usize,
    suffix_total: [i32; NN + 2],
    suffix_odds: [i32; NN + 2],
    even_types: [usize; 16],
    n_even: usize,
    l: i32,
    l2: i32,
}

unsafe impl Send for Ctx {}
unsafe impl Sync for Ctx {}

struct Job {
    counts: [i32; NN + 1],
    other: [i32; NN + 1],
    encoded: i32,
    used: i32,
    odds: i32,
    active_l: u64,
    active_r: u64,
}

#[inline(always)]
unsafe fn num_perms(
    ctx: &Ctx,
    counts: &mut [i32; NN + 1],
    encoded: i32,
    remaining: i32,
    prev: usize,
    active: u64,
) -> i32 {
    if remaining <= 1 {
        if remaining == 0 {
            return 1;
        }
        // SAFETY: prev is a live type in 1..=NN
        return (*ctx.adj_mask.get_unchecked(prev) & active).count_ones() as i32;
    }
    num_perms_cached(ctx, counts, encoded, remaining, prev, active)
}

unsafe fn num_perms_cached(
    ctx: &Ctx,
    counts: &mut [i32; NN + 1],
    encoded: i32,
    remaining: i32,
    prev: usize,
    active: u64,
) -> i32 {
    // SAFETY: prev is a live type; encoded is a valid mixed-radix count tuple
    let pk = *ctx.prev_key.get_unchecked(prev);
    let key = encoded as usize * NPREV + pk;
    let slot = ctx.cache.add(key);
    let cached = (*slot).load(Ordering::Relaxed);
    if cached != -1 {
        return cached;
    }
    let mut result = 0i32;
    let mut bits = *ctx.adj_mask.get_unchecked(prev) & active;
    while bits != 0 {
        let t = bits.trailing_zeros() as usize;
        bits &= bits - 1;
        let c = *counts.get_unchecked(t);
        *counts.get_unchecked_mut(t) = c - 1;
        let new_active = if c == 1 { active ^ (1u64 << t) } else { active };
        result += num_perms(
            ctx,
            counts,
            encoded - *ctx.prods.get_unchecked(t),
            remaining - 1,
            t,
            new_active,
        );
        *counts.get_unchecked_mut(t) = c;
    }
    // Each recursive value is in [0, MOD); at most 10 neighbors, so the sum fits in i32.
    result %= MOD32;
    (*slot).store(result, Ordering::Relaxed);
    result
}

unsafe fn leaf_work(
    ctx: &Ctx,
    counts: &mut [i32; NN + 1],
    other: &mut [i32; NN + 1],
    encoded: i32,
    active_l: u64,
    active_r: u64,
) -> i64 {
    let mut acc = 0u64;
    let l = ctx.l;
    // SAFETY: n_even even types stored in even_types[0..n_even]
    for i in 0..ctx.n_even {
        let mid = *ctx.even_types.get_unchecked(i);
        let oc = *other.get_unchecked(mid);
        if oc > 0 {
            *other.get_unchecked_mut(mid) = oc - 1;
            let other_encoded = ctx.prod_val - 1 - *ctx.prods.get_unchecked(mid) - encoded;
            let new_ar = if oc == 1 { active_r ^ (1u64 << mid) } else { active_r };
            let val1 = num_perms(ctx, counts, encoded, l, mid, active_l);
            let val2 = num_perms(ctx, other, other_encoded, l, mid, new_ar);
            acc += val1 as u64 * val2 as u64;
            *other.get_unchecked_mut(mid) = oc;
        }
    }
    (acc % MODU) as i64
}

unsafe fn helper(
    ctx: &Ctx,
    idx: usize,
    counts: &mut [i32; NN + 1],
    other: &mut [i32; NN + 1],
    encoded: i32,
    used: i32,
    odds: i32,
    active_l: u64,
    active_r: u64,
) -> i64 {
    if idx == ctx.n_live {
        if used == ctx.l && odds == ctx.l2 {
            return leaf_work(ctx, counts, other, encoded, active_l, active_r);
        }
        return 0;
    }
    let num = *ctx.live.get_unchecked(idx);
    let mc = *ctx.max_counts.get_unchecked(num);
    let prod = *ctx.prods.get_unchecked(num);
    let is_odd = (num & 1) as i32;
    let suf_t = *ctx.suffix_total.get_unchecked(idx + 1);
    let suf_o = *ctx.suffix_odds.get_unchecked(idx + 1);
    let l = ctx.l;
    let l2 = ctx.l2;
    let mut acc = 0i64;
    for count in 0..=mc {
        let nused = used + count;
        let nods = odds + is_odd * count;
        if nused > l || nused + suf_t < l || nods > l2 || nods + suf_o < l2 {
            continue;
        }
        *counts.get_unchecked_mut(num) = count;
        *other.get_unchecked_mut(num) = mc - count;
        let new_al = if count > 0 { active_l | (1u64 << num) } else { active_l };
        let new_ar = if count == mc { active_r ^ (1u64 << num) } else { active_r };
        acc += helper(
            ctx,
            idx + 1,
            counts,
            other,
            encoded + count * prod,
            nused,
            nods,
            new_al,
            new_ar,
        );
    }
    *counts.get_unchecked_mut(num) = 0;
    *other.get_unchecked_mut(num) = mc;
    acc
}

fn collect_jobs(
    ctx: &Ctx,
    idx: usize,
    counts: &mut [i32; NN + 1],
    other: &mut [i32; NN + 1],
    encoded: i32,
    used: i32,
    odds: i32,
    active_l: u64,
    active_r: u64,
    jobs: &mut Vec<Job>,
) {
    if idx == SPLIT {
        jobs.push(Job {
            counts: *counts,
            other: *other,
            encoded,
            used,
            odds,
            active_l,
            active_r,
        });
        return;
    }
    let num = ctx.live[idx];
    let mc = ctx.max_counts[num];
    let prod = ctx.prods[num];
    let is_odd = (num & 1) as i32;
    let suf_t = ctx.suffix_total[idx + 1];
    let suf_o = ctx.suffix_odds[idx + 1];
    let l = ctx.l;
    let l2 = ctx.l2;
    for count in 0..=mc {
        let nused = used + count;
        let nods = odds + is_odd * count;
        if nused > l || nused + suf_t < l || nods > l2 || nods + suf_o < l2 {
            continue;
        }
        counts[num] = count;
        other[num] = mc - count;
        let new_al = if count > 0 { active_l | (1u64 << num) } else { active_l };
        let new_ar = if count == mc { active_r ^ (1u64 << num) } else { active_r };
        collect_jobs(
            ctx,
            idx + 1,
            counts,
            other,
            encoded + count * prod,
            nused,
            nods,
            new_al,
            new_ar,
            jobs,
        );
    }
    counts[num] = 0;
    other[num] = mc;
}

fn main() {
    let l = (NN - 2) / 2;

    let sieve_limit = NN / 2;
    let mut is_p = [true; NN + 1];
    is_p[0] = false;
    is_p[1] = false;
    for i in 2..=sieve_limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= sieve_limit {
                is_p[j] = false;
                j += i;
            }
        }
    }
    let primes_list: Vec<i32> = (2..=sieve_limit).filter(|&i| is_p[i]).map(|i| i as i32).collect();

    let mut max_counts = [0i32; NN + 1];
    for i in 2..=NN {
        let mut num = 1;
        for &p in &primes_list {
            if i as i32 % p == 0 {
                num *= p;
            }
        }
        max_counts[num as usize] += 1;
    }

    let mut prods = [0i32; NN + 1];
    let mut prod_val: i32 = 1;
    for num in 1..=NN {
        prods[num] = prod_val;
        prod_val *= max_counts[num] + 1;
    }

    let mut live = [0usize; NN + 1];
    let mut n_live = 0;
    let mut prev_key = [0usize; NN + 1];
    let mut even_types = [0usize; 16];
    let mut n_even = 0;
    let mut odd_i = 0;
    let mut active_all = 0u64;
    for t in 1..=NN {
        if max_counts[t] > 0 {
            live[n_live] = t;
            n_live += 1;
            active_all |= 1u64 << t;
            if t % 2 == 0 {
                prev_key[t] = n_even;
                even_types[n_even] = t;
                n_even += 1;
            } else {
                prev_key[t] = odd_i;
                odd_i += 1;
            }
        }
    }

    let mut suffix_total = [0i32; NN + 2];
    let mut suffix_odds = [0i32; NN + 2];
    for i in (0..n_live).rev() {
        let t = live[i];
        suffix_total[i] = suffix_total[i + 1] + max_counts[t];
        suffix_odds[i] = suffix_odds[i + 1] + if t % 2 == 1 { max_counts[t] } else { 0 };
    }

    let mut adj_mask = [0u64; NN + 1];
    for i in 1..=NN {
        if max_counts[i] == 0 {
            continue;
        }
        for j in 1..=NN {
            if max_counts[j] == 0 {
                continue;
            }
            if (i & 1) != (j & 1) && gcd(i as i32, j as i32) == 1 {
                adj_mask[i] |= 1u64 << j;
            }
        }
    }

    let cache_size = prod_val as usize * NPREV;

    unsafe {
        let layout = std::alloc::Layout::array::<AtomicI32>(cache_size).unwrap();
        let cache = std::alloc::alloc(layout) as *mut AtomicI32;
        std::ptr::write_bytes(cache as *mut u8, 0xFF, cache_size * std::mem::size_of::<AtomicI32>());

        let ctx = Ctx {
            cache,
            prod_val,
            prods,
            prev_key,
            adj_mask,
            max_counts,
            live,
            n_live,
            suffix_total,
            suffix_odds,
            even_types,
            n_even,
            l: l as i32,
            l2: (l / 2) as i32,
        };

        let mut counts = [0i32; NN + 1];
        let mut other = max_counts;
        let mut jobs = Vec::with_capacity(2048);
        collect_jobs(&ctx, 0, &mut counts, &mut other, 0, 0, 0, 0, active_all, &mut jobs);

        let ans: i64 = jobs
            .par_iter()
            .map(|job| {
                let mut c = job.counts;
                let mut o = job.other;
                helper(
                    &ctx,
                    SPLIT,
                    &mut c,
                    &mut o,
                    job.encoded,
                    job.used,
                    job.odds,
                    job.active_l,
                    job.active_r,
                )
            })
            .sum();

        let mut ans = ans % MOD;
        let mut factorials = [0i64; NN + 1];
        factorials[0] = 1;
        for i in 1..=NN {
            factorials[i] = factorials[i - 1] * i as i64 % MOD;
        }
        for num in 1..=NN {
            ans = ans * factorials[max_counts[num] as usize] % MOD;
        }

        println!("{}", ans);

        std::alloc::dealloc(cache as *mut u8, layout);
    }
}
