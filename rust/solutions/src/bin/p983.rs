// Problem 983: Consonant Circle Crossing
// Port of the parity-family constructive search used in the Python reference.
// Optimized: flat-array with generation counters (u32), per-combo bounds, rayon parallelism.

use itertools::Itertools;
use rayon::prelude::*;
use std::cell::RefCell;

type Point = (i32, i32);

fn isqrt_i32(n: i32) -> i32 {
    let mut x = (n as f64).sqrt() as i32;
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    while x * x > n {
        x -= 1;
    }
    x
}

fn lattice_points_on_circle(m: i32) -> Vec<Point> {
    let lim = isqrt_i32(m);
    let mut points = Vec::new();
    for x in -lim..=lim {
        let y2 = m - x * x;
        if y2 < 0 {
            continue;
        }
        let y = isqrt_i32(y2);
        if y * y == y2 {
            points.push((x, y));
            if y != 0 {
                points.push((x, -y));
            }
        }
    }
    points
}

fn opposite_pairs(points: &[Point]) -> Vec<(Point, Point)> {
    let mut sorted = points.to_vec();
    sorted.sort_unstable();
    let mut pairs = Vec::new();
    let n = sorted.len();
    let mut used = vec![false; n];
    for i in 0..n {
        if used[i] {
            continue;
        }
        let v = sorted[i];
        let w = (-v.0, -v.1);
        for j in (i + 1)..n {
            if sorted[j] == w {
                used[i] = true;
                used[j] = true;
                pairs.push((v, w));
                break;
            }
        }
    }
    pairs
}

fn even_masks(k: usize) -> Vec<u16> {
    (0u16..(1u16 << k))
        .filter(|m| m.count_ones() & 1 == 0)
        .collect()
}

fn centers_from_vectors_into(vectors: &[Point], masks: &[u16], centers: &mut Vec<Point>) {
    centers.clear();
    for &mask in masks {
        let mut x = 0i32;
        let mut y = 0i32;
        let mut mm = mask;
        while mm != 0 {
            let idx = mm.trailing_zeros() as usize;
            let &(vx, vy) = unsafe { vectors.get_unchecked(idx) };
            x += vx;
            y += vy;
            mm &= mm - 1;
        }
        centers.push((x, y));
    }
}

fn has_unit_coordinate(points: &[Point]) -> bool {
    points.iter().any(|&(x, y)| x.abs() == 1 || y.abs() == 1)
}

/// Flat grid with generation-based reset. Each cell is u32:
/// high 16 bits = generation, low 16 bits = count/value.
struct FlatGrid {
    data: Vec<u32>,
    gn: u16,
    width: usize,
    off_x: i32,
    off_y: i32,
}

impl FlatGrid {
    fn new() -> Self {
        FlatGrid {
            data: Vec::new(),
            gn: 0,
            width: 0,
            off_x: 0,
            off_y: 0,
        }
    }

    fn configure(&mut self, hx: i32, hy: i32) {
        let w = (2 * hx + 1) as usize;
        let h = (2 * hy + 1) as usize;
        let needed = w * h;
        if self.data.len() < needed {
            self.data.resize(needed, 0);
        }
        self.width = w;
        self.off_x = hx;
        self.off_y = hy;
    }

    #[inline]
    fn reset(&mut self) {
        self.gn = self.gn.wrapping_add(1);
        if self.gn == 0 {
            for v in self.data.iter_mut() {
                *v = 0;
            }
            self.gn = 1;
        }
    }

    #[inline]
    fn idx(&self, x: i32, y: i32) -> usize {
        ((y + self.off_y) as usize) * self.width + (x + self.off_x) as usize
    }

    #[inline]
    fn increment(&mut self, x: i32, y: i32) -> u16 {
        let idx = self.idx(x, y);
        let cell = unsafe { self.data.get_unchecked_mut(idx) };
        if (*cell >> 16) as u16 != self.gn {
            *cell = ((self.gn as u32) << 16) | 1;
            1
        } else {
            let cnt = (*cell & 0xFFFF) + 1;
            *cell = ((self.gn as u32) << 16) | cnt;
            cnt as u16
        }
    }

    #[inline]
    fn store(&mut self, x: i32, y: i32, val: u16) {
        let idx = self.idx(x, y);
        unsafe {
            *self.data.get_unchecked_mut(idx) = ((self.gn as u32) << 16) | (val as u32);
        }
    }

    #[inline]
    fn get(&self, x: i32, y: i32) -> Option<u16> {
        let idx = self.idx(x, y);
        let cell = unsafe { *self.data.get_unchecked(idx) };
        if (cell >> 16) as u16 == self.gn {
            Some((cell & 0xFFFF) as u16)
        } else {
            None
        }
    }
}

#[inline]
fn quick_harmony_count_equals_n(
    centers: &[Point],
    circle_points: &[Point],
    n: usize,
    grid: &mut FlatGrid,
) -> bool {
    grid.reset();
    let mut harmony_count = 0usize;

    for &(cx, cy) in centers {
        for &(vx, vy) in circle_points {
            let cnt = grid.increment(cx + vx, cy + vy);
            if cnt == 2 {
                harmony_count += 1;
                if harmony_count > n {
                    return false;
                }
            }
        }
    }

    harmony_count == n
}

fn strict_perfect_check(
    centers: &[Point],
    circle_points: &[Point],
    n: usize,
    grid: &mut FlatGrid,
    center_grid: &mut FlatGrid,
) -> bool {
    center_grid.reset();
    for &(cx, cy) in centers {
        center_grid.increment(cx, cy);
    }

    for &(cx, cy) in centers {
        let ci = center_grid.idx(cx, cy);
        for &(vx, vy) in circle_points {
            let ox = cx + 2 * vx;
            let oy = cy + 2 * vy;
            let oi = center_grid.idx(ox, oy);
            if ci < oi {
                if center_grid.get(ox, oy).is_some() {
                    return false;
                }
            }
        }
    }

    grid.reset();
    let mut harmony_points: Vec<(i32, i32)> = Vec::new();
    for &(cx, cy) in centers {
        for &(vx, vy) in circle_points {
            let px = cx + vx;
            let py = cy + vy;
            if grid.increment(px, py) == 2 {
                harmony_points.push((px, py));
            }
        }
    }
    if harmony_points.len() != n {
        return false;
    }

    grid.reset();
    for (i, &(px, py)) in harmony_points.iter().enumerate() {
        grid.store(px, py, (i + 1) as u16);
    }

    let mut parent: Vec<u16> = (0..n as u16).collect();
    let mut sz = vec![1u16; n];

    #[inline]
    fn find(parent: &mut [u16], x: u16) -> u16 {
        let mut cur = x;
        while parent[cur as usize] != cur {
            parent[cur as usize] = parent[parent[cur as usize] as usize];
            cur = parent[cur as usize];
        }
        cur
    }

    #[inline]
    fn union(parent: &mut [u16], sz: &mut [u16], a: u16, b: u16) {
        let mut ra = find(parent, a);
        let mut rb = find(parent, b);
        if ra == rb {
            return;
        }
        if sz[ra as usize] < sz[rb as usize] {
            std::mem::swap(&mut ra, &mut rb);
        }
        parent[rb as usize] = ra;
        sz[ra as usize] += sz[rb as usize];
    }

    for &(cx, cy) in centers {
        let mut first_hp: Option<u16> = None;
        for &(vx, vy) in circle_points {
            if let Some(hi_plus1) = grid.get(cx + vx, cy + vy) {
                if hi_plus1 > 0 {
                    let hi = hi_plus1 - 1;
                    match first_hp {
                        None => first_hp = Some(hi),
                        Some(fh) => union(&mut parent, &mut sz, fh, hi),
                    }
                }
            }
        }
    }

    let root = find(&mut parent, 0);
    (1..n as u16).all(|i| find(&mut parent, i) == root)
}

struct Workspace {
    grid: FlatGrid,
    center_grid: FlatGrid,
    oriented: Vec<Point>,
    centers: Vec<Point>,
}

fn find_min_radius_sq_for_parity_family(k: usize, m_limit: i32, filtered: bool) -> i32 {
    let masks = even_masks(k);
    let n = 1usize << (k - 1);

    let mut candidates: Vec<(i32, Vec<Point>, Vec<(Point, Point)>)> = Vec::new();

    for m in 1..=m_limit {
        let circle_points = lattice_points_on_circle(m);
        let p = circle_points.len() / 2;
        if p < k {
            continue;
        }
        if filtered {
            if p != k && p != k + 2 {
                continue;
            }
            if !has_unit_coordinate(&circle_points) {
                continue;
            }
        }
        let pairs = opposite_pairs(&circle_points);
        if pairs.len() != p {
            continue;
        }
        candidates.push((m, circle_points, pairs));
    }

    thread_local! {
        static TL_WS: RefCell<Option<Workspace>> = const { RefCell::new(None) };
    }

    for &(m, ref circle_points, ref pairs) in &candidates {
        let p = pairs.len();

        let max_circle_x = circle_points.iter().map(|&(x, _)| x.abs()).max().unwrap_or(0);
        let max_circle_y = circle_points.iter().map(|&(_, y)| y.abs()).max().unwrap_or(0);

        let combs: Vec<Vec<usize>> = (0..p).combinations(k).collect();

        let combo_info: Vec<(i32, i32, i32, i32)> = combs
            .iter()
            .map(|comb| {
                let max_cx: i32 = comb.iter().map(|&i| pairs[i].0 .0.abs()).sum();
                let max_cy: i32 = comb.iter().map(|&i| pairs[i].0 .1.abs()).sum();
                (
                    max_cx + max_circle_x,
                    max_cy + max_circle_y,
                    max_cx + 2 * max_circle_x,
                    max_cy + 2 * max_circle_y,
                )
            })
            .collect();

        let found = (0..combs.len()).into_par_iter().any(|ci| {
            let comb = &combs[ci];
            let (phx, phy, chx, chy) = combo_info[ci];

            TL_WS.with(|tl| {
                let mut ws_opt = tl.borrow_mut();
                let ws = ws_opt.get_or_insert_with(|| Workspace {
                    grid: FlatGrid::new(),
                    center_grid: FlatGrid::new(),
                    oriented: Vec::with_capacity(k),
                    centers: Vec::with_capacity(n),
                });
                ws.grid.configure(phx, phy);
                ws.center_grid.configure(chx, chy);

                let mut chosen_pairs: Vec<(Point, Point)> = Vec::with_capacity(k);
                for &idx in comb {
                    chosen_pairs.push(pairs[idx]);
                }

                for bits in 0usize..(1usize << (k - 1)) {
                    ws.oriented.clear();
                    ws.oriented.push(chosen_pairs[0].0);
                    for i in 1..k {
                        if (bits >> (i - 1)) & 1 == 1 {
                            ws.oriented.push(chosen_pairs[i].1);
                        } else {
                            ws.oriented.push(chosen_pairs[i].0);
                        }
                    }

                    centers_from_vectors_into(&ws.oriented, &masks, &mut ws.centers);
                    if !quick_harmony_count_equals_n(
                        &ws.centers,
                        circle_points,
                        n,
                        &mut ws.grid,
                    ) {
                        continue;
                    }
                    if strict_perfect_check(
                        &ws.centers,
                        circle_points,
                        n,
                        &mut ws.grid,
                        &mut ws.center_grid,
                    ) {
                        return true;
                    }
                }
                false
            })
        });

        if found {
            return m;
        }
    }

    panic!("no solution found up to m={m_limit}");
}

fn main() {
    assert_eq!(find_min_radius_sq_for_parity_family(2, 20, false), 1);
    assert_eq!(find_min_radius_sq_for_parity_family(3, 50, false), 5);
    println!("{}", find_min_radius_sq_for_parity_family(10, 20_000, true));
}
