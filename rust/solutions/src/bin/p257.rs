// Project Euler 257: Angular Bisectors
//
// Four cases: r=2, r=3 even, r=3 odd, r=4 equilateral (N/3).
// Perimeters are (n+m)(n+2m) and (n+m)(n+3m). Parallelize over m.

use rayon::prelude::*;

const BIG_N: i64 = 100_000_000;

#[inline(always)]
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    let m_r2 = (BIG_N / 6).isqrt();
    let m_r3e = (BIG_N / 8).isqrt();
    let m_r3o = (BIG_N / 4).isqrt();
    let m_hi = m_r2.max(m_r3e).max(m_r3o);

    let ans: i64 = (1..m_hi as usize + 1)
        .into_par_iter()
        .with_min_len(1)
        .map(|m| {
            let m = m as i64;
            let mut local = 0i64;
            let two_n = BIG_N << 1;
            let mu = m as u32;

            if m <= m_r2 {
                let two_m = m << 1;
                let mut n = m + 1 + (m & 1);
                while n < two_m {
                    let p = (n + m) * (n + two_m);
                    if p > BIG_N {
                        break;
                    }
                    if gcd(mu, n as u32) == 1 {
                        local += BIG_N / p;
                    }
                    n += 2;
                }
            }

            if m <= m_r3e {
                let three_m = m * 3;
                let mut n = m + 1;
                if n % 3 == 0 {
                    n += 2;
                }
                let mut step = if n % 3 == 1 { 4 } else { 2 };
                while n < three_m {
                    let p = (n + m) * (n + three_m);
                    if p > BIG_N {
                        break;
                    }
                    if gcd(mu, n as u32) == 1 {
                        local += BIG_N / p;
                    }
                    n += step;
                    step = 6 - step;
                }
            }

            if m <= m_r3o && m & 1 == 1 {
                let three_m = m * 3;
                let mut n = m + 2;
                if n % 3 == 0 {
                    n += 2;
                }
                let mut step = if n % 3 == 1 { 4 } else { 2 };
                while n < three_m {
                    let p = (n + m) * (n + three_m);
                    if p > two_n {
                        break;
                    }
                    if gcd(mu, n as u32) == 1 {
                        local += two_n / p;
                    }
                    n += step;
                    step = 6 - step;
                }
            }

            local
        })
        .sum();

    println!("{}", ans + BIG_N / 3);
}
