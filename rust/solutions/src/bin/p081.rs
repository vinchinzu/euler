// Project Euler 81: Path sum two ways
// DP minimum path through 80x80 matrix, moving only right and down.

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

    let mut dp = vec![vec![0i64; n]; n];
    dp[0][0] = matrix[0][0];
    for j in 1..n {
        dp[0][j] = matrix[0][j] + dp[0][j - 1];
    }
    for i in 1..n {
        dp[i][0] = matrix[i][0] + dp[i - 1][0];
    }
    for i in 1..n {
        for j in 1..n {
            dp[i][j] = matrix[i][j] + dp[i - 1][j].min(dp[i][j - 1]);
        }
    }

    println!("{}", dp[n - 1][n - 1]);
}
