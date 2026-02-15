// Project Euler 221: Alexandrian Integers

use rayon::prelude::*;

fn main() {
    let n_target = 150_000usize;
    let l = 80_000i64;

    let mut alexandrians: Vec<i64> = (1..=l).into_par_iter()
        .flat_map(|a| {
            let n = a * a + 1;
            let sq = (n as f64).sqrt() as i64;
            let mut local = Vec::new();
            for d in 1..=sq {
                if n % d == 0 {
                    let e = n / d;
                    let p = a + d;
                    let q = a + e;
                    let a_val = a * p;
                    if a_val <= 9_000_000_000_000_000_000i64 / q {
                        local.push(a_val * q);
                    }
                }
            }
            local
        })
        .collect();

    alexandrians.sort();
    alexandrians.dedup();

    println!("{}", alexandrians[n_target - 1]);
}
