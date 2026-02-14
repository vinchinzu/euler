// Project Euler 195: Inscribed circles of 60-degree triangles
use euler_utils::gcd;

fn main() {
    let n_limit: i64 = 1_053_779;
    let sqrt3 = 3.0_f64.sqrt();
    let inv_2sqrt3 = 1.0 / (2.0 * sqrt3);
    let limit_3n = 3.0 * n_limit as f64;

    let mut ans: i64 = 0;

    for n in 1.. {
        let m_start = 2 * n + 1;
        let ir_start = (m_start + n) as f64 * (m_start - 2 * n) as f64 * inv_2sqrt3;
        if ir_start > limit_3n { break; }

        for m in m_start.. {
            let ir = (m + n) as f64 * (m - 2 * n) as f64 * inv_2sqrt3;
            if ir > limit_3n { break; }
            if gcd(m as u64, n as u64) == 1 {
                if (m + n) % 3 == 0 {
                    ans += (n_limit as f64 / (ir / 3.0)) as i64;
                } else {
                    ans += (n_limit as f64 / ir) as i64;
                }
            }
        }
    }

    println!("{}", ans);
}
