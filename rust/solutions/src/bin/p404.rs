use euler_utils::gcd;

fn main() {
    let n: i64 = 100_000_000_000_000_000; // 10^17
    let mut ans: i64 = 0;

    // First case: m <= 2n
    let mut limit1 = ((n as f64 / 4.0).powf(0.25)) as i64;
    while 4 * limit1 * limit1 * limit1 * limit1 <= n { limit1 += 1; }
    limit1 -= 1;

    for nn in 1..=limit1 {
        if 4 * nn * nn * nn * nn > n { break; }
        let mut m = nn + 1;
        if (m - nn) % 2 == 0 { m += 1; }
        while m <= 2 * nn {
            if gcd(m as u64, nn as u64) != 1 { m += 2; continue; }

            let x = (m * m - nn * nn - 4 * m * nn).abs();
            let y = 2 * (m * m - nn * nn + m * nn);
            let xy = x * y;
            if xy == 0 { m += 2; continue; }
            let a_base = xy / 2;
            if a_base > n || a_base == 0 { m += 2; continue; }

            if x % 5 == 0 && y % 5 == 0 { m += 2; continue; }

            ans += n / a_base;
            m += 2;
        }
    }

    // Second case: m >= 3n
    let mut limit2 = ((n as f64 / 20.0).powf(0.25)) as i64;
    while 20 * limit2 * limit2 * limit2 * limit2 <= n { limit2 += 1; }
    limit2 -= 1;

    for nn in 1..=limit2 {
        if 20 * nn * nn * nn * nn > n { break; }
        let mut m_start = 3 * nn;
        if (m_start - nn) % 2 == 0 { m_start += 1; }

        let mut m = m_start;
        loop {
            if gcd(m as u64, nn as u64) != 1 { m += 2; continue; }

            let x = m * m - nn * nn + 4 * m * nn;
            let y_val = (m * m - nn * nn - m * nn).abs();
            let y = 2 * y_val;

            let xy = x * y;
            if xy == 0 { m += 2; continue; }
            let a_base = xy / 2;
            if a_base > n { break; }
            if a_base == 0 { m += 2; continue; }

            if x % 5 == 0 && y % 5 == 0 { m += 2; continue; }

            ans += n / a_base;
            m += 2;
        }
    }

    println!("{}", ans);
}
