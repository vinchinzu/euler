// Project Euler 608 - Divisor Sums
// Prefix of d(n) to LIM, then parallel DFS over square-free 200-smooth d.

use rayon::prelude::*;

const N: u64 = 1_000_000_000_000;
const K: usize = 200;
const MOD: u64 = 1_000_000_007;
// D(LIM) ≈ LIM log LIM < MOD, so small[] needs no reduction.
const LIM: usize = 8_000_000;
const PAR_D: u64 = 2_310;

struct Ctx<'a> {
    primes: &'a [u64],
    pu: &'a [u64],
    small: &'a [u32],
    lim: u64,
}

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

#[inline(always)]
fn mod_inv(a: u64) -> u64 {
    let mut t: i64 = 0;
    let mut new_t: i64 = 1;
    let mut r: i64 = MOD as i64;
    let mut new_r: i64 = (a % MOD) as i64;
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    let m = MOD as i64;
    ((t % m + m) % m) as u64
}

#[inline(always)]
fn tr(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// Σ_{k=1..=n} floor(n/k) = 2 Σ_{d=1..=√n} floor(n/d) − ⌊√n⌋², then % MOD.
#[inline]
fn sum_floor_quotients(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let sq = n.isqrt();
    let mut result = 0u64;
    let mut d = 1u64;
    let sq8 = sq.saturating_sub(7);
    while d <= sq8 {
        result += n / d;
        result += n / (d + 1);
        result += n / (d + 2);
        result += n / (d + 3);
        result += n / (d + 4);
        result += n / (d + 5);
        result += n / (d + 6);
        result += n / (d + 7);
        d += 8;
    }
    while d <= sq {
        result += n / d;
        d += 1;
    }
    (result * 2 - sq * sq) % MOD
}

#[inline(always)]
fn contrib(d: u64, mult: u64, ctx: &Ctx) -> u64 {
    let q = N / d;
    let s = if q <= ctx.lim {
        // SAFETY: q <= LIM and small.len() == LIM+1.
        unsafe { *ctx.small.get_unchecked(q as usize) as u64 }
    } else {
        sum_floor_quotients(q)
    };
    mul_mod(s, mult)
}

fn build_prefix(lim: usize) -> Vec<u32> {
    const CHUNK: usize = 262_144;
    let mut d = vec![0u32; lim + 1];
    let tail = &mut d[1..=lim];
    tail.par_chunks_mut(CHUNK).enumerate().for_each(|(ci, slice)| {
        let start = 1 + ci * CHUNK;
        let end = start + slice.len();
        for i in 1..end {
            let mut j = ((start + i - 1) / i) * i;
            if j < start {
                j += i;
            }
            while j < end {
                // SAFETY: j in [start, end).
                unsafe {
                    *slice.get_unchecked_mut(j - start) += 1;
                }
                j += i;
            }
        }
    });
    let mut acc = 0u32;
    for i in 1..=lim {
        // SAFETY: i in 1..=lim.
        unsafe {
            acc += *d.get_unchecked(i);
            *d.get_unchecked_mut(i) = acc;
        }
    }
    d
}

fn dfs_seq(min_idx: usize, d0: u64, mult0: u64, ctx: &Ctx) -> u64 {
    let primes = ctx.primes;
    let pu = ctx.pu;
    let plen = primes.len();
    let mut stack = Vec::with_capacity(64);
    stack.push((min_idx, d0, mult0));
    let mut ans = 0u64;
    while let Some((min_idx, d, mult)) = stack.pop() {
        ans += contrib(d, mult, ctx);
        let q = N / d;
        for idx in min_idx..plen {
            let p = unsafe { *primes.get_unchecked(idx) };
            if q < p {
                break;
            }
            let nm = mul_mod(mult, unsafe { *pu.get_unchecked(idx) });
            stack.push((idx + 1, d * p, nm));
        }
    }
    ans
}

fn dfs_par(min_idx: usize, d: u64, mult: u64, ctx: &Ctx) -> u64 {
    if d > PAR_D {
        return dfs_seq(min_idx, d, mult, ctx);
    }
    let ans0 = contrib(d, mult, ctx);
    let primes = ctx.primes;
    let plen = primes.len();
    let q = N / d;
    let mut end = min_idx;
    while end < plen {
        let p = unsafe { *primes.get_unchecked(end) };
        if q < p {
            break;
        }
        end += 1;
    }
    if end <= min_idx {
        return ans0;
    }
    if end - min_idx >= 2 {
        let rest: u64 = (min_idx..end)
            .into_par_iter()
            .map(|idx| {
                let p = unsafe { *primes.get_unchecked(idx) };
                let nm = mul_mod(mult, unsafe { *ctx.pu.get_unchecked(idx) });
                dfs_par(idx + 1, d * p, nm, ctx)
            })
            .sum();
        ans0 + rest
    } else {
        let p = unsafe { *primes.get_unchecked(min_idx) };
        let nm = mul_mod(mult, unsafe { *ctx.pu.get_unchecked(min_idx) });
        ans0 + dfs_seq(min_idx + 1, d * p, nm, ctx)
    }
}

fn main() {
    let mut is_prime = [true; K + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= K {
        if is_prime[i] {
            let mut j = i * i;
            while j <= K {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<u64> = (2..=K as u64).filter(|&p| is_prime[p as usize]).collect();

    let mut pu = vec![0u64; primes.len()];
    let mut mult = 1u64;
    for (i, &p) in primes.iter().enumerate() {
        let mut e = 0u64;
        let mut pw = p;
        while pw <= K as u64 {
            e += (K as u64) / pw;
            pw *= p;
        }
        let tr_e1 = tr(e + 1);
        mult = mul_mod(mult, tr_e1);
        let inv = mod_inv(tr_e1);
        let t = mul_mod(tr(e), inv);
        pu[i] = if t == 0 { 0 } else { MOD - t };
    }

    let small = build_prefix(LIM);
    let ctx = Ctx {
        primes: &primes,
        pu: &pu,
        small: &small,
        lim: LIM as u64,
    };

    let ans = dfs_par(0, 1, mult, &ctx);
    println!("{}", ans % MOD);
}
