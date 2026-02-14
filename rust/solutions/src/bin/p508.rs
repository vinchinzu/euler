// Project Euler 508 - Integers in Base i-1
// Count 1s in base (i-1) representation using recursive rectangle decomposition.

const MOD: i64 = 1_000_000_007;
const L: usize = 128;

#[derive(Clone, Copy)]
struct Point { x: i64, y: i64 }

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Rect { min_x: i64, max_x: i64, min_y: i64, max_y: i64 }

impl Rect {
    fn empty(self) -> bool {
        self.min_x > self.max_x || self.min_y > self.max_y
    }
    fn translate(self, d: Point) -> Rect {
        Rect { min_x: self.min_x + d.x, max_x: self.max_x + d.x,
               min_y: self.min_y + d.y, max_y: self.max_y + d.y }
    }
    fn union(self, other: Rect) -> Rect {
        Rect {
            min_x: self.min_x.min(other.min_x), max_x: self.max_x.max(other.max_x),
            min_y: self.min_y.min(other.min_y), max_y: self.max_y.max(other.max_y),
        }
    }
    fn intersection(self, other: Rect) -> Rect {
        Rect {
            min_x: self.min_x.max(other.min_x), max_x: self.max_x.min(other.max_x),
            min_y: self.min_y.max(other.min_y), max_y: self.max_y.min(other.max_y),
        }
    }
}

use std::collections::HashMap;

fn complex_multiply(a: Point, b: Point) -> Point {
    Point { x: a.x * b.x - a.y * b.y, y: a.x * b.y + a.y * b.x }
}

fn solve(
    r: Rect, k: i32, extra: i32,
    ds: &[Point; L], bounds: &[Rect; 129],
    cache: &mut HashMap<(Rect, i32, i32), i64>,
) -> i64 {
    if r.empty() { return 0; }
    if k == -1 { return extra as i64; }

    let key = (r, k, extra);
    if let Some(&v) = cache.get(&key) { return v; }

    let bound = bounds[k as usize];
    let r1 = r.intersection(bound);
    let neg_dk = Point { x: -ds[k as usize].x, y: -ds[k as usize].y };
    let r2 = r.translate(neg_dk).intersection(bound);

    let result = (solve(r1, k - 1, extra, ds, bounds, cache)
                + solve(r2, k - 1, extra + 1, ds, bounds, cache)) % MOD;

    cache.insert(key, result);
    result
}

fn main() {
    let n: i64 = 1_000_000_000_000_000; // 10^15

    let mut ds = [Point { x: 0, y: 0 }; L];
    ds[0] = Point { x: 1, y: 0 };
    for k in 1..L {
        ds[k] = complex_multiply(ds[k - 1], Point { x: -1, y: 1 });
    }

    let mut bounds = [Rect { min_x: 0, max_x: 0, min_y: 0, max_y: 0 }; 129];
    for k in 0..L {
        bounds[k + 1] = bounds[k].union(bounds[k].translate(ds[k]));
    }

    let query = Rect { min_x: -n, max_x: n, min_y: -n, max_y: n };
    let mut cache = HashMap::new();
    let ans = solve(query, (L - 1) as i32, 0, &ds, &bounds, &mut cache);
    println!("{}", ans);
}
