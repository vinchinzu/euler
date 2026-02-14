// Project Euler 165 - Intersections
// Generate 5000 line segments from pseudo-random sequence,
// find all true intersection points (not at endpoints), count distinct.
use std::collections::HashSet;

fn gcd_abs(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

struct Segment {
    x1: i32, y1: i32, x2: i32, y2: i32,
    min_x: i32, max_x: i32, min_y: i32, max_y: i32,
    dx: i64, dy: i64, cross: i64,
}

fn main() {
    const SEGMENT_COUNT: usize = 5000;
    const MOD: i64 = 50515093;
    const SEED: i64 = 290797;
    const RANGE: i64 = 500;

    let mut values = vec![0i32; SEGMENT_COUNT * 4];
    let mut v: i64 = SEED;
    for val in values.iter_mut() {
        v = v * v % MOD;
        *val = (v % RANGE) as i32;
    }

    let mut segs: Vec<Segment> = Vec::new();
    for idx in (0..SEGMENT_COUNT * 4).step_by(4) {
        let (x1, y1, x2, y2) = (values[idx], values[idx+1], values[idx+2], values[idx+3]);
        if x1 == x2 && y1 == y2 { continue; }
        segs.push(Segment {
            x1, y1, x2, y2,
            min_x: x1.min(x2), max_x: x1.max(x2),
            min_y: y1.min(y2), max_y: y1.max(y2),
            dx: (x2 - x1) as i64, dy: (y2 - y1) as i64,
            cross: x1 as i64 * y2 as i64 - y1 as i64 * x2 as i64,
        });
    }
    segs.sort_by_key(|s| s.min_x);

    let mut points: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    let count = segs.len();

    for i in 0..count {
        let s1 = &segs[i];
        for j in (i + 1)..count {
            let s2 = &segs[j];
            if s2.min_x > s1.max_x { break; }
            if s1.max_y < s2.min_y || s2.max_y < s1.min_y { continue; }

            let o1 = s1.dx * (s2.y1 as i64 - s1.y1 as i64) - s1.dy * (s2.x1 as i64 - s1.x1 as i64);
            if o1 == 0 { continue; }
            let o2 = s1.dx * (s2.y2 as i64 - s1.y1 as i64) - s1.dy * (s2.x2 as i64 - s1.x1 as i64);
            if o2 == 0 || (o1 > 0) == (o2 > 0) { continue; }

            let o3 = s2.dx * (s1.y1 as i64 - s2.y1 as i64) - s2.dy * (s1.x1 as i64 - s2.x1 as i64);
            if o3 == 0 { continue; }
            let o4 = s2.dx * (s1.y2 as i64 - s2.y1 as i64) - s2.dy * (s1.x2 as i64 - s2.x1 as i64);
            if o4 == 0 || (o3 > 0) == (o4 > 0) { continue; }

            let mut den = s1.dx * s2.dy - s1.dy * s2.dx;
            if den == 0 { continue; }

            let mut num_x = s1.dx * s2.cross - s1.cross * s2.dx;
            let mut num_y = s1.dy * s2.cross - s1.cross * s2.dy;

            if den < 0 { den = -den; num_x = -num_x; num_y = -num_y; }

            let g1 = gcd_abs(num_x, den);
            let g2 = gcd_abs(num_y, den);

            points.insert((num_x / g1, den / g1, num_y / g2, den / g2));
        }
    }

    println!("{}", points.len());
}
