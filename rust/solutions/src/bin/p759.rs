// Project Euler 759 - A Squared Recurrence
// f(n) = n * popcount(n). Compute sum_{n=1}^N f(n)^2 mod 10^9+7 via binary splitting.

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

static NCR: [[i64; 3]; 3] = [
    [1, 0, 0],
    [1, 1, 0],
    [1, 2, 1],
];
static POW2: [i64; 3] = [1, 2, 4];

fn s(n: i64, k: usize, l: usize, maps: &mut [[HashMap<i64, i64>; 3]; 3]) -> i64 {
    if n == 0 {
        return if k == 0 && l == 0 { 1 } else { 0 };
    }

    if let Some(&cached) = maps[k][l].get(&n) {
        return cached;
    }

    let mut result = POW2[k] % MOD * s(n / 2, k, l, maps) % MOD;

    for kk in 0..=k {
        for ll in 0..=l {
            let term = POW2[kk] * NCR[k][kk] % MOD * NCR[l][ll] % MOD;
            let sub = s((n - 1) / 2, kk, ll, maps);
            result = (result + term * sub) % MOD;
        }
    }

    result %= MOD;
    maps[k][l].insert(n, result);
    result
}

fn main() {
    let n: i64 = 10_000_000_000_000_000; // 10^16

    let mut maps: [[HashMap<i64, i64>; 3]; 3] = Default::default();

    println!("{}", s(n, 2, 2, &mut maps));
}
