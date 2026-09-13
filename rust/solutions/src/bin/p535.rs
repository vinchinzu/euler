// Project Euler 535 - Fractal Sequence
//
// Recursive computation of T(n) with memoization using FxHashMap.
// T(n) = sum_{i=1}^n S_i mod 10^9.
//
// Key optimizations:
// 1. FxHashMap only allocates for actual keys (vs 4M open-address zeroing)
// 2. sum_sqrts() returns i128 to avoid overflow (values can reach ~10^27 for n=10^18)
// 3. tr() correctly computes n*(n+1)/2 mod 10^9 (modular inverse of 2 doesn't exist mod 10^9)

use fxhash::FxHashMap;

const MOD: i64 = 1_000_000_000;

#[inline(always)]
fn isqrt_ll(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

/// Triangular number n*(n+1)/2 mod 10^9.
/// Since gcd(2, 10^9) = 2, the modular inverse of 2 doesn't exist mod 10^9.
/// Instead, divide whichever of n, n+1 is even by 2 first, then reduce mod MOD.
#[inline(always)]
fn tr(n: i64) -> i64 {
    if n % 2 == 0 {
        let half = (n / 2) % MOD;
        let other = (n + 1) % MOD;
        half * other % MOD
    } else {
        let other = n % MOD;
        let half = ((n + 1) / 2) % MOD;
        other * half % MOD
    }
}

struct Solver {
    cache_f: FxHashMap<i64, i64>,
    cache_ss: FxHashMap<i64, i128>,
    cache_t: FxHashMap<i64, i64>,
}

impl Solver {
    fn new() -> Self {
        Solver {
            cache_f: FxHashMap::default(),
            cache_ss: FxHashMap::default(),
            cache_t: FxHashMap::default(),
        }
    }

    /// f(n): number of non-circled numbers in first n terms.
    /// Binary search for largest m such that sum_sqrts(m) + m <= n.
    fn f(&mut self, n: i64) -> i64 {
        if n == 0 { return 0; }
        if let Some(&v) = self.cache_f.get(&n) { return v; }

        let mut lo = 0i64;
        let mut hi = n;
        while lo + 1 < hi {
            let mid = lo + (hi - lo) / 2;
            if self.sum_sqrts(mid) + mid as i128 <= n as i128 {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        self.cache_f.insert(n, lo);
        lo
    }

    /// sum_sqrts(n): sum of floor(sqrt(S_i)) for i=1..n.
    /// Returns i128 because for n~10^18 the sum can reach ~10^27.
    fn sum_sqrts(&mut self, n: i64) -> i128 {
        if n == 0 { return 0; }
        if let Some(&v) = self.cache_ss.get(&n) { return v; }

        let fn_val = self.f(n);
        let c = n - fn_val;
        let l = isqrt_ll(c);

        let c128 = c as i128;
        let l128 = l as i128;
        let lm1 = (l - 1) as i128;

        let res = self.sum_sqrts(fn_val)
            + (c128 - l128 * l128 + 1) * l128
            + 2 * lm1 * (lm1 + 1) * (2 * lm1 + 1) / 6
            + lm1 * (lm1 + 1) / 2;

        self.cache_ss.insert(n, res);
        res
    }

    /// T(n) = sum_{i=1}^n S_i mod 10^9.
    fn t(&mut self, n: i64) -> i64 {
        if n == 0 { return 0; }
        if let Some(&v) = self.cache_t.get(&n) { return v; }

        let fn_val = self.f(n);
        let res = (self.t(fn_val) + tr(n - fn_val)) % MOD;

        self.cache_t.insert(n, res);
        res
    }
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;
    let mut solver = Solver::new();
    let result = solver.t(n);
    println!("{result}");
}
