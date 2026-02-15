// Project Euler 414: Kaprekar cycles

const MAXB: usize = 1810;
const MOD: i64 = 1_000_000_000_000_000_000; // 10^18

fn sb(b: i32, d1: i32, d2: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
    if d1 == 2 * b / 3 && d2 == b / 3 { return 1; }

    if cache[d1 as usize][d2 as usize] != 0 {
        return cache[d1 as usize][d2 as usize];
    }

    let mut digits = [0i32; 5];
    if d2 == 0 {
        digits[0] = d1 - 1;
        digits[1] = b - 1;
        digits[2] = b - 1;
        digits[3] = b - 1;
        digits[4] = b - d1;
    } else {
        digits[0] = d1;
        digits[1] = d2 - 1;
        digits[2] = b - 1;
        digits[3] = b - d2 - 1;
        digits[4] = b - d1;
    }

    digits.sort();

    let new_d1 = digits[4] - digits[0];
    let new_d2 = digits[3] - digits[1];
    let result = 1 + sb(b, new_d1, new_d2, cache);
    cache[d1 as usize][d2 as usize] = result;
    result
}

fn main() {
    let n = 300;
    let mut ans: i64 = 0;

    for k in 2..=n {
        let b = 6 * k + 3;
        let mut cache = vec![vec![0i32; MAXB]; MAXB];

        for d1 in 1..b {
            for d2 in 0..=d1 {
                let mut mult = (b - d1) as i64;

                if d2 == 0 {
                    mult *= 20 * d1 as i64 - 10;
                } else if d2 == d1 {
                    mult *= 30 * d1 as i64 - 10;
                } else {
                    mult *= 120 * d2 as i64 * (d1 - d2) as i64 - 20;
                }

                let s = sb(b, d1, d2, &mut cache) as i64;
                ans = (ans + mult % MOD * s) % MOD;
            }
        }

        ans = (ans - 1 + MOD) % MOD;
    }

    println!("{}", ans);
}
