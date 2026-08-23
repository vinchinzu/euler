// Project Euler 799 - Pentagonal Number Representations
// Two-pass sieve with Gaussian integer factorization to count representations.

use rayon::prelude::*;

const CLIMIT: usize = 28_000_000;
const PLIMIT: usize = 500_000;
const THRESHOLD: u16 = 1000;
const TARGET: i32 = 100;
const CHUNK: usize = 1 << 18; // 512 KiB of u16, one L2 per core
const CAND_CHUNK: usize = 1024;
const MAX_F: usize = 20;

struct PrimeInfo {
    p: u32,
    r1: u32,
    r2: u32,
    p2: u64,
    l1: u64,
    l2: u64,
    a: u32,
    b: u32,
}

struct CandInfo {
    c: i64,
    remaining: i64,
    nf: u8,
    pidx: [u16; MAX_F],
    pexp: [u8; MAX_F],
}

#[inline(always)]
fn mod_pow_u64(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

#[inline(always)]
fn mod_pow_u128(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % m as u128) as u64;
        }
        base = ((base as u128 * base as u128) % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn sqrt_mod_small(n: u64, p: u64) -> u64 {
    let n = n % p;
    if n == 0 {
        return 0;
    }
    if mod_pow_u64(n, (p - 1) / 2, p) != 1 {
        return u64::MAX;
    }
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    if s == 1 {
        return mod_pow_u64(n, (p + 1) / 4, p);
    }
    let mut z = 2u64;
    while mod_pow_u64(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut mm = s;
    let mut c = mod_pow_u64(z, q, p);
    let mut t = mod_pow_u64(n, q, p);
    let mut r = mod_pow_u64(n, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 1u32;
        let mut temp = t * t % p;
        while temp != 1 {
            if i >= mm {
                return u64::MAX;
            }
            temp = temp * temp % p;
            i += 1;
        }
        let b = mod_pow_u64(c, 1u64 << (mm - i - 1), p);
        mm = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }
    r
}

fn sqrt_mod_big(n: u64, p: u64) -> u64 {
    let n = n % p;
    if n == 0 {
        return 0;
    }
    if mod_pow_u128(n, (p - 1) / 2, p) != 1 {
        return u64::MAX;
    }
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    if s == 1 {
        return mod_pow_u128(n, (p + 1) / 4, p);
    }
    let mut z = 2u64;
    while mod_pow_u128(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut mm = s;
    let mut c = mod_pow_u128(z, q, p);
    let mut t = mod_pow_u128(n, q, p);
    let mut r = mod_pow_u128(n, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 1u32;
        let mut temp = ((t as u128 * t as u128) % p as u128) as u64;
        while temp != 1 {
            if i >= mm {
                return u64::MAX;
            }
            temp = ((temp as u128 * temp as u128) % p as u128) as u64;
            i += 1;
        }
        let b = mod_pow_u128(c, 1u64 << (mm - i - 1), p);
        mm = i;
        c = ((b as u128 * b as u128) % p as u128) as u64;
        t = ((t as u128 * c as u128) % p as u128) as u64;
        r = ((r as u128 * b as u128) % p as u128) as u64;
    }
    r
}

fn cornacchia(p: u64, sqrt_m1: u64) -> (u64, u64) {
    let mut y = if sqrt_m1 > p / 2 { p - sqrt_m1 } else { sqrt_m1 };
    let mut x = p;
    while (y as u128) * (y as u128) >= p as u128 {
        let t = x % y;
        x = y;
        y = t;
    }
    let rem = p - y * y;
    let s = rem.isqrt();
    if s * s == rem {
        return (y.min(s), y.max(s));
    }
    // Only small primes should hit this fallback.
    let lim = p.isqrt().min(2_000_000);
    for a in 1..=lim {
        let bsq = p - a * a;
        let b = bsq.isqrt();
        if b * b == bsq {
            return (a, b);
        }
    }
    (0, 0)
}

fn lift_root(r: i64, p: i64) -> u64 {
    let f = 18 * r * r - 6 * r + 1;
    let fdiv = f / p;
    let mut fp = (36 * r - 6) % p;
    if fp < 0 {
        fp += p;
    }
    let inv = mod_pow_u64(fp as u64, (p - 2) as u64, p as u64) as i64;
    let t = (p - fdiv % p) * inv % p;
    (r + t * p) as u64
}

#[inline(always)]
fn mul3_ap(chunk: &mut [u16], base: usize, end: usize, mut c: usize, p: usize) {
    if c < 2 {
        c += p;
    }
    if c < base {
        c += (base - c + p - 1) / p * p;
    }
    while c < end {
        // SAFETY: base <= c < end maps into this chunk
        unsafe {
            let slot = chunk.get_unchecked_mut(c - base);
            *slot = slot.saturating_mul(3);
        }
        c += p;
    }
}

#[inline(always)]
fn correct_hp(chunk: &mut [u16], base: usize, end: usize, lift: u64, p2: u64, p: u64) {
    if p2 == 0 {
        return;
    }
    let mut c = lift;
    if c < 2 {
        c += p2;
    }
    let b64 = base as u64;
    let e64 = end as u64;
    if c < b64 {
        c += (b64 - c + p2 - 1) / p2 * p2;
    }
    let pu = p as i64;
    while c < e64 {
        let cc = c as i64;
        let t = 6 * cc - 1;
        let mut m = (t * t + 1) / 2;
        let mut e = 0u32;
        while m % pu == 0 {
            m /= pu;
            e += 1;
        }
        // SAFETY: base <= c < end maps into this chunk
        unsafe {
            let slot = chunk.get_unchecked_mut(c as usize - base);
            if *slot != u16::MAX && e >= 2 {
                let v = *slot as u32 / 3 * (2 * e + 1);
                *slot = if v > 65535 { 65535 } else { v as u16 };
            }
        }
        c += p2;
    }
}

fn factor_ap(
    slice: &mut [CandInfo],
    r_arr: &[u16],
    lo: usize,
    hi: usize,
    mut c: usize,
    p: usize,
    pidx: usize,
) {
    if c < 2 {
        c += p;
    }
    if c < lo {
        c += (lo - c + p - 1) / p * p;
    }
    let pp = p as i64;
    while c < hi {
        // SAFETY: c < hi <= CLIMIT; r_arr.len() == CLIMIT
        let r = unsafe { *r_arr.get_unchecked(c) };
        if r >= THRESHOLD {
            if let Ok(i) = slice.binary_search_by_key(&(c as i64), |x| x.c) {
                let info = &mut slice[i];
                if info.remaining % pp == 0 {
                    let mut e = 0u8;
                    while info.remaining % pp == 0 {
                        info.remaining /= pp;
                        e += 1;
                    }
                    let nf = info.nf as usize;
                    info.pidx[nf] = pidx as u16;
                    info.pexp[nf] = e;
                    info.nf += 1;
                }
            }
        }
        c += p;
    }
}

#[inline(always)]
fn gmul(re: i64, im: i64, a: i64, b: i64) -> (i64, i64) {
    (re * a - im * b, re * b + im * a)
}

#[inline(always)]
fn check_xy(x: i64, y: i64, cnt: &mut i32) {
    if x > 0 && y > 0 && x <= y && x % 6 == 5 && y % 6 == 5 {
        *cnt += 1;
    }
}

#[inline(always)]
fn check_unit(re: i64, im: i64, cnt: &mut i32) {
    check_xy(re, im, cnt);
    check_xy(-im, re, cnt);
    check_xy(-re, -im, cnt);
    check_xy(im, -re, cnt);
}

fn enumerate(pf: &[(i64, i64, i32)], idx: usize, re: i64, im: i64, cnt: &mut i32) {
    if *cnt > TARGET {
        return;
    }
    if idx == pf.len() {
        check_unit(re, im, cnt);
        return;
    }
    let (a, b, e) = pf[idx];
    if e == 1 {
        let n1 = gmul(re, im, a, -b);
        enumerate(pf, idx + 1, n1.0, n1.1, cnt);
        if *cnt > TARGET {
            return;
        }
        let n2 = gmul(re, im, a, b);
        enumerate(pf, idx + 1, n2.0, n2.1, cnt);
        return;
    }
    let el = e as usize;
    let mut minus_pow = [(0i64, 0i64); 32];
    minus_pow[0] = (1, 0);
    for k in 1..=el {
        minus_pow[k] = gmul(minus_pow[k - 1].0, minus_pow[k - 1].1, a, -b);
    }
    let mut plus = (1i64, 0i64);
    for j in 0..=el {
        let f = gmul(plus.0, plus.1, minus_pow[el - j].0, minus_pow[el - j].1);
        let n = gmul(re, im, f.0, f.1);
        enumerate(pf, idx + 1, n.0, n.1, cnt);
        if *cnt > TARGET {
            return;
        }
        plus = gmul(plus.0, plus.1, a, b);
    }
}

fn count_reps(info: &CandInfo, gauss_a: &[u32], gauss_b: &[u32]) -> i32 {
    let mut pfactors = [(0i64, 0i64, 0i32); 24];
    let mut n = 0usize;
    for i in 0..info.nf as usize {
        let pi = info.pidx[i] as usize;
        pfactors[n] = (gauss_a[pi] as i64, gauss_b[pi] as i64, info.pexp[i] as i32);
        n += 1;
    }
    if info.remaining > 1 {
        let rem = info.remaining as u64;
        let sq = sqrt_mod_big(rem - 1, rem);
        let (a, b) = if sq == u64::MAX {
            cornacchia(rem, 0)
        } else {
            cornacchia(rem, sq)
        };
        pfactors[n] = (a as i64, b as i64, 1);
        n += 1;
    }
    let mut cnt = 0i32;
    enumerate(&pfactors[..n], 0, 1, 1, &mut cnt);
    cnt
}

fn main() {
    let mut is_prime = vec![true; PLIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2usize;
    while i * i <= PLIMIT {
        if is_prime[i] {
            let mut j = i * i;
            while j <= PLIMIT {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes1: Vec<u32> = (5..=PLIMIT)
        .filter(|&i| is_prime[i] && i % 4 == 1)
        .map(|i| i as u32)
        .collect();

    let infos: Vec<PrimeInfo> = primes1
        .par_iter()
        .map(|&p| {
            let pu = p as u64;
            let sq = sqrt_mod_small(pu - 1, pu);
            if sq == u64::MAX {
                return PrimeInfo {
                    p,
                    r1: u32::MAX,
                    r2: u32::MAX,
                    p2: pu * pu,
                    l1: 0,
                    l2: 0,
                    a: 0,
                    b: 0,
                };
            }
            let inv6 = mod_pow_u64(6, pu - 2, pu);
            let r1 = ((1 + sq) % pu) * inv6 % pu;
            let r2 = ((1 + pu - sq % pu) % pu) * inv6 % pu;
            let (a, b) = cornacchia(pu, sq);
            let l1 = lift_root(r1 as i64, p as i64);
            let l2 = lift_root(r2 as i64, p as i64);
            PrimeInfo {
                p,
                r1: r1 as u32,
                r2: r2 as u32,
                p2: pu * pu,
                l1,
                l2,
                a: a as u32,
                b: b as u32,
            }
        })
        .collect();

    let mut r_arr = vec![1u16; CLIMIT];
    r_arr
        .par_chunks_mut(CHUNK)
        .enumerate()
        .for_each(|(ci, chunk)| {
            let base = ci * CHUNK;
            let end = base + chunk.len();
            for info in &infos {
                if info.r1 == u32::MAX {
                    continue;
                }
                let p = info.p as usize;
                mul3_ap(chunk, base, end, info.r1 as usize, p);
                if info.r2 != info.r1 {
                    mul3_ap(chunk, base, end, info.r2 as usize, p);
                }
                correct_hp(chunk, base, end, info.l1, info.p2, info.p as u64);
                if info.l2 != info.l1 {
                    correct_hp(chunk, base, end, info.l2, info.p2, info.p as u64);
                }
            }
        });

    let mut cands: Vec<CandInfo> = Vec::with_capacity(150_000);
    for c in 2..CLIMIT {
        // SAFETY: c < CLIMIT
        if unsafe { *r_arr.get_unchecked(c) } >= THRESHOLD {
            let cc = c as i64;
            let t = 6 * cc - 1;
            cands.push(CandInfo {
                c: cc,
                remaining: (t * t + 1) / 2,
                nf: 0,
                pidx: [0; MAX_F],
                pexp: [0; MAX_F],
            });
        }
    }

    cands.par_chunks_mut(CAND_CHUNK).for_each(|slice| {
        if slice.is_empty() {
            return;
        }
        let lo = slice[0].c as usize;
        let hi = slice.last().unwrap().c as usize + 1;
        for (idx, info) in infos.iter().enumerate() {
            if info.r1 == u32::MAX {
                continue;
            }
            let p = info.p as usize;
            factor_ap(slice, &r_arr, lo, hi, info.r1 as usize, p, idx);
            if info.r2 != info.r1 {
                factor_ap(slice, &r_arr, lo, hi, info.r2 as usize, p, idx);
            }
        }
    });

    let gauss_a: Vec<u32> = infos.iter().map(|p| p.a).collect();
    let gauss_b: Vec<u32> = infos.iter().map(|p| p.b).collect();

    if let Some(info) = cands.par_iter().find_first(|info| count_reps(info, &gauss_a, &gauss_b) > TARGET)
    {
        let c = info.c;
        println!("{}", c * (3 * c - 1) / 2);
    } else {
        println!("Not found");
    }
}
