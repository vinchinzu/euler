// Project Euler 766 - Sliding Block Puzzle
// DFS exploring all reachable configurations with indistinguishable same-shape pieces.

use std::collections::HashMap;

const H: usize = 5;
const W: usize = 6;
const NCELLS: usize = H * W;
const MAX_PIECES: usize = 14;

const GRID_STR: [&str; H] = [
    ".AABCC",
    ".ABBCD",
    "EFGGHD",
    "IJGGHK",
    "LMNNKK",
];

const DY: [i32; 4] = [-1, 1, 0, 0];
const DX: [i32; 4] = [0, 0, -1, 1];

fn main() {
    // Parse grid
    let mut cell_count = [0usize; 26];
    let mut cell_pos = [[0usize; 4]; 26];
    for y in 0..H {
        for (x, ch) in GRID_STR[y].chars().enumerate() {
            if ch != '.' {
                let ci = (ch as u8 - b'A') as usize;
                cell_pos[ci][cell_count[ci]] = y * W + x;
                cell_count[ci] += 1;
            }
        }
    }

    // Identify pieces and shape types
    let mut npieces = 0usize;
    let mut piece_ncells = [0usize; MAX_PIECES];
    let mut piece_cells = [[0usize; 4]; MAX_PIECES];
    let mut piece_type = [0u8; MAX_PIECES];

    let mut shapes: Vec<(usize, Vec<(i32, i32)>)> = Vec::new();

    for ci in 0..26 {
        if cell_count[ci] == 0 { continue; }
        let pi = npieces;
        npieces += 1;
        let nc = cell_count[ci];
        piece_ncells[pi] = nc;
        for j in 0..nc {
            piece_cells[pi][j] = cell_pos[ci][j];
        }

        // Compute relative shape
        let mut rdy: Vec<i32> = Vec::new();
        let mut rdx: Vec<i32> = Vec::new();
        let min_y = (0..nc).map(|j| cell_pos[ci][j] / W).min().unwrap() as i32;
        let min_x = (0..nc).map(|j| cell_pos[ci][j] % W).min().unwrap() as i32;
        for j in 0..nc {
            rdy.push(cell_pos[ci][j] as i32 / W as i32 - min_y);
            rdx.push(cell_pos[ci][j] as i32 % W as i32 - min_x);
        }
        let mut pairs: Vec<(i32, i32)> = rdy.into_iter().zip(rdx).collect();
        pairs.sort();

        let found = shapes.iter().position(|(sn, sp)| *sn == nc && *sp == pairs);
        let type_id = match found {
            Some(idx) => idx,
            None => {
                shapes.push((nc, pairs));
                shapes.len() - 1
            }
        };
        piece_type[pi] = (type_id + 1) as u8;
    }

    // State: tgrid (type grid)
    let mut pgrid = [0u8; NCELLS];
    let mut tgrid = [0u8; NCELLS];
    let mut poff_y = [0i32; MAX_PIECES];
    let mut poff_x = [0i32; MAX_PIECES];

    let place = |pi: usize, pgrid: &mut [u8; NCELLS], tgrid: &mut [u8; NCELLS], poff_y: &[i32], poff_x: &[i32], piece_ncells: &[usize], piece_cells: &[[usize; 4]], piece_type: &[u8]| {
        for j in 0..piece_ncells[pi] {
            let orig = piece_cells[pi][j];
            let r = (orig / W) as i32 + poff_y[pi];
            let c = (orig % W) as i32 + poff_x[pi];
            pgrid[(r as usize) * W + c as usize] = (pi + 1) as u8;
            tgrid[(r as usize) * W + c as usize] = piece_type[pi];
        }
    };

    let remove = |pi: usize, pgrid: &mut [u8; NCELLS], tgrid: &mut [u8; NCELLS], poff_y: &[i32], poff_x: &[i32], piece_ncells: &[usize], piece_cells: &[[usize; 4]]| {
        for j in 0..piece_ncells[pi] {
            let orig = piece_cells[pi][j];
            let r = (orig / W) as i32 + poff_y[pi];
            let c = (orig % W) as i32 + poff_x[pi];
            pgrid[(r as usize) * W + c as usize] = 0;
            tgrid[(r as usize) * W + c as usize] = 0;
        }
    };

    for i in 0..npieces {
        place(i, &mut pgrid, &mut tgrid, &poff_y, &poff_x, &piece_ncells, &piece_cells, &piece_type);
    }

    let mut visited: HashMap<[u8; NCELLS], bool> = HashMap::new();
    visited.insert(tgrid, true);

    // Iterative DFS
    struct Frame { ti: usize, di: usize, moved: bool }
    let mut stack: Vec<Frame> = vec![Frame { ti: 0, di: 0, moved: false }];

    let can_move = |ti: usize, di: usize, pgrid: &[u8; NCELLS], poff_y: &[i32], poff_x: &[i32], piece_ncells: &[usize], piece_cells: &[[usize; 4]]| -> bool {
        let pid = (ti + 1) as u8;
        for j in 0..piece_ncells[ti] {
            let orig = piece_cells[ti][j];
            let nr = (orig / W) as i32 + poff_y[ti] + DY[di];
            let nc = (orig % W) as i32 + poff_x[ti] + DX[di];
            if nr < 0 || nr >= H as i32 || nc < 0 || nc >= W as i32 { return false; }
            let v = pgrid[nr as usize * W + nc as usize];
            if v != 0 && v != pid { return false; }
        }
        true
    };

    while let Some(f) = stack.last_mut() {
        if f.moved {
            let ti = f.ti;
            let di = f.di;
            remove(ti, &mut pgrid, &mut tgrid, &poff_y, &poff_x, &piece_ncells, &piece_cells);
            poff_y[ti] -= DY[di];
            poff_x[ti] -= DX[di];
            place(ti, &mut pgrid, &mut tgrid, &poff_y, &poff_x, &piece_ncells, &piece_cells, &piece_type);
            f.moved = false;
            f.di += 1;
            if f.di >= 4 {
                f.di = 0;
                f.ti += 1;
            }
        }

        let mut found = false;
        while stack.last().unwrap().ti < npieces {
            let f = stack.last_mut().unwrap();
            while f.di < 4 {
                if can_move(f.ti, f.di, &pgrid, &poff_y, &poff_x, &piece_ncells, &piece_cells) {
                    let ti = f.ti;
                    let di = f.di;
                    remove(ti, &mut pgrid, &mut tgrid, &poff_y, &poff_x, &piece_ncells, &piece_cells);
                    poff_y[ti] += DY[di];
                    poff_x[ti] += DX[di];
                    place(ti, &mut pgrid, &mut tgrid, &poff_y, &poff_x, &piece_ncells, &piece_cells, &piece_type);
                    f.moved = true;

                    if !visited.contains_key(&tgrid) {
                        visited.insert(tgrid, true);
                        stack.push(Frame { ti: 0, di: 0, moved: false });
                    }
                    found = true;
                    break;
                }
                f.di += 1;
            }
            if found { break; }
            let f = stack.last_mut().unwrap();
            f.di = 0;
            f.ti += 1;
        }

        if !found {
            stack.pop();
        }
    }

    println!("{}", visited.len());
}
