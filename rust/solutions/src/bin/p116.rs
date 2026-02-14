// Project Euler 116: Red, Green or Blue Tiles
// Count ways to tile a row of 50, using tiles of one colour only.

fn ways_for_colour(row_length: usize, tile_length: usize) -> i64 {
    let mut dp = vec![0i64; row_length + 1];
    dp[0] = 1;
    for i in 1..=row_length {
        dp[i] = dp[i - 1];
        if i >= tile_length {
            dp[i] += dp[i - tile_length];
        }
    }
    dp[row_length] - 1 // subtract all-grey case
}

fn main() {
    let total = ways_for_colour(50, 2) + ways_for_colour(50, 3) + ways_for_colour(50, 4);
    println!("{total}");
}
