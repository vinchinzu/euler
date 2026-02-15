// Project Euler 539 - Odd Elimination
//
// P(n) = last number after alternately removing every other from left then right.
// S(n) = sum_{k=1}^n P(k) mod M. Recursive with memoization.

use std::collections::HashMap;

const M: i64 = 987_654_321;

// M = 3^2 * 17^2 * 379721, NOT prime. So Fermat's little theorem doesn't apply.
// Since M is odd, inv2 = (M + 1) / 2.
const INV2: i64 = (M + 1) / 2;

fn tr(n: i64) -> i64 {
    // n*(n+1)/2 mod M
    let a = n % M;
    let b = (n + 1) % M;
    // a * b fits in i64 since both < 10^9
    (a * b % M * INV2) % M
}

struct Solver {
    ht_p: HashMap<i64, i64>,
    ht_s: HashMap<i64, i64>,
}

impl Solver {
    fn new() -> Self {
        Solver {
            ht_p: HashMap::new(),
            ht_s: HashMap::new(),
        }
    }

    fn p(&mut self, n: i64) -> i64 {
        if n <= 1 { return n; }
        if let Some(&v) = self.ht_p.get(&n) { return v; }
        let result = if n % 2 == 1 {
            self.p(n - 1)
        } else {
            n + 2 - 2 * self.p(n / 2)
        };
        self.ht_p.insert(n, result);
        result
    }

    fn s(&mut self, n: i64) -> i64 {
        if n <= 1 { return n % M; }
        if let Some(&v) = self.ht_s.get(&n) { return v; }
        let result = if n % 2 == 0 {
            (self.p(n) % M + self.s(n - 1)) % M
        } else {
            let half = n / 2;
            let s_half = self.s(half);
            (1 + 2 * (2 * tr(half) + 2 * (half % M) - 2 * s_half)) % M
        };
        let result = ((result % M) + M) % M;
        self.ht_s.insert(n, result);
        result
    }
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;
    let mut solver = Solver::new();
    let result = solver.s(n) % M;
    println!("{result}");
}
