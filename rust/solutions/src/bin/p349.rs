// Project Euler 349: Langton's Ant
const L: usize = 20000;
const GRID: usize = 512;

fn main() {
    let big_n: i64 = 1_000_000_000_000_000_000;
    let p: i64 = 104;

    let mut grid = vec![vec![false; GRID]; GRID];
    let mut black_count: i64 = 0;
    let mut ax = GRID / 2;
    let mut ay = GRID / 2;
    let dx = [0i32, 1, 0, -1];
    let dy = [1i32, 0, -1, 0];
    let mut d: usize = 0;

    let mut num_blacks = vec![0i64; L];

    for step in 0..L {
        num_blacks[step] = black_count;
        if grid[ax][ay] {
            d = (d + 1) % 4;
            grid[ax][ay] = false;
            black_count -= 1;
        } else {
            d = (d + 3) % 4;
            grid[ax][ay] = true;
            black_count += 1;
        }
        ax = (ax as i32 + dx[d]) as usize;
        ay = (ay as i32 + dy[d]) as usize;
    }

    let base = ((L as i64 - p) / p) * p + big_n % p;
    let ans = num_blacks[base as usize] + (big_n - base) / p * (num_blacks[base as usize] - num_blacks[(base - p) as usize]);
    println!("{ans}");
}
