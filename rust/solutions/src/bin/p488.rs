// Project Euler 488: Unbalanced Nim
use std::collections::HashMap;

const MOD: i64 = 6_000_000_000;
const M: i64 = 1_000_000_000;

fn f(n: u64, g: u8, cache: &mut HashMap<(u64, u8), (i64, i64)>) -> (i64, i64) {
    if n == 0 {
        if g != 0 { return (0, 0); }
        return (1, 0);
    }

    if let Some(&res) = cache.get(&(n, g)) {
        return res;
    }

    let mut count: i64 = 0;
    let mut total: i64 = 0;
    let nbit = (n & 1) as u8;

    let combos: [(u8, u8, u8); 4] = [(0,0,0), (0,1,1), (1,0,1), (1,1,0)];

    for &(b0, b1, b2) in &combos {
        let bits = [b0, b1, b2];
        let mut new_g: u8 = 0;
        for i in 0..3 {
            let bi = bits[i];
            let gi = (g >> i) & 1;
            let ng = if bi > nbit || (bi == nbit && gi != 0) { 1 } else { 0 };
            new_g |= ng << i;
        }

        let (sub_count, sub_total) = f(n / 2, new_g, cache);
        let sum_bits = (b0 + b1 + b2) as i64;
        count = (count + sub_count) % MOD;
        total = (total + 2 * sub_total % MOD + sum_bits * sub_count % MOD) % MOD;
    }

    count = ((count % MOD) + MOD) % MOD;
    total = ((total % MOD) + MOD) % MOD;

    cache.insert((n, g), (count, total));
    (count, total)
}

fn main() {
    let n: u64 = 1_000_000_000_000_000_000;

    let mut cache = HashMap::new();
    let (f0, f1) = f(n, 0, &mut cache);

    let nm = (n % MOD as u64) as i64;
    let nsq = (nm as u128 * nm as u128 % MOD as u128) as i64;

    let numerator = (f1 - 3 * f0 % MOD - 6 * nsq % MOD + 15 * nm % MOD - 3 + 10 * MOD) % MOD;
    let numerator = ((numerator % MOD) + MOD) % MOD;
    let ans = (numerator / 6) % M;

    println!("{}", ans);
}
