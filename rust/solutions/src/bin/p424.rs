// Project Euler 424: Kakuro puzzles
use std::fs;

const MAXCELLS: usize = 50;
const MAXCLUES: usize = 50;
const MAXSIZE: usize = 8;

#[derive(Clone, Default)]
struct Clue {
    sum_letters: [usize; 3],
    n_sum_letters: usize,
    cells: [usize; 6],
    n_cells: usize,
}

#[derive(Clone)]
struct Puzzle {
    size: usize,
    n_cells: usize,
    n_clues: usize,
    cell_letter: [i32; MAXCELLS],
    cell_clues: [[usize; 8]; MAXCELLS],
    n_cell_clues: [usize; MAXCELLS],
    clues: Vec<Clue>,
    nonzero_letters: u32,
    letter_to_cells: [[usize; MAXCELLS]; 10],
    n_letter_cells: [usize; 10],
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            size: 0, n_cells: 0, n_clues: 0,
            cell_letter: [-1; MAXCELLS],
            cell_clues: [[0; 8]; MAXCELLS],
            n_cell_clues: [0; MAXCELLS],
            clues: vec![Clue::default(); MAXCLUES],
            nonzero_letters: 0,
            letter_to_cells: [[0; MAXCELLS]; 10],
            n_letter_cells: [0; 10],
        }
    }
}

fn compute_target(p: &Puzzle, ci: usize, letter_vals: &[i32; 10]) -> i32 {
    let c = &p.clues[ci];
    let mut target = 0;
    for i in 0..c.n_sum_letters {
        let v = letter_vals[c.sum_letters[i]];
        if v < 0 { return -1; }
        target = target * 10 + v;
    }
    target
}

fn check_clue(p: &Puzzle, ci: usize, letter_vals: &[i32; 10], cell_vals: &[i32; MAXCELLS]) -> bool {
    let c = &p.clues[ci];
    let target = compute_target(p, ci, letter_vals);
    if target < 0 { return true; }

    let mut partial_sum = 0;
    let mut unassigned = 0;
    let mut seen = 0u32;
    for i in 0..c.n_cells {
        let v = cell_vals[c.cells[i]];
        if v > 0 {
            if seen & (1 << v) != 0 { return false; }
            seen |= 1 << v;
            partial_sum += v;
        } else {
            unassigned += 1;
        }
    }

    if unassigned == 0 { return partial_sum == target; }

    let remaining = target - partial_sum;
    let avail = 0x3FEu32 & !seen;
    let count = avail.count_ones() as i32;
    if count < unassigned { return false; }

    let mut min_sum = 0;
    let mut max_sum = 0;
    let mut cnt = 0;
    for v in 1..=9 {
        if cnt >= unassigned { break; }
        if avail & (1 << v) != 0 { min_sum += v; cnt += 1; }
    }
    cnt = 0;
    for v in (1..=9).rev() {
        if cnt >= unassigned { break; }
        if avail & (1 << v) != 0 { max_sum += v; cnt += 1; }
    }

    min_sum <= remaining && remaining <= max_sum
}

fn propagate(p: &Puzzle, letter_vals: &mut [i32; 10], cell_vals: &mut [i32; MAXCELLS], used_letters: &mut u32) -> bool {
    let mut changed = true;
    while changed {
        changed = false;
        for ci in 0..p.n_cells {
            let li = p.cell_letter[ci];
            if li < 0 { continue; }
            let li = li as usize;
            if letter_vals[li] >= 0 && cell_vals[ci] <= 0 {
                if letter_vals[li] < 1 || letter_vals[li] > 9 { return false; }
                cell_vals[ci] = letter_vals[li];
                changed = true;
            } else if cell_vals[ci] > 0 && letter_vals[li] < 0 {
                let v = cell_vals[ci];
                if *used_letters & (1 << v) != 0 { return false; }
                letter_vals[li] = v;
                *used_letters |= 1 << v;
                for k in 0..p.n_letter_cells[li] {
                    let ci2 = p.letter_to_cells[li][k];
                    if cell_vals[ci2] <= 0 { cell_vals[ci2] = v; changed = true; }
                    else if cell_vals[ci2] != v { return false; }
                }
                changed = true;
            } else if letter_vals[li] >= 0 && cell_vals[ci] > 0 {
                if letter_vals[li] != cell_vals[ci] { return false; }
            }
        }

        for ci_idx in 0..p.n_clues {
            let c = &p.clues[ci_idx];
            let target = compute_target(p, ci_idx, letter_vals);
            if target < 0 { continue; }

            let mut unassigned_ci: i32 = -2;
            let mut partial_sum = 0;
            let mut seen = 0u32;
            for i in 0..c.n_cells {
                let v = cell_vals[c.cells[i]];
                if v > 0 {
                    if seen & (1 << v) != 0 { return false; }
                    seen |= 1 << v;
                    partial_sum += v;
                } else if unassigned_ci == -2 {
                    unassigned_ci = c.cells[i] as i32;
                } else {
                    unassigned_ci = -1;
                }
            }

            if unassigned_ci >= 0 {
                let needed = target - partial_sum;
                if needed < 1 || needed > 9 || (seen & (1 << needed)) != 0 { return false; }
                cell_vals[unassigned_ci as usize] = needed;
                changed = true;
            }
        }
    }
    true
}

fn check_all(p: &Puzzle, letter_vals: &[i32; 10], cell_vals: &[i32; MAXCELLS]) -> bool {
    for ci_idx in 0..p.n_clues {
        if !check_clue(p, ci_idx, letter_vals, cell_vals) { return false; }
    }
    true
}

fn solve_bt(p: &Puzzle, letter_vals: &mut [i32; 10], cell_vals: &mut [i32; MAXCELLS], used_letters: &mut u32) -> bool {
    let save_l = *letter_vals;
    let save_c = *cell_vals;
    let save_used = *used_letters;

    if !propagate(p, letter_vals, cell_vals, used_letters) || !check_all(p, letter_vals, cell_vals) {
        *letter_vals = save_l; *cell_vals = save_c; *used_letters = save_used;
        return false;
    }

    // Find unassigned letter
    let mut best_li: i32 = -1;
    for li in 0..10 {
        if letter_vals[li] < 0 { best_li = li as i32; break; }
    }

    if best_li >= 0 {
        for val in 0..=9i32 {
            if *used_letters & (1 << val) != 0 { continue; }
            if (p.nonzero_letters & (1 << best_li as u32)) != 0 && val == 0 { continue; }

            let sl = *letter_vals;
            let sc = *cell_vals;
            let su = *used_letters;

            letter_vals[best_li as usize] = val;
            *used_letters |= 1 << val;

            if solve_bt(p, letter_vals, cell_vals, used_letters) { return true; }

            *letter_vals = sl; *cell_vals = sc; *used_letters = su;
        }
        return false;
    }

    // Find unassigned cell
    let mut best_ci: i32 = -1;
    for ci in 0..p.n_cells {
        if cell_vals[ci] <= 0 { best_ci = ci as i32; break; }
    }

    if best_ci < 0 { return true; }

    let mut used_in_groups = 0u32;
    for k in 0..p.n_cell_clues[best_ci as usize] {
        let c = &p.clues[p.cell_clues[best_ci as usize][k]];
        for i in 0..c.n_cells {
            if cell_vals[c.cells[i]] > 0 {
                used_in_groups |= 1 << cell_vals[c.cells[i]];
            }
        }
    }

    for val in 1..=9i32 {
        if used_in_groups & (1 << val) != 0 { continue; }

        let sl = *letter_vals;
        let sc = *cell_vals;
        let su = *used_letters;

        cell_vals[best_ci as usize] = val;

        if solve_bt(p, letter_vals, cell_vals, used_letters) { return true; }

        *letter_vals = sl; *cell_vals = sc; *used_letters = su;
    }
    false
}

fn parse_puzzle(line: &str) -> Puzzle {
    let mut p = Puzzle::default();

    // Split into tokens by comma, handling parenthesized groups
    let mut tokens: Vec<String> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ',' { i += 1; continue; }
        let mut tok = String::new();
        if chars[i] == '(' {
            while i < chars.len() && chars[i] != ')' {
                tok.push(chars[i]);
                i += 1;
            }
            if i < chars.len() { tok.push(chars[i]); i += 1; }
        } else {
            while i < chars.len() && chars[i] != ',' {
                tok.push(chars[i]);
                i += 1;
            }
        }
        tokens.push(tok);
    }

    p.size = tokens[0].parse().unwrap();
    let mut grid = vec![vec![String::new(); MAXSIZE]; MAXSIZE];
    for i in 0..p.size {
        for j in 0..p.size {
            grid[i][j] = tokens[1 + i * p.size + j].clone();
        }
    }

    let mut cell_map = [[-1i32; MAXSIZE]; MAXSIZE];

    for i in 0..p.size {
        for j in 0..p.size {
            let c = &grid[i][j];
            if c.starts_with('X') || c.starts_with('(') { continue; }
            let ci = p.n_cells;
            p.n_cells += 1;
            cell_map[i][j] = ci as i32;
            let first = c.chars().next().unwrap_or(' ');
            if first >= 'A' && first <= 'J' {
                let li = (first as u8 - b'A') as usize;
                p.cell_letter[ci] = li as i32;
                p.letter_to_cells[li][p.n_letter_cells[li]] = ci;
                p.n_letter_cells[li] += 1;
            }
        }
    }

    for i in 0..p.size {
        for j in 0..p.size {
            let c = grid[i][j].clone();
            if !c.starts_with('(') { continue; }

            let inner = &c[1..c.len() - 1];
            for tok in inner.split(',') {
                let tok = tok.trim();
                if tok.is_empty() { continue; }
                let dir = tok.chars().next().unwrap();
                let letters_str = &tok[1..];

                let cl = &mut p.clues[p.n_clues];
                cl.n_sum_letters = 0;
                cl.n_cells = 0;

                for ch in letters_str.chars() {
                    if ch >= 'A' && ch <= 'J' {
                        cl.sum_letters[cl.n_sum_letters] = (ch as u8 - b'A') as usize;
                        cl.n_sum_letters += 1;
                    }
                }

                if cl.n_sum_letters >= 2 {
                    p.nonzero_letters |= 1 << cl.sum_letters[0];
                }

                if dir == 'h' {
                    for dj in 1..p.size {
                        if j + dj >= p.size || cell_map[i][j + dj] < 0 { break; }
                        cl.cells[cl.n_cells] = cell_map[i][j + dj] as usize;
                        cl.n_cells += 1;
                    }
                } else if dir == 'v' {
                    for di in 1..p.size {
                        if i + di >= p.size || cell_map[i + di][j] < 0 { break; }
                        cl.cells[cl.n_cells] = cell_map[i + di][j] as usize;
                        cl.n_cells += 1;
                    }
                }

                if cl.n_cells > 0 {
                    for k in 0..cl.n_cells {
                        let ci = cl.cells[k];
                        p.cell_clues[ci][p.n_cell_clues[ci]] = p.n_clues;
                        p.n_cell_clues[ci] += 1;
                    }
                    p.n_clues += 1;
                }
            }
        }
    }

    p
}

fn main() {
    let content = fs::read_to_string("data/kakuro200.txt")
        .or_else(|_| fs::read_to_string("../data/kakuro200.txt"))
        .expect("Cannot open kakuro200.txt");

    let mut total: i64 = 0;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let p = parse_puzzle(line);
        let mut letter_vals = [-1i32; 10];
        let mut cell_vals = [0i32; MAXCELLS];
        let mut used_letters = 0u32;

        if solve_bt(&p, &mut letter_vals, &mut cell_vals, &mut used_letters) {
            let mut val: i64 = 0;
            for i in 0..10 {
                val = val * 10 + if letter_vals[i] >= 0 { letter_vals[i] as i64 } else { 0 };
            }
            total += val;
        }
    }

    println!("{}", total);
}
