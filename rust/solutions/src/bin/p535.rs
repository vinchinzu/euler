// Project Euler 535 - Fractal Sequence
//
// Recursive computation of T(n) with memoization using open-addressing hash tables.
// T(n) = sum_{i=1}^n S_i mod 10^9.
//
// Key fixes from original:
// 1. sum_sqrts() returns i128 to avoid overflow (values can reach ~10^27 for n=10^18)
// 2. tr() correctly computes n*(n+1)/2 mod 10^9 (modular inverse of 2 doesn't exist mod 10^9)

const MOD: i64 = 1_000_000_000;
const HT_SIZE: usize = 1 << 22;
const HT_MASK: usize = HT_SIZE - 1;
const HT_PROBE: usize = 128;

/// Hash table for i64 -> i64 mappings (used by f and T)
struct HTableI64 {
    keys: Vec<i64>,
    vals: Vec<i64>,
    occupied: Vec<bool>,
}

impl HTableI64 {
    fn new() -> Self {
        HTableI64 {
            keys: vec![0; HT_SIZE],
            vals: vec![0; HT_SIZE],
            occupied: vec![false; HT_SIZE],
        }
    }

    #[inline(always)]
    fn get(&self, key: i64) -> Option<i64> {
        let h = (key as u64).wrapping_mul(2654435761) as usize & HT_MASK;
        for i in 0..HT_PROBE {
            let idx = (h + i) & HT_MASK;
            unsafe {
                if !*self.occupied.get_unchecked(idx) { return None; }
                if *self.keys.get_unchecked(idx) == key {
                    return Some(*self.vals.get_unchecked(idx));
                }
            }
        }
        None
    }

    #[inline(always)]
    fn put(&mut self, key: i64, val: i64) {
        let h = (key as u64).wrapping_mul(2654435761) as usize & HT_MASK;
        for i in 0..HT_PROBE {
            let idx = (h + i) & HT_MASK;
            unsafe {
                if !*self.occupied.get_unchecked(idx) || *self.keys.get_unchecked(idx) == key {
                    *self.keys.get_unchecked_mut(idx) = key;
                    *self.vals.get_unchecked_mut(idx) = val;
                    *self.occupied.get_unchecked_mut(idx) = true;
                    return;
                }
            }
        }
        let idx = h;
        self.keys[idx] = key;
        self.vals[idx] = val;
        self.occupied[idx] = true;
    }
}

/// Hash table for i64 -> i128 mappings (used by sum_sqrts which can overflow i64)
struct HTableI128 {
    keys: Vec<i64>,
    vals: Vec<i128>,
    occupied: Vec<bool>,
}

impl HTableI128 {
    fn new() -> Self {
        HTableI128 {
            keys: vec![0; HT_SIZE],
            vals: vec![0; HT_SIZE],
            occupied: vec![false; HT_SIZE],
        }
    }

    #[inline(always)]
    fn get(&self, key: i64) -> Option<i128> {
        let h = (key as u64).wrapping_mul(2654435761) as usize & HT_MASK;
        for i in 0..HT_PROBE {
            let idx = (h + i) & HT_MASK;
            unsafe {
                if !*self.occupied.get_unchecked(idx) { return None; }
                if *self.keys.get_unchecked(idx) == key {
                    return Some(*self.vals.get_unchecked(idx));
                }
            }
        }
        None
    }

    #[inline(always)]
    fn put(&mut self, key: i64, val: i128) {
        let h = (key as u64).wrapping_mul(2654435761) as usize & HT_MASK;
        for i in 0..HT_PROBE {
            let idx = (h + i) & HT_MASK;
            unsafe {
                if !*self.occupied.get_unchecked(idx) || *self.keys.get_unchecked(idx) == key {
                    *self.keys.get_unchecked_mut(idx) = key;
                    *self.vals.get_unchecked_mut(idx) = val;
                    *self.occupied.get_unchecked_mut(idx) = true;
                    return;
                }
            }
        }
        let idx = h;
        self.keys[idx] = key;
        self.vals[idx] = val;
        self.occupied[idx] = true;
    }
}

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
    ht_f: HTableI64,
    ht_ss: HTableI128,
    ht_t: HTableI64,
}

impl Solver {
    fn new() -> Self {
        Solver {
            ht_f: HTableI64::new(),
            ht_ss: HTableI128::new(),
            ht_t: HTableI64::new(),
        }
    }

    /// f(n): number of non-circled numbers in first n terms.
    /// Binary search for largest m such that sum_sqrts(m) + m <= n.
    fn f(&mut self, n: i64) -> i64 {
        if n == 0 { return 0; }
        if let Some(v) = self.ht_f.get(n) { return v; }

        let mut lo = 0i64;
        let mut hi = n;
        while lo + 1 < hi {
            let mid = lo + (hi - lo) / 2;
            // Use i128 comparison since sum_sqrts can be very large
            if self.sum_sqrts(mid) + mid as i128 <= n as i128 {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        self.ht_f.put(n, lo);
        lo
    }

    /// sum_sqrts(n): sum of floor(sqrt(S_i)) for i=1..n.
    /// Returns i128 because for n~10^18 the sum can reach ~10^27.
    fn sum_sqrts(&mut self, n: i64) -> i128 {
        if n == 0 { return 0; }
        if let Some(v) = self.ht_ss.get(n) { return v; }

        let fn_val = self.f(n);
        let c = n - fn_val;
        let l = isqrt_ll(c);

        // All arithmetic in i128 to avoid overflow
        let c128 = c as i128;
        let l128 = l as i128;
        let lm1 = (l - 1) as i128;

        let res = self.sum_sqrts(fn_val)
            + (c128 - l128 * l128 + 1) * l128
            + 2 * lm1 * (lm1 + 1) * (2 * lm1 + 1) / 6   // sum_powers_2(l-1)
            + lm1 * (lm1 + 1) / 2;                         // sum_powers_1(l-1)

        self.ht_ss.put(n, res);
        res
    }

    /// T(n) = sum_{i=1}^n S_i mod 10^9.
    fn t(&mut self, n: i64) -> i64 {
        if n == 0 { return 0; }
        if let Some(v) = self.ht_t.get(n) { return v; }

        let fn_val = self.f(n);
        let res = (self.t(fn_val) + tr(n - fn_val)) % MOD;

        self.ht_t.put(n, res);
        res
    }
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;
    let mut solver = Solver::new();
    let result = solver.t(n);
    println!("{result}");
}
