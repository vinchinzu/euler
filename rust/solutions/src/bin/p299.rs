// Project Euler 299: Three similar triangles

use euler_utils::gcd;

fn f1(m: i64, n: i64) -> i64 {
    m * m - n * n + 2 * m * n
}

fn f2(m: i64, n: i64) -> i64 {
    2 * (m * m + n * n)
}

fn main() {
    let n: i64 = 100_000_000;
    let mut ans: i64 = 0;

    // Case 1: ABP = DBP
    let mut nn = 1i64;
    while f1(nn, nn) < n {
        let mut m = nn + 1;
        while f1(m, nn) < n {
            if gcd(m as u64, nn as u64) == 1 {
                ans += ((n - 1) / f1(m, nn)) * 2;
            }
            m += 2;
        }
        nn += 1;
    }

    // Case 2: ABP = BDP
    nn = 1;
    while f2(nn, nn) < n {
        let mut m = nn + 1;
        while f2(m, nn) < n {
            if gcd(m as u64, nn as u64) == 1 {
                ans += (n - 1) / f2(m, nn);
            }
            m += 2;
        }
        nn += 1;
    }

    println!("{}", ans);
}
