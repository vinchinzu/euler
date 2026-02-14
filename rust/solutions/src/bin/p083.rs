// Project Euler 83: Path sum four ways
// Dijkstra's algorithm on 80x80 matrix, moving in all 4 directions.

use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let data = include_str!("../../../../data/matrix.txt");
    let mut matrix: Vec<Vec<i64>> = Vec::new();
    for line in data.lines() {
        let row: Vec<i64> = line.split(',').filter_map(|s| s.trim().parse().ok()).collect();
        if !row.is_empty() {
            matrix.push(row);
        }
    }
    let n = matrix.len();

    let mut dist = vec![vec![i64::MAX; n]; n];
    dist[0][0] = matrix[0][0];

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((matrix[0][0], 0usize, 0usize)));

    let dr: [i32; 4] = [0, 1, 0, -1];
    let dc: [i32; 4] = [1, 0, -1, 0];

    while let Some(Reverse((cost, r, c))) = heap.pop() {
        if cost > dist[r][c] {
            continue;
        }
        for d in 0..4 {
            let nr = r as i32 + dr[d];
            let nc = c as i32 + dc[d];
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                let new_cost = cost + matrix[nr][nc];
                if new_cost < dist[nr][nc] {
                    dist[nr][nc] = new_cost;
                    heap.push(Reverse((new_cost, nr, nc)));
                }
            }
        }
    }

    println!("{}", dist[n - 1][n - 1]);
}
