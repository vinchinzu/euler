// Project Euler 346: Strong Repunits
use std::collections::HashSet;

fn main() {
    let limit: i64 = 1_000_000_000_000;
    let max_base: i64 = 1_000_000;

    let mut found: HashSet<i64> = HashSet::new();

    for b in 2..=max_base {
        let mut rep = 1 + b + b * b;
        while rep < limit {
            found.insert(rep);
            rep = rep * b + 1;
        }
    }

    let total: i64 = 1 + found.iter().sum::<i64>(); // include 1
    println!("{}", total);
}
