// Project Euler 803 - Pseudorandom Sequence
// LCG: a_{n+1} = (25214903917 * a_n + 11) mod 2^48
// Find index of first occurrence of "LuckyText" given sequence starts with "PuzzleOne"

use rayon::prelude::*;
use std::sync::atomic::{AtomicI64, Ordering};

const MASK: u64 = (1u64 << 48) - 1;
const L: u64 = 1u64 << 16;
const MULT: u64 = 25214903917;
const INC: u64 = 11;
const MAX_L_STEPS: i64 = 1 << 28;

#[inline]
fn char_to_code(c: u8) -> u64 {
    if c >= b'a' && c <= b'z' {
        (c - b'a') as u64
    } else {
        (c - b'A') as u64 + 26
    }
}

#[inline]
fn next_val(a: u64) -> u64 {
    (MULT.wrapping_mul(a).wrapping_add(INC)) & MASK
}

/// Affine map x -> (m*x + c) mod 2^48, raised to the k-th power.
#[inline]
fn affine_pow(mut m: u64, mut c: u64, mut k: u64) -> (u64, u64) {
    let mut rm = 1u64;
    let mut rc = 0u64;
    while k > 0 {
        if k & 1 == 1 {
            rc = (m.wrapping_mul(rc).wrapping_add(c)) & MASK;
            rm = m.wrapping_mul(rm) & MASK;
        }
        c = (m.wrapping_mul(c).wrapping_add(c)) & MASK;
        m = m.wrapping_mul(m) & MASK;
        k >>= 1;
    }
    (rm, rc)
}

fn find_r(codes: &[u64]) -> i32 {
    for r in 0..L as i32 {
        let mut a = r as u64;
        let mut good = true;
        for i in 1..codes.len() {
            a = next_val(a % L);
            if ((a / L + codes[i - 1] - codes[i]) % 4 + 4) % 4 != 0 {
                good = false;
                break;
            }
        }
        if good {
            return r;
        }
    }
    -1
}

#[inline]
fn is_substring(mut a: u64, codes: &[u64]) -> bool {
    if (a >> 16) % 52 != codes[0] {
        return false;
    }
    for &c in &codes[1..] {
        a = next_val(a);
        if (a >> 16) % 52 != c {
            return false;
        }
    }
    true
}

fn main() {
    let s_str = b"PuzzleOne";
    let t_str = b"LuckyText";
    let s: Vec<u64> = s_str.iter().map(|&c| char_to_code(c)).collect();
    let t: Vec<u64> = t_str.iter().map(|&c| char_to_code(c)).collect();

    // Find starting value a such that sequence starts with S
    let r_s = find_r(&s);
    let mut a = s[0] * L + r_s as u64;
    while !is_substring(a, &s) {
        a += 52 * L;
    }

    // Find remainder for T
    let r_t = find_r(&t);
    let mut ans: i64 = 0;
    while (a % L) as i32 != r_t {
        a = next_val(a);
        ans += 1;
    }

    let (step_mult, step_add) = affine_pow(MULT, INC, L);

    let nthreads = rayon::current_num_threads().max(1);
    let (jump_m, jump_c) = affine_pow(step_mult, step_add, nthreads as u64);
    let jump_n = (nthreads as i64) * (L as i64);
    let t0 = t[0];
    let found = AtomicI64::new(i64::MAX);
    let start_a = a;
    let start_ans = ans;
    let limit = start_ans + MAX_L_STEPS * (L as i64);

    (0..nthreads).into_par_iter().for_each(|tid| {
        let (m_t, c_t) = affine_pow(step_mult, step_add, tid as u64);
        let mut cur = (m_t.wrapping_mul(start_a).wrapping_add(c_t)) & MASK;
        let mut n = start_ans + (tid as i64) * (L as i64);
        while n < limit {
            if n >= found.load(Ordering::Relaxed) {
                break;
            }
            if (cur >> 16) % 52 == t0 && is_substring(cur, &t) {
                found.fetch_min(n, Ordering::Relaxed);
                break;
            }
            cur = (jump_m.wrapping_mul(cur).wrapping_add(jump_c)) & MASK;
            n += jump_n;
        }
    });

    println!("{}", found.load(Ordering::Relaxed));
}
