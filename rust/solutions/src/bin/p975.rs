// Problem 975: A Winding Path
// Ported from python/975.py.

use rayon::prelude::*;
use std::f64::consts::PI;

fn primes_up_to(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;
    let r = (n as f64).sqrt() as usize;
    for p in 2..=r {
        if !sieve[p] {
            continue;
        }
        let mut m = p * p;
        while m <= n {
            sieve[m] = false;
            m += p;
        }
    }
    let mut out = Vec::new();
    for (i, &is_prime) in sieve.iter().enumerate() {
        if is_prime {
            out.push(i);
        }
    }
    out
}

fn critical_z_values(a: i32, b: i32) -> Vec<f64> {
    debug_assert!(a > 0 && b > 0 && (a & 1) == 1 && (b & 1) == 1);
    debug_assert!(a != b);
    let s = (a + b) as i64;
    let d = (a - b).abs() as i64;
    debug_assert!(s % 2 == 0 && d % 2 == 0);

    let a_len = s / 2 + 1;
    let b_len = d / 2;

    let mut i = 0_i64;
    let mut j = 0_i64;
    let mut out = Vec::<f64>::with_capacity((a_len + b_len) as usize);
    let mut prev_num: Option<i64> = None;
    let mut prev_den = 1_i64;

    let a_f = a as f64;
    let s_f = s as f64;
    let d_f = d as f64;
    let two_pi_a_over_s = 2.0 * PI * a_f / s_f;
    let pi_a_over_d = PI * a_f / d_f;
    let d_over_2s = d_f / (2.0 * s_f);

    while i < a_len || j < b_len {
        let (num, den, kind, k) = if j == b_len {
            let val = (2 * i, s, 0_i32, i);
            i += 1;
            val
        } else if i == a_len {
            let val = (2 * j + 1, d, 1_i32, j);
            j += 1;
            val
        } else {
            let n1 = 2 * i;
            let d1 = s;
            let n2 = 2 * j + 1;
            let d2 = d;
            let left = n1 * d2;
            let right = n2 * d1;
            if left < right {
                let val = (n1, d1, 0_i32, i);
                i += 1;
                val
            } else if left > right {
                let val = (n2, d2, 1_i32, j);
                j += 1;
                val
            } else {
                let val = (n1, d1, 0_i32, i);
                i += 1;
                j += 1;
                val
            }
        };

        if let Some(pn) = prev_num {
            if pn * den == num * prev_den {
                continue;
            }
        }
        prev_num = Some(num);
        prev_den = den;

        let z = if kind == 0 {
            0.5 - 0.5 * (two_pi_a_over_s * k as f64).cos()
        } else {
            0.5 - d_over_2s * (pi_a_over_d * (2 * k + 1) as f64).cos()
        };
        out.push(z);
    }

    out
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Pt {
    X(usize, usize),      // ("x", ix, j)
    Y(usize, usize),      // ("y", jy, i)
    Corner(usize, usize), // ("corner", ix, jy)
}

#[inline(always)]
fn sgn(v: f64, eps: f64) -> i32 {
    (v > eps) as i32 - (v < -eps) as i32
}

#[inline]
fn cell_segment(i: usize, j: usize, zx: &[f64], zy: &[f64], eps: f64) -> Option<(Pt, Pt)> {
    // SAFETY: traversal stays on the (m x n) cell grid, so i+1 < zx.len() and j+1 < zy.len().
    let (zxi, zxip, zyj, zyjp) = unsafe {
        (
            *zx.get_unchecked(i),
            *zx.get_unchecked(i + 1),
            *zy.get_unchecked(j),
            *zy.get_unchecked(j + 1),
        )
    };

    let s00 = sgn(zxi - zyj, eps);
    let s10 = sgn(zxip - zyj, eps);
    let s01 = sgn(zxi - zyjp, eps);
    let s11 = sgn(zxip - zyjp, eps);

    let mut p1: Option<Pt> = None;
    let mut p2: Option<Pt> = None;
    let mut add = |pt: Pt| {
        if p1.is_none() {
            p1 = Some(pt);
            return;
        }
        if p1 == Some(pt) {
            return;
        }
        if p2.is_none() {
            p2 = Some(pt);
            return;
        }
        if p2 == Some(pt) {
            return;
        }
        panic!("Degenerate cell (too many boundary points)");
    };

    // Each zero-corner is recorded once (originally left/right and bottom/top both added it).
    if s00 == 0 {
        add(Pt::Corner(i, j));
    }
    if s10 == 0 {
        add(Pt::Corner(i + 1, j));
    }
    if s01 == 0 {
        add(Pt::Corner(i, j + 1));
    }
    if s11 == 0 {
        add(Pt::Corner(i + 1, j + 1));
    }

    // left / right / bottom / top interior crossings
    if s00 != 0 && s01 != 0 && s00 != s01 {
        add(Pt::X(i, j));
    }
    if s10 != 0 && s11 != 0 && s10 != s11 {
        add(Pt::X(i + 1, j));
    }
    if s00 != 0 && s10 != 0 && s00 != s10 {
        add(Pt::Y(j, i));
    }
    if s01 != 0 && s11 != 0 && s01 != s11 {
        add(Pt::Y(j + 1, i));
    }

    match (p1, p2) {
        (None, _) => None,
        (Some(_), None) => panic!("Degenerate cell (only one boundary point)"),
        (Some(a), Some(b)) => Some((a, b)),
    }
}

#[inline]
fn z_of(pt: Pt, zx: &[f64], zy: &[f64]) -> f64 {
    // SAFETY: points are vertices or edges of the critical grid built from zx/zy.
    unsafe {
        match pt {
            Pt::X(ix, _) => *zx.get_unchecked(ix),
            Pt::Y(jy, _) => *zy.get_unchecked(jy),
            Pt::Corner(ix, jy) => 0.5 * (*zx.get_unchecked(ix) + *zy.get_unchecked(jy)),
        }
    }
}

#[inline]
fn next_cell_across_edge(cur: (usize, usize), boundary_pt: Pt) -> (usize, usize) {
    match boundary_pt {
        Pt::X(ix, jy) => {
            if cur.0 + 1 == ix && cur.1 == jy {
                (ix, jy)
            } else {
                (ix - 1, jy)
            }
        }
        Pt::Y(jy, ix) => {
            if cur.0 == ix && cur.1 + 1 == jy {
                (ix, jy)
            } else {
                (ix, jy - 1)
            }
        }
        Pt::Corner(_, _) => panic!("next_cell_across_edge called on corner"),
    }
}

fn choose_next_cell_at_corner(
    corner: Pt,
    from_cell: (usize, usize),
    prev_pt: Pt,
    m: usize,
    n: usize,
    zx: &[f64],
    zy: &[f64],
    eps: f64,
) -> (usize, usize) {
    let (ix, jy) = match corner {
        Pt::Corner(ix, jy) => (ix as isize, jy as isize),
        _ => panic!("choose_next_cell_at_corner requires corner"),
    };
    for di in [-1isize, 0] {
        for dj in [-1isize, 0] {
            let ci = ix + di;
            let cj = jy + dj;
            if ci < 0 || cj < 0 || ci >= m as isize || cj >= n as isize {
                continue;
            }
            let cell = (ci as usize, cj as usize);
            if cell == from_cell {
                continue;
            }
            let seg = cell_segment(cell.0, cell.1, zx, zy, eps);
            let Some((a, b)) = seg else {
                continue;
            };
            if a != corner && b != corner {
                continue;
            }
            let other = if a == corner { b } else { a };
            if other == prev_pt {
                continue;
            }
            return cell;
        }
    }
    from_cell
}

fn f_value(a: i32, b: i32, c: i32, d: i32) -> f64 {
    let zx = critical_z_values(a, b);
    let zy = critical_z_values(c, d);
    let m = zx.len() - 1;
    let n = zy.len() - 1;
    let end_corner = Pt::Corner(m, n);
    let eps = 1e-16_f64;

    let mut cell = (0usize, 0usize);
    let mut pt = Pt::Corner(0, 0);
    let mut prev_pt: Option<Pt> = None;
    let mut total = 0.0_f64;
    let max_steps = 4 * m * n + 10;

    for _ in 0..max_steps {
        if pt == end_corner {
            return total;
        }

        let seg = cell_segment(cell.0, cell.1, &zx, &zy, eps).expect("lost curve");
        let nxt = if pt == seg.0 {
            seg.1
        } else if pt == seg.1 {
            seg.0
        } else {
            let (ix, jy) = match pt {
                Pt::Corner(ix, jy) => (ix as isize, jy as isize),
                _ => panic!("lost curve (point not on segment)"),
            };

            let mut found: Option<(usize, usize, Pt)> = None;
            for di in [-1isize, 0] {
                for dj in [-1isize, 0] {
                    let ci = ix + di;
                    let cj = jy + dj;
                    if ci < 0 || cj < 0 || ci >= m as isize || cj >= n as isize {
                        continue;
                    }
                    let ccell = (ci as usize, cj as usize);
                    let seg2 = cell_segment(ccell.0, ccell.1, &zx, &zy, eps);
                    let Some((a2, b2)) = seg2 else {
                        continue;
                    };
                    if a2 != pt && b2 != pt {
                        continue;
                    }
                    let other = if a2 == pt { b2 } else { a2 };
                    if prev_pt.is_some() && Some(other) == prev_pt {
                        continue;
                    }
                    found = Some((ccell.0, ccell.1, other));
                    break;
                }
                if found.is_some() {
                    break;
                }
            }

            let Some((ci, cj, other)) = found else {
                panic!("corner relocation failed");
            };
            cell = (ci, cj);
            other
        };

        total += (z_of(nxt, &zx, &zy) - z_of(pt, &zx, &zy)).abs();

        cell = match nxt {
            Pt::Corner(_, _) => choose_next_cell_at_corner(nxt, cell, pt, m, n, &zx, &zy, eps),
            Pt::X(_, _) | Pt::Y(_, _) => next_cell_across_edge(cell, nxt),
        };

        prev_pt = Some(pt);
        pt = nxt;
    }

    panic!("Exceeded max_steps");
}

fn g_value(m: usize, n: usize) -> f64 {
    let ps: Vec<i32> = primes_up_to(n)
        .into_iter()
        .filter(|&p| p >= m)
        .map(|p| p as i32)
        .collect();
    let np = ps.len();
    // Flatten pairs: outer i is ~70x uneven (C(k,2) vs 1), so rayon must steal per pair.
    let mut pairs = Vec::with_capacity(np.saturating_mul(np.saturating_sub(1)) / 2);
    for i in 0..np {
        let p = ps[i];
        for &q in &ps[i + 1..] {
            pairs.push((p, q));
        }
    }
    pairs
        .into_par_iter()
        .map(|(p, q)| f_value(p, q, p, 2 * q - p))
        .sum()
}

fn main() {
    let ans = g_value(500, 1000);
    println!("{ans:.5}");
}
