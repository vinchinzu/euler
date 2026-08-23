// Project Euler 747 - Triangular Pizza
//
// Counting triangular configurations on a grid.

use rayon::prelude::*;

const MOD: i64 = 1_000_000_007;

#[inline]
fn mul(a: i64, b: i64) -> i64 {
    ((a as u64 * b as u64) % MOD as u64) as i64
}

fn pow_mod(mut base: i64, mut exp: i64) -> i64 {
    let mut result: i64 = 1;
    base = ((base % MOD) + MOD) % MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(result, base);
        }
        exp >>= 1;
        base = mul(base, base);
    }
    result
}

fn ncr(n: i64, r: i64) -> i64 {
    if r < 0 || r > n {
        return 0;
    }
    let r = r.min(n - r);
    let mut result: i64 = 1;
    for i in 0..r {
        result = mul(result, (n - i) % MOD);
        result = mul(result, pow_mod(i + 1, MOD - 2));
    }
    result
}

fn tr(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn isqrt_ll(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

fn main() {
    let n: i64 = 100_000_000; // 10^8

    let mut ans = ncr(n, 3);
    ans = (ans + 6 * (tr(n - 2) % MOD)) % MOD;

    let sqrt_2n = isqrt_ll(2 * n);
    let extra: i64 = (1..=sqrt_2n)
        .into_par_iter()
        .map(|a| {
            let mut local = 0i64;
            let min_n = (2 * a + 1) * (2 * a + 1);
            if min_n <= n {
                local = (local + 6 * ((n - min_n) % MOD) % MOD + 3) % MOD;
            }

            let mut b = a + 1;
            loop {
                let prod = 4 * (a + 1) * (b + 1) * a * b;
                let sq_root = isqrt_ll(prod);
                let min_n2 = (a + 1) * (b + 1) + a * b + sq_root;
                if min_n2 > n {
                    break;
                }
                local = (local + 12 * ((n - min_n2) % MOD)) % MOD;
                if sq_root * sq_root == prod {
                    local = (local + 6) % MOD;
                }
                b += 1;
            }
            local
        })
        .sum();

    ans = (ans + extra) % MOD;
    println!("{}", ((ans % MOD) + MOD) % MOD);
}
