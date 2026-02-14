// Project Euler 141 - Progressive perfect squares below 10^12
//
// For n = q*d + r with d > r, the sequence r, d, n/d forms a geometric
// progression. Parameterize with coprime p > q: d = a*p*q, r = a*q^2,
// n/d = a*p^2, so n = a^2*p^3*q + a*q^2.

use euler_utils::gcd;
use std::collections::HashSet;

fn is_square(v: i64) -> bool {
    if v < 0 {
        return false;
    }
    let mut r = (v as f64).sqrt() as i64;
    while r * r > v {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= v {
        r += 1;
    }
    r * r == v
}

fn main() {
    let limit: i64 = 1_000_000_000_000;
    let mut found = HashSet::new();

    let max_q = (limit as f64).powf(0.25) as i64 + 2;

    for q in 1..=max_q {
        let max_p = ((limit as f64) / q as f64).powf(1.0 / 3.0) as i64 + 2;
        for p in (q + 1)..=max_p {
            if gcd(p as u64, q as u64) != 1 {
                continue;
            }

            let coeff: i64 = p * p * p * q;
            let linear: i64 = q * q;

            let disc = (linear * linear) as f64 + 4.0 * coeff as f64 * (limit - 1) as f64;
            let max_a = ((-linear as f64 + disc.sqrt()) / (2.0 * coeff as f64)) as i64;
            if max_a < 1 {
                continue;
            }

            for a in 1..=max_a {
                let n = coeff * a * a + linear * a;
                if n >= limit {
                    break;
                }
                if is_square(n) {
                    found.insert(n);
                }
            }
        }
    }

    let total: i64 = found.iter().sum();
    println!("{}", total);
}
