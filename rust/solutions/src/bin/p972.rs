// Project Euler 972 - Hyperbolic geodesics T(12) = 3575508
// Count ordered triples of V(12) points on a common geodesic
// (diameter or circle orthogonal to unit disc)
//
// For each pair of points, compute the unique geodesic.
// Count pairs per geodesic, then recover s from C(s,2) = pairs.
// Sum s*(s-1)*(s-2) over all geodesics with s >= 3.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use rayon::prelude::*;

#[inline(always)]
fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Clone, Copy)]
struct PrePoint {
    xn: i32, xd: i32,
    yn: i32, yd: i32,
    a: i32,
    b: i32,
    s_num: i64,
    ad: i64,
    bd: i64,
}

// FxHash-like hasher
struct FxHasher {
    hash: u64,
}

const SEED: u64 = 0x517cc1b727220a95;

impl Hasher for FxHasher {
    #[inline]
    fn finish(&self) -> u64 { self.hash }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut buf = [0u8; 8];
            buf[..chunk.len()].copy_from_slice(chunk);
            self.hash = (self.hash.rotate_left(5) ^ u64::from_le_bytes(buf)).wrapping_mul(SEED);
        }
    }
    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.hash = (self.hash.rotate_left(5) ^ i as u64).wrapping_mul(SEED);
    }
    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.hash = (self.hash.rotate_left(5) ^ i as u64).wrapping_mul(SEED);
    }
}

impl Default for FxHasher {
    #[inline]
    fn default() -> Self { FxHasher { hash: 0 } }
}

type FxBuildHasher = BuildHasherDefault<FxHasher>;
type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

type GeoKey = (i8, i64, i64, i64);

#[inline(always)]
fn compute_geodesic(p1: &PrePoint, p2: &PrePoint) -> Option<GeoKey> {
    let lhs = p1.xn as i64 * p2.yn as i64 * p2.xd as i64 * p1.yd as i64;
    let rhs = p2.xn as i64 * p1.yn as i64 * p1.xd as i64 * p2.yd as i64;

    if lhs == rhs {
        let (mut da, mut db) = if p1.xn != 0 || p1.yn != 0 {
            (p1.a as i64, p1.b as i64)
        } else {
            (p2.a as i64, p2.b as i64)
        };
        if da == 0 && db == 0 { return None; }
        let g = gcd(da.abs(), db.abs());
        da /= g;
        db /= g;
        if da < 0 || (da == 0 && db < 0) { da = -da; db = -db; }
        Some((0, da, db, 0))
    } else {
        let det = p1.ad * p2.bd - p1.bd * p2.ad;
        if det == 0 { return None; }

        let mut h_num = p1.s_num * p2.bd - p2.s_num * p1.bd;
        let mut k_num = p1.ad * p2.s_num - p2.ad * p1.s_num;
        let mut den = 2 * det;

        let g1 = gcd(h_num.abs(), k_num.abs());
        let g2 = gcd(g1, den.abs());
        if g2 > 1 {
            h_num /= g2;
            k_num /= g2;
            den /= g2;
        }
        if den < 0 { h_num = -h_num; k_num = -k_num; den = -den; }

        Some((1, h_num, k_num, den))
    }
}

fn main() {
    let n = 12i32;

    let mut rats: Vec<(i32, i32)> = vec![(0, 1)];
    for q in 1..=n {
        for p in (-(q - 1))..=q - 1 {
            if p == 0 { continue; }
            if gcd(p.abs() as i64, q as i64) == 1 {
                rats.push((p, q));
            }
        }
    }

    let mut pts: Vec<PrePoint> = Vec::new();
    for &(xn, xd) in &rats {
        for &(yn, yd) in &rats {
            let x2 = xn as i64 * xn as i64 * yd as i64 * yd as i64;
            let y2 = yn as i64 * yn as i64 * xd as i64 * xd as i64;
            let r2 = xd as i64 * xd as i64 * yd as i64 * yd as i64;
            if x2 + y2 < r2 {
                let a = xn * yd;
                let b = yn * xd;
                let d = xd as i64 * yd as i64;
                pts.push(PrePoint {
                    xn, xd, yn, yd,
                    a, b,
                    s_num: (a as i64) * (a as i64) + (b as i64) * (b as i64) + d * d,
                    ad: a as i64 * d,
                    bd: b as i64 * d,
                });
            }
        }
    }

    let npts = pts.len();

    // Create load-balanced work chunks.
    // Point i has (npts - i - 1) pairs. Lower i has more work.
    let total_pairs: usize = npts * (npts - 1) / 2;
    let num_threads = rayon::current_num_threads().max(1);
    let target_per_chunk = total_pairs / (num_threads * 4) + 1;

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

    // Each chunk builds its own HashMap
    let chunk_results: Vec<FxHashMap<GeoKey, i64>> = chunks.par_iter()
        .map(|&(start, end)| {
            let mut map = FxHashMap::<GeoKey, i64>::with_capacity_and_hasher(100_000, Default::default());
            for i in start..end {
                // SAFETY: i < npts guaranteed by chunk bounds
                let pi = unsafe { pts.get_unchecked(i) };
                for j in (i + 1)..npts {
                    // SAFETY: j < npts guaranteed by loop bound
                    let pj = unsafe { pts.get_unchecked(j) };
                    if let Some(key) = compute_geodesic(pi, pj) {
                        *map.entry(key).or_insert(0) += 1;
                    }
                }
            }
            map
        })
        .collect();

    // Merge all chunk maps
    let mut geodesic_pairs = FxHashMap::<GeoKey, i64>::with_capacity_and_hasher(200_000, Default::default());
    for map in chunk_results {
        for (key, count) in map {
            *geodesic_pairs.entry(key).or_insert(0) += count;
        }
    }

    // For each geodesic, pair_count = C(s,2) = s*(s-1)/2.
    let mut total: i64 = 0;
    for &pairs in geodesic_pairs.values() {
        if pairs >= 3 {
            let sp = 2 * pairs;
            let disc = 1 + 4 * sp;
            let sqrt_disc = (disc as f64).sqrt() as i64;
            let s = (1 + sqrt_disc) / 2;
            if s * (s - 1) == sp && s >= 3 {
                total += sp * (s - 2);
            }
        }
    }

    println!("{}", total);
}
