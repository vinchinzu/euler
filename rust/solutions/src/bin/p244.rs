// Project Euler 244: Sliders
// BFS on 4x4 board states (2-color sliding puzzle).

use std::collections::{HashMap, VecDeque};

const M: i64 = 100000007;
const C: i64 = 243;

fn get_cell(grid: u16, r: usize, c: usize) -> u16 {
    (grid >> (r * 4 + c)) & 1
}

fn set_cell(mut grid: u16, r: usize, c: usize, v: u16) -> u16 {
    let bit = r * 4 + c;
    grid &= !(1 << bit);
    grid |= v << bit;
    grid
}

fn board_hash(hole_i: usize, hole_j: usize, grid: u16) -> i32 {
    let mut h = (hole_i * 4 + hole_j) as i32;
    for r in 0..4 {
        for c in 0..4 {
            h = 2 * h + get_cell(grid, r, c) as i32;
        }
    }
    h
}

fn main() {
    let init_grid_vals = [
        [0, 1, 0, 0],
        [1, 1, 0, 0],
        [1, 1, 0, 0],
        [1, 1, 0, 0],
    ];
    let target_grid_vals = [
        [0, 0, 1, 0],
        [0, 1, 0, 1],
        [1, 0, 1, 0],
        [0, 1, 0, 1],
    ];

    let mut start_grid: u16 = 0;
    for r in 0..4 {
        for c in 0..4 {
            if init_grid_vals[r][c] != 0 {
                start_grid = set_cell(start_grid, r, c, 1);
            }
        }
    }

    let mut target_grid: u16 = 0;
    for r in 0..4 {
        for c in 0..4 {
            if target_grid_vals[r][c] != 0 {
                target_grid = set_cell(target_grid, r, c, 1);
            }
        }
    }

    let target_hash = board_hash(0, 0, target_grid);

    let dx: [i32; 4] = [-1, 1, 0, 0];
    let dy: [i32; 4] = [0, 0, -1, 1];
    let keys: [i64; 4] = [b'U' as i64, b'D' as i64, b'L' as i64, b'R' as i64];

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((0usize, 0usize, start_grid, 0i64));

    while let Some((hole_i, hole_j, grid, checksum)) = queue.pop_front() {
        let h = board_hash(hole_i, hole_j, grid);

        if h == target_hash {
            println!("{}", checksum);
            return;
        }

        if visited.contains_key(&h) { continue; }
        visited.insert(h, checksum);

        for d in 0..4 {
            let ni = hole_i as i32 - dx[d];
            let nj = hole_j as i32 - dy[d];
            if ni < 0 || ni >= 4 || nj < 0 || nj >= 4 { continue; }
            let ni = ni as usize;
            let nj = nj as usize;

            let val = get_cell(grid, ni, nj);
            let mut ng = grid;
            ng = set_cell(ng, hole_i, hole_j, val);
            ng = set_cell(ng, ni, nj, 0);

            let nh = board_hash(ni, nj, ng);
            if !visited.contains_key(&nh) {
                let nc = (checksum * C + keys[d]) % M;
                queue.push_back((ni, nj, ng, nc));
            }
        }
    }

    println!("0");
}
