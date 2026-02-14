// Project Euler 551 - Sum of Digits Sequence
//
// a_0=1, a_n = a_{n-1} + sumDigits(a_{n-1}). Find a_{10^15}.
// Uses memoized "jump" approach for logarithmic-depth recursion.

use std::collections::HashMap;

const B: i64 = 10;
const CB_B: i64 = 1000; // B^3

fn sum_digits(mut n: i64) -> i64 {
    let mut s = 0;
    while n > 0 { s += n % 10; n /= 10; }
    s
}

fn get_jump(
    r: i64, m: i64, sum_q: i32,
    cache: &mut HashMap<(i64, i64, i32), (i64, i64)>,
) -> (i64, i64) {
    let key = (r, m, sum_q);
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let mut di: i64 = 0;
    let mut da: i64 = 0;

    while r + da < m {
        if m <= CB_B {
            di += 1;
            da += sum_q as i64 + sum_digits(r + da);
        } else {
            let sub_r = (r + da) % (m / B);
            let sub_sq = sum_q as i64 + sum_digits((r + da) / (m / B));
            let (sub_di, sub_da) = get_jump(sub_r, m / B, sub_sq as i32, cache);
            di += sub_di;
            da += sub_da;
        }
    }

    cache.insert(key, (di, da));
    (di, da)
}

fn main() {
    let n_val: i64 = 1_000_000_000_000_000; // 10^15
    let mut cache: HashMap<(i64, i64, i32), (i64, i64)> = HashMap::new();

    let mut di: i64 = 0;
    let mut ans: i64 = 1;
    let mut m = n_val;

    while m >= 1 {
        loop {
            let (j_di, j_da) = get_jump(ans % m, m, sum_digits(ans / m) as i32, &mut cache);
            if di + j_di >= n_val { break; }
            di += j_di;
            ans += j_da;
        }
        m /= B;
    }

    println!("{ans}");
}
