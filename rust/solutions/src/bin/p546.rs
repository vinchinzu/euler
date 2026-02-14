// Project Euler 546 - Floor Function Recurrence
//
// f_k(n) = sum_{i=0}^{n} f_k(floor(i/k)). Find sum_{k=2}^{10} f_k(10^14) mod 10^9+7.
// Uses recursive coefficient tables with memoization.

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn imod(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

fn f(n: i64, k: i32, s: i32, cache: &mut HashMap<(i64, i32, i32), i64>) -> i64 {
    if n == 0 { return 1; }
    if n < 0 { return 0; }

    let key = (n, k, s);
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let max_t = (s + 2) as usize;
    let ku = k as usize;
    let mut c = vec![0i64; max_t * ku];

    for r in 0..ku {
        c[r] = 1;
    }

    for ss in 0..=s {
        for t in (0..=ss as usize).rev() {
            for r in 1..ku {
                c[t * ku + r] = (c[t * ku + r] + c[t * ku + r - 1]) % MOD;
            }
            for r in 0..ku {
                c[(t + 1) * ku + r] = (c[(t + 1) * ku + r] + c[t * ku + ku - 1]) % MOD;
            }
        }
    }

    let r = imod(n, k as i64) as usize;
    let mut result: i64 = 0;
    for t in 0..max_t {
        let sub_val = f(n / k as i64 - t as i64, k, t as i32, cache);
        result = ((result as i128 + c[t * ku + r] as i128 * sub_val as i128) % MOD as i128) as i64;
    }

    cache.insert(key, result);
    result
}

fn main() {
    let n: i64 = 100_000_000_000_000; // 10^14
    let k_max = 10;

    let mut ans: i64 = 0;
    for k in 2..=k_max {
        let mut cache: HashMap<(i64, i32, i32), i64> = HashMap::new();
        ans = (ans + f(n, k, 0, &mut cache)) % MOD;
    }

    println!("{ans}");
}
