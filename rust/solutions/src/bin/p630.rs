// Project Euler 630 - Crossed Lines
// BBS generator for points, count crossing line pairs by slope grouping

use fxhash::{FxHashMap, FxHashSet};

const NPTS: usize = 2500;
const L: i64 = 2000;

#[inline]
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let mut s: i64 = 290797;
    let mut px = vec![0i32; NPTS];
    let mut py = vec![0i32; NPTS];
    for i in 0..NPTS {
        s = s * s % 50515093;
        px[i] = (s % L - 1000) as i32;
        s = s * s % 50515093;
        py[i] = (s % L - 1000) as i32;
    }

    // slope (dx,dy) -> set of distinct intercepts
    let mut slope_map: FxHashMap<(i32, i32), FxHashSet<i64>> = FxHashMap::default();

    for i in 0..NPTS {
        for j in (i + 1)..NPTS {
            let mut dx = px[j] - px[i];
            let mut dy = py[j] - py[i];
            if dx == 0 && dy == 0 {
                continue;
            }
            if dy < 0 || (dy == 0 && dx < 0) {
                dx = -dx;
                dy = -dy;
            }
            let g = gcd(dx.abs(), dy.abs());
            if g > 0 {
                dx /= g;
                dy /= g;
            }
            let intercept = dy as i64 * px[i] as i64 - dx as i64 * py[i] as i64;
            slope_map
                .entry((dx, dy))
                .or_default()
                .insert(intercept);
        }
    }

    let total: i64 = slope_map.values().map(|v| v.len() as i64).sum();
    let mut ans: i64 = 0;
    for v in slope_map.values() {
        let c = v.len() as i64;
        ans += c * (total - c);
    }

    println!("{}", ans);
}
