// Project Euler 279: Triangles with integral sides and integral angle
use euler_utils::gcd;

fn main() {
    let n: i64 = 100_000_000;
    let mut ans: i64 = n / 3; // equilateral

    let m_limit = ((n / 2) as f64).sqrt() as i64 + 2;

    // 60-degree Type I: perimeter = 2m^2 + 2n^2 + 5mn
    for m in 2..=m_limit {
        let mm2 = 2 * m * m;
        let m5 = 5 * m;
        if mm2 + m5 + 2 > n { break; }
        let bad3 = m % 3;
        for ni in 1..m {
            let p = mm2 + 2 * ni * ni + m5 * ni;
            if p > n { break; }
            if ni % 3 != bad3 && gcd(m as u64, ni as u64) == 1 {
                ans += n / p;
            }
        }
    }

    // 60-degree Type II: perimeter = 3m(m+n)
    for m in 2..=m_limit {
        let m3 = 3 * m;
        if m3 * (m + 1) > n { break; }
        let bad3 = m % 3;
        for ni in 1..m {
            let p = m3 * (m + ni);
            if p > n { break; }
            if ni % 3 != bad3 && gcd(m as u64, ni as u64) == 1 {
                ans += n / p;
            }
        }
    }

    // 120-degree: perimeter = 2m^2 + n^2 + 3mn
    for m in 2..=m_limit {
        let mm2 = 2 * m * m;
        let m3 = 3 * m;
        if mm2 + m3 + 1 > n { break; }
        let bad3 = m % 3;
        for ni in 1..m {
            let p = mm2 + ni * ni + m3 * ni;
            if p > n { break; }
            if ni % 3 != bad3 && gcd(m as u64, ni as u64) == 1 {
                ans += n / p;
            }
        }
    }

    // 90-degree (Pythagorean): perimeter = 2m(m+n), (m-n) odd
    for m in 2..=m_limit {
        let m2 = 2 * m;
        if m2 * (m + 1) > n { break; }
        let n_start = if m & 1 == 1 { 2 } else { 1 };
        let mut ni = n_start;
        while ni < m {
            let p = m2 * (m + ni);
            if p > n { break; }
            if gcd(m as u64, ni as u64) == 1 {
                ans += n / p;
            }
            ni += 2;
        }
    }

    println!("{}", ans);
}
