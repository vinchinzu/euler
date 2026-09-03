// Project Euler 972 - Hyperbolic geodesics T(12) = 3575508
// Count ordered triples of V(12) points on a common geodesic
// (diameter or circle orthogonal to unit disc)
//
// For each pair of points, compute the unique geodesic.
// Count pairs per geodesic, then recover s from C(s,2) = pairs.
// Sum s*(s-1)*(s-2) over all geodesics with s >= 3.
//
// Geodesics are packed into 128-bit keys:
// - Diameters (through origin) have den = 0 and use a unique diam_index in the lower bits.
// - Orthogonal circles have den > 0 and store (den, h, k) in canonical lowest terms.
// All keys are written directly into a pre-allocated buffer across Rayon chunks,
// sorted in parallel with par_sort_unstable(), and multiplicity counted in a linear pass.

use rayon::prelude::*;

#[inline(always)]
fn gcd_u32(mut u: u32, mut v: u32) -> u32 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    let shift = (u | v).trailing_zeros();
    u >>= u.trailing_zeros();
    while v != 0 {
        v >>= v.trailing_zeros();
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
    }
    u << shift
}

const DABS: i32 = 144;
const DSTRIDE: i32 = 2 * DABS + 1;

#[inline(always)]
fn diam_index(mut da: i32, mut db: i32) -> u32 {
    let g = gcd_u32(da.unsigned_abs(), db.unsigned_abs());
    if g == 0 {
        return 0;
    }
    da /= g as i32;
    db /= g as i32;
    if da < 0 || (da == 0 && db < 0) {
        da = -da;
        db = -db;
    }
    (da * DSTRIDE + db + DABS + 1) as u32
}

#[derive(Clone, Copy)]
struct PrePoint {
    s_num: i32,
    ad: i32,
    bd: i32,
    diam: u32,
}

#[inline(always)]
fn triples_from_pairs(pairs: u64) -> i64 {
    let sp = 2 * pairs;
    let disc = 1 + 4 * sp;
    let sqrt_disc = (disc as f64).sqrt() as u64;
    let s = sqrt_disc.div_ceil(2);
    if s * (s - 1) == sp && s >= 3 {
        (sp * (s - 2)) as i64
    } else {
        0
    }
}

fn main() {
    let n = 12i32;

    let mut rats: Vec<(i32, i32)> = vec![(0, 1)];
    for q in 1..=n {
        for p in (-(q - 1))..=q - 1 {
            if p == 0 {
                continue;
            }
            if gcd_u32(p.unsigned_abs(), q as u32) == 1 {
                rats.push((p, q));
            }
        }
    }

    let mut pts: Vec<PrePoint> = Vec::new();
    for &(xn, xd) in &rats {
        for &(yn, yd) in &rats {
            let x2 = xn * xn * yd * yd;
            let y2 = yn * yn * xd * xd;
            let r2 = xd * xd * yd * yd;
            if x2 + y2 < r2 {
                let a = xn * yd;
                let b = yn * xd;
                let d = xd * yd;
                pts.push(PrePoint {
                    s_num: a * a + b * b + d * d,
                    ad: a * d,
                    bd: b * d,
                    diam: if a == 0 && b == 0 { 0 } else { diam_index(a, b) },
                });
            }
        }
    }

    let npts = pts.len();
    let pts = pts.as_slice();

    // Create load-balanced work chunks.
    // Point i has (npts - i - 1) pairs.
    let total_pairs: usize = npts * (npts - 1) / 2;
    let num_threads = rayon::current_num_threads().max(1);
    let target_per_chunk = total_pairs / (num_threads * 2) + 1;

    let mut chunks: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut chunk_start = 0;
    let mut chunk_pairs = 0usize;
    let mut current_offset = 0usize;
    for i in 0..npts {
        chunk_pairs += npts - i - 1;
        if chunk_pairs >= target_per_chunk || i == npts - 1 {
            chunks.push((chunk_start, i + 1, current_offset, chunk_pairs));
            chunk_start = i + 1;
            current_offset += chunk_pairs;
            chunk_pairs = 0;
        }
    }

    let mut all_keys_uninit: Vec<std::mem::MaybeUninit<u128>> = Vec::with_capacity(total_pairs);
    unsafe { all_keys_uninit.set_len(total_pairs); }
    let all_keys_ptr = all_keys_uninit.as_mut_ptr() as usize;

    chunks.par_iter().for_each(|&(start, end, offset, len)| {
        let out = unsafe { std::slice::from_raw_parts_mut((all_keys_ptr as *mut u128).add(offset), len) };
        let mut out_idx = 0;
        for i in start..end {
            let pi = unsafe { *pts.get_unchecked(i) };
            let s1 = pi.s_num;
            let ad1 = pi.ad;
            let bd1 = pi.bd;
            let diam1 = pi.diam;
            for j in (i + 1)..npts {
                let pj = unsafe { *pts.get_unchecked(j) };
                let det = ad1 * pj.bd - bd1 * pj.ad;
                if det == 0 {
                    let idx = if diam1 != 0 { diam1 } else { pj.diam };
                    unsafe { *out.get_unchecked_mut(out_idx) = idx as u128; }
                    out_idx += 1;
                    continue;
                }

                let mut h = s1 * pj.bd - pj.s_num * bd1;
                let mut k = ad1 * pj.s_num - pj.ad * s1;
                let mut den = det * 2;

                let g1 = gcd_u32(h.unsigned_abs(), k.unsigned_abs());
                if g1 > 1 {
                    let g2 = gcd_u32(g1, den.unsigned_abs());
                    if g2 > 1 {
                        let g = g2 as i32;
                        h /= g;
                        k /= g;
                        den /= g;
                    }
                }
                if den < 0 {
                    h = -h;
                    k = -k;
                    den = -den;
                }

                let packed = ((den as u64 as u128) << 64) | ((h as u32 as u128) << 32) | (k as u32 as u128);
                unsafe { *out.get_unchecked_mut(out_idx) = packed; }
                out_idx += 1;
            }
        }
    });

    let mut all_keys: Vec<u128> = unsafe {
        let mut v = std::mem::ManuallyDrop::new(all_keys_uninit);
        Vec::from_raw_parts(v.as_mut_ptr() as *mut u128, v.len(), v.capacity())
    };
    all_keys.par_sort_unstable();

    let mut total: i64 = 0;
    if !all_keys.is_empty() {
        let mut prev = all_keys[0];
        let mut count = 1u64;
        for &k in &all_keys[1..] {
            if k == prev {
                count += 1;
            } else {
                if count >= 3 {
                    total += triples_from_pairs(count);
                }
                prev = k;
                count = 1;
            }
        }
        if count >= 3 {
            total += triples_from_pairs(count);
        }
    }

    println!("{}", total);
}
