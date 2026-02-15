// Project Euler 649 - Low-Prime Chessboard Nim
// Moves: 2, 3, 5, 7 squares left or up.
// N = 10000019, C = 100 coins, L = 9 (period), H = 8 (max nimber).

const N: i64 = 10_000_019;
const C_COINS: usize = 100;
const M: i64 = 1_000_000_000;
const L: usize = 9;
const H: usize = 8;

fn ceil_div(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

fn main() {
    // Compute grid nimbers
    let moves = [2usize, 3, 5, 7];
    let mut grid = [[0i32; L]; L];

    for i in 0..L {
        for j in 0..L {
            let mut used = [false; H];
            for &d in &moves {
                if i >= d && (grid[i - d][j] as usize) < H {
                    used[grid[i - d][j] as usize] = true;
                }
                if j >= d && (grid[i][j - d] as usize) < H {
                    used[grid[i][j - d] as usize] = true;
                }
            }
            let mut nimber = 0;
            while nimber < H && used[nimber] { nimber += 1; }
            grid[i][j] = nimber as i32;
        }
    }

    // Count squares with each nimber value
    let mut counts = [0i64; H];
    for i in 0..L {
        for j in 0..L {
            let count_i = ceil_div(N - i as i64, L as i64);
            let count_j = ceil_div(N - j as i64, L as i64);
            let v = grid[i][j] as usize;
            counts[v] = (counts[v] + count_i * count_j) % M;
        }
    }

    // DP: dp[c][total] = number of ways with c coins and nimber total
    // Flat array: dp[c * H + total]
    let mut dp = vec![0i64; (C_COINS + 1) * H];
    dp[0 * H + 0] = 1;

    for c in 1..=C_COINS {
        for total in 0..H {
            let mut sum = 0i64;
            for curr in 0..H {
                sum = (sum + dp[(c - 1) * H + (total ^ curr)] * counts[curr]) % M;
            }
            dp[c * H + total] = sum;
        }
    }

    let mut ans = 0i64;
    for curr in 1..H {
        ans = (ans + dp[C_COINS * H + curr]) % M;
    }

    println!("{}", ans);
}
