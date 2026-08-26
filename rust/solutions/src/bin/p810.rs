// Project Euler 810 - XOR-Primes
// XOR-primes are irreducible polynomials over GF(2). Count I(n) to find the
// degree of the 5,000,000th, then Gray-code sieve that degree.

use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

const TARGET: u64 = 5_000_000;
const JOB_CHUNK: u64 = 1 << 17;

fn xor_mul(a: u64, b: u64) -> u64 {
    let mut r = 0u64;
    let mut x = a;
    while x != 0 {
        let bit = x & x.wrapping_neg();
        r ^= b * bit;
        x ^= bit;
    }
    r
}

fn poly_mod(mut a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    let db = 63 - b.leading_zeros();
    while a != 0 {
        let da = 63 - a.leading_zeros();
        if da < db {
            break;
        }
        a ^= b << (da - db);
    }
    a
}

fn is_irreducible_small(f: u64) -> bool {
    if f < 2 {
        return false;
    }
    let n = 63 - f.leading_zeros();
    if n == 0 {
        return false;
    }
    for d in 1..=n / 2 {
        let lo = 1u64 << d;
        let hi = 1u64 << (d + 1);
        for g in lo..hi {
            if poly_mod(f, g) == 0 {
                return false;
            }
        }
    }
    true
}

fn small_irreducibles(max_deg: u32) -> Vec<u64> {
    let hi = 1u64 << (max_deg + 1);
    let mut v = Vec::new();
    for f in 2..hi {
        if is_irreducible_small(f) {
            v.push(f);
        }
    }
    v
}

fn mu(n: u32) -> i32 {
    let mut x = n;
    let mut m = 1i32;
    if x.is_multiple_of(2) {
        x /= 2;
        m = -m;
        if x.is_multiple_of(2) {
            return 0;
        }
    }
    let mut p = 3u32;
    while p * p <= x {
        if x.is_multiple_of(p) {
            x /= p;
            m = -m;
            if x.is_multiple_of(p) {
                return 0;
            }
        }
        p += 2;
    }
    if x > 1 {
        m = -m;
    }
    m
}

fn irreducible_count(n: u32) -> u64 {
    let mut s: i64 = 0;
    let mut d = 1u32;
    while d * d <= n {
        if n.is_multiple_of(d) {
            s += mu(d) as i64 * (1i64 << (n / d));
            let d2 = n / d;
            if d2 != d {
                s += mu(d2) as i64 * (1i64 << (n / d2));
            }
        }
        d += 1;
    }
    (s / n as i64) as u64
}

struct Job {
    p: u64,
    k: u32,
    n0: u64,
    n1: u64,
}

fn apply_job(job: &Job, sieve: &[AtomicU64], high: u64) {
    let Job { p, k, n0, n1 } = *job;
    let gray = n0 ^ (n0 >> 1);
    let q = (1u64 << k) | 1 | (gray << 1);
    let mut product = xor_mul(p, q);
    // SAFETY: p ⊗ q is an odd degree-`deg` integer; idx = (product-high)/2 < 2^{deg-1}.
    unsafe {
        mark(sieve, product, high);
        let mut n = n0 + 1;
        while n < n1 {
            product ^= p << (n.trailing_zeros() + 1);
            mark(sieve, product, high);
            n += 1;
        }
    }
}

#[inline(always)]
unsafe fn mark(sieve: &[AtomicU64], product: u64, high: u64) {
    let idx = ((product - high) >> 1) as usize;
    // SAFETY: idx < n_odd = 64 * sieve.len() because product is an odd degree-`deg` integer.
    unsafe {
        sieve
            .get_unchecked(idx >> 6)
            .fetch_or(1u64 << (idx & 63), Ordering::Relaxed);
    }
}

fn main() {
    debug_assert_eq!(irreducible_count(1), 2);
    debug_assert_eq!(irreducible_count(5), 6);
    debug_assert_eq!(irreducible_count(26), 2_580_795);

    let mut cum = 0u64;
    let mut deg = 0u32;
    while cum < TARGET {
        deg += 1;
        cum += irreducible_count(deg);
    }
    let need = TARGET - (cum - irreducible_count(deg));

    if deg == 1 {
        println!("{}", if need == 1 { 2 } else { 3 });
        return;
    }

    let high = 1u64 << deg;
    let n_odd = 1usize << (deg - 1);
    let n_words = n_odd / 64;
    let max_factor_deg = deg / 2;
    let primes = small_irreducibles(max_factor_deg);

    let mut jobs = Vec::new();
    for &p in &primes {
        if p == 2 {
            continue;
        }
        let d = 63 - p.leading_zeros();
        let k = deg - d;
        if k == 0 {
            continue;
        }
        let variants = 1u64 << (k - 1);
        let mut n0 = 0u64;
        while n0 < variants {
            let n1 = (n0 + JOB_CHUNK).min(variants);
            jobs.push(Job { p, k, n0, n1 });
            n0 = n1;
        }
    }

    let sieve: Vec<AtomicU64> = (0..n_words).map(|_| AtomicU64::new(0)).collect();
    jobs.into_par_iter().for_each(|job| apply_job(&job, &sieve, high));

    let mut seen = 0u64;
    for w in 0..n_words {
        let unmarked = !sieve[w].load(Ordering::Relaxed);
        let c = unmarked.count_ones() as u64;
        if seen + c < need {
            seen += c;
            continue;
        }
        let mut bits = unmarked;
        while bits != 0 {
            let b = bits.trailing_zeros();
            bits &= bits - 1;
            seen += 1;
            if seen == need {
                let idx = ((w as u64) << 6) | b as u64;
                println!("{}", high + 2 * idx + 1);
                return;
            }
        }
    }
}
