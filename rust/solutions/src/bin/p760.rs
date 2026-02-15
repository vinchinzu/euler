// Project Euler 760 - Sum over Bitwise Operators
// G(n) computed via memoized recurrence with halving.

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn tr(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let nm = n % MOD;
    let np1m = (n + 1) % MOD;
    let inv2 = (MOD + 1) / 2;
    (nm as i128 * np1m as i128 % MOD as i128 * inv2 as i128 % MOD as i128) as i64
}

fn g(n: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if n <= 0 { return 0; }

    if let Some(&cached) = memo.get(&n) {
        return cached;
    }

    let half = n / 2;
    let ceil_half = (n + 1) / 2;

    let result = (
        2i128 * g(half, memo) as i128 % MOD as i128
        + 2i128 * g(half - 1, memo) as i128 % MOD as i128
        + 2i128 * tr(half) as i128 % MOD as i128
        + 2i128 * (2i128 * g(ceil_half - 1, memo) as i128 % MOD as i128
            + 2i128 * tr(ceil_half) as i128 % MOD as i128) % MOD as i128
    ) % MOD as i128;

    let result = ((result % MOD as i128) + MOD as i128) as i64 % MOD;
    memo.insert(n, result);
    result
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000; // 10^18
    let mut memo = HashMap::new();
    println!("{}", g(n, &mut memo));
}
