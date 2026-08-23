// Project Euler 216 - Investigating the Primality of 2n^2-1
//
// Count primes of the form 2n^2 - 1 for 2 <= n <= 50,000,000.
// Segmented prime sieve + Tonelli-Shanks, both parallel; hybrid n-sieve.

use rayon::prelude::*;
use std::sync::atomic::{AtomicU8, Ordering};

const N: usize = 50_000_000;
/// Sequential mark threshold: tiny p dominate write volume / imbalance.
const SEQ_P: u32 = 4096;

#[inline(always)]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    // p <= sqrt(2)*N < 2^27, so a*b fits in u64.
    a.wrapping_mul(b) % m
}

#[inline(always)]
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    result
}

/// Jacobi symbol (a/n) for odd n > 0.
#[inline(always)]
fn jacobi(mut a: u64, mut n: u64) -> i32 {
    let mut t = 1i32;
    a %= n;
    while a != 0 {
        while a & 1 == 0 {
            a >>= 1;
            let r = n & 7;
            if r == 3 || r == 5 {
                t = -t;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if (a & 3) == 3 && (n & 3) == 3 {
            t = -t;
        }
        a %= n;
    }
    if n == 1 { t } else { 0 }
}

fn sqrt_mod(n: u64, p: u64) -> u64 {
    if p & 3 == 3 {
        return pow_mod(n, (p + 1) >> 2, p);
    }
    let s = (p - 1).trailing_zeros();
    let q = (p - 1) >> s;
    let mut z = 3u64;
    while jacobi(z, p) != -1 {
        z += 1;
    }
    let mut m = s;
    let mut c = pow_mod(z, q, p);
    let mut t = pow_mod(n, q, p);
    let mut r = pow_mod(n, (q + 1) >> 1, p);
    while t != 1 {
        let mut i = 1u32;
        let mut tmp = mul_mod(t, t, p);
        while tmp != 1 {
            tmp = mul_mod(tmp, tmp, p);
            i += 1;
        }
        let mut b = c;
        for _ in 0..(m - i - 1) {
            b = mul_mod(b, b, p);
        }
        m = i;
        c = mul_mod(b, b, p);
        t = mul_mod(t, c, p);
        r = mul_mod(r, b, p);
    }
    r
}

fn primes_upto(limit: usize) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }
    let mut comp = vec![0u8; limit + 1];
    let mut ps = Vec::with_capacity(limit / 10);
    let mut i = 2usize;
    while i * i <= limit {
        if comp[i] == 0 {
            ps.push(i as u32);
            let mut j = i * i;
            while j <= limit {
                comp[j] = 1;
                j += i;
            }
        }
        i += 1;
    }
    while i <= limit {
        if comp[i] == 0 {
            ps.push(i as u32);
        }
        i += 1;
    }
    ps
}

#[inline(always)]
fn start_from_root(r: u64, p: u64) -> u32 {
    if 2 * r * r - 1 == p {
        (r + p) as u32
    } else {
        r as u32
    }
}

fn compute_residues(limit: usize) -> Vec<(u32, u32, u32)> {
    let sqrt_l = limit.isqrt() + 1;
    let small_odd: Vec<u32> = primes_upto(sqrt_l)
        .into_iter()
        .filter(|&p| p > 2)
        .collect();

    const SEG: usize = 1 << 16;
    let n_seg = limit.div_ceil(SEG);

    let parts: Vec<Vec<(u32, u32, u32)>> = (0..n_seg)
        .into_par_iter()
        .map(|si| {
            let lo = si * SEG;
            let hi = (lo + SEG).min(limit + 1);
            let len = hi - lo;
            let mut comp = vec![0u8; len];
            if lo == 0 {
                if len > 0 {
                    comp[0] = 1;
                }
                if len > 1 {
                    comp[1] = 1;
                }
            }
            for &p in &small_odd {
                let p = p as usize;
                let p2 = p * p;
                if p2 >= hi {
                    break;
                }
                let mut j = if p2 > lo {
                    p2
                } else {
                    lo.div_ceil(p) * p
                };
                if j & 1 == 0 {
                    j += p;
                }
                while j < hi {
                    // SAFETY: j in [lo, hi)
                    unsafe {
                        *comp.get_unchecked_mut(j - lo) = 1;
                    }
                    j += p << 1;
                }
            }

            let mut out = Vec::with_capacity(len / 16);
            let mut n = if lo <= 3 { 3 } else { lo | 1 };
            while n < hi {
                // SAFETY: n in [lo, hi)
                if unsafe { *comp.get_unchecked(n - lo) } == 0 {
                    let m8 = n & 7;
                    if m8 == 1 || m8 == 7 {
                        let p = n as u64;
                        let r = sqrt_mod((p + 1) >> 1, p);
                        out.push((p as u32, start_from_root(r, p), start_from_root(p - r, p)));
                    }
                }
                n += 2;
            }
            out
        })
        .collect();
    let mut out = Vec::with_capacity(limit / 8);
    for mut part in parts {
        out.append(&mut part);
    }
    out
}

#[inline(always)]
fn mark_plain(sieve: &mut [u8], mut i: usize, p: usize) {
    let n = sieve.len();
    while i < n {
        // SAFETY: i < len
        unsafe {
            *sieve.get_unchecked_mut(i) = 0;
        }
        i += p;
    }
}

#[inline(always)]
fn mark_atomic(sieve: &[AtomicU8], mut i: usize, p: usize) {
    let n = sieve.len();
    while i < n {
        // SAFETY: i < len
        unsafe {
            sieve.get_unchecked(i).store(0, Ordering::Relaxed);
        }
        i += p;
    }
}

fn vec_u8_to_atomic(v: Vec<u8>) -> Vec<AtomicU8> {
    let mut v = std::mem::ManuallyDrop::new(v);
    unsafe { Vec::from_raw_parts(v.as_mut_ptr() as *mut AtomicU8, v.len(), v.capacity()) }
}

fn main() {
    let l = (std::f64::consts::SQRT_2 * N as f64) as usize;

    let residues = compute_residues(l);
    let (small, mut large): (Vec<_>, Vec<_>) =
        residues.into_iter().partition(|&(p, _, _)| p < SEQ_P);
    // Largest p first so rayon's range splits give light work to the
    // left; stealers take the heavy small-p tail.
    large.reverse();

    let mut sieve = vec![1u8; N + 1];
    sieve[0] = 0;
    sieve[1] = 0;

    for &(p, r1, r2) in &small {
        let p = p as usize;
        mark_plain(&mut sieve, r1 as usize, p);
        mark_plain(&mut sieve, r2 as usize, p);
    }

    let sieve = vec_u8_to_atomic(sieve);
    large.par_iter().for_each(|&(p, r1, r2)| {
        let p = p as usize;
        mark_atomic(&sieve, r1 as usize, p);
        mark_atomic(&sieve, r2 as usize, p);
    });

    let ans: u32 = sieve[2..]
        .iter()
        .map(|x| x.load(Ordering::Relaxed) as u32)
        .sum();
    println!("{}", ans);
}
