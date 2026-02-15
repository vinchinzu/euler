// Project Euler 300: Protein Folding
use std::collections::HashSet;

const N_LEN: usize = 15;
const HALF: usize = 7;
const GRID_SIZE: usize = 2 * N_LEN + 1;
const OFFSET: i32 = N_LEN as i32;

const DX: [i32; 4] = [1, -1, 0, 0];
const DY: [i32; 4] = [0, 0, 1, -1];

fn dfs(step: usize, x: i32, y: i32, bitset: u64,
       grid: &mut [[i32; GRID_SIZE]; GRID_SIZE],
       bs_set: &mut HashSet<u64>, bitsets: &mut Vec<u64>) {
    if step == N_LEN {
        if bs_set.insert(bitset) {
            bitsets.push(bitset);
        }
        return;
    }

    for d in 0..4usize {
        if step == 1 && d != 0 { continue; }
        if step == 2 && d == 1 { continue; }
        if step == 2 && d == 3 { continue; }

        let nx = x + DX[d];
        let ny = y + DY[d];
        let gx = (nx + OFFSET) as usize;
        let gy = (ny + OFFSET) as usize;

        if gx >= GRID_SIZE || gy >= GRID_SIZE { continue; }
        if grid[gy][gx] >= 0 { continue; }

        let mut new_bits: u64 = 0;
        for dd in 0..4 {
            let nnx = nx + DX[dd];
            let nny = ny + DY[dd];
            let ggx = (nnx + OFFSET) as usize;
            let ggy = (nny + OFFSET) as usize;
            if ggx >= GRID_SIZE || ggy >= GRID_SIZE { continue; }
            let prev = grid[ggy][ggx];
            if prev < 0 { continue; }
            let prev = prev as usize;
            if step % 2 == 0 && prev % 2 == 1 {
                new_bits |= 1u64 << (HALF * (step / 2) + prev / 2);
            } else if step % 2 == 1 && prev % 2 == 0 {
                new_bits |= 1u64 << (HALF * (prev / 2) + step / 2);
            }
        }

        grid[gy][gx] = step as i32;
        dfs(step + 1, nx, ny, bitset | new_bits, grid, bs_set, bitsets);
        grid[gy][gx] = -1;
    }
}

fn main() {
    let mut grid = [[-1i32; GRID_SIZE]; GRID_SIZE];
    let mut bs_set = HashSet::new();
    let mut bitsets = Vec::new();

    grid[OFFSET as usize][OFFSET as usize] = 0;
    dfs(1, 0, 0, 0, &mut grid, &mut bs_set, &mut bitsets);

    // Prune dominated bitsets
    let n = bitsets.len();
    let mut keep = vec![true; n];
    for i in 0..n {
        if !keep[i] { continue; }
        for j in 0..n {
            if i == j || !keep[j] { continue; }
            if (bitsets[j] & bitsets[i]) == bitsets[j] && bitsets[j] != bitsets[i] {
                keep[j] = false;
            }
        }
    }
    let pruned: Vec<u64> = (0..n).filter(|&i| keep[i]).map(|i| bitsets[i]).collect();

    // For each protein mask, compute max contacts
    let total_proteins = 1usize << N_LEN;
    let mut sum_contacts: i64 = 0;

    for protein in 0..total_proteins {
        let mut even_bits = 0u32;
        for i in 0..8 {
            if protein & (1 << (2 * i)) != 0 { even_bits |= 1 << i; }
        }
        let mut odd_bits = 0u32;
        for j in 0..7 {
            if protein & (1 << (2 * j + 1)) != 0 { odd_bits |= 1 << j; }
        }
        let mut pbs: u64 = 0;
        for i in 0..8 {
            if even_bits & (1 << i) != 0 {
                pbs |= (odd_bits as u64) << (7 * i);
            }
        }

        let mut max_contacts = 0;
        for &bs in &pruned {
            let c = (pbs & bs).count_ones();
            if c > max_contacts { max_contacts = c; }
        }
        sum_contacts += max_contacts as i64;
    }

    let avg = sum_contacts as f64 / total_proteins as f64;
    println!("{:.13}", avg);
}
