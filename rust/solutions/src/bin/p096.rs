// Project Euler 96: Su Doku
// Solve 50 Sudoku puzzles and sum the 3-digit numbers from top-left corners.

type Grid = [[u8; 9]; 9];

fn box_id(r: usize, c: usize) -> usize {
    (r / 3) * 3 + c / 3
}

fn solve(
    grid: &mut Grid,
    row: &mut [u16; 9],
    col: &mut [u16; 9],
    bx: &mut [u16; 9],
) -> bool {
    let mut br = 0;
    let mut bc = 0;
    let mut best_mask = 0u16;
    let mut best_bits = 10u32;
    let mut filled = 0usize;

    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c] != 0 {
                filled += 1;
                continue;
            }
            let mask = 0x1FF & !(row[r] | col[c] | bx[box_id(r, c)]);
            let bits = mask.count_ones();
            if bits == 0 {
                return false;
            }
            if bits < best_bits {
                best_bits = bits;
                best_mask = mask;
                br = r;
                bc = c;
                if bits == 1 {
                    break;
                }
            }
        }
    }
    if filled == 81 {
        return true;
    }

    let mut mask = best_mask;
    let b = box_id(br, bc);
    while mask != 0 {
        let bit = mask & mask.wrapping_neg();
        mask ^= bit;
        let num = bit.trailing_zeros() as u8 + 1;
        grid[br][bc] = num;
        row[br] |= bit;
        col[bc] |= bit;
        bx[b] |= bit;
        if solve(grid, row, col, bx) {
            return true;
        }
        grid[br][bc] = 0;
        row[br] ^= bit;
        col[bc] ^= bit;
        bx[b] ^= bit;
    }
    false
}

fn main() {
    let data = include_str!("../../../../data/p096_sudoku.txt");
    let lines: Vec<&str> = data.lines().collect();
    let mut total_sum = 0u32;
    let mut i = 0;

    while i < lines.len() {
        if lines[i].starts_with("Grid") {
            let mut grid = [[0u8; 9]; 9];
            let mut row = [0u16; 9];
            let mut col = [0u16; 9];
            let mut bx = [0u16; 9];
            for r in 0..9 {
                if i + 1 + r >= lines.len() {
                    break;
                }
                let row_bytes = lines[i + 1 + r].as_bytes();
                for c in 0..9 {
                    if c < row_bytes.len() {
                        let v = row_bytes[c] - b'0';
                        grid[r][c] = v;
                        if v != 0 {
                            let bit = 1u16 << (v - 1);
                            row[r] |= bit;
                            col[c] |= bit;
                            bx[box_id(r, c)] |= bit;
                        }
                    }
                }
            }
            if solve(&mut grid, &mut row, &mut col, &mut bx) {
                total_sum += grid[0][0] as u32 * 100 + grid[0][1] as u32 * 10 + grid[0][2] as u32;
            }
            i += 10;
        } else {
            i += 1;
        }
    }

    println!("{total_sum}");
}
