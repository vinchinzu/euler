// Project Euler 291: Panaitopol Primes
//
// Count primes p < 5*10^15 of the form 2y^2 + 2y + 1.
// Parallel segmented sieve of q ≡ 1 (mod 4), sqrt(-1) via a^{(q-1)/4},
// then hybrid y-sieve (seq small q, atomic large q).

use rayon::prelude::*;
use std::sync::atomic::{AtomicU8, Ordering};

const N: u64 = 5_000_000_000_000_000;
const LIMIT: usize = 50_000_000;
/// Sequential mark threshold: tiny p dominate write volume / imbalance.
const SEQ_P: u32 = 4096;

#[inline(always)]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    // p <= sqrt(N) < 2^27, so a*b fits in u64.
    a.wrapping_mul(b) % m
}

#[inline(always)]
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
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

/// sqrt(-1) mod p for p ≡ 1 (mod 4): a^{(p-1)/4} with (a/p) = -1.
#[inline(always)]
fn sqrt_neg1(p: u64) -> u64 {
    let exp = (p - 1) >> 2;
    // p ≡ 5 (mod 8) => 2 is a non-residue, so 2^exp ≡ sqrt(-1).
    if p & 7 == 5 {
        return pow_mod(2, exp, p);
    }
    let mut z = 3u64;
    while jacobi(z, p) != -1 {
        z += 2;
    }
    pow_mod(z, exp, p)
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

/// First unmarked y ≡ yr (mod p), skipping y=0 and the generator f(y)=p.
#[inline(always)]
fn start_from_root(yr: u64, p: u64) -> u32 {
    if yr == 0 {
        p as u32
    } else if 2 * yr * yr + 2 * yr + 1 == p {
        (yr + p) as u32
    } else {
        yr as u32
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
            // First n >= max(lo, 5) with n ≡ 1 (mod 4).
            let mut n = lo.max(5);
            let r4 = n & 3;
            if r4 != 1 {
                n += (5 - r4) & 3;
            }
            while n < hi {
                // SAFETY: n in [lo, hi)
                if unsafe { *comp.get_unchecked(n - lo) } == 0 {
                    let p = n as u64;
                    let r = sqrt_neg1(p);
                    let inv2 = (p + 1) >> 1;
                    let y1 = ((r + p - 1) * inv2) % p;
                    let y2 = ((p * 2 - r - 1) * inv2) % p;
                    out.push((p as u32, start_from_root(y1, p), start_from_root(y2, p)));
                }
                n += 4;
            }
            out
        })
        .collect();

    let mut out = Vec::with_capacity(limit / 16);
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
    let sqrt_n = (N as f64).sqrt() as usize + 2;

    let residues = compute_residues(sqrt_n);
    let (small, mut large): (Vec<_>, Vec<_>) =
        residues.into_iter().partition(|&(p, _, _)| p < SEQ_P);
    // Largest p first so rayon's range splits give light work to the
    // left; stealers take the heavy small-p tail.
    large.reverse();

    let mut sieve = vec![1u8; LIMIT];
    sieve[0] = 0;

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

    let ans: u32 = sieve.iter().map(|x| x.load(Ordering::Relaxed) as u32).sum();
    println!("{}", ans);
}
