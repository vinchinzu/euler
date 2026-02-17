// Problem 983: Consonant Circle Crossing
// Port of the parity-family constructive search used in the Python reference.

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

type Point = (i32, i32);

const OFF: i32 = 1 << 15;
const SHIFT: i32 = 17;

#[derive(Default)]
struct FastIntHasher {
    state: u64,
}

impl Hasher for FastIntHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.state
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // Fallback only; integer-key paths below are the hot path.
        let mut h = 0xcbf29ce484222325u64;
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        self.state = h;
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.state = i.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.write_u64(i as u64);
    }
}

type FastBuildHasher = BuildHasherDefault<FastIntHasher>;
type FastHashMap<K, V> = HashMap<K, V, FastBuildHasher>;
type FastHashSet<T> = HashSet<T, FastBuildHasher>;

#[inline]
fn encode(x: i32, y: i32) -> i64 {
    (((x + OFF) as i64) << SHIFT) | (y + OFF) as i64
}

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
    let mut used: FastHashSet<Point> =
        FastHashSet::with_capacity_and_hasher(sorted.len() * 2, FastBuildHasher::default());
    for v in sorted {
        if used.contains(&v) {
            continue;
        }
        let w = (-v.0, -v.1);
        used.insert(v);
        used.insert(w);
        pairs.push((v, w));
    }
    pairs
}

fn even_masks(k: usize) -> Vec<u16> {
    let mut masks = Vec::new();
    for mask in 0u16..(1u16 << k) {
        if (mask.count_ones() & 1) == 0 {
            masks.push(mask);
        }
    }
    masks
}

fn centers_from_vectors(vectors: &[Point], masks: &[u16]) -> Vec<Point> {
    let mut centers = Vec::with_capacity(masks.len());
    for &mask in masks {
        let mut x = 0i32;
        let mut y = 0i32;
        let mut mm = mask;
        while mm != 0 {
            let lsb = mm & (!mm + 1);
            let idx = lsb.trailing_zeros() as usize;
            let (vx, vy) = vectors[idx];
            x += vx;
            y += vy;
            mm ^= lsb;
        }
        centers.push((x, y));
    }
    centers
}

fn has_unit_coordinate(points: &[Point]) -> bool {
    points.iter().any(|&(x, y)| x.abs() == 1 || y.abs() == 1)
}

fn quick_harmony_count_equals_n(centers: &[Point], circle_points: &[Point], n: usize) -> bool {
    let mut counts: FastHashMap<i64, u8> = FastHashMap::with_capacity_and_hasher(
        centers.len() * circle_points.len(),
        FastBuildHasher::default(),
    );
    let mut harmony_count = 0usize;

    for &(cx, cy) in centers {
        for &(vx, vy) in circle_points {
            let key = encode(cx + vx, cy + vy);
            match counts.get_mut(&key) {
                None => {
                    counts.insert(key, 1);
                }
                Some(cur) => {
                    if *cur == 1 {
                        *cur = 2;
                        harmony_count += 1;
                        if harmony_count > n {
                            return false;
                        }
                    } else {
                        *cur = cur.saturating_add(1);
                    }
                }
            }
        }
    }

    harmony_count == n
}

fn strict_perfect_check(centers: &[Point], circle_points: &[Point], n: usize) -> bool {
    let center_codes: Vec<i64> = centers.iter().map(|&(x, y)| encode(x, y)).collect();
    let center_set: FastHashSet<i64> =
        center_codes.iter().copied().collect::<FastHashSet<i64>>();

    // Requirement: no tangent circle pairs.
    let tangent_diffs: Vec<i64> = circle_points
        .iter()
        .map(|&(x, y)| encode(2 * x, 2 * y))
        .collect();
    for &c in &center_codes {
        for &d in &tangent_diffs {
            let other = c + d;
            if c < other && center_set.contains(&other) {
                return false;
            }
        }
    }

    let mut point_to_centers: FastHashMap<i64, Vec<usize>> = FastHashMap::with_capacity_and_hasher(
        centers.len() * circle_points.len(),
        FastBuildHasher::default(),
    );
    for (idx, &(cx, cy)) in centers.iter().enumerate() {
        for &(vx, vy) in circle_points {
            let key = encode(cx + vx, cy + vy);
            point_to_centers.entry(key).or_default().push(idx);
        }
    }

    let mut harmony_lists: Vec<&Vec<usize>> = Vec::new();
    harmony_lists.reserve(point_to_centers.len());
    for lst in point_to_centers.values() {
        if lst.len() >= 2 {
            harmony_lists.push(lst);
        }
    }
    if harmony_lists.len() != n {
        return false;
    }

    let mut parent: Vec<usize> = (0..n).collect();
    let mut size = vec![1usize; n];

    fn find(parent: &mut [usize], x: usize) -> usize {
        let mut cur = x;
        while parent[cur] != cur {
            parent[cur] = parent[parent[cur]];
            cur = parent[cur];
        }
        cur
    }

    fn union(parent: &mut [usize], size: &mut [usize], a: usize, b: usize) {
        let mut ra = find(parent, a);
        let mut rb = find(parent, b);
        if ra == rb {
            return;
        }
        if size[ra] < size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        parent[rb] = ra;
        size[ra] += size[rb];
    }

    for lst in harmony_lists {
        let base = lst[0];
        for &v in &lst[1..] {
            union(&mut parent, &mut size, base, v);
        }
    }

    let root = find(&mut parent, 0);
    for i in 1..n {
        if find(&mut parent, i) != root {
            return false;
        }
    }

    true
}

fn find_min_radius_sq_for_parity_family(k: usize, m_limit: i32, filtered: bool) -> i32 {
    let masks = even_masks(k);
    let n = 1usize << (k - 1);

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

        for comb in (0..p).combinations(k) {
            let mut chosen_pairs = Vec::with_capacity(k);
            for idx in comb {
                chosen_pairs.push(pairs[idx]);
            }

            for bits in 0usize..(1usize << (k - 1)) {
                let mut oriented = Vec::with_capacity(k);
                oriented.push(chosen_pairs[0].0);
                for i in 1..k {
                    let pick_second = ((bits >> (i - 1)) & 1) == 1;
                    oriented.push(if pick_second {
                        chosen_pairs[i].1
                    } else {
                        chosen_pairs[i].0
                    });
                }

                let centers = centers_from_vectors(&oriented, &masks);
                if !quick_harmony_count_equals_n(&centers, &circle_points, n) {
                    continue;
                }
                if strict_perfect_check(&centers, &circle_points, n) {
                    return m;
                }
            }
        }
    }

    panic!("no solution found up to m={m_limit}");
}

fn main() {
    assert_eq!(find_min_radius_sq_for_parity_family(2, 20, false), 1);
    assert_eq!(find_min_radius_sq_for_parity_family(3, 50, false), 5);
    println!("{}", find_min_radius_sq_for_parity_family(10, 20_000, true));
}
