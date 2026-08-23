// Problem 925 – Larger Digit Permutation III
//
// B(n) = smallest number larger than n formed by rearranging digits of n, or 0.
// T(N) = sum_{n=1..N} B(n^2).  Compute T(10^16) mod 10^9+7.
//
// Algorithm: decompose T(N) = sum(n^2) + sum(delta) where delta = B(n^2) - n^2.
// Use suffix pruning: build n from lsd upward; when the last t digits of n^2
// already contain a non-trivial permutation pivot, delta is fixed for all
// extensions of n, so we can count completions analytically.
//
// Hot path uses u64 (t = c+tz <= 18 almost always). Independent DFS stacks are
// parallelized over 4-digit LSD prefixes (and small lengths).

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const INV6: u64 = 166_666_668;

const POW10: [u64; 19] = {
    let mut a = [1u64; 19];
    let mut i = 1;
    while i < 19 {
        a[i] = a[i - 1] * 10;
        i += 1;
    }
    a
};

const POW10_MOD: [u64; 17] = {
    let mut a = [1u64; 17];
    let mut i = 1;
    while i < 17 {
        a[i] = a[i - 1] * 10 % MOD;
        i += 1;
    }
    a
};

/// In-place next lexicographic permutation (msd -> lsd order).
/// Returns true if a next permutation exists.
#[inline(always)]
fn next_permutation(digs: &mut [u8]) -> bool {
    let n = digs.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    loop {
        // SAFETY: 0 <= i < n-1, so i+1 < n.
        if unsafe { *digs.get_unchecked(i) < *digs.get_unchecked(i + 1) } {
            break;
        }
        if i == 0 {
            return false;
        }
        i -= 1;
    }
    let mut j = n - 1;
    // SAFETY: j decreases from n-1 toward i; a pivot exists so j >= i.
    while unsafe { *digs.get_unchecked(j) <= *digs.get_unchecked(i) } {
        j -= 1;
    }
    digs.swap(i, j);
    digs[i + 1..].reverse();
    true
}

#[inline(always)]
fn count_tz(mut v: u64, c: usize) -> usize {
    if v == 0 {
        return c;
    }
    let mut tz = 0usize;
    while v % 10 == 0 {
        v /= 10;
        tz += 1;
    }
    tz
}

#[inline(always)]
fn sq_mod(v: u64, m: u64) -> u64 {
    if v <= u32::MAX as u64 {
        v * v % m
    } else {
        ((v as u128).wrapping_mul(v as u128) % (m as u128)) as u64
    }
}

#[inline(always)]
fn add_mod(total: &mut u64, x: u64) {
    let s = *total + x;
    *total = if s >= MOD { s - MOD } else { s };
}

/// Sum_{n=1..N} n^2 mod MOD, using n(n+1)(2n+1)/6.
fn sum_squares_mod(big_n: u64) -> u64 {
    if big_n == 0 {
        return 0;
    }
    let n = big_n % MOD;
    let n1 = (big_n + 1) % MOD;
    let n2 = (2 * n + 1) % MOD;
    let part = n * n1 % MOD;
    part * n2 % MOD * INV6 % MOD
}

/// Digits of x (msd first). x = v^2 has at most 32 digits.
#[inline]
fn write_digits_u128(x: u128, buf: &mut [u8; 40]) -> usize {
    const E18: u128 = 1_000_000_000_000_000_000;
    let lo = (x % E18) as u64;
    let hi = (x / E18) as u64;
    if hi == 0 {
        if lo == 0 {
            buf[0] = 0;
            return 1;
        }
        let mut tmp = lo;
        let mut n = 0usize;
        while tmp > 0 {
            buf[n] = (tmp % 10) as u8;
            tmp /= 10;
            n += 1;
        }
        buf[..n].reverse();
        n
    } else {
        let mut hid = [0u8; 16];
        let mut hn = 0usize;
        let mut tmp = hi;
        while tmp > 0 {
            hid[hn] = (tmp % 10) as u8;
            tmp /= 10;
            hn += 1;
        }
        hid[..hn].reverse();
        buf[..hn].copy_from_slice(&hid[..hn]);
        let mut tmp = lo;
        for i in 0..18 {
            buf[hn + 17 - i] = (tmp % 10) as u8;
            tmp /= 10;
        }
        hn + 18
    }
}

/// B(v^2) mod MOD, or 0 if no next permutation.
#[cold]
fn b_square_mod(v: u64) -> u64 {
    if v == 0 {
        return 0;
    }
    let x = (v as u128).wrapping_mul(v as u128);
    let mut buf = [0u8; 40];
    let len = write_digits_u128(x, &mut buf);
    if next_permutation(&mut buf[..len]) {
        let mut r = 0u64;
        for &d in &buf[..len] {
            r = (r * 10 + d as u64) % MOD;
        }
        r
    } else {
        0
    }
}

/// Rare path: t = c+tz > 18 (many trailing zeros). Fill msd[0..t].
#[cold]
fn fill_msd_wide(v: u64, c: usize, tz: usize, msd: &mut [u8; 33]) -> usize {
    let t = c + tz;
    let k = c - tz;
    msd[..t].fill(0);
    if k == 0 {
        return t;
    }
    let w = v / POW10[tz];
    let sw = sq_mod(w, POW10[k]);
    let mut tmp = sw;
    for i in (0..k).rev() {
        msd[i] = (tmp % 10) as u8;
        tmp /= 10;
    }
    t
}

/// Sum of (B(n^2)-n^2) mod MOD over L-digit n whose last `start_c` digits equal `start_v`.
fn dfs_delta(l: usize, start_c: usize, start_v: u64, start_tz: usize) -> u64 {
    let mut total = 0u64;
    // Worst-case DFS stack: 9 pending digits per remaining level.
    let mut stack = [(0u8, 0u64, 0u8); 160];
    stack[0] = (start_c as u8, start_v, start_tz as u8);
    let mut sp = 1usize;

    while sp > 0 {
        sp -= 1;
        // SAFETY: sp was in 1..=160 before decrement.
        let (c8, v, tz8) = unsafe { *stack.get_unchecked(sp) };
        let c = c8 as usize;
        let tz = tz8 as usize;
        let t = c + tz;

        let mut msd = [0u8; 33];
        let mut nondecreasing = true;

        if t <= 18 {
            // SAFETY: t in 1..=18, POW10 has 19 entries.
            let m = unsafe { *POW10.get_unchecked(t) };
            let s = sq_mod(v, m);
            let mut tmp = s;
            let mut prev = 0u8;
            for k in 0..t {
                let d = (tmp % 10) as u8;
                tmp /= 10;
                if k > 0 && prev > d {
                    nondecreasing = false;
                }
                prev = d;
                // SAFETY: t-1-k in 0..t, t <= 18 < 33.
                unsafe {
                    *msd.get_unchecked_mut(t - 1 - k) = d;
                }
            }
            if !nondecreasing {
                let _ = next_permutation(&mut msd[..t]);
                let mut ns = 0u64;
                for i in 0..t {
                    // SAFETY: i < t <= 18.
                    ns = ns * 10 + unsafe { *msd.get_unchecked(i) } as u64;
                }
                let delta_mod = (ns - s) % MOD;
                let contrib = if c < l {
                    // SAFETY: 0 <= l-c-1 <= 15, POW10_MOD has 17 entries.
                    let count_mod = 9 * unsafe { *POW10_MOD.get_unchecked(l - c - 1) } % MOD;
                    delta_mod * count_mod % MOD
                } else {
                    delta_mod
                };
                add_mod(&mut total, contrib);
                continue;
            }
        } else {
            let t = fill_msd_wide(v, c, tz, &mut msd);
            // LSD-first order is msd[t-1], msd[t-2], ..., msd[0].
            nondecreasing = true;
            for j in (0..t - 1).rev() {
                // SAFETY: t <= 32, j+1 < t.
                if unsafe { *msd.get_unchecked(j + 1) > *msd.get_unchecked(j) } {
                    nondecreasing = false;
                    break;
                }
            }
            if !nondecreasing {
                let mut s_mod = 0u64;
                for i in 0..t {
                    s_mod = (s_mod * 10 + msd[i] as u64) % MOD;
                }
                let _ = next_permutation(&mut msd[..t]);
                let mut ns_mod = 0u64;
                for i in 0..t {
                    ns_mod = (ns_mod * 10 + msd[i] as u64) % MOD;
                }
                let delta_mod = (ns_mod + MOD - s_mod) % MOD;
                let contrib = if c < l {
                    let count_mod = 9 * POW10_MOD[l - c - 1] % MOD;
                    delta_mod * count_mod % MOD
                } else {
                    delta_mod
                };
                add_mod(&mut total, contrib);
                continue;
            }
        }

        if c == l {
            let xmod = (v % MOD) * (v % MOD) % MOD;
            let bmod = b_square_mod(v);
            add_mod(&mut total, (bmod + MOD - xmod) % MOD);
            continue;
        }

        let digit_start: u8 = if c + 1 == l { 1 } else { 0 };
        // SAFETY: c < l <= 16, POW10 has 19 entries.
        let p = unsafe { *POW10.get_unchecked(c) };
        for d in (digit_start..=9).rev() {
            let nv = v + d as u64 * p;
            let ntz = if tz == c && d == 0 { c + 1 } else { tz };
            // SAFETY: at most 9 pending siblings per remaining level; sp < 160.
            unsafe {
                *stack.get_unchecked_mut(sp) = ((c + 1) as u8, nv, ntz as u8);
            }
            sp += 1;
        }
    }

    total
}

fn main() {
    let big_n: u64 = 10u64.pow(16) - 1;

    const K: usize = 4;
    const NPREF: u64 = 10_000;

    let mut units: Vec<(u8, u8, u64, u8)> = Vec::with_capacity((16 - K) * NPREF as usize + 40);
    for l in 1..=K {
        let start = if l == 1 { 1u8 } else { 0u8 };
        for d0 in start..10 {
            let tz = if d0 == 0 { 1 } else { 0 };
            units.push((l as u8, 1, d0 as u64, tz));
        }
    }
    for l in (K + 1)..=16 {
        for pref in 0..NPREF {
            units.push((l as u8, K as u8, pref, count_tz(pref, K) as u8));
        }
    }

    let sum_delta: u64 = units
        .par_iter()
        .map(|&(l, c, v, tz)| dfs_delta(l as usize, c as usize, v, tz as usize))
        .sum::<u64>()
        % MOD;

    let ans = (sum_squares_mod(big_n) + sum_delta) % MOD;
    println!("{}", ans);
}
