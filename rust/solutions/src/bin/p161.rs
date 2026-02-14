// Project Euler 161: Triominoes
// Backtracking with hash-map memoization on 108-bit grid state.

use std::collections::HashMap;

const WIDTH: usize = 9;
const HEIGHT: usize = 12;

const SHAPES: [[(i32, i32); 3]; 6] = [
    [(0,0), (0,1), (0,2)],    // I horizontal
    [(0,0), (1,0), (2,0)],    // I vertical
    [(0,0), (0,1), (1,0)],    // L shape 1
    [(0,0), (1,0), (1,1)],    // L shape 2
    [(0,0), (0,1), (1,1)],    // L shape 3
    [(0,0), (1,0), (1,-1)],   // L shape 4
];

fn get_bit(lo: u64, hi: u64, pos: usize) -> bool {
    if pos < 64 { (lo >> pos) & 1 == 1 }
    else { (hi >> (pos - 64)) & 1 == 1 }
}

fn set_bit(lo: u64, hi: u64, pos: usize) -> (u64, u64) {
    if pos < 64 { (lo | (1u64 << pos), hi) }
    else { (lo, hi | (1u64 << (pos - 64))) }
}

fn backtrack(lo: u64, hi: u64, memo: &mut HashMap<(u64, u64), i64>) -> i64 {
    if let Some(&v) = memo.get(&(lo, hi)) {
        return v;
    }

    // Find first empty cell
    let mut pos = None;
    for i in 0..(WIDTH * HEIGHT) {
        if !get_bit(lo, hi, i) {
            pos = Some(i);
            break;
        }
    }

    let pos = match pos {
        None => { memo.insert((lo, hi), 1); return 1; }
        Some(p) => p,
    };

    let r = (pos / WIDTH) as i32;
    let c = (pos % WIDTH) as i32;

    let mut count: i64 = 0;

    for s in &SHAPES {
        let mut ok = true;
        let mut positions = [0usize; 3];
        for (k, &(dr, dc)) in s.iter().enumerate() {
            let nr = r + dr;
            let nc = c + dc;
            if nr < 0 || nr >= HEIGHT as i32 || nc < 0 || nc >= WIDTH as i32 {
                ok = false;
                break;
            }
            let p = nr as usize * WIDTH + nc as usize;
            if get_bit(lo, hi, p) {
                ok = false;
                break;
            }
            positions[k] = p;
        }
        if !ok { continue; }

        let mut nlo = lo;
        let mut nhi = hi;
        for &p in &positions {
            let (nl, nh) = set_bit(nlo, nhi, p);
            nlo = nl;
            nhi = nh;
        }

        count += backtrack(nlo, nhi, memo);
    }

    memo.insert((lo, hi), count);
    count
}

fn main() {
    let mut memo = HashMap::new();
    let result = backtrack(0, 0, &mut memo);
    println!("{}", result);
}
