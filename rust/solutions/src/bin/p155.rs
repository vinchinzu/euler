// Project Euler 155 - Counting Capacitor Circuits
// Distinct capacitances with N=18 unit capacitors.
// Reduced p/q satisfy p,q <= Fib(19)=4181 (OEIS A048211); packed as u16.

use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

const MAXN: usize = 18;
const SHIFT: usize = 13; // 8192 > Fib(19)
const DIM: usize = 1 << SHIFT;
const WORDS: usize = DIM * DIM / 64;

#[inline(always)]
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline(always)]
fn bit_loc(n: u32, d: u32) -> (usize, u64) {
    let idx = ((n as usize) << SHIFT) | (d as usize);
    (idx >> 6, 1u64 << (idx & 63))
}

// SAFETY: AtomicU64 is #[repr(transparent)] over u64.
fn atomic_slice(bits: &[u64]) -> &[AtomicU64] {
    unsafe { std::slice::from_raw_parts(bits.as_ptr().cast::<AtomicU64>(), bits.len()) }
}

#[inline(always)]
fn try_set(bits: &[AtomicU64], n: u32, d: u32) -> bool {
    let (w, m) = bit_loc(n, d);
    // SAFETY: n,d < DIM ⇒ word < WORDS
    let prev = unsafe { bits.get_unchecked(w).fetch_or(m, Ordering::Relaxed) };
    prev & m == 0
}

#[inline(always)]
fn add_pair(n1: u32, d1: u32, n2: u32, d2: u32, bits: &[AtomicU64], out: &mut Vec<u32>) {
    // n1,d1,n2,d2 <= 4181 ⇒ products fit in u32
    let num = n1 * d2 + n2 * d1;
    let den = d1 * d2;
    let g = gcd(num, den);
    let sn = num / g;
    let sd = den / g;
    if try_set(bits, sn, sd) {
        out.push((sn << 16) | sd);
    }
    if sn != sd && try_set(bits, sd, sn) {
        out.push((sd << 16) | sn);
    }
}

fn main() {
    let nthreads = rayon::current_num_threads().max(1);
    let mut exact: Vec<Vec<u32>> = vec![Vec::new(); MAXN + 1];
    exact[1].push((1u32 << 16) | 1);

    let mut bits = vec![0u64; WORDS];
    let mut all_bits = vec![0u64; WORDS];
    {
        let (w, m) = bit_loc(1, 1);
        all_bits[w] |= m;
    }

    for k in 2..=MAXN {
        bits.fill(0);
        let cap = if k >= 16 { 1 << 18 } else { 256 };
        let parts: Vec<Vec<u32>> = {
            let abits = atomic_slice(&bits);
            let exact_ref = &exact;
            (0..nthreads)
                .into_par_iter()
                .map(|tid| {
                    let mut local = Vec::with_capacity(cap);
                    for i in 1..=k / 2 {
                        let l1 = &exact_ref[i];
                        let l2 = &exact_ref[k - i];
                        if i == k - i {
                            let mut a = tid;
                            while a < l1.len() {
                                // SAFETY: a < l1.len()
                                let x = unsafe { *l1.get_unchecked(a) };
                                let n1 = x >> 16;
                                let d1 = x & 0xffff;
                                for b in a..l1.len() {
                                    let y = unsafe { *l1.get_unchecked(b) };
                                    add_pair(n1, d1, y >> 16, y & 0xffff, abits, &mut local);
                                }
                                a += nthreads;
                            }
                        } else {
                            let (outer, inner) = if l1.len() >= l2.len() {
                                (l1.as_slice(), l2.as_slice())
                            } else {
                                (l2.as_slice(), l1.as_slice())
                            };
                            let mut idx = tid;
                            while idx < outer.len() {
                                let x = unsafe { *outer.get_unchecked(idx) };
                                let n1 = x >> 16;
                                let d1 = x & 0xffff;
                                for &y in inner {
                                    add_pair(n1, d1, y >> 16, y & 0xffff, abits, &mut local);
                                }
                                idx += nthreads;
                            }
                        }
                    }
                    local
                })
                .collect()
        };

        let mut found_len = 0usize;
        for p in &parts {
            found_len += p.len();
        }
        let mut found = Vec::with_capacity(found_len);
        for mut p in parts {
            found.append(&mut p);
        }

        for (dst, src) in all_bits.iter_mut().zip(bits.iter()) {
            *dst |= *src;
        }
        exact[k] = found;
    }

    let ans: u64 = all_bits.iter().map(|w| w.count_ones() as u64).sum();
    println!("{ans}");
}
