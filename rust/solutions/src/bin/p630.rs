// Project Euler 630 - Crossed Lines
// BBS points; unique lines as packed (slope, intercept); ans = T^2 - sum c_s^2.

use rayon::prelude::*;

const NPTS: usize = 2500;
const L: i64 = 2000;

#[inline(always)]
fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Pack reduced (dx, dy, intercept). Coincident points return 0.
/// Slope in the high 32 bits so sort groups by slope then intercept.
#[inline(always)]
fn pack_line(pxi: i32, pyi: i32, pxj: i32, pyj: i32) -> u64 {
    let mut dx = pxj - pxi;
    let mut dy = pyj - pyi;
    if dx == 0 && dy == 0 {
        return 0;
    }
    if dy < 0 || (dy == 0 && dx < 0) {
        dx = -dx;
        dy = -dy;
    }
    let g = gcd_u32(dx.unsigned_abs(), dy as u32);
    dx /= g as i32;
    dy /= g as i32;
    // |dx|,|dy| <= 1999, coords in [-1000,999] => intercept fits in i32
    let intercept = dy * pxi - dx * pyi;
    let slope = ((dy as u32) << 16) | (dx as u16 as u32);
    ((slope as u64) << 32) | (intercept as u32 as u64)
}

fn pair_offset(i: usize) -> usize {
    i * (2 * NPTS - i - 1) / 2
}

fn fill_pairs(out: &mut [u64], i0: usize, i1: usize, px: &[i32], py: &[i32]) {
    if i0 >= i1 {
        return;
    }
    let n_i = i1 - i0;
    if n_i == 1 || out.len() <= 16_384 {
        let base = pair_offset(i0);
        for i in i0..i1 {
            let pxi = unsafe { *px.get_unchecked(i) };
            let pyi = unsafe { *py.get_unchecked(i) };
            let mut slot = pair_offset(i) - base;
            for j in i + 1..NPTS {
                let packed = pack_line(
                    pxi,
                    pyi,
                    unsafe { *px.get_unchecked(j) },
                    unsafe { *py.get_unchecked(j) },
                );
                unsafe {
                    *out.get_unchecked_mut(slot) = packed;
                }
                slot += 1;
            }
        }
        return;
    }
    let mid = i0 + n_i / 2;
    let split = pair_offset(mid) - pair_offset(i0);
    let (lo, hi) = out.split_at_mut(split);
    rayon::join(
        || fill_pairs(lo, i0, mid, px, py),
        || fill_pairs(hi, mid, i1, px, py),
    );
}

fn main() {
    let mut s: i64 = 290797;
    let mut px = [0i32; NPTS];
    let mut py = [0i32; NPTS];
    for i in 0..NPTS {
        s = s * s % 50515093;
        px[i] = (s % L - 1000) as i32;
        s = s * s % 50515093;
        py[i] = (s % L - 1000) as i32;
    }

    let n_pairs = NPTS * (NPTS - 1) / 2;
    let mut pairs = vec![0u64; n_pairs];
    fill_pairs(&mut pairs, 0, NPTS, &px, &py);
    pairs.par_sort_unstable();

    let n = pairs.len();
    let mut idx = 0usize;
    // packed 0 is coincident points only (dx = dy = 0)
    while idx < n && unsafe { *pairs.get_unchecked(idx) } == 0 {
        idx += 1;
    }

    let mut total = 0i64;
    let mut sum_sq = 0i64;
    while idx < n {
        let first = unsafe { *pairs.get_unchecked(idx) };
        let slope = first >> 32;
        let mut prev = first;
        let mut c = 1i64;
        idx += 1;
        while idx < n {
            let v = unsafe { *pairs.get_unchecked(idx) };
            if v >> 32 != slope {
                break;
            }
            if v != prev {
                c += 1;
                prev = v;
            }
            idx += 1;
        }
        total += c;
        sum_sq += c * c;
    }

    println!("{}", total * total - sum_sq);
}
