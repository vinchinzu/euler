// Project Euler 67: Maximum path sum II
// Bottom-up DP on a 100-row triangle read from data file.

fn main() {
    let data = include_str!("../../../../data/0067_triangle.txt");
    let mut tri: Vec<Vec<u32>> = Vec::new();

    for line in data.lines() {
        let row: Vec<u32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if !row.is_empty() {
            tri.push(row);
        }
    }

    // Bottom-up DP
    for i in (0..tri.len() - 1).rev() {
        for j in 0..=i {
            tri[i][j] += tri[i + 1][j].max(tri[i + 1][j + 1]);
        }
    }

    println!("{}", tri[0][0]);
}
