// Project Euler 630 - Crossed Lines
// BBS generator for points, count crossing line pairs by slope grouping

use std::collections::HashMap;

const NPTS: usize = 2500;
const L: i64 = 2000;

fn gcd(mut a: i64, mut b: i64) -> i64 { while b != 0 { let t = b; b = a % b; a = t; } a }

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

    // Map: (dx, dy) -> set of intercepts
    let mut slope_map: HashMap<(i32, i32), Vec<i64>> = HashMap::new();

    for i in 0..NPTS {
        for j in 0..NPTS {
            if i == j { continue; }
            let mut dx = px[j] - px[i];
            let mut dy = py[j] - py[i];
            if dy < 0 || (dy == 0 && dx < 0) { dx = -dx; dy = -dy; }
            if dx == 0 && dy == 0 { continue; }
            let g = gcd(dx.abs() as i64, dy.abs() as i64) as i32;
            if g > 0 { dx /= g; dy /= g; }
            let intercept = dy as i64 * px[i] as i64 - dx as i64 * py[i] as i64;

            let entry = slope_map.entry((dx, dy)).or_insert_with(Vec::new);
            if !entry.contains(&intercept) {
                entry.push(intercept);
            }
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
