// Project Euler 314: The Mouse on the Moon
// Parametric Dijkstra on a grid to maximize area/perimeter ratio.

use std::collections::BinaryHeap;
use std::cmp::Ordering;

const NN: usize = 250;
const N: usize = NN + 1;
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

fn dijkstra(r: f64, step_len: &[[f64; MAX_STEP + 1]; MAX_STEP + 1],
            dist: &mut [f64; N * N], visited: &mut [bool; N * N],
            heap: &mut BinaryHeap<State>) -> f64 {
    dist.fill(f64::MAX);
    visited.fill(false);
    heap.clear();

    dist[NN] = 0.0; // idx(0, NN) = 0 * N + NN = NN
    heap.push(State { d: 0.0, x: 0, y: NN });

    while let Some(State { d, x, y }) = heap.pop() {
        let i = x * N + y;
        // SAFETY: x <= NN, y <= NN, so i < N*N
        unsafe {
            if *visited.get_unchecked(i) { continue; }
            *visited.get_unchecked_mut(i) = true;
        }

        if x == NN && y == 0 { return d; }

        let max_dx = (NN - x).min(MAX_STEP);
        let max_dy = y.min(MAX_STEP);

        for dx in 0..=max_dx {
            for dy in 0..=max_dy {
                if dx == 0 && dy == 0 { continue; }
                let nx = x + dx;
                let ny = y - dy;
                let ni = nx * N + ny;
                // SAFETY: nx <= NN, ny <= NN, so ni < N*N
                unsafe {
                    if *visited.get_unchecked(ni) { continue; }
                    let len = *step_len.get_unchecked(dx).get_unchecked(dy);
                    let area = dx as f64 * (2.0 * y as f64 - dy as f64) * 0.5;
                    let nd = d + r * len + area;
                    if nd < *dist.get_unchecked(ni) {
                        *dist.get_unchecked_mut(ni) = nd;
                        heap.push(State { d: nd, x: nx, y: ny });
                    }
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

    let k = 500.0 * 500.0 / 4.0;

    let mut dist = Box::new([f64::MAX; N * N]);
    let mut visited = Box::new([false; N * N]);
    let mut heap = BinaryHeap::with_capacity(N * N / 4);

    let mut lo = 125.0_f64;
    let mut hi = 140.0_f64;
    for _ in 0..60 {
        let mid = (lo + hi) / 2.0;
        let w = dijkstra(mid, &step_len, &mut dist, &mut visited, &mut heap);
        if w <= k { lo = mid; } else { hi = mid; }
    }

    println!("{:.8}", lo);
}
