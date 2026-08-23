// Project Euler 972 - Hyperbolic geodesics T(12) = 3575508
// Count ordered triples of V(12) points on a common geodesic
// (diameter or circle orthogonal to unit disc)
//
// For each pair of points, compute the unique geodesic.
// Count pairs per geodesic, then recover s from C(s,2) = pairs.
// Sum s*(s-1)*(s-2) over all geodesics with s >= 3.
//
// Diameters (tag 0) are counted in a dense Vec; circles use a packed
// 12-byte key so HashMap traffic is 16 bytes/entry instead of 32+8.

use std::hash::{Hash, Hasher};
use fxhash::FxHashMap;
use rayon::prelude::*;

#[inline(always)]
fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// |a|,|b| <= 132 for N=12; 144 covers d=xd*yd as well.
const DABS: i32 = 144;
const DSTRIDE: i32 = 2 * DABS + 1;
const DIAM_LEN: usize = (DABS as usize + 1) * (DSTRIDE as usize) + 2;

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

// Packed orthogonal-circle key (h_num, k_num, den) after gcd + sign-norm.
// 12 bytes / align 4; with a u32 count the HashMap KV is 16 bytes.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct CircleKey {
    h: i32,
    k: i32,
    d: i32,
}

impl Hash for CircleKey {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64((self.h as u32 as u64) | ((self.k as u32 as u64) << 32));
        state.write_u32(self.d as u32);
    }
}

#[inline(always)]
fn triples_from_pairs(pairs: u64) -> i64 {
    if pairs < 3 {
        return 0;
    }
    let sp = 2 * pairs;
    let disc = 1 + 4 * sp;
    let sqrt_disc = (disc as f64).sqrt() as u64;
    let s = (1 + sqrt_disc) / 2;
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
    // Point i has (npts - i - 1) pairs. Lower i has more work.
    let total_pairs: usize = npts * (npts - 1) / 2;
    let num_threads = rayon::current_num_threads().max(1);
    let target_per_chunk = total_pairs / (num_threads * 2) + 1;

    let mut chunks: Vec<(usize, usize)> = Vec::new();
    let mut chunk_start = 0;
    let mut chunk_pairs = 0usize;
    for i in 0..npts {
        chunk_pairs += npts - i - 1;
        if chunk_pairs >= target_per_chunk || i == npts - 1 {
            chunks.push((chunk_start, i + 1));
            chunk_start = i + 1;
            chunk_pairs = 0;
        }
    }

    let chunk_results: Vec<(FxHashMap<CircleKey, u32>, Vec<u32>)> = chunks
        .par_iter()
        .map(|&(start, end)| {
            let mut circles =
                FxHashMap::<CircleKey, u32>::with_capacity_and_hasher(96_000, Default::default());
            let mut diams = vec![0u32; DIAM_LEN];
            for i in start..end {
                // SAFETY: i < npts guaranteed by chunk bounds
                let pi = unsafe { *pts.get_unchecked(i) };
                let s1 = pi.s_num;
                let ad1 = pi.ad;
                let bd1 = pi.bd;
                let diam1 = pi.diam;
                for j in (i + 1)..npts {
                    // SAFETY: j < npts guaranteed by loop bound
                    let pj = unsafe { *pts.get_unchecked(j) };
                    let det = ad1 * pj.bd - bd1 * pj.ad;
                    if det == 0 {
                        let idx = if diam1 != 0 { diam1 } else { pj.diam } as usize;
                        // SAFETY: diam_index + origin sentinel are in 0..DIAM_LEN
                        unsafe { *diams.get_unchecked_mut(idx) += 1; }
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

                    *circles.entry(CircleKey { h, k, d: den }).or_insert(0) += 1;
                }
            }
            (circles, diams)
        })
        .collect();

    let mut circles =
        FxHashMap::<CircleKey, u32>::with_capacity_and_hasher(192_000, Default::default());
    let mut diams = vec![0u32; DIAM_LEN];
    for (map, dm) in chunk_results {
        for (key, count) in map {
            *circles.entry(key).or_insert(0) += count;
        }
        for (i, c) in dm.iter().enumerate() {
            diams[i] += *c;
        }
    }

    let mut total: i64 = 0;
    for &pairs in circles.values() {
        total += triples_from_pairs(pairs as u64);
    }
    for &pairs in &diams {
        total += triples_from_pairs(pairs as u64);
    }

    println!("{}", total);
}
