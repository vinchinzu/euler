// Project Euler 252: Convex Holes
use std::collections::HashMap;

const NPTS: usize = 500;

#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn cross(p1: Point, p2: Point, p3: Point) -> i64 {
    (p2.x as i64 - p1.x as i64) * (p3.y as i64 - p1.y as i64)
        - (p2.y as i64 - p1.y as i64) * (p3.x as i64 - p1.x as i64)
}

fn shoestring(p1: Point, p2: Point, p3: Point) -> i64 {
    let v = p1.x as i64 * (p2.y as i64 - p3.y as i64)
        + p2.x as i64 * (p3.y as i64 - p1.y as i64)
        + p3.x as i64 * (p1.y as i64 - p2.y as i64);
    v.abs()
}

fn main() {
    let mut pts = Vec::with_capacity(NPTS);
    let mut x: i64 = 290797;
    for _ in 0..NPTS {
        x = (x * x) % 50515093;
        let xv = (x % 2000) as i32 - 1000;
        x = (x * x) % 50515093;
        let yv = (x % 2000) as i32 - 1000;
        pts.push(Point { x: xv, y: yv });
    }
    pts.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    let mut best_area = 0.0f64;
    let mut ht: HashMap<(usize, usize, usize), f64> = HashMap::new();

    for k in 0..NPTS {
        let pa = pts[k];
        let nrem = NPTS - k - 1;
        if nrem < 2 { continue; }

        let mut remaining: Vec<Point> = (0..nrem).map(|i| pts[k + 1 + i]).collect();
        let mut rem_idx: Vec<usize> = (0..nrem).map(|i| k + 1 + i).collect();

        // Sort by angle from pa
        let mut order: Vec<usize> = (0..nrem).collect();
        order.sort_by(|&a, &b| {
            let ang_a = f64::atan2((remaining[a].y - pa.y) as f64, (remaining[a].x - pa.x) as f64);
            let ang_b = f64::atan2((remaining[b].y - pa.y) as f64, (remaining[b].x - pa.x) as f64);
            ang_a.partial_cmp(&ang_b).unwrap()
        });

        let sorted_pts: Vec<Point> = order.iter().map(|&i| remaining[i]).collect();
        let sorted_idx: Vec<usize> = order.iter().map(|&i| rem_idx[i]).collect();
        remaining = sorted_pts;
        rem_idx = sorted_idx;

        // Build visibility graph
        let mut out_edges: Vec<Vec<usize>> = vec![vec![]; nrem];
        let mut in_edges: Vec<Vec<usize>> = vec![vec![]; nrem];
        let mut queues: Vec<std::collections::VecDeque<usize>> = vec![std::collections::VecDeque::new(); nrem];

        fn proceed(
            pi_idx: usize, pj_idx: usize,
            remaining: &[Point],
            out_edges: &mut [Vec<usize>],
            in_edges: &mut [Vec<usize>],
            queues: &mut [std::collections::VecDeque<usize>],
        ) {
            let pi = remaining[pi_idx];
            let pj = remaining[pj_idx];
            while let Some(&pk_idx) = queues[pi_idx].front() {
                if cross(remaining[pk_idx], pi, pj) > 0 {
                    queues[pi_idx].pop_front();
                    proceed(pk_idx, pj_idx, remaining, out_edges, in_edges, queues);
                } else {
                    break;
                }
            }
            out_edges[pi_idx].push(pj_idx);
            in_edges[pj_idx].push(pi_idx);
            queues[pj_idx].push_back(pi_idx);
        }

        for i in 0..nrem - 1 {
            proceed(i, i + 1, &remaining, &mut out_edges, &mut in_edges, &mut queues);
        }

        // DP
        for p2_i in 0..nrem {
            let mut p1_ptr = 0;
            let mut max_area_local = 0.0f64;

            for oi in 0..out_edges[p2_i].len() {
                let p3_i = out_edges[p2_i][oi];
                let p2 = remaining[p2_i];
                let p3 = remaining[p3_i];
                let key = (k, rem_idx[p2_i], rem_idx[p3_i]);

                if !ht.contains_key(&key) {
                    while p1_ptr < in_edges[p2_i].len() {
                        let p1_local_i = in_edges[p2_i][p1_ptr];
                        if cross(remaining[p1_local_i], p2, p3) > 0 {
                            p1_ptr += 1;
                            let prev_key = (k, rem_idx[p1_local_i], rem_idx[p2_i]);
                            if let Some(&area) = ht.get(&prev_key) {
                                if area > max_area_local { max_area_local = area; }
                            }
                        } else {
                            break;
                        }
                    }
                    let new_area = max_area_local + shoestring(pa, p2, p3) as f64 / 2.0;
                    ht.insert(key, new_area);
                    if new_area > best_area { best_area = new_area; }
                }
            }
        }
    }

    println!("{:.1}", best_area);
}
