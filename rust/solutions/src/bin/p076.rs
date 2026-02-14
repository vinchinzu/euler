// Project Euler 76: Counting summations
// Number of ways to write 100 as a sum of at least two positive integers.
// Classic partition DP, excluding the trivial partition {100} itself.

fn main() {
    let target = 100usize;
    let mut ways = vec![0i64; target + 1];
    ways[0] = 1;

    // Parts from 1 to target-1 (exclude target itself to disallow n = 100 alone)
    for part in 1..target {
        for s in part..=target {
            ways[s] += ways[s - part];
        }
    }

    println!("{}", ways[target]);
}
