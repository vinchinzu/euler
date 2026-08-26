// Project Euler 332: Spherical triangles
//
// A(r) = min spherical area of a non-degenerate lattice triangle on x²+y²+z²=r².
// Van Oosterom–Strackee: area = 2 r² atan2(|det|, r(r² + A·B + B·C + C·A)).
// Hemisphere x≥0 is enough (octahedral images of the minimizer).

use rayon::prelude::*;

#[inline(always)]
fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.unsigned_abs();
    let mut b = b.unsigned_abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a as i32
}

#[inline(always)]
fn det_cutoff(best: f64, r: i32) -> i32 {
    let r2 = r as f64 * r as f64;
    let r3 = r2 * r as f64;
    let t = (best / r2 * 0.5).tan();
    let d = (4.0 * r3 * t + 1e-12).floor() as i64;
    d.clamp(1, i32::MAX as i64) as i32
}

#[inline(always)]
fn triple_area(
    ax: i32,
    ay: i32,
    az: i32,
    bx: i32,
    by: i32,
    bz: i32,
    cx: i32,
    cy: i32,
    cz: i32,
    r: i32,
) -> Option<(f64, i32)> {
    let det = (ay as i64 * bz as i64 - az as i64 * by as i64) * cx as i64
        + (az as i64 * bx as i64 - ax as i64 * bz as i64) * cy as i64
        + (ax as i64 * by as i64 - ay as i64 * bx as i64) * cz as i64;
    let adet = det.abs();
    if adet == 0 {
        return None;
    }
    let r2 = r as i64 * r as i64;
    let dots = ax as i64 * bx as i64
        + ay as i64 * by as i64
        + az as i64 * bz as i64
        + bx as i64 * cx as i64
        + by as i64 * cy as i64
        + bz as i64 * cz as i64
        + cx as i64 * ax as i64
        + cy as i64 * ay as i64
        + cz as i64 * az as i64;
    let den = r as i64 * (r2 + dots);
    if den <= 0 {
        return None;
    }
    Some((2.0 * r2 as f64 * (adet as f64).atan2(den as f64), adet as i32))
}

fn lattice_points(r: i32) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let r2 = r * r;
    let cap = ((r + 1) * (2 * r + 1) * 2) as usize;
    let mut xs = Vec::with_capacity(cap);
    let mut ys = Vec::with_capacity(cap);
    let mut zs = Vec::with_capacity(cap);
    for x in 0..=r {
        let x2 = x * x;
        for y in -r..=r {
            let rem = r2 - x2 - y * y;
            if rem < 0 {
                continue;
            }
            let z = (rem as u32).isqrt() as i32;
            if z * z == rem {
                xs.push(x);
                ys.push(y);
                zs.push(z);
                if z != 0 {
                    xs.push(x);
                    ys.push(y);
                    zs.push(-z);
                }
            }
        }
    }
    (xs, ys, zs)
}

fn min_area(r: i32) -> f64 {
    let (xs, ys, zs) = lattice_points(r);
    let n = xs.len();
    if n < 3 {
        return 0.0;
    }

    let r2 = r as i64 * r as i64;
    let r2f = r2 as f64;
    let mut best = std::f64::consts::FRAC_PI_2 * r2f;

    // Seed from each point plus its K nearest neighbours (tight D_max early).
    const K: usize = 20;
    let mut best_dot = [i32::MIN; K];
    let mut best_j = [0usize; K];
    for i in 0..n {
        best_dot.fill(i32::MIN);
        let axi = xs[i];
        let ayi = ys[i];
        let azi = zs[i];
        for j in 0..n {
            if i == j {
                continue;
            }
            let d = axi * xs[j] + ayi * ys[j] + azi * zs[j];
            if d <= best_dot[K - 1] {
                continue;
            }
            let mut p = K - 1;
            while p > 0 && d > best_dot[p - 1] {
                best_dot[p] = best_dot[p - 1];
                best_j[p] = best_j[p - 1];
                p -= 1;
            }
            best_dot[p] = d;
            best_j[p] = j;
        }
        let mut m = 0;
        while m < K && best_dot[m] != i32::MIN {
            m += 1;
        }
        for a in 0..m {
            let j = best_j[a];
            for b in (a + 1)..m {
                let k = best_j[b];
                if let Some((area, _)) =
                    triple_area(axi, ayi, azi, xs[j], ys[j], zs[j], xs[k], ys[k], zs[k], r)
                {
                    if area < best {
                        best = area;
                    }
                }
            }
        }
    }

    let mut d_max = det_cutoff(best, r);

    for i in 0..n.saturating_sub(2) {
        // SAFETY: i < n by loop bound.
        let axi = unsafe { *xs.get_unchecked(i) };
        let ayi = unsafe { *ys.get_unchecked(i) };
        let azi = unsafe { *zs.get_unchecked(i) };
        for j in (i + 1)..n - 1 {
            // SAFETY: j < n - 1 < n.
            let bx = unsafe { *xs.get_unchecked(j) };
            let by = unsafe { *ys.get_unchecked(j) };
            let bz = unsafe { *zs.get_unchecked(j) };
            let cxp = ayi * bz - azi * by;
            let cyp = azi * bx - axi * bz;
            let czp = axi * by - ayi * bx;
            if cxp == 0 && cyp == 0 && czp == 0 {
                continue;
            }
            let g = gcd(cxp, gcd(cyp, czp));
            if g > d_max {
                continue;
            }
            let dot_ij = axi * bx + ayi * by + azi * bz;
            for k in (j + 1)..n {
                // SAFETY: k < n.
                let px = unsafe { *xs.get_unchecked(k) };
                let py = unsafe { *ys.get_unchecked(k) };
                let pz = unsafe { *zs.get_unchecked(k) };
                let det = cxp * px + cyp * py + czp * pz;
                let adet = det.abs();
                if adet == 0 || adet > d_max {
                    continue;
                }
                let dot_ik = axi * px + ayi * py + azi * pz;
                let dot_jk = bx * px + by * py + bz * pz;
                let den = r as i64 * (r2 + dot_ij as i64 + dot_ik as i64 + dot_jk as i64);
                if den <= 0 {
                    continue;
                }
                let area = 2.0 * r2f * (adet as f64).atan2(den as f64);
                if area < best {
                    best = area;
                    d_max = det_cutoff(best, r);
                    if g > d_max {
                        break;
                    }
                }
            }
        }
    }
    best
}

fn main() {
    let total: f64 = (1..=50i32).into_par_iter().map(min_area).sum();
    println!("{:.6}", total);
}
