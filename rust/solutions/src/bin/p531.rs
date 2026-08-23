// Project Euler 531 - Chinese Leftovers
//
// Sum g(phi(n), n, phi(m), m) for 1000000 <= n < m < 1005000.

use rayon::prelude::*;

const LO: usize = 1_000_000;
const HI: usize = 1_005_000;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn ext_gcd(mut a: i64, mut b: i64) -> (i64, i64) {
    let (mut x0, mut x1) = (1i64, 0i64);
    while b != 0 {
        let q = a / b;
        let r = a - q * b;
        a = b;
        b = r;
        let nx = x0 - q * x1;
        x0 = x1;
        x1 = nx;
    }
    (a, x0)
}

fn main() {
    let mut phi = vec![0u32; HI];
    for i in 0..HI {
        phi[i] = i as u32;
    }
    for i in 2..HI {
        if phi[i] == i as u32 {
            for j in (i..HI).step_by(i) {
                phi[j] -= phi[j] / i as u32;
            }
        }
    }

    let ans: u64 = (LO..HI)
        .into_par_iter()
        .map(|n| {
            let a = phi[n] as i64;
            let mut local = 0u64;
            for m in (n + 1)..HI {
                let b = phi[m] as i64;
                let g = gcd(n as i64, m as i64);
                let diff = b - a;
                if diff % g != 0 {
                    continue;
                }

                let n_g = n as i64 / g;
                let m_g = m as i64 / g;
                let lcm_val = n_g * m as i64;
                let rhs = diff / g;
                let (_, inv_x) = ext_gcd(n_g, m_g);
                let k = (rhs % m_g).rem_euclid(m_g) * inv_x.rem_euclid(m_g) % m_g;
                let x = (a + k * n as i64).rem_euclid(lcm_val);
                local += x as u64;
            }
            local
        })
        .sum();

    println!("{ans}");
}
