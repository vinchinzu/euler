// Project Euler 353: Minimal risk paths on a spherical grid
// Dijkstra with sum-of-three-squares representation.

use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const L_BOX: i32 = 300;

#[derive(Clone, Copy)]
struct State {
    risk: f64,
    idx: u32,
}
impl PartialEq for State {
    fn eq(&self, o: &Self) -> bool {
        self.risk == o.risk
    }
}
impl Eq for State {}
impl PartialOrd for State {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        o.risk.partial_cmp(&self.risk)
    }
}
impl Ord for State {
    fn cmp(&self, o: &Self) -> Ordering {
        self.partial_cmp(o).unwrap()
    }
}

#[inline(always)]
fn mod_pow(mut base: i64, mut exp: i64, p: i64) -> i64 {
    let mut r = 1i64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % p;
        }
        base = base * base % p;
        exp >>= 1;
    }
    r
}

fn sqrt_neg1_mod(p: i64) -> i64 {
    // p ≡ 1 (mod 4), p < 2^16 so i64 modular mul is exact.
    if p & 7 == 5 {
        return mod_pow(2, (p - 1) / 4, p);
    }
    let mut g = 3i64;
    loop {
        let r = mod_pow(g, (p - 1) / 4, p);
        if r * r % p == p - 1 {
            return r;
        }
        g += 1;
    }
}

fn find_sq_rep(p: u32) -> (u16, u16) {
    let p64 = p as i64;
    let r = sqrt_neg1_mod(p64);
    let mut aa = p64;
    let mut bb = r;
    let limit = (p as f64).sqrt() as i64;
    while bb > limit {
        let t = aa % bb;
        aa = bb;
        bb = t;
    }
    let other = ((p64 - bb * bb) as f64).sqrt() as i64;
    (bb as u16, other as u16)
}

fn build_spf(limit: usize) -> Vec<u32> {
    let mut spf = vec![0u32; limit];
    for i in 2..limit {
        if spf[i] == 0 {
            spf[i] = i as u32;
            let sq = i.saturating_mul(i);
            if sq < limit {
                let mut j = sq;
                while j < limit {
                    if spf[j] == 0 {
                        spf[j] = i as u32;
                    }
                    j += i;
                }
            }
        }
    }
    spf
}

fn precompute_sq_reps(spf: &[u32]) -> Vec<(u16, u16)> {
    let n = spf.len();
    let mut sq = vec![(0u16, 0u16); n];
    for p in 5..n {
        if spf[p] == p as u32 && p & 3 == 1 {
            sq[p] = find_sq_rep(p as u32);
        }
    }
    sq
}

fn factor_one(mut n: u32, spf: &[u32], out: &mut Vec<(u32, u32)>) {
    out.clear();
    while n > 1 {
        let p = spf[n as usize];
        let mut e = 0u32;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        out.push((p, e));
    }
}

fn merge_factors(a: &[(u32, u32)], b: &[(u32, u32)], out: &mut Vec<(u32, u32)>) {
    out.clear();
    let (mut i, mut j) = (0usize, 0usize);
    while i < a.len() && j < b.len() {
        if a[i].0 == b[j].0 {
            out.push((a[i].0, a[i].1 + b[j].1));
            i += 1;
            j += 1;
        } else if a[i].0 < b[j].0 {
            out.push(a[i]);
            i += 1;
        } else {
            out.push(b[j]);
            j += 1;
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
}

fn sum_of_two_squares(
    n: i64,
    factors: &[(u32, u32)],
    sq: &[(u16, u16)],
    reps: &mut Vec<(i64, i64)>,
    out: &mut Vec<(i32, i32)>,
) {
    out.clear();
    if n < 0 {
        return;
    }
    if n == 0 {
        out.push((0, 0));
        return;
    }
    for &(p, e) in factors {
        if p & 3 == 3 && e & 1 == 1 {
            return;
        }
    }

    reps.clear();
    reps.push((1, 0));

    for &(p, e) in factors {
        if p == 2 {
            for ri in 0..reps.len() {
                let (mut ca, mut cb) = reps[ri];
                for _ in 0..e {
                    let na = ca - cb;
                    let nb = ca + cb;
                    ca = na;
                    cb = nb;
                }
                reps[ri] = (ca, cb);
            }
        } else if p & 3 == 1 {
            let (a, b) = sq[p as usize];
            let a = a as i64;
            let b = b as i64;
            let eu = e as usize;
            let mut pow_plus = Vec::with_capacity(eu + 1);
            let mut pow_minus = Vec::with_capacity(eu + 1);
            pow_plus.push((1i64, 0i64));
            pow_minus.push((1i64, 0i64));
            for j in 1..=eu {
                let (pre, pim) = pow_plus[j - 1];
                pow_plus.push((pre * a - pim * b, pre * b + pim * a));
                let (pre, pim) = pow_minus[j - 1];
                pow_minus.push((pre * a + pim * b, -pre * b + pim * a));
            }
            let mut new_reps = Vec::with_capacity(reps.len() * (eu + 1));
            for &(rr, ri) in reps.iter() {
                for j in 0..=eu {
                    let (pr, pi) = pow_plus[j];
                    let (mr, mi) = pow_minus[eu - j];
                    let gre = pr * mr - pi * mi;
                    let gim = pr * mi + pi * mr;
                    new_reps.push((rr * gre - ri * gim, rr * gim + ri * gre));
                }
            }
            *reps = new_reps;
        } else {
            let mut factor = 1i64;
            for _ in 0..e / 2 {
                factor *= p as i64;
            }
            for r in reps.iter_mut() {
                r.0 *= factor;
                r.1 *= factor;
            }
        }
    }

    for &(a, b) in reps.iter() {
        let a = a.abs() as i32;
        let b = b.abs() as i32;
        if a as i64 * a as i64 + b as i64 * b as i64 != n {
            continue;
        }
        if !out.contains(&(a, b)) {
            out.push((a, b));
        }
        if a != b && !out.contains(&(b, a)) {
            out.push((b, a));
        }
    }
}

fn compute_mr(r: i32, spf: &[u32], sq: &[(u16, u16)]) -> f64 {
    let r2 = r as i64 * r as i64;
    let mut stations: Vec<(i32, i32, i32)> = Vec::with_capacity(((r as usize) << 2) + 8);

    let mut fa = Vec::with_capacity(16);
    let mut fb = Vec::with_capacity(16);
    let mut factors = Vec::with_capacity(16);
    let mut reps = Vec::with_capacity(32);
    let mut yz = Vec::with_capacity(32);

    for x in 0..=r {
        let remainder = r2 - x as i64 * x as i64;
        if remainder == 0 {
            stations.push((x, 0, 0));
            if x != 0 {
                stations.push((-x, 0, 0));
            }
            continue;
        }
        let lo = (r - x) as u32;
        let hi = (r + x) as u32;
        factor_one(lo, spf, &mut fa);
        factor_one(hi, spf, &mut fb);
        merge_factors(&fa, &fb, &mut factors);
        sum_of_two_squares(remainder, &factors, sq, &mut reps, &mut yz);
        for &(y, z) in &yz {
            stations.push((x, y, z));
            if x != 0 {
                stations.push((-x, y, z));
            }
        }
    }

    stations.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    let n = stations.len();
    if n == 0 {
        return 0.0;
    }

    // Boxes match (coord+r)/L; y,z >= 0 so their indices start at r/L.
    // (2r)/L - r/L + 1 is not always r/L+1 because of integer division.
    let y_off = r / L_BOX;
    let dim_x = 2 * r / L_BOX + 1;
    let dim_y = 2 * r / L_BOX - y_off + 1;
    let dim_z = dim_y;
    let n_cells = (dim_x * dim_y * dim_z) as usize;

    #[inline(always)]
    fn cell_id(sx: i32, sy: i32, sz: i32, r: i32, y_off: i32, dim_y: i32, dim_z: i32) -> usize {
        let rx = (sx + r) / L_BOX;
        let ry = (sy + r) / L_BOX - y_off;
        let rz = (sz + r) / L_BOX - y_off;
        ((rx * dim_y + ry) * dim_z + rz) as usize
    }

    let (offsets, packed) = {
        let mut counts = vec![0u32; n_cells];
        for &(sx, sy, sz) in &stations {
            counts[cell_id(sx, sy, sz, r, y_off, dim_y, dim_z)] += 1;
        }
        let mut offsets = vec![0u32; n_cells + 1];
        for i in 0..n_cells {
            offsets[i + 1] = offsets[i] + counts[i];
        }
        let mut packed = vec![0u32; n];
        for (i, &(sx, sy, sz)) in stations.iter().enumerate() {
            let cid = cell_id(sx, sy, sz, r, y_off, dim_y, dim_z);
            packed[(offsets[cid] + counts[cid] - 1) as usize] = i as u32;
            counts[cid] -= 1;
        }
        (offsets, packed)
    };

    let mut risks = vec![1e30f64; n];
    let mut visited = vec![false; n];
    risks[0] = 0.0;

    let mut heap = BinaryHeap::with_capacity(n);
    heap.push(State { risk: 0.0, idx: 0 });

    let inv_pi = std::f64::consts::FRAC_1_PI;
    let r2f = r2 as f64;
    let target = (n - 1) as u32;

    while let Some(State { risk, idx: i }) = heap.pop() {
        let i_us = i as usize;
        if visited[i_us] {
            continue;
        }
        visited[i_us] = true;
        if i == target {
            return risk;
        }

        let (sx, sy, sz) = stations[i_us];
        let rx = (sx + r) / L_BOX;
        let ry = (sy + r) / L_BOX - y_off;
        let rz = (sz + r) / L_BOX - y_off;

        for ndx in -1..=1 {
            let rxx = rx + ndx;
            if rxx < 0 || rxx >= dim_x {
                continue;
            }
            for ndy in -1..=1 {
                let ryy = ry + ndy;
                if ryy < 0 || ryy >= dim_y {
                    continue;
                }
                for ndz in -1..=1 {
                    let rzz = rz + ndz;
                    if rzz < 0 || rzz >= dim_z {
                        continue;
                    }
                    let cid = ((rxx * dim_y + ryy) * dim_z + rzz) as usize;
                    let start = offsets[cid] as usize;
                    let end = offsets[cid + 1] as usize;
                    for &j in &packed[start..end] {
                        let ju = j as usize;
                        if visited[ju] {
                            continue;
                        }
                        let (jx, jy, jz) = stations[ju];
                        let dot = sx as i64 * jx as i64
                            + sy as i64 * jy as i64
                            + sz as i64 * jz as i64;
                        let cos_theta = (dot as f64 / r2f).clamp(-1.0, 1.0);
                        let t = cos_theta.acos() * inv_pi;
                        let new_risk = risk + t * t;
                        if new_risk < risks[ju] {
                            risks[ju] = new_risk;
                            heap.push(State {
                                risk: new_risk,
                                idx: j,
                            });
                        }
                    }
                }
            }
        }
    }

    risks[n - 1]
}

fn main() {
    const R_MAX: usize = (1 << 15) - 1;
    let spf = build_spf(2 * R_MAX + 2);
    let sq = precompute_sq_reps(&spf);
    let total: f64 = (1u32..=15)
        .into_par_iter()
        .map(|k| {
            let r = (1i32 << k) - 1;
            compute_mr(r, &spf, &sq)
        })
        .sum();
    println!("{:.10}", total);
}
