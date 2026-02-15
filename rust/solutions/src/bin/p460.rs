// Project Euler 460: An ant on the move - Dijkstra on lattice points near semicircle

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct State { dist: f64, idx: usize }
impl PartialEq for State { fn eq(&self, o: &Self) -> bool { self.dist == o.dist } }
impl Eq for State {}
impl PartialOrd for State {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> { o.dist.partial_cmp(&self.dist) }
}
impl Ord for State {
    fn cmp(&self, o: &Self) -> Ordering { self.partial_cmp(o).unwrap() }
}

fn main() {
    let d = 10000i32;
    let half = d / 2;

    let mut pts: Vec<(i32, i32)> = Vec::new();
    let mut pt_map: HashMap<(i32, i32), usize> = HashMap::new();
    let mut by_x: Vec<Vec<usize>> = vec![Vec::new(); (d + 1) as usize];

    for x in 0..=d {
        for y in 1..=d {
            let dx = (x - half) as i64;
            let dist = dx * dx + y as i64 * y as i64;
            let r_lo = (half - 1) as i64 * (half - 1) as i64;
            let r_hi = (half + 1) as i64 * (half + 1) as i64;
            if r_lo <= dist && dist <= r_hi {
                let idx = pts.len();
                pt_map.insert((x, y), idx);
                by_x[x as usize].push(idx);
                pts.push((x, y));
            }
        }
    }

    let n = pts.len();
    let mut dists = vec![1e18f64; n];
    let mut visited = vec![false; n];

    let start = *pt_map.get(&(0, 1)).expect("No start point");
    dists[start] = 0.0;

    let mut heap = BinaryHeap::new();
    heap.push(State { dist: 0.0, idx: start });

    let mut result = 1e18f64;

    while let Some(State { dist: cur_dist, idx: i }) = heap.pop() {
        if visited[i] { continue; }
        visited[i] = true;

        let (px, py) = pts[i];

        if px == d && py == 1 {
            result = cur_dist;
            break;
        }

        for dx_off in 0..=(d - px) {
            let nx = px + dx_off;
            if nx > d { break; }
            let mut done = true;
            for &j in &by_x[nx as usize] {
                if visited[j] { continue; }
                let (_, y) = pts[j];

                let vel = if py == y {
                    py as f64
                } else {
                    (y - py) as f64 / ((y as f64).ln() - (py as f64).ln())
                };

                let dist_sq = dx_off as f64 * dx_off as f64 + (y - py) as f64 * (y - py) as f64;
                let edge_dist = dist_sq.sqrt() / vel;
                let new_dist = cur_dist + edge_dist;

                if new_dist < dists[j] {
                    dists[j] = new_dist;
                    heap.push(State { dist: new_dist, idx: j });
                }

                if (dx_off as i64 * dx_off as i64 + (y - py) as i64 * (y - py) as i64) < d as i64 {
                    done = false;
                }
            }
            if done && dx_off > 0 { break; }
        }
    }

    println!("{:.9}", result);
}
