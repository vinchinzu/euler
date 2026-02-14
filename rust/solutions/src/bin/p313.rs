// Project Euler Problem 313: Sliding game
// For each odd prime p < 10^6, count grids via formula.
use euler_utils::sieve;

fn main() {
    let limit = 1_000_000usize;
    let sv = sieve(limit);

    let mut total: i64 = 0;
    for p in (3..limit).filter(|&i| sv[i]) {
        let sq = p as i64 * p as i64;
        let t = (sq + 13) / 2;
        let a_min = t / 4 + 1;
        let a_max = (t - 2) / 3;
        if a_max >= a_min {
            total += 2 * (a_max - a_min + 1);
        }
    }

    println!("{}", total);
}
