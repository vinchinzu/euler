// Project Euler 82: Path sum three ways
// Minimum path through matrix moving right, up, or down.

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

    // Initialize first column
    for i in 0..n {
        dp[i][0] = matrix[i][0];
    }

    for j in 1..n {
        // Pass 1: from left
        for i in 0..n {
            dp[i][j] = dp[i][j - 1] + matrix[i][j];
        }
        // Pass 2: from above
        for i in 1..n {
            dp[i][j] = dp[i][j].min(dp[i - 1][j] + matrix[i][j]);
        }
        // Pass 3: from below
        for i in (0..n - 1).rev() {
            dp[i][j] = dp[i][j].min(dp[i + 1][j] + matrix[i][j]);
        }
    }

    let result = (0..n).map(|i| dp[i][n - 1]).min().unwrap();
    println!("{result}");
}
