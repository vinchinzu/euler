// Project Euler 311 - Biclinic Integral Quadrilaterals
// 1-mod-4 prime products; Dirichlet last-prime floors + rayon range splits.

use rayon::prelude::*;

const N_VAL: i64 = 10_000_000_000;
const SIEVE1: usize = 100_000_000;
const SIEVE3: usize = 2775;
const PAR_T: i64 = 250_000;
const SMALL_CUM: usize = 16_384;

struct Ctx {
    primes: Vec<i32>,
    prods: Vec<i32>,
    small_cum: Vec<i32>,
    l: i64,
    l2: i64,
}

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    (n as u64).isqrt() as i64
}

#[inline(always)]
fn icbrt(n: i64) -> i64 {
    let mut x = (n as f64).cbrt() as i64;
    while (x + 1).saturating_mul(x + 1).saturating_mul(x + 1) <= n {
        x += 1;
    }
    while x > 0 && x * x * x > n {
        x -= 1;
    }
    x
}

#[inline(always)]
fn ways(b: i32, a0_odd: bool) -> i64 {
    let r2 = (b + i32::from(a0_odd)) >> 1;
    if r2 >= 3 {
        let r = r2 as i64;
        r * (r - 1) * (r - 2) / 6
    } else {
        0
    }
}

#[inline(always)]
fn count_le(ctx: &Ctx, x: i64) -> i64 {
    if x < 1 {
        0
    } else if (x as usize) < ctx.small_cum.len() {
        unsafe { *ctx.small_cum.get_unchecked(x as usize) as i64 }
    } else {
        ctx.prods.partition_point(|&p| (p as i64) <= x) as i64
    }
}

#[inline(always)]
fn upper_idx(primes: &[i32], start: usize, lim: i64) -> usize {
    if start >= primes.len() || lim < 5 {
        return start;
    }
    let lim32 = if lim >= i32::MAX as i64 {
        i32::MAX
    } else {
        lim as i32
    };
    start + primes[start..].partition_point(|&p| p <= lim32)
}

/// Sum count_le(t/p) over primes[start..end), all p <= t.
fn sum_floor(ctx: &Ctx, start: usize, end: usize, t: i64) -> i64 {
    let primes = ctx.primes.as_slice();
    let mut i = start;
    let mut acc = 0i64;
    while i < end {
        let p = unsafe { *primes.get_unchecked(i) } as i64;
        let q = t / p;
        let last = t / q;
        let lim = if last >= i32::MAX as i64 {
            i32::MAX
        } else {
            last as i32
        };
        let j = i + unsafe { primes.get_unchecked(i..end) }.partition_point(|&pp| pp <= lim);
        acc += count_le(ctx, q) * (j - i) as i64;
        i = j;
    }
    acc
}

fn extend_prime(ctx: &Ctx, idx: usize, n: i64, a0_odd: bool, b: i32, par: bool) -> i64 {
    let p = unsafe { *ctx.primes.get_unchecked(idx) } as i64;
    let mut ans = 0i64;
    let mut nn = n;
    let mut e = 1i32;
    let nxt = idx + 1;
    while nn <= ctx.l / p {
        nn *= p;
        ans += helper(ctx, nxt, nn, a0_odd, b * (e + 1), par);
        e += 1;
    }
    ans
}

fn process_primes(ctx: &Ctx, lo: usize, hi: usize, n: i64, a0_odd: bool, b: i32, par: bool) -> i64 {
    if lo >= hi {
        return 0;
    }
    let t = ctx.l / n;
    if par && hi - lo > 1 && t >= PAR_T {
        let mid = (lo + hi) / 2;
        let (x, y) = rayon::join(
            || process_primes(ctx, lo, mid, n, a0_odd, b, true),
            || process_primes(ctx, mid, hi, n, a0_odd, b, true),
        );
        return x + y;
    }
    let mut ans = 0i64;
    for idx in lo..hi {
        ans += extend_prime(ctx, idx, n, a0_odd, b, par && t >= PAR_T);
    }
    ans
}

fn helper(ctx: &Ctx, min_idx: usize, n: i64, a0_odd: bool, b: i32, par: bool) -> i64 {
    let mut ans = 0i64;
    let w = ways(b, a0_odd);
    if w != 0 {
        let q = ctx.l / n;
        if q <= ctx.l2 {
            ans += w * count_le(ctx, q);
        }
    }
    if n > ctx.l {
        return ans;
    }

    let t = ctx.l / n;
    let ilimit = if b == 1 {
        icbrt(t)
    } else if b == 2 {
        isqrt(t)
    } else {
        t
    };
    if ilimit < 5 {
        return ans;
    }

    let sqrt_t = isqrt(t);
    let rec_lim = if ilimit < sqrt_t { ilimit } else { sqrt_t };
    let rec_end = upper_idx(&ctx.primes, min_idx, rec_lim);
    let last_end = if b >= 3 {
        upper_idx(&ctx.primes, rec_end, ilimit)
    } else {
        rec_end
    };

    let w2 = ways(b.saturating_mul(2), a0_odd);
    if w2 != 0 && last_end > rec_end {
        ans += w2 * sum_floor(ctx, rec_end, last_end, t);
    }

    ans += process_primes(ctx, min_idx, rec_end, n, a0_odd, b, par);
    ans
}

fn sieve_small(limit: usize) -> Vec<bool> {
    let mut isp = vec![true; limit + 1];
    isp[0] = false;
    if limit >= 1 {
        isp[1] = false;
    }
    let mut i = 2usize;
    while i * i <= limit {
        if isp[i] {
            let mut j = i * i;
            while j <= limit {
                isp[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    isp
}

fn segment_1mod4(lo: usize, hi: usize, odd_primes: &[u32]) -> Vec<i32> {
    let len = hi - lo + 1;
    let nwords = (len + 63) / 64;
    let mut bits = vec![u64::MAX; nwords];
    // numbers < 2 in a segment never happen (lo >= 3)
    for &p in odd_primes {
        let p = p as usize;
        let mut start = ((lo + p - 1) / p) * p;
        let pp = p * p;
        if start < pp {
            start = pp;
        }
        if start > hi {
            continue;
        }
        let mut j = start - lo;
        while j < len {
            unsafe {
                *bits.get_unchecked_mut(j >> 6) &= !(1u64 << (j & 63));
            }
            j += p;
        }
    }
    let mut out = Vec::new();
    let mut n = lo;
    let r = n % 4;
    if r == 0 {
        n += 1;
    } else if r == 2 {
        n += 3;
    } else if r == 3 {
        n += 2;
    }
    while n <= hi {
        if n >= 5 {
            let j = n - lo;
            if unsafe { (*bits.get_unchecked(j >> 6) >> (j & 63)) & 1 } == 1 {
                out.push(n as i32);
            }
        }
        n += 4;
    }
    out
}

fn sieve_primes(limit: usize) -> (Vec<i32>, Vec<i32>) {
    let sqrt = isqrt(limit as i64) as usize + 1;
    let small = sieve_small(sqrt);

    let mut p3 = Vec::with_capacity(256);
    let mut i = 3usize;
    while i <= SIEVE3 {
        if i <= sqrt && small[i] {
            p3.push(i as i32);
        }
        i += 4;
    }

    let odd_primes: Vec<u32> = (3..=sqrt)
        .step_by(2)
        .filter(|&p| small[p])
        .map(|p| p as u32)
        .collect();

    let mut p1 = Vec::with_capacity(3_000_000);
    let mut i = 5usize;
    while i <= sqrt {
        if small[i] {
            p1.push(i as i32);
        }
        i += 4;
    }

    let seg = 1 << 20;
    let mut lo = sqrt + 1;
    if lo % 2 == 0 {
        lo += 1;
    }
    let mut bounds = Vec::new();
    while lo <= limit {
        let hi = (lo + seg - 1).min(limit);
        bounds.push((lo, hi));
        lo = hi + 1;
        if lo % 2 == 0 {
            lo += 1;
        }
    }

    let parts: Vec<Vec<i32>> = bounds
        .into_par_iter()
        .map(|(lo, hi)| segment_1mod4(lo, hi, &odd_primes))
        .collect();
    for part in parts {
        p1.extend(part);
    }
    (p1, p3)
}

fn collect_3mod4_products(primes3: &[i32], l2: i64) -> Vec<i32> {
    let mut prods = Vec::with_capacity(512);
    fn rec(min_idx: usize, n: i64, primes3: &[i32], l2: i64, prods: &mut Vec<i32>) {
        prods.push(n as i32);
        for idx in min_idx..primes3.len() {
            let p = primes3[idx] as i64;
            let p2 = p * p;
            if n > l2 / p2 {
                return;
            }
            let mut new_n = n;
            while new_n <= l2 / p2 {
                new_n *= p2;
                rec(idx + 1, new_n, primes3, l2, prods);
            }
        }
    }
    rec(0, 1, primes3, l2, &mut prods);
    prods.sort_unstable();
    prods
}

fn main() {
    let l = N_VAL / 4;
    let l2 = l / (5 * 5 * 13);

    let (primes1, primes3) = sieve_primes(SIEVE1);
    let prods = collect_3mod4_products(&primes3, l2);

    let mut small_cum = vec![0i32; SMALL_CUM];
    let mut k = 0usize;
    for i in 0..SMALL_CUM {
        while k < prods.len() && (prods[k] as usize) <= i {
            k += 1;
        }
        small_cum[i] = k as i32;
    }

    let ctx = Ctx {
        primes: primes1,
        prods,
        small_cum,
        l,
        l2,
    };

    // Powers of 2 are independent; a0 = 0 dominates and splits internally.
    let ans: i64 = (0u32..32)
        .into_par_iter()
        .map(|a0| {
            let prod = 1i64 << a0;
            if prod > l {
                0
            } else {
                helper(&ctx, 0, prod, a0 % 2 == 1, 1, true)
            }
        })
        .sum();

    println!("{}", ans);
}
