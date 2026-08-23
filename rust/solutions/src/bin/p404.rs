use euler_utils::gcd;
use rayon::prelude::*;

fn contrib_first(nn: i64, n: i64) -> i64 {
    if 4 * nn * nn * nn * nn > n {
        return 0;
    }
    let mut ans = 0i64;
    let mut m = nn + 1;
    if (m - nn) % 2 == 0 {
        m += 1;
    }
    while m <= 2 * nn {
        if gcd(m as u64, nn as u64) != 1 {
            m += 2;
            continue;
        }

        let x = (m * m - nn * nn - 4 * m * nn).abs();
        let y = 2 * (m * m - nn * nn + m * nn);
        let xy = x * y;
        if xy == 0 {
            m += 2;
            continue;
        }
        let a_base = xy / 2;
        if a_base > n || a_base == 0 {
            m += 2;
            continue;
        }

        if x % 5 == 0 && y % 5 == 0 {
            m += 2;
            continue;
        }

        ans += n / a_base;
        m += 2;
    }
    ans
}

fn contrib_second(nn: i64, n: i64) -> i64 {
    if 20 * nn * nn * nn * nn > n {
        return 0;
    }
    let mut ans = 0i64;
    let mut m_start = 3 * nn;
    if (m_start - nn) % 2 == 0 {
        m_start += 1;
    }

    let mut m = m_start;
    loop {
        if gcd(m as u64, nn as u64) != 1 {
            m += 2;
            continue;
        }

        let x = m * m - nn * nn + 4 * m * nn;
        let y_val = (m * m - nn * nn - m * nn).abs();
        let y = 2 * y_val;

        let xy = x * y;
        if xy == 0 {
            m += 2;
            continue;
        }
        let a_base = xy / 2;
        if a_base > n {
            break;
        }
        if a_base == 0 {
            m += 2;
            continue;
        }

        if x % 5 == 0 && y % 5 == 0 {
            m += 2;
            continue;
        }

        ans += n / a_base;
        m += 2;
    }
    ans
}

fn main() {
    let n: i64 = 100_000_000_000_000_000; // 10^17

    let mut limit1 = ((n as f64 / 4.0).powf(0.25)) as i64;
    while 4 * limit1 * limit1 * limit1 * limit1 <= n {
        limit1 += 1;
    }
    limit1 -= 1;

    let mut limit2 = ((n as f64 / 20.0).powf(0.25)) as i64;
    while 20 * limit2 * limit2 * limit2 * limit2 <= n {
        limit2 += 1;
    }
    limit2 -= 1;

    let ans1: i64 = (1..=limit1).into_par_iter().map(|nn| contrib_first(nn, n)).sum();
    let ans2: i64 = (1..=limit2).into_par_iter().map(|nn| contrib_second(nn, n)).sum();

    println!("{}", ans1 + ans2);
}
