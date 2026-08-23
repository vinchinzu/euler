// Project Euler 659 - Largest prime
// P(k) = largest prime dividing (2k)^2+1.
// Parallel Tonelli-Shanks of -1 mod p (p ≡ 1 mod 4), then chunked exact-div.

use rayon::prelude::*;

const N: usize = 10_000_000;
const M: u64 = 1_000_000_000_000_000_000;
const SEG: usize = 1 << 16;

#[derive(Clone, Copy)]
struct Job {
    p: u32,
    k1: u32,
    k2: u32,
    pinv: u64,
    maxq: u64,
}

#[inline(always)]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    a.wrapping_mul(b) % m
}

#[inline(always)]
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul_mod(r, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    r
}

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

/// sqrt(-1) mod p for prime p ≡ 1 (mod 4): z^{(p-1)/4} for non-residue z.
#[inline(always)]
fn sqrt_neg1(p: u64) -> u64 {
    if p & 7 == 5 {
        return pow_mod(2, (p - 1) >> 2, p);
    }
    let mut z = 3u64;
    while jacobi(z, p) != -1 {
        z += 2;
    }
    pow_mod(z, (p - 1) >> 2, p)
}

/// Modular inverse of odd `a` modulo 2^64 (Newton).
#[inline(always)]
fn inv64(a: u64) -> u64 {
    let mut x = a;
    x = x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x)));
    x = x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x)));
    x = x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x)));
    x = x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x)));
    x = x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x)));
    x
}

fn primes_upto(n: usize) -> Vec<u32> {
    if n < 2 {
        return Vec::new();
    }
    let mut c = vec![0u8; n + 1];
    let mut ps = Vec::with_capacity(n / 5);
    let sq = n.isqrt();
    for i in 2..=n {
        if c[i] == 0 {
            ps.push(i as u32);
            if i <= sq {
                let mut j = i * i;
                while j <= n {
                    c[j] = 1;
                    j += i;
                }
            }
        }
    }
    ps
}

fn collect_jobs(limit: usize) -> Vec<Job> {
    let small: Vec<u32> = primes_upto(limit.isqrt())
        .into_iter()
        .filter(|&p| p > 2)
        .collect();
    let n_seg = limit.div_ceil(SEG);
    let parts: Vec<Vec<Job>> = (0..n_seg)
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
            for &p in &small {
                let p = p as usize;
                let p2 = p * p;
                if p2 >= hi {
                    break;
                }
                let mut j = if p2 > lo { p2 } else { lo.div_ceil(p) * p };
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
            let mut n = if lo <= 5 {
                5
            } else {
                let r = lo & 3;
                lo + ((5 - r) & 3)
            };
            while n < hi {
                // SAFETY: n in [lo, hi)
                if unsafe { *comp.get_unchecked(n - lo) } == 0 {
                    let p = n as u64;
                    let sv = sqrt_neg1(p);
                    let inv2 = (p + 1) >> 1;
                    out.push(Job {
                        p: n as u32,
                        k1: ((sv * inv2) % p) as u32,
                        k2: (((p - sv) * inv2) % p) as u32,
                        pinv: inv64(p),
                        maxq: u64::MAX / p,
                    });
                }
                n += 4;
            }
            out
        })
        .collect();
    let mut jobs = Vec::with_capacity(limit / 8);
    for mut part in parts {
        jobs.append(&mut part);
    }
    jobs
}

#[inline(always)]
fn first_idx(start: u32, p: u32, lo: u32) -> usize {
    let k = if start >= lo {
        start
    } else {
        let rem = (lo - start) % p;
        if rem == 0 { lo } else { lo + (p - rem) }
    };
    (k - lo) as usize
}

#[inline(always)]
fn peel(slice: &mut [u64], p: usize, pinv: u64, maxq: u64, mut idx: usize, do_peel: bool) {
    let n = slice.len();
    if do_peel {
        while idx < n {
            // SAFETY: idx < n
            let v = unsafe { slice.get_unchecked_mut(idx) };
            let q = v.wrapping_mul(pinv);
            if q != 1 {
                *v = q;
                loop {
                    let q2 = v.wrapping_mul(pinv);
                    if q2 > maxq || q2 == 1 {
                        break;
                    }
                    *v = q2;
                }
            }
            idx += p;
        }
    } else {
        while idx < n {
            // SAFETY: idx < n
            let v = unsafe { slice.get_unchecked_mut(idx) };
            let q = v.wrapping_mul(pinv);
            if q != 1 {
                *v = q;
            }
            idx += p;
        }
    }
}

fn main() {
    let jobs = collect_jobs(2 * N);

    let mut p_arr = vec![0u64; N + 1];
    p_arr[1..].par_chunks_mut(SEG).enumerate().for_each(|(ci, slice)| {
        let lo = 1 + ci * SEG;
        let hi = lo + slice.len() - 1;
        for (i, v) in slice.iter_mut().enumerate() {
            let k = (lo + i) as u64;
            *v = (k * k << 2) + 1;
        }
        let lo32 = lo as u32;
        // p^2 cannot divide 4k^2+1 for k <= hi if p > 2*hi
        let peel_lim = ((2 * hi as u64) | 1) as u32;
        for job in jobs.iter() {
            let p = job.p;
            let pu = p as usize;
            let do_peel = p <= peel_lim;
            let i1 = first_idx(job.k1, p, lo32);
            if i1 < slice.len() {
                peel(slice, pu, job.pinv, job.maxq, i1, do_peel);
            }
            let i2 = first_idx(job.k2, p, lo32);
            if i2 < slice.len() {
                peel(slice, pu, job.pinv, job.maxq, i2, do_peel);
            }
        }
    });

    let mut ans = 0u64;
    for &x in &p_arr {
        ans += x;
        if ans >= M {
            ans -= M;
        }
    }
    println!("{}", ans);
}
