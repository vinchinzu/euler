// Project Euler 738 - Counting Ordered Factorisations
//
// Memoized recursion: num_products(min_val, count, n_val).

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn num_products(
    min_val: i64,
    count: i32,
    n_val: i64,
    memo: &mut HashMap<(i64, i32, i64), i64>,
) -> i64 {
    if count == 1 {
        let v = n_val - min_val + 1;
        if v < 0 {
            return 0;
        }
        return v % MOD;
    }

    if let Some(&cached) = memo.get(&(min_val, count, n_val)) {
        return cached;
    }

    let mut result: i64 = 0;
    let mut i = min_val;
    loop {
        // Check i^count <= n_val
        let mut pw: i64 = 1;
        let mut overflow = false;
        for _ in 0..count {
            if pw > n_val / i + 1 {
                overflow = true;
                break;
            }
            pw *= i;
            if pw > n_val {
                overflow = true;
                break;
            }
        }
        if overflow {
            break;
        }

        result = (result + num_products(i, count - 1, n_val / i, memo)) % MOD;
        i += 1;
    }

    memo.insert((min_val, count, n_val), result);
    result
}

fn main() {
    let n: i64 = 10_000_000_000; // 10^10

    let mut memo = HashMap::new();

    let mut ans = n % MOD;
    let mut k = 1;
    while (1i64 << k) <= n {
        let np = num_products(2, k, n, &mut memo);
        let mult = (n - k as i64 + 1) % MOD;
        ans = (ans + mult * np % MOD) % MOD;
        k += 1;
    }

    println!("{}", ans);
}
