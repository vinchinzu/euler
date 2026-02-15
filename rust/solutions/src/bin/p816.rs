// Project Euler 816 - Shortest distance among points
// Generate 2M points via Blum Blum Shub, find closest pair

fn main() {
    const NUM_POINTS: usize = 2_000_000;

    let mut points = Vec::with_capacity(NUM_POINTS);
    let mut s: u64 = 290797;
    let m: u64 = 50515093;

    for _ in 0..NUM_POINTS {
        let x = s;
        s = s.wrapping_mul(s) % m;
        let y = s;
        s = s.wrapping_mul(s) % m;
        points.push((x, y));
    }

    // Sort by x
    points.sort_unstable_by_key(|&(x, _)| x);

    let mut ans_sq: f64 = 1e36;
    for i in 0..NUM_POINTS {
        for j in (i + 1)..NUM_POINTS {
            let dx = points[j].0 as f64 - points[i].0 as f64;
            let dx_sq = dx * dx;
            if dx_sq >= ans_sq { break; }
            let dy = points[j].1 as f64 - points[i].1 as f64;
            let d_sq = dx_sq + dy * dy;
            if d_sq < ans_sq { ans_sq = d_sq; }
        }
    }

    println!("{:.9}", ans_sq.sqrt());
}
