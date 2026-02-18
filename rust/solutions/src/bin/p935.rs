// Problem 935 - Rolling Square
//
// Computes F(N) = number of values b in (0,1) such that a small square of
// side b returns to its initial position within N rolls inside a unit square.
//
// Uses Mobius inversion with Du Jiao sieve for the Mertens function, plus
// parity-aware splitting to handle residue classes mod 4.

use std::collections::HashMap;

/// T(n) = 1 + 2 + ... + n
#[inline]
fn tri(n: i64) -> i64 {
    n * (n + 1) / 2
}

/// Linear sieve for Mobius mu up to `limit`, plus prefix sums M and Modd.
/// M[i] = sum_{k<=i} mu(k)
/// Modd[i] = sum_{k<=i, k odd} mu(k)
fn build_mu_prefix(limit: usize) -> (Vec<i32>, Vec<i32>) {
    let mut mu = vec![0i8; limit + 1];
    let mut is_comp = vec![false; limit + 1];
    let mut primes = Vec::new();
    mu[1] = 1;
    for i in 2..=limit {
        if !is_comp[i] {
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            let ip = i * p;
            if ip > limit {
                break;
            }
            is_comp[ip] = true;
            if i % p == 0 {
                mu[ip] = 0;
                break;
            }
            mu[ip] = -mu[i];
        }
    }

    let mut m_arr = vec![0i32; limit + 1];
    let mut modd_arr = vec![0i32; limit + 1];
    let mut s: i32 = 0;
    let mut so: i32 = 0;
    for i in 1..=limit {
        s += mu[i] as i32;
        m_arr[i] = s;
        if i & 1 == 1 {
            so += mu[i] as i32;
        }
        modd_arr[i] = so;
    }

    (m_arr, modd_arr)
}

struct RollingSquareCounter {
    limit: usize,
    m_small: Vec<i32>,
    modd_small: Vec<i32>,
    cache_m: HashMap<i64, i64>,
    cache_modd: HashMap<i64, i64>,
}

impl RollingSquareCounter {
    fn new(max_n: i64) -> Self {
        let max_x = max_n + 1;
        // Du Jiao sieve: pre-sieve up to n^(2/3) + safety margin
        let limit = (max_x as f64).powf(2.0 / 3.0) as usize + 10;
        let (m_small, modd_small) = build_mu_prefix(limit);
        RollingSquareCounter {
            limit,
            m_small,
            modd_small,
            cache_m: HashMap::new(),
            cache_modd: HashMap::new(),
        }
    }

    /// Mertens function M(n) = sum_{k<=n} mu(k), computed via Du Jiao sieve.
    fn mertens(&mut self, n: i64) -> i64 {
        if n <= self.limit as i64 {
            return self.m_small[n as usize] as i64;
        }
        if let Some(&v) = self.cache_m.get(&n) {
            return v;
        }

        let mut res: i64 = 1;
        let mut l: i64 = 2;
        while l <= n {
            let q = n / l;
            let r = n / q;
            res -= (r - l + 1) * self.mertens(q);
            l = r + 1;
        }

        self.cache_m.insert(n, res);
        res
    }

    /// Odd-restricted Mertens: Modd(n) = sum_{k<=n, k odd} mu(k)
    /// Using Modd(n) = M(n) + Modd(n/2)
    fn modd(&mut self, n: i64) -> i64 {
        if n <= 0 {
            return 0;
        }
        if n <= self.limit as i64 {
            return self.modd_small[n as usize] as i64;
        }
        if let Some(&v) = self.cache_modd.get(&n) {
            return v;
        }

        let m = self.mertens(n);
        let modd_half = self.modd(n / 2);
        let res = m + modd_half;
        self.cache_modd.insert(n, res);
        res
    }

    #[inline]
    fn sum_mu_odd(&mut self, l: i64, r: i64) -> i64 {
        self.modd(r) - self.modd(l - 1)
    }

    #[inline]
    fn sum_mu_2mod4(&mut self, l: i64, r: i64) -> i64 {
        -self.modd(r / 2) + self.modd((l - 1) / 2)
    }

    /// Computes S_cls(X) = sum_{u in cls, u<=X} (C(X,u) - phi(u))
    /// where C(X,u) = #{t<=X : gcd(t,u)=1}.
    /// cls: 0 = div4, 1 = 2mod4, 2 = odd
    fn class_sum(&mut self, x: i64, cls: u8) -> i64 {
        let mut a: i64 = 0; // sum_{u in cls} C(X,u)
        let mut b: i64 = 0; // sum_{u in cls} phi(u)

        let mut l: i64 = 1;
        while l <= x {
            let q = x / l;
            let r = x / q;

            let odd_mu = self.sum_mu_odd(l, r);
            let mu2 = self.sum_mu_2mod4(l, r);

            match cls {
                0 => {
                    // div4
                    a += q * ((q / 4) * odd_mu + (q / 2) * mu2);
                    b += (4 * tri(q / 4)) * odd_mu + (2 * tri(q / 2)) * mu2;
                }
                1 => {
                    // 2mod4
                    let c = (q + 2) / 4; // count of k <= q with k ≡ 2 mod 4
                    a += q * (c * odd_mu + ((q + 1) / 2) * mu2);
                    b += (2 * c * c) * odd_mu + (tri(q) - 2 * tri(q / 2)) * mu2;
                }
                2 => {
                    // odd
                    a += q * (((q + 1) / 2) * odd_mu);
                    b += (tri(q) - 2 * tri(q / 2)) * odd_mu;
                }
                _ => unreachable!(),
            }

            l = r + 1;
        }

        a - b
    }

    /// F(N) = number of b in (0,1) such that rolling square returns within N steps.
    fn f(&mut self, n: i64) -> i64 {
        if n < 0 {
            return 0;
        }

        let x1 = n + 1;
        let x2 = n / 2 + 1;
        let x3 = n / 4 + 1;

        let s1 = self.class_sum(x1, 0); // div4
        let s2 = self.class_sum(x2, 1); // 2mod4
        let s3 = self.class_sum(x3, 2); // odd

        let mut res = s1 + s2 + s3;

        // Remove u=1 (odd class) which doesn't correspond to valid (u,v) with 1<=v<u
        res -= x3 - 1;

        // Integer L = h cases (x = 0): 4*(h-1) rolls, with h >= 2
        res += n / 4;

        res
    }
}

fn main() {
    let n: i64 = 100_000_000;
    let mut counter = RollingSquareCounter::new(n);

    // Verify test cases
    debug_assert_eq!(counter.f(6), 4);
    debug_assert_eq!(counter.f(100), 805);

    println!("{}", counter.f(n));
}
