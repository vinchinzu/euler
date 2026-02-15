// Project Euler 270: Cutting Squares
// DP with memoization on sub-shapes.

use std::collections::HashMap;

const NN: i32 = 30;
const MOD: i64 = 100_000_000;

fn solve(
    type_: i32, a: i32, b: i32, c: i32, d: i32,
    memo: &mut HashMap<(i32, i32, i32, i32, i32), i64>,
) -> i64 {
    // Remove leading zeros
    if type_ >= 2 && a == 0 {
        if type_ == 2 { return 0; }
        if type_ == 3 { return memo_get(2, b, c, 0, 0, memo); }
        if type_ == 4 { return memo_get(3, b, c, d, 0, memo); }
    }
    // Remove trailing zeros
    if type_ == 2 && b == 0 { return 0; }
    if type_ == 3 && c == 0 { return memo_get(2, a, b, 0, 0, memo); }
    if type_ == 4 && d == 0 { return memo_get(3, a, b, c, 0, memo); }

    if type_ <= 1 { return 0; }

    if type_ == 2 {
        if a == 1 && b == 1 { return 1; }
        return (memo_get(2, a-1, b, 0, 0, memo) + memo_get(2, a, b-1, 0, 0, memo)) % MOD;
    }

    if type_ == 3 {
        let mut res = (memo_get(3, a-1, NN, c, 0, memo) + memo_get(3, a, NN, c-1, 0, memo)) % MOD;
        for i in 1..NN {
            res = (res + memo_get(2, a, i, 0, 0, memo) * memo_get(2, NN-i, c, 0, 0, memo)) % MOD;
        }
        return res;
    }

    if type_ == 4 {
        if a == NN && b == NN && c == NN && d == NN {
            let mut res = memo_get(4, NN-1, NN, NN, NN-1, memo) % MOD;
            for i in 1..NN {
                res = (res + memo_get(3, NN, NN, i, 0, memo) * memo_get(2, NN-i, NN-1, 0, 0, memo)) % MOD;
            }
            for i in 1..=NN {
                res = (res + memo_get(2, NN, i, 0, 0, memo) * memo_get(3, NN-i, NN, NN-1, 0, memo)) % MOD;
            }
            return res % MOD;
        }
        // General pentagon (a, N, N, d)
        let mut res = (memo_get(4, a-1, NN, NN, d, memo) + memo_get(4, a, NN, NN, d-1, memo)) % MOD;
        for i in 1..NN {
            res = (res + memo_get(3, a, NN, i, 0, memo) * memo_get(2, NN-i, d, 0, 0, memo)) % MOD;
        }
        for i in 1..=NN {
            res = (res + memo_get(2, a, i, 0, 0, memo) * memo_get(3, NN-i, NN, d, 0, memo)) % MOD;
        }
        return res % MOD;
    }

    0
}

fn memo_get(
    type_: i32, a: i32, b: i32, c: i32, d: i32,
    memo: &mut HashMap<(i32, i32, i32, i32, i32), i64>,
) -> i64 {
    let key = (type_, a, b, c, d);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let v = solve(type_, a, b, c, d, memo);
    memo.insert(key, v);
    v
}

fn main() {
    let mut memo = HashMap::new();
    let result = memo_get(4, NN, NN, NN, NN, &mut memo);
    println!("{}", result);
}
