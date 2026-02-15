// Project Euler 785 - Binary Quadratic Diophantine
// Parameterization with two cases based on (m+n) mod 3.

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn main() {
    let n: i64 = 1_000_000_000;
    let mut ans: i64 = 0;

    // Case 1: m+n != 0 (mod 3)
    let m_max1 = ((n as f64 / 95.0).sqrt() as i64) + 1;
    for m in 1..=m_max1 {
        if 95 * m * m > n { break; }
        let mut nv = m + 1;
        loop {
            let z = (14 * m + 5 * nv) as i64 * (4 * m + nv) as i64;
            if z > n { break; }
            if m % 19 != nv % 19 && gcd(m, nv) == 1 && (m + nv) % 3 != 0 {
                ans += 8 * (13 * m * m + 5 * m * nv + nv * nv);
            }
            nv += 1;
        }
    }

    // Case 2: m+n = 0 (mod 3)
    let m_max2 = ((9.0 * n as f64 / 95.0).sqrt() as i64) + 1;
    for m in 1..=m_max2 {
        if 95 * m * m > 9 * n { break; }
        // Smallest n > m with (m+n) % 3 == 0
        let rem = (3 - (2 * m + 1) % 3) % 3;
        let n_start = m + 1 + rem;
        let mut nv = n_start;
        loop {
            let z = (14 * m + 5 * nv) as i64 * (4 * m + nv) as i64;
            if z > 9 * n { break; }
            if m % 19 != nv % 19 && gcd(m, nv) == 1 {
                ans += 8 * (13 * m * m + 5 * m * nv + nv * nv) / 9;
            }
            nv += 3;
        }
    }

    println!("{}", ans);
}
