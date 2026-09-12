// Project Euler 785 - Binary Quadratic Diophantine
// Parameterization with two cases based on (m+n) mod 3.
// u32 gcd, incremental z, coprime-by-mod-19/3 filters first.

use rayon::prelude::*;

#[inline(always)]
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline(always)]
fn case1_m(m: i64, n: i64) -> i64 {
    if 95 * m * m > n {
        return 0;
    }
    let mu = m as u32;
    let m19 = mu % 19;
    let m3 = mu % 3;
    let mut ans = 0i64;
    // z = (14m+5 nv)(4m+nv) = 5 nv^2 + 34 m nv + 56 m^2
    let mut nv = m + 1;
    let mut z = (14 * m + 5 * nv) * (4 * m + nv);
    let dz0 = 10 * nv + 34 * m + 5; // z(nv+1)-z(nv)
    let mut dz = dz0;
    while z <= n {
        let nvu = nv as u32;
        if nvu % 19 != m19 && (m3 + nvu % 3) % 3 != 0 && gcd(mu, nvu) == 1 {
            ans += 8 * (13 * m * m + 5 * m * nv + nv * nv);
        }
        nv += 1;
        z += dz;
        dz += 10;
    }
    ans
}

#[inline(always)]
fn case2_m(m: i64, n: i64) -> i64 {
    if 95 * m * m > 9 * n {
        return 0;
    }
    let mu = m as u32;
    let m19 = mu % 19;
    let rem = (3 - (2 * m + 1) % 3) % 3;
    let mut nv = m + 1 + rem;
    let mut z = (14 * m + 5 * nv) * (4 * m + nv);
    // nv += 3: Δz = z(n+3)-z(n) = 5(6n+9) + 34m*3 = 30n + 45 + 102m
    let mut dz = 30 * nv + 102 * m + 45;
    let mut ans = 0i64;
    let lim = 9 * n;
    while z <= lim {
        let nvu = nv as u32;
        if nvu % 19 != m19 && gcd(mu, nvu) == 1 {
            ans += 8 * (13 * m * m + 5 * m * nv + nv * nv) / 9;
        }
        nv += 3;
        z += dz;
        dz += 90; // 30*3
    }
    ans
}

fn main() {
    let n: i64 = 1_000_000_000;

    let m_max1 = (n / 95).isqrt() as usize + 1;
    let part1: i64 = (1..m_max1 + 1)
        .into_par_iter()
        .with_max_len(8)
        .map(|m| case1_m(m as i64, n))
        .sum();

    let m_max2 = (9 * n / 95).isqrt() as usize + 1;
    let part2: i64 = (1..m_max2 + 1)
        .into_par_iter()
        .with_max_len(8)
        .map(|m| case2_m(m as i64, n))
        .sum();

    println!("{}", part1 + part2);
}
