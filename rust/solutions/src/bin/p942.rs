// Problem 942
// Gauss sum G = sum_{a=1}^{q-1} (a/q) 2^a  (mod 1e9+7).
// Packed QR bitset, 64-wide signed geometric accumulation, large-chunk rayon.

use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

const MOD: u64 = 1_000_000_007;

const TWO64_MOD: u64 = {
    let r = (1u64 << 32) % MOD;
    r * r % MOD
};
const TWO64_M1_MOD: u64 = TWO64_MOD - 1;
const TWO256_MOD: u64 = {
    let t2 = TWO64_MOD * TWO64_MOD % MOD;
    t2 * t2 % MOD
};

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

#[inline(always)]
fn add_mod(a: u64, b: u64) -> u64 {
    let t = a + b;
    if t >= MOD { t - MOD } else { t }
}

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mul_mod(res, base);
        }
        base = mul_mod(base, base);
        exp >>= 1;
    }
    res
}

/// (2*w - (2^n - 1)) mod MOD for a full 64-bit residue mask.
#[inline(always)]
fn signed_64(w: u64) -> u64 {
    let twice = (w % MOD) * 2;
    let m = TWO64_M1_MOD;
    if twice >= m {
        let t = twice - m;
        if t >= MOD { t - MOD } else { t }
    } else {
        twice + MOD - m
    }
}

/// Same as signed_64 for the low `n` bits of `w` (1 <= n <= 63).
#[inline(always)]
fn signed_n(w: u64, n: u32) -> u64 {
    let mask = (1u64 << n) - 1;
    let twice = ((w & mask) % MOD) * 2;
    let m = mask % MOD;
    if twice >= m {
        let t = twice - m;
        if t >= MOD { t - MOD } else { t }
    } else {
        twice + MOD - m
    }
}

#[inline(always)]
fn add_contrib(s: &mut u64, pow2: u64, inner: u64) {
    *s = add_mod(*s, mul_mod(pow2, inner));
}

fn accum_range(words: &[u64], start: usize, end: usize, mut pow2: u64) -> u64 {
    debug_assert!(start < end);
    let mut s = 0u64;
    let mut a = start;

    let r = a & 63;
    if r != 0 {
        let n = (64 - r).min(end - a);
        let w = unsafe { *words.get_unchecked(a >> 6) } >> r;
        add_contrib(&mut s, pow2, signed_n(w, n as u32));
        pow2 = mul_mod(pow2, (1u64 << n) % MOD);
        a += n;
        if a == end {
            return s;
        }
    }

    let end_words = end >> 6;
    let mut wi = a >> 6;

    if wi + 4 <= end_words {
        let t256 = TWO256_MOD;
        let t64 = TWO64_MOD;
        let mut p0 = pow2;
        let mut p1 = mul_mod(p0, t64);
        let mut p2 = mul_mod(p1, t64);
        let mut p3 = mul_mod(p2, t64);

        while wi + 4 <= end_words {
            let w0 = unsafe { *words.get_unchecked(wi) };
            let w1 = unsafe { *words.get_unchecked(wi + 1) };
            let w2 = unsafe { *words.get_unchecked(wi + 2) };
            let w3 = unsafe { *words.get_unchecked(wi + 3) };

            add_contrib(&mut s, p0, signed_64(w0));
            add_contrib(&mut s, p1, signed_64(w1));
            add_contrib(&mut s, p2, signed_64(w2));
            add_contrib(&mut s, p3, signed_64(w3));

            p0 = mul_mod(p0, t256);
            p1 = mul_mod(p1, t256);
            p2 = mul_mod(p2, t256);
            p3 = mul_mod(p3, t256);
            wi += 4;
        }
        pow2 = p0;
    }

    while wi < end_words {
        let w = unsafe { *words.get_unchecked(wi) };
        add_contrib(&mut s, pow2, signed_64(w));
        pow2 = mul_mod(pow2, TWO64_MOD);
        wi += 1;
    }

    a = wi << 6;
    if a < end {
        let n = end - a;
        let w = unsafe { *words.get_unchecked(wi) };
        add_contrib(&mut s, pow2, signed_n(w, n as u32));
    }
    s
}

fn fill_qr_bitset(q: usize) -> Vec<u64> {
    let nwords = (q + 63) >> 6;
    let half = (q - 1) / 2;
    let nt = rayon::current_num_threads().max(1);

    let mut words: Vec<AtomicU64> = Vec::with_capacity(nwords);
    unsafe {
        words.as_mut_ptr().write_bytes(0, nwords);
        words.set_len(nwords);
    }

    (0..nt).into_par_iter().for_each(|t| {
        let start_i = 1 + t * half / nt;
        let end_i = 1 + (t + 1) * half / nt;
        if start_i >= end_i {
            return;
        }
        let q64 = q as u64;
        let mut sq = (start_i as u64 * start_i as u64 % q64) as usize;
        let mut delta = 2 * start_i + 1;
        for _ in start_i..end_i {
            unsafe {
                words
                    .get_unchecked(sq >> 6)
                    .fetch_or(1u64 << (sq & 63), Ordering::Relaxed);
            }
            sq += delta;
            if sq >= q {
                sq -= q;
            }
            delta += 2;
        }
    });

    unsafe {
        let mut words = std::mem::ManuallyDrop::new(words);
        Vec::from_raw_parts(words.as_mut_ptr().cast(), words.len(), words.capacity())
    }
}

fn gauss_sum_mod(q: usize) -> u64 {
    let words = fill_qr_bitset(q);
    let nwords = words.len();
    let nt = rayon::current_num_threads().min(nwords).max(1);

    (0..nt)
        .into_par_iter()
        .map(|t| {
            let lo = t * nwords / nt;
            let hi = (t + 1) * nwords / nt;
            if lo >= hi {
                return 0;
            }
            let start = if lo == 0 { 1 } else { lo << 6 };
            let end = (hi << 6).min(q);
            if start >= end {
                return 0;
            }
            accum_range(&words, start, end, mod_pow(2, start as u64))
        })
        .reduce(|| 0, add_mod)
}

fn legendre_minus_two_prime(q: usize) -> i8 {
    match q & 7 {
        1 | 3 => 1,
        5 | 7 => -1,
        _ => unreachable!("q is odd prime"),
    }
}

fn r_mod(q: usize) -> u64 {
    debug_assert!(q >= 3 && q % 2 == 1, "q must be an odd prime >= 3");

    let g = gauss_sum_mod(q);
    let p_mod = (mod_pow(2, q as u64) + MOD - 1) % MOD;

    if legendre_minus_two_prime(q) == 1 {
        (p_mod + MOD - g) % MOD
    } else {
        g
    }
}

fn main() {
    assert_eq!(r_mod(5), 6);
    assert_eq!(r_mod(17), 47_569);

    let q = 74_207_281usize;
    println!("{}", r_mod(q));
}
