// Project Euler 88: Product-sum numbers
// Find the sum of all minimal product-sum numbers for k = 2..12000.

use std::collections::HashSet;

const K_MAX: usize = 12000;
const ULIMIT: usize = 2 * K_MAX;

fn main() {
    let mut min_n = vec![ULIMIT as u64; K_MAX + 1];

    fn search(prod: u64, sum: u64, num_factors: u64, start: u64, min_n: &mut [u64]) {
        let k = (prod - sum + num_factors) as usize;
        if k >= 2 && k <= K_MAX {
            min_n[k] = min_n[k].min(prod);
        }
        for i in start..=(ULIMIT as u64 / prod) {
            search(prod * i, sum + i, num_factors + 1, i, min_n);
        }
    }

    search(1, 0, 0, 2, &mut min_n);

    let unique: HashSet<u64> = (2..=K_MAX).map(|k| min_n[k]).collect();
    let total: u64 = unique.iter().sum();
    println!("{total}");
}
