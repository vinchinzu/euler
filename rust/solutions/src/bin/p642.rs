// Project Euler 642 - Sum of largest prime factors
// Lucy hedgehog for sum of primes, then recursive enumeration

use rayon::prelude::*;

const MOD: i64 = 1_000_000_000;
const N: i64 = 201_820_182_018;

struct Ctx<'a> {
    primes: &'a [i64],
    small: &'a [i64],
    big: &'a [i64],
    r: i64,
}

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// v*(v+1)/2 mod MOD; v*(v+1) is even so no modular inverse is needed.
fn tri_mod(v: i64) -> i64 {
    let (a, b) = if v % 2 == 0 {
        ((v / 2) % MOD, (v + 1) % MOD)
    } else {
        (v % MOD, ((v + 1) / 2) % MOD)
    };
    a * b % MOD
}

/// a - p*(s-sp) mod MOD with a,s,sp in [0, MOD) and p < MOD.
#[inline(always)]
fn msub(a: i64, p: i64, s: i64, sp: i64) -> i64 {
    let mut d = s - sp;
    if d < 0 {
        d += MOD;
    }
    let t = ((p as u64) * (d as u64) % (MOD as u64)) as i64;
    let mut x = a - t;
    if x < 0 {
        x += MOD;
    }
    x
}

/// Sum of primes <= N/n (mod MOD).
#[inline(always)]
fn sum_to_nlim(n: i64, ctx: &Ctx) -> i64 {
    let max_p = N / n;
    if max_p <= ctx.r {
        // SAFETY: max_p >= 0 and max_p <= r; small has length r+1
        unsafe { *ctx.small.get_unchecked(max_p as usize) }
    } else {
        // N/n > r ⇒ n <= r, so big[n] = sum of primes <= N/n
        unsafe { *ctx.big.get_unchecked(n as usize) }
    }
}

#[inline(always)]
fn node_contrib(min_index: usize, n: i64, ctx: &Ctx) -> i64 {
    let primes = ctx.primes;
    if min_index >= primes.len() {
        return 0;
    }
    let min_p = unsafe { *primes.get_unchecked(min_index) };
    let max_p = N / n;
    if min_p > max_p {
        return 0;
    }
    // min_p is a prime <= r, so min_p-1 indexes small[]
    let s_min = unsafe { *ctx.small.get_unchecked((min_p - 1) as usize) };
    sum_to_nlim(n, ctx) - s_min
}

fn child_end(min_index: usize, n: i64, ctx: &Ctx) -> usize {
    let max_p = N / n;
    let primes = ctx.primes;
    let mut end = min_index;
    while end < primes.len() {
        let p = unsafe { *primes.get_unchecked(end) };
        if p > max_p / p {
            break;
        }
        end += 1;
    }
    end
}

fn dfs_par(min_index: usize, n: i64, ctx: &Ctx) -> i64 {
    let mut ans = node_contrib(min_index, n, ctx);
    let end = child_end(min_index, n, ctx);
    if end <= min_index {
        return ans;
    }
    // Nested split only for tiny n (heavy subtrees). Root n=1 is handled in main.
    if n <= 32 && end - min_index > 16 {
        ans += (min_index..end)
            .into_par_iter()
            .map(|index| {
                let p = unsafe { *ctx.primes.get_unchecked(index) };
                dfs_par(index, n * p, ctx)
            })
            .sum::<i64>();
    } else {
        ans += dfs_seq_from(min_index, end, n, ctx);
    }
    ans
}

fn dfs_seq_from(min_index: usize, end: usize, n: i64, ctx: &Ctx) -> i64 {
    let mut stack = Vec::with_capacity(256);
    let primes = ctx.primes;
    for index in min_index..end {
        let p = unsafe { *primes.get_unchecked(index) };
        stack.push((index, n * p));
    }
    let mut ans = 0i64;
    while let Some((min_index, n)) = stack.pop() {
        ans += node_contrib(min_index, n, ctx);
        let max_p = N / n;
        for index in min_index..primes.len() {
            let p = unsafe { *primes.get_unchecked(index) };
            if p > max_p / p {
                break;
            }
            stack.push((index, n * p));
        }
    }
    ans
}

fn main() {
    let r = isqrt(N) as usize;
    let ru = r as i64;

    // Lucy hedgehog: small[k] = sum of primes <= k, big[i] = sum of primes <= N/i
    let mut small = vec![0i64; r + 1];
    let mut big = vec![0i64; r + 1];

    for k in 2..=r {
        small[k] = ((k as i64) * (k as i64 + 1) / 2 - 1) % MOD;
    }
    for i in 1..=r {
        let v = N / i as i64;
        big[i] = (tri_mod(v) - 1 + MOD) % MOD;
    }

    // Sequential over primes (loop-carried). Inner i-updates use old larger indices.
    for p in 2..=r {
        if small[p] == small[p - 1] {
            continue;
        }
        let sp = small[p - 1];
        let pi = p as i64;
        let p2 = pi * pi;

        let i_max = (N / p2).min(ru);
        if i_max >= 1 {
            // d = N/(i*p) > r  iff  i <= N/(p*(r+1))
            let i_mid = (N / (pi * (ru + 1))).min(i_max);
            unsafe {
                let mut ip = p;
                for i in 1..=i_mid as usize {
                    let s = *big.get_unchecked(ip);
                    *big.get_unchecked_mut(i) = msub(*big.get_unchecked(i), pi, s, sp);
                    ip += p;
                }
                let m = N / pi;
                let mut i = i_mid + 1;
                while i <= i_max {
                    let q = m / i;
                    let mut i_last = m / q;
                    if i_last > i_max {
                        i_last = i_max;
                    }
                    let s = *small.get_unchecked(q as usize);
                    while i <= i_last {
                        *big.get_unchecked_mut(i as usize) =
                            msub(*big.get_unchecked(i as usize), pi, s, sp);
                        i += 1;
                    }
                }
            }
        }

        if p2 <= ru {
            let mut k = ru;
            while k >= p2 {
                let q = k / pi;
                let lo = (q * pi).max(p2);
                unsafe {
                    let s = *small.get_unchecked(q as usize);
                    let mut j = k;
                    while j >= lo {
                        *small.get_unchecked_mut(j as usize) =
                            msub(*small.get_unchecked(j as usize), pi, s, sp);
                        j -= 1;
                    }
                }
                k = lo - 1;
            }
        }
    }

    // Primes are exactly the Lucy jump points (difference ≡ p ≠ 0 mod MOD).
    let primes: Vec<i64> = (2..=r)
        .filter(|&i| small[i] != small[i - 1])
        .map(|i| i as i64)
        .collect();

    let ctx = Ctx {
        primes: &primes,
        small: &small,
        big: &big,
        r: ru,
    };

    let mut ans = node_contrib(0, 1, &ctx);
    let end = child_end(0, 1, &ctx);

    // First-level frames (n = p) with p^3 <= N can have heavy subtrees; the rest are leaves.
    let mut heavy = 0usize;
    while heavy < end {
        let p = primes[heavy];
        if p > N / p / p {
            break;
        }
        heavy += 1;
    }

    ans += (0..heavy)
        .into_par_iter()
        .map(|idx| {
            let p = primes[idx];
            dfs_par(idx, p, &ctx)
        })
        .sum::<i64>();

    for idx in heavy..end {
        ans += node_contrib(idx, primes[idx], &ctx);
    }

    ans %= MOD;
    if ans < 0 {
        ans += MOD;
    }
    println!("{}", ans);
}
