// Project Euler 029: Distinct Powers
// Count distinct values of a^b for 2 <= a,b <= 100 via canonical prime factorizations.

use std::collections::HashSet;

fn main() {
    let mut seen = HashSet::new();

    for a in 2u32..=100 {
        // Factorize a
        let mut factors: Vec<(u32, u32)> = Vec::new();
        let mut num = a;
        let mut p = 2;
        while p * p <= num {
            let mut exp = 0;
            while num % p == 0 {
                num /= p;
                exp += 1;
            }
            if exp > 0 {
                factors.push((p, exp));
            }
            p += 1;
        }
        if num > 1 {
            factors.push((num, 1));
        }

        for b in 2u32..=100 {
            let key: Vec<(u32, u32)> = factors.iter().map(|&(p, e)| (p, e * b)).collect();
            seen.insert(key);
        }
    }

    println!("{}", seen.len());
}
