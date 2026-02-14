// Project Euler 96: Su Doku
// Solve 50 Sudoku puzzles and sum the 3-digit numbers from top-left corners.

type Grid = [[u8; 9]; 9];

fn find_empty(grid: &Grid) -> Option<(usize, usize)> {
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c] == 0 {
                return Some((r, c));
            }
        }
    }
    None
}

fn is_safe(grid: &Grid, row: usize, col: usize, num: u8) -> bool {
    for c in 0..9 {
        if grid[row][c] == num { return false; }
    }
    for r in 0..9 {
        if grid[r][col] == num { return false; }
    }
    let br = row - row % 3;
    let bc = col - col % 3;
    for r in 0..3 {
        for c in 0..3 {
            if grid[br + r][bc + c] == num { return false; }
        }
    }
    true
}

fn solve(grid: &mut Grid) -> bool {
    let (row, col) = match find_empty(grid) {
        Some(pos) => pos,
        None => return true,
    };
    for num in 1..=9 {
        if is_safe(grid, row, col, num) {
            grid[row][col] = num;
            if solve(grid) { return true; }
            grid[row][col] = 0;
        }
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
            for r in 0..9 {
                if i + 1 + r >= lines.len() { break; }
                let row_bytes = lines[i + 1 + r].as_bytes();
                for c in 0..9 {
                    if c < row_bytes.len() {
                        grid[r][c] = row_bytes[c] - b'0';
                    }
                }
            }
            if solve(&mut grid) {
                total_sum += grid[0][0] as u32 * 100 + grid[0][1] as u32 * 10 + grid[0][2] as u32;
            }
            i += 10;
        } else {
            i += 1;
        }
    }

    println!("{total_sum}");
}
