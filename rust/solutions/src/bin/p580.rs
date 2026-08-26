// Project Euler 580 - Squarefree Hilbert Numbers
//
// Count Hilbert squarefree numbers below N=10^16.
// A Hilbert number is any positive integer of form 4k+1 (k >= 0).
// Hilbert-squarefree: not divisible by the square of any Hilbert number > 1.
//
// Inclusion-exclusion:
//   answer = Sum_{i odd, i^2 <= N} hilbert_mu(i) * floor((N/i^2 + 3) / 4)
//
// hilbert_mu(i) from the odd prime factorization:
//   n1     = # of 4k+1 primes (all exponent 1; any square => 0)
//   n3_sf  = # of 4k+3 primes with exponent 1
//   n3_sq  = # of 4k+3 primes with exponent 2 (at most one; exp >= 3 => 0)
//   n3_sq == 0: (-1)^{n1+n3_sf} * (1 - n3_sf)
//   n3_sq == 1: (-1)^{n1+n3_sf+1}
//
// Generate those i by multiplicative DFS over odd primes (increasing),
// batching the last prime via floor(N/(m^2 p^2)) groups and π_{4,±1} prefixes.

use rayon::prelude::*;

const N: u64 = 10_000_000_000_000_000;
const PAR_M: u64 = 20_000;
const PAR_LAST: usize = 4096;
const SEG: usize = 1 << 20;

#[inline(always)]
fn isqrt(n: u64) -> u64 {
    n.isqrt()
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

/// Odd primes in [lo, hi], lo odd. `small` = odd primes <= sqrt(hi).
fn segment_odd_primes(lo: usize, hi: usize, small: &[u32]) -> Vec<u32> {
    if lo > hi {
        return Vec::new();
    }
    let len = (hi - lo) / 2 + 1;
    let nwords = (len + 63) / 64;
    let mut bits = vec![u64::MAX; nwords];
    if lo == 1 {
        bits[0] &= !1u64;
    }
    for &p32 in small {
        let p = p32 as usize;
        let pp = p * p;
        if pp > hi {
            break;
        }
        let mut start = lo.max(pp);
        let rem = start % p;
        if rem != 0 {
            start += p - rem;
        }
        if (start & 1) == 0 {
            start += p;
        }
        if start > hi {
            continue;
        }
        let mut idx = (start - lo) >> 1;
        while idx < len {
            // SAFETY: idx < len, nwords = ceil(len/64)
            unsafe {
                *bits.get_unchecked_mut(idx >> 6) &= !(1u64 << (idx & 63));
            }
            idx += p;
        }
    }
    let mut out = Vec::new();
    for idx in 0..len {
        // SAFETY: idx < len
        let bit = unsafe { (*bits.get_unchecked(idx >> 6) >> (idx & 63)) & 1 };
        if bit == 1 {
            let n = lo + (idx << 1);
            if n >= 3 {
                out.push(n as u32);
            }
        }
    }
    out
}

fn sieve_odd_primes(limit: usize) -> Vec<u32> {
    let sq = isqrt(limit as u64) as usize;
    let small = sieve_small(sq);
    let small_odd: Vec<u32> = (3..=sq)
        .step_by(2)
        .filter(|&p| small[p])
        .map(|p| p as u32)
        .collect();

    let mut primes = small_odd.clone();
    primes.reserve(limit / 10);

    let mut lo = sq + 1;
    if (lo & 1) == 0 {
        lo += 1;
    }
    let mut bounds = Vec::new();
    while lo <= limit {
        let mut hi = (lo + SEG - 1).min(limit);
        if (hi & 1) == 0 {
            hi -= 1;
        }
        if hi >= lo {
            bounds.push((lo, hi));
        }
        lo = hi + 2;
    }

    let parts: Vec<Vec<u32>> = bounds
        .into_par_iter()
        .map(|(lo, hi)| segment_odd_primes(lo, hi, &small_odd))
        .collect();
    for part in parts {
        primes.extend(part);
    }
    primes
}

struct Ctx {
    primes: Vec<u32>,
    pi1: Vec<u32>,
    pi3: Vec<u32>,
    n: u64,
    l: u64,
}

impl Ctx {
    #[inline(always)]
    fn upper_idx(&self, start: usize, lim: u64) -> usize {
        let primes = self.primes.as_slice();
        if start >= primes.len() || lim < 3 {
            return start;
        }
        let lim32 = if lim >= u32::MAX as u64 {
            u32::MAX
        } else {
            lim as u32
        };
        start + primes[start..].partition_point(|&p| p <= lim32)
    }

    #[inline(always)]
    fn counts(&self, lo: usize, hi: usize) -> (i64, i64) {
        (
            (self.pi1[hi] - self.pi1[lo]) as i64,
            (self.pi3[hi] - self.pi3[lo]) as i64,
        )
    }
}

fn last_batch(ctx: &Ctx, start: usize, end: usize, m: u64, s: i32, used_sq: bool, sign: i64) -> i64 {
    if start >= end {
        return 0;
    }
    let nn = ctx.n / (m * m);
    let t = ctx.l / m;
    let primes = ctx.primes.as_slice();
    let mut ans = 0i64;
    let mut i = start;
    while i < end {
        // SAFETY: i < end <= primes.len()
        let p = unsafe { *primes.get_unchecked(i) } as u64;
        let q = nn / (p * p);
        let mut p_hi = isqrt(nn / q);
        if p_hi > t {
            p_hi = t;
        }
        let p_hi32 = p_hi as u32;
        let rest = end - i;
        let j = if rest < 64 {
            let mut j = i + 1;
            while j < end && unsafe { *primes.get_unchecked(j) } <= p_hi32 {
                j += 1;
            }
            j
        } else {
            i + unsafe { primes.get_unchecked(i..end) }.partition_point(|&pp| pp <= p_hi32)
        };
        let j = if j > end { end } else { j.max(i + 1) };
        let (c1, c3) = ctx.counts(i, j);
        let count = ((q + 3) / 4) as i64;
        if used_sq {
            ans += sign * (c1 + c3) * count;
        } else {
            let mu1 = -sign * (1 - s as i64);
            let mu3 = sign * (s as i64);
            ans += (mu1 * c1 + mu3 * c3) * count;
        }
        i = j;
    }
    ans
}

fn last_batch_par(
    ctx: &Ctx,
    start: usize,
    end: usize,
    m: u64,
    s: i32,
    used_sq: bool,
    sign: i64,
    par: bool,
) -> i64 {
    if par && end.saturating_sub(start) > PAR_LAST {
        let mid = (start + end) / 2;
        let (a, b) = rayon::join(
            || last_batch_par(ctx, start, mid, m, s, used_sq, sign, true),
            || last_batch_par(ctx, mid, end, m, s, used_sq, sign, true),
        );
        a + b
    } else {
        last_batch(ctx, start, end, m, s, used_sq, sign)
    }
}

fn extend(ctx: &Ctx, idx: usize, m: u64, n1: i32, s: i32, used_sq: bool, sign: i64, par: bool) -> i64 {
    let p = unsafe { *ctx.primes.get_unchecked(idx) } as u64;
    if m > ctx.l / p {
        return 0;
    }
    let n1n = n1 + i32::from((p & 3) == 1);
    let sn = s + i32::from((p & 3) == 3);
    let mut ans = dfs(ctx, idx + 1, m * p, n1n, sn, used_sq, -sign, par);
    if !used_sq && (p & 3) == 3 {
        let p2 = p * p;
        if m <= ctx.l / p2 {
            ans += dfs(ctx, idx + 1, m * p2, n1, s, true, sign, par);
        }
    }
    ans
}

fn process_rec(
    ctx: &Ctx,
    start: usize,
    rec_end: usize,
    m: u64,
    n1: i32,
    s: i32,
    used_sq: bool,
    sign: i64,
    par: bool,
) -> i64 {
    if start >= rec_end {
        return 0;
    }
    if par && rec_end - start > 4 && m <= PAR_M {
        let mid = (start + rec_end) / 2;
        let (a, b) = rayon::join(
            || process_rec(ctx, start, mid, m, n1, s, used_sq, sign, true),
            || process_rec(ctx, mid, rec_end, m, n1, s, used_sq, sign, true),
        );
        return a + b;
    }
    let mut ans = 0i64;
    for idx in start..rec_end {
        ans += extend(ctx, idx, m, n1, s, used_sq, sign, false);
    }
    ans
}

fn dfs(ctx: &Ctx, start: usize, m: u64, n1: i32, s: i32, used_sq: bool, sign: i64, par: bool) -> i64 {
    let mu = if used_sq {
        -sign
    } else {
        sign * (1 - s as i64)
    };
    let q = ctx.n / (m * m);
    let mut ans = if mu != 0 { mu * ((q + 3) / 4) as i64 } else { 0 };

    let t = ctx.l / m;
    let sqrt_t = isqrt(t);
    let rec_end = ctx.upper_idx(start, sqrt_t);
    let all_end = ctx.upper_idx(rec_end, t);

    ans += last_batch_par(ctx, rec_end, all_end, m, s, used_sq, sign, par && m == 1);
    ans += process_rec(ctx, start, rec_end, m, n1, s, used_sq, sign, par);
    ans
}

fn main() {
    let mut l = isqrt(N);
    while l * l > N {
        l -= 1;
    }
    while (l + 1) * (l + 1) <= N {
        l += 1;
    }

    let primes = sieve_odd_primes(l as usize);
    let np = primes.len();
    let mut pi1 = vec![0u32; np + 1];
    let mut pi3 = vec![0u32; np + 1];
    for i in 0..np {
        let p = primes[i];
        pi1[i + 1] = pi1[i] + u32::from((p & 3) == 1);
        pi3[i + 1] = pi3[i] + u32::from((p & 3) == 3);
    }

    let ctx = Ctx {
        primes,
        pi1,
        pi3,
        n: N,
        l,
    };
    let ans = dfs(&ctx, 0, 1, 0, 0, false, 1, true);
    println!("{}", ans);
}
