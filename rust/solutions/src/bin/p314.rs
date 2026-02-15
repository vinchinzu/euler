// Project Euler 314: The Mouse on the Moon
// Parametric Dijkstra on a grid to maximize area/perimeter ratio.

use std::collections::BinaryHeap;
use std::cmp::Ordering;

const NN: usize = 250;
const MAX_STEP: usize = 15;

#[derive(Clone, PartialEq)]
struct State { d: f64, x: usize, y: usize }

impl Eq for State {}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.partial_cmp(&self.d).unwrap_or(Ordering::Equal)
    }
}

fn dijkstra(r: f64, step_len: &[[f64; MAX_STEP + 1]; MAX_STEP + 1]) -> f64 {
    let mut dist = vec![vec![f64::MAX; NN + 1]; NN + 1];
    let mut visited = vec![vec![false; NN + 1]; NN + 1];

    dist[0][NN] = 0.0;
    let mut heap = BinaryHeap::new();
    heap.push(State { d: 0.0, x: 0, y: NN });

    while let Some(State { d, x, y }) = heap.pop() {
        if visited[x][y] { continue; }
        visited[x][y] = true;

        if x == NN && y == 0 { return d; }

        let max_dx = (NN - x).min(MAX_STEP);
        let max_dy = y.min(MAX_STEP);

        for dx in 0..=max_dx {
            for dy in 0..=max_dy {
                if dx == 0 && dy == 0 { continue; }
                let nx = x + dx;
                let ny = y - dy;
                if visited[nx][ny] { continue; }

                let len = step_len[dx][dy];
                let area = dx as f64 * (2.0 * y as f64 - dy as f64) / 2.0;
                let w = r * len + area;
                let nd = d + w;
                if nd < dist[nx][ny] {
                    dist[nx][ny] = nd;
                    heap.push(State { d: nd, x: nx, y: ny });
                }
            }
        }
    }
    f64::MAX
}

fn main() {
    let mut step_len = [[0.0f64; MAX_STEP + 1]; MAX_STEP + 1];
    for dx in 0..=MAX_STEP {
        for dy in 0..=MAX_STEP {
            step_len[dx][dy] = ((dx * dx + dy * dy) as f64).sqrt();
        }
    }

    let k = 500.0 * 500.0 / 4.0; // 62500: total_area/4 = 250000/4

    let mut lo = 125.0_f64;
    let mut hi = 140.0_f64;
    for _ in 0..100 {
        let mid = (lo + hi) / 2.0;
        let w = dijkstra(mid, &step_len);
        if w <= k { lo = mid; } else { hi = mid; }
    }

    println!("{:.8}", lo);
}
