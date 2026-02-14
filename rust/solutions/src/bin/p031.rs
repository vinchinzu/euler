// Project Euler 031: Coin Sums
// Number of ways to make 200p using British coins.

fn main() {
    const TARGET: usize = 200;
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];

    let mut ways = [0u64; TARGET + 1];
    ways[0] = 1;

    for &c in &coins {
        for amount in c..=TARGET {
            ways[amount] += ways[amount - c];
        }
    }

    println!("{}", ways[TARGET]);
}
