// Project Euler 535 - Fractal Sequence
//
// Recursive computation of T(n) with memoization using hash maps.
// T(n) = sum_{i=1}^n S_i mod 10^9.

use std::collections::HashMap;

const MOD: i64 = 1_000_000_000;

fn isqrt_ll(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn sq(n: i64) -> i64 { n * n }

fn sum_powers_1(n: i64) -> i64 {
    if n <= 0 { return 0; }
    n * (n + 1) / 2
}

fn sum_powers_2(n: i64) -> i64 {
    if n <= 0 { return 0; }
    n * (n + 1) * (2 * n + 1) / 6
}

fn tr(n: i64) -> i64 {
    // n*(n+1)/2 mod MOD, using modular inverse of 2
    (n % MOD) * ((n + 1) % MOD) % MOD * 500_000_000 % MOD
}

struct Solver {
    ht_f: HashMap<i64, i64>,
    ht_ss: HashMap<i64, i64>,
    ht_t: HashMap<i64, i64>,
}

impl Solver {
    fn new() -> Self {
        Solver {
            ht_f: HashMap::new(),
            ht_ss: HashMap::new(),
            ht_t: HashMap::new(),
        }
    }

    fn f(&mut self, n: i64) -> i64 {
        if n == 0 { return 0; }
        if let Some(&v) = self.ht_f.get(&n) { return v; }

        // Binary search: find largest m such that sum_sqrts(m) + m <= n
        let mut lo = 0i64;
        let mut hi = n;
        while lo + 1 < hi {
            let mid = lo + (hi - lo) / 2;
            if self.sum_sqrts(mid) + mid <= n {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        self.ht_f.insert(n, lo);
        lo
    }

    fn sum_sqrts(&mut self, n: i64) -> i64 {
        if n == 0 { return 0; }
        if let Some(&v) = self.ht_ss.get(&n) { return v; }

        let fn_val = self.f(n);
        let c = n - fn_val;
        let l = isqrt_ll(c);

        let res = self.sum_sqrts(fn_val)
            + (c - sq(l) + 1) * l
            + 2 * sum_powers_2(l - 1)
            + sum_powers_1(l - 1);

        self.ht_ss.insert(n, res);
        res
    }

    fn t(&mut self, n: i64) -> i64 {
        if n == 0 { return 0; }
        if let Some(&v) = self.ht_t.get(&n) { return v; }

        let fn_val = self.f(n);
        let res = (self.t(fn_val) + tr(n - fn_val)) % MOD;

        self.ht_t.insert(n, res);
        res
    }
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;
    let mut solver = Solver::new();
    let result = solver.t(n);
    println!("{result}");
}
