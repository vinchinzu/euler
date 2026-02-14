use euler_utils::gcd;

fn main() {
    const PERIMETER_LIMIT: i64 = 100_000_000;
    let mut total: i64 = 0;
    let m_limit = ((PERIMETER_LIMIT as f64 / 2.0).sqrt() as i64) + 1;

    for m in 2..=m_limit {
        for k in 1..m {
            if (m - k) % 2 == 0 { continue; }
            if gcd(m as u64, k as u64) != 1 { continue; }

            let a0 = m * m - k * k;
            let b0 = 2 * m * k;
            let c0 = m * m + k * k;
            let p0 = a0 + b0 + c0;

            if p0 >= PERIMETER_LIMIT { break; }

            let diff = (a0 - b0).unsigned_abs() as i64;
            if diff > 0 && c0 % diff == 0 {
                total += (PERIMETER_LIMIT - 1) / p0;
            }
        }
    }
    println!("{}", total);
}
