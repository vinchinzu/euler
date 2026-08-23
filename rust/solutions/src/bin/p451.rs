// Project Euler 451 - Modular inverses
// Find sum_{n=3}^N l(n), where l(n) is the largest x < n-1 with x^2 ≡ 1 (mod n).

use rayon::prelude::*;

const MAXN: i32 = 20_000_000;
const MAX_SQRTS: usize = 512;

#[inline(always)]
fn mod_inv(a: i32, m: i32) -> i32 {
    let (mut t, mut nt) = (0i64, 1i64);
    let (mut r, mut nr) = (m as i64, a as i64);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    if t < 0 { (t + m as i64) as i32 } else { t as i32 }
}

/// CRT-lift sv (mod n) against ±1 (mod pe). Products fit in i64 (n,pe <= 2e7).
#[inline(always)]
fn combine_pair(sv: i32, pe: i32, pe_inv: i32, b: i64, n: i32, npe: i32) -> (i32, i32) {
    let t = (sv as u64 * pe_inv as u64 % n as u64) as i64;
    let a = pe as i64 * t;
    let m = npe as i64;
    let mut xp = a + b;
    let mut xm = a - b;
    if xp >= m {
        xp -= m;
    } else if xp < 0 {
        xp += m;
    }
    if xm >= m {
        xm -= m;
    } else if xm < 0 {
        xm += m;
    }
    (xp as i32, xm as i32)
}

#[inline(always)]
fn max_root(sqrts: &[i32], n: i32) -> i32 {
    let cap = n - 1;
    let mut l = 0i32;
    for i in 0..sqrts.len() {
        // SAFETY: i < sqrts.len()
        let s = unsafe { *sqrts.get_unchecked(i) };
        if s < cap && s > l {
            l = s;
        }
    }
    l
}

fn extend_prime(
    index: usize,
    n: i32,
    sqrts: &[i32],
    primes: &[i32],
    n_limit: i32,
    par: bool,
) -> i64 {
    let p = unsafe { *primes.get_unchecked(index) };
    let mut ans = 0i64;
    let mut pe = p;
    loop {
        ans += extend_one(index, n, pe, sqrts, primes, n_limit, par);
        // next pe' = pe * p; require n * pe' <= n_limit
        if (pe as i64) * (p as i64) > (n_limit / n) as i64 {
            break;
        }
        pe *= p;
    }
    ans
}

fn extend_one(
    index: usize,
    n: i32,
    pe: i32,
    sqrts: &[i32],
    primes: &[i32],
    n_limit: i32,
    par: bool,
) -> i64 {
    let npe = n * pe;
    let next_p = if index + 1 < primes.len() {
        unsafe { *primes.get_unchecked(index + 1) }
    } else {
        n_limit.saturating_add(1)
    };
    let will_recurse = next_p <= n_limit / npe;

    // n=1 and n=2 have closed-form lifts: only ±1 mod npe.
    if n == 1 {
        if !will_recurse {
            return 1;
        }
        return helper(index + 1, npe, &[1, npe - 1], primes, n_limit, par);
    }
    if n == 2 {
        if !will_recurse {
            return 1;
        }
        let npe_m1 = npe - 1;
        return helper(index + 1, npe, &[1, npe_m1], primes, n_limit, par);
    }

    let pe_inv = mod_inv(pe, n);
    let n_inv = (1 - pe as i64 * pe_inv as i64) / n as i64;
    let b = n as i64 * n_inv;

    if !will_recurse {
        let cap = npe - 1;
        let mut l = 0i32;
        for i in 0..sqrts.len() {
            let sv = unsafe { *sqrts.get_unchecked(i) };
            let (xp, xm) = combine_pair(sv, pe, pe_inv, b, n, npe);
            if xp < cap && xp > l {
                l = xp;
            }
            if xm < cap && xm > l {
                l = xm;
            }
        }
        return l as i64;
    }

    let mut new_sqrts = [0i32; MAX_SQRTS];
    let mut k = 0usize;
    for i in 0..sqrts.len() {
        let sv = unsafe { *sqrts.get_unchecked(i) };
        let (xp, xm) = combine_pair(sv, pe, pe_inv, b, n, npe);
        unsafe {
            *new_sqrts.get_unchecked_mut(k) = xp;
            *new_sqrts.get_unchecked_mut(k + 1) = xm;
        }
        k += 2;
    }
    helper(
        index + 1,
        npe,
        unsafe { new_sqrts.get_unchecked(..k) },
        primes,
        n_limit,
        par,
    )
}

fn helper(
    min_index: usize,
    n: i32,
    sqrts: &[i32],
    primes: &[i32],
    n_limit: i32,
    par: bool,
) -> i64 {
    let mut ans = max_root(sqrts, n) as i64;
    if n > n_limit / 3 {
        return ans;
    }
    let max_p = n_limit / n;
    let rest = unsafe { primes.get_unchecked(min_index..) };
    let len = rest.partition_point(|&p| p <= max_p);
    if len == 0 {
        return ans;
    }

    // p^k (and 2 p^k) contribute I=1 and need no CRT against further primes once p^2
    // no longer fits. Batch those leaves.
    let (work_end, extra) = if n == 1 {
        let split = rest[..len].partition_point(|&p| (p as i64) * (p as i64) <= n_limit as i64);
        (min_index + split, (len - split) as i64)
    } else if n == 2 {
        let split = rest[..len]
            .partition_point(|&p| (p as i64) * (p as i64) * 2 <= n_limit as i64);
        (min_index + split, (len - split) as i64)
    } else {
        (min_index + len, 0i64)
    };
    ans += extra;
    if work_end <= min_index {
        return ans;
    }

    let nwork = work_end - min_index;
    let do_par = par && nwork > 64;
    if do_par {
        let min_len = if n < 20 {
            1
        } else if n < 400 {
            4
        } else {
            32
        };
        let nested = n < n_limit / 80;
        ans += (min_index..work_end)
            .into_par_iter()
            .with_min_len(min_len)
            .map(|index| extend_prime(index, n, sqrts, primes, n_limit, nested))
            .sum::<i64>();
    } else {
        for index in min_index..work_end {
            ans += extend_prime(index, n, sqrts, primes, n_limit, false);
        }
    }
    ans
}

fn sieve_odd_primes(limit: usize) -> Vec<i32> {
    let mut is_prime = vec![1u8; limit + 1];
    is_prime[0] = 0;
    is_prime[1] = 0;
    let mut p = 2usize;
    while p * p <= limit {
        if is_prime[p] != 0 {
            let mut j = p * p;
            while j <= limit {
                is_prime[j] = 0;
                j += p;
            }
        }
        p += 1;
    }
    let mut primes = Vec::with_capacity(1_300_000);
    let mut i = 3usize;
    while i <= limit {
        if is_prime[i] != 0 {
            primes.push(i as i32);
        }
        i += 2;
    }
    primes
}

fn main() {
    let n_limit = MAXN;
    let primes = sieve_odd_primes(n_limit as usize);

    let mut starts: Vec<(i32, Vec<i32>)> = vec![
        (1, vec![0]),
        (2, vec![1]),
        (4, vec![1, 3]),
    ];
    let mut pow2 = 8i32;
    while pow2 <= n_limit {
        starts.push((pow2, vec![1, pow2 / 2 - 1, pow2 / 2 + 1, pow2 - 1]));
        pow2 *= 2;
    }

    let ans: i64 = starts
        .into_par_iter()
        .map(|(n, sqrts)| helper(0, n, &sqrts, &primes, n_limit, true))
        .sum();

    println!("{}", ans);
}
