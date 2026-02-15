// Project Euler 755 - Not Zeckendorf
// Count ways to express numbers up to N as sums of distinct Fibonacci numbers.
// Uses memoized recursion with hash maps per Fibonacci index.

use std::collections::HashMap;

fn main() {
    const TARGET: i64 = 10_000_000_000_000; // 10^13

    let mut fibs = vec![1i64, 2];
    while *fibs.last().unwrap() < 1_000_000_000_000_000 {
        let len = fibs.len();
        fibs.push(fibs[len - 1] + fibs[len - 2]);
    }
    let num_fibs = fibs.len();

    let mut maps: Vec<HashMap<i64, i64>> = vec![HashMap::new(); num_fibs];

    fn helper(
        index: i32,
        n: i64,
        fibs: &[i64],
        num_fibs: usize,
        maps: &mut [HashMap<i64, i64>],
    ) -> i64 {
        if n < 0 { return 0; }
        if index < 0 { return 1; }
        let idx = index as usize;

        if let Some(&cached) = maps[idx].get(&n) {
            return cached;
        }

        // If sum of all remaining fibs <= n+1, any subset works
        if idx + 2 < num_fibs && fibs[idx + 2] <= n + 2 {
            let result = 1i64 << (idx + 1);
            maps[idx].insert(n, result);
            return result;
        }

        let result = helper(index - 1, n, fibs, num_fibs, maps)
            + helper(index - 1, n - fibs[idx], fibs, num_fibs, maps);
        maps[idx].insert(n, result);
        result
    }

    let result = helper(
        (num_fibs as i32) - 3,
        TARGET,
        &fibs,
        num_fibs,
        &mut maps,
    );
    println!("{}", result);
}
