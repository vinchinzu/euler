// Project Euler 531 - Chinese Leftovers
//
// Find sum of g(phi(n), n, phi(m), m) for 1000000 <= n < m < 1005000,
// where g is the CRT solution.

use euler_utils::gcd;

const LO: usize = 1_000_000;
const HI: usize = 1_005_000;

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x1, y1) = ext_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn main() {
    let mut phi = vec![0u32; HI];
    for i in 0..HI { phi[i] = i as u32; }
    for i in 2..HI {
        if phi[i] == i as u32 {
            for j in (i..HI).step_by(i) {
                phi[j] -= phi[j] / i as u32;
            }
        }
    }

    let mut ans: u64 = 0;

    for n in LO..HI {
        let a = phi[n] as i64;
        for m in (n + 1)..HI {
            let b = phi[m] as i64;
            let g = gcd(n as u64, m as u64) as i64;
            let diff = b - a;
            if diff % g != 0 { continue; }

            let n_g = n as i64 / g;
            let m_g = m as i64 / g;
            let lcm_val = n_g * m as i64;

            let rhs = diff / g;
            let (_, inv_x, _) = ext_gcd(n_g, m_g);
            let k = ((rhs as i128 % m_g as i128 * (inv_x as i128 % m_g as i128)) % m_g as i128 + m_g as i128) % m_g as i128;
            let x = ((a as i128 + k * n as i128) % lcm_val as i128 + lcm_val as i128) % lcm_val as i128;
            ans += x as u64;
        }
    }

    println!("{ans}");
}
