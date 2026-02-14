// Project Euler 512 - Sums of Totients of Powers
// Sum of phi(n) for odd n <= N, using Lucy DP + recursive h function.

use std::collections::HashMap;

const N: i64 = 500_000_000;

struct Solver {
    sqrt_n: usize,
    small_sum: Vec<i64>,
    sp_cache: HashMap<i64, i64>,
    h_cache: HashMap<i64, i64>,
}

impl Solver {
    fn sum_phi(&mut self, m: i64) -> i64 {
        if m <= 0 { return 0; }
        if m <= self.sqrt_n as i64 { return self.small_sum[m as usize]; }
        if let Some(&v) = self.sp_cache.get(&m) { return v; }

        let mut result = m * (m + 1) / 2;
        let mut d = 2i64;
        while d <= m {
            let q = m / d;
            let d_next = m / q + 1;
            result -= (d_next - d) * self.sum_phi(q);
            d = d_next;
        }

        self.sp_cache.insert(m, result);
        result
    }

    fn h(&mut self, k: i64) -> i64 {
        if k > N { return 0; }
        if let Some(&v) = self.h_cache.get(&k) { return v; }

        let m = N / k;
        let mut result = self.sum_phi(m);

        let mut e = 1i64;
        while 2 * k * e <= N {
            result -= e * self.h(2 * e * k);
            e *= 2;
        }

        self.h_cache.insert(k, result);
        result
    }
}

fn main() {
    let sqrt_n = (N as f64).sqrt() as usize + 1;

    // Compute phi sieve
    let mut phi = vec![0i32; sqrt_n + 1];
    for i in 0..=sqrt_n { phi[i] = i as i32; }
    for i in 2..=sqrt_n {
        if phi[i] == i as i32 {
            let mut j = i;
            while j <= sqrt_n {
                phi[j] -= phi[j] / i as i32;
                j += i;
            }
        }
    }

    // Prefix sums of phi
    let mut small_sum = vec![0i64; sqrt_n + 1];
    for i in 1..=sqrt_n {
        small_sum[i] = small_sum[i - 1] + phi[i] as i64;
    }

    let mut solver = Solver {
        sqrt_n,
        small_sum,
        sp_cache: HashMap::new(),
        h_cache: HashMap::new(),
    };

    let answer = solver.h(1);
    println!("{}", answer);
}
