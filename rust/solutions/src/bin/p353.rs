// Project Euler 353: Minimal risk paths on a spherical grid
// Dijkstra with sum-of-three-squares representation.

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct State { risk: f64, idx: usize }
impl PartialEq for State { fn eq(&self, o: &Self) -> bool { self.risk == o.risk } }
impl Eq for State {}
impl PartialOrd for State {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        o.risk.partial_cmp(&self.risk) // min-heap
    }
}
impl Ord for State {
    fn cmp(&self, o: &Self) -> Ordering { self.partial_cmp(o).unwrap() }
}

fn sqrt_neg1_mod(p: i64) -> i64 {
    for g in 2..p {
        let exp = (p - 1) / 4;
        let mut r = 1i128;
        let mut base = g as i128 % p as i128;
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 { r = r * base % p as i128; }
            base = base * base % p as i128;
            e >>= 1;
        }
        if r * r % p as i128 == (p - 1) as i128 { return r as i64; }
    }
    -1
}

fn find_sq_rep(p: i64) -> (i64, i64) {
    let r = sqrt_neg1_mod(p);
    let mut aa = p;
    let mut bb = r;
    let limit = (p as f64).sqrt() as i64;
    while bb > limit {
        let t = aa % bb;
        aa = bb;
        bb = t;
    }
    let other = ((p - bb * bb) as f64).sqrt() as i64;
    (bb, other)
}

fn sum_of_two_squares(n: i64) -> Vec<(i64, i64)> {
    if n < 0 { return vec![]; }
    if n == 0 { return vec![(0, 0)]; }

    // Factorize
    let mut factors: Vec<(i64, u32)> = Vec::new();
    let mut tmp = n;
    let mut d = 2i64;
    while d * d <= tmp {
        if tmp % d == 0 {
            let mut e = 0u32;
            while tmp % d == 0 { tmp /= d; e += 1; }
            factors.push((d, e));
        }
        d += 1;
    }
    if tmp > 1 { factors.push((tmp, 1)); }

    for &(p, e) in &factors {
        if p % 4 == 3 && e % 2 == 1 { return vec![]; }
    }

    // Build using Gaussian integers
    let mut reps: Vec<(i64, i64)> = vec![(1, 0)];

    for &(p, e) in &factors {
        if p == 2 {
            for ri in 0..reps.len() {
                for _ in 0..e {
                    let (ca, cb) = reps[ri];
                    reps[ri] = (ca - cb, ca + cb);
                }
            }
        } else if p % 4 == 1 {
            let (a, b) = find_sq_rep(p);
            let mut pow_plus: Vec<(i64, i64)> = vec![(1, 0)];
            let mut pow_minus: Vec<(i64, i64)> = vec![(1, 0)];
            for j in 1..=e as usize {
                let (pre, pim) = pow_plus[j - 1];
                pow_plus.push((pre * a - pim * b, pre * b + pim * a));
                let (pre, pim) = pow_minus[j - 1];
                pow_minus.push((pre * a - pim * (-b), pre * (-b) + pim * a));
            }
            let mut new_reps = Vec::new();
            for &(rr, ri) in &reps {
                for j in 0..=e as usize {
                    let (gre, gim) = {
                        let (pr, pi) = pow_plus[j];
                        let (mr, mi) = pow_minus[e as usize - j];
                        (pr * mr - pi * mi, pr * mi + pi * mr)
                    };
                    new_reps.push((rr * gre - ri * gim, rr * gim + ri * gre));
                }
            }
            reps = new_reps;
        } else {
            let mut factor = 1i64;
            for _ in 0..e / 2 { factor *= p; }
            for ri in 0..reps.len() {
                reps[ri].0 *= factor;
                reps[ri].1 *= factor;
            }
        }
    }

    let mut result: Vec<(i64, i64)> = Vec::new();
    for &(a, b) in &reps {
        let a = a.abs();
        let b = b.abs();
        if a * a + b * b != n { continue; }
        if !result.contains(&(a, b)) { result.push((a, b)); }
        if a != b && !result.contains(&(b, a)) { result.push((b, a)); }
    }
    result
}

fn compute_mr(r: i32) -> f64 {
    let r2 = r as i64 * r as i64;
    let mut stations: Vec<(i32, i32, i32)> = Vec::new();

    for x in 0..=r {
        let remainder = r2 - x as i64 * x as i64;
        let reps = sum_of_two_squares(remainder);
        for &(y, z) in &reps {
            if y >= 0 && z >= 0 {
                stations.push((x, y as i32, z as i32));
                if x != 0 {
                    stations.push((-x, y as i32, z as i32));
                }
            }
        }
    }

    // Sort by x descending
    stations.sort_by(|a, b| b.0.cmp(&a.0));
    let n = stations.len();

    // Build spatial regions
    let l_box = 300i32;
    let mut regions: HashMap<(i32, i32, i32), Vec<usize>> = HashMap::new();
    for (i, &(sx, sy, sz)) in stations.iter().enumerate() {
        let rx = (sx + r) / l_box + 1;
        let ry = (sy + r) / l_box + 1;
        let rz = (sz + r) / l_box + 1;
        regions.entry((rx, ry, rz)).or_default().push(i);
    }

    // Dijkstra
    let mut risks = vec![1e30f64; n];
    let mut visited = vec![false; n];
    risks[0] = 0.0;

    let mut heap = BinaryHeap::new();
    heap.push(State { risk: 0.0, idx: 0 });

    let pi_val = std::f64::consts::PI;

    while let Some(State { risk, idx: i }) = heap.pop() {
        if visited[i] { continue; }
        visited[i] = true;

        if i == n - 1 { return risk; }

        let (sx, sy, sz) = stations[i];
        let rx = (sx + r) / l_box + 1;
        let ry = (sy + r) / l_box + 1;
        let rz = (sz + r) / l_box + 1;

        for dx in -1..=1i32 {
            for dy in -1..=1i32 {
                for dz in -1..=1i32 {
                    let key = (rx + dx, ry + dy, rz + dz);
                    if let Some(indices) = regions.get(&key) {
                        for &j in indices {
                            if visited[j] { continue; }
                            let dot = sx as i64 * stations[j].0 as i64
                                    + sy as i64 * stations[j].1 as i64
                                    + sz as i64 * stations[j].2 as i64;
                            let cos_theta = (dot as f64 / r2 as f64).clamp(-1.0, 1.0);
                            let theta = cos_theta.acos();
                            let edge_risk = (theta / pi_val) * (theta / pi_val);
                            let new_risk = risk + edge_risk;
                            if new_risk < risks[j] {
                                risks[j] = new_risk;
                                heap.push(State { risk: new_risk, idx: j });
                            }
                        }
                    }
                }
            }
        }
    }

    risks[n - 1]
}

fn main() {
    let mut total = 0.0f64;
    for k in 1..=15 {
        let r = (1 << k) - 1;
        let mr = compute_mr(r);
        total += mr;
    }
    println!("{:.10}", total);
}
