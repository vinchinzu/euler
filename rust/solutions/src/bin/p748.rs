// Project Euler 748 - Upside Down Diophantine Equation
//
// Enumerate coprime pairs (m,n) and compute Pythagorean-like triples.

use rayon::prelude::*;

const N: i64 = 10_000_000_000_000_000; // 10^16
const M: i64 = 1_000_000_000; // 10^9

#[inline(always)]
fn gcd(mut u: u32, mut v: u32) -> u32 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    let shift = (u | v).trailing_zeros();
    u >>= u.trailing_zeros();
    loop {
        v >>= v.trailing_zeros();
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
        if v == 0 {
            break;
        }
    }
    u << shift
}

/// m/n > (√6.5 - 2)/(3 - √6.5)
#[inline(always)]
fn above_a(m: i64, n: i64) -> bool {
    let s = 3 * m + 2 * n;
    let t = m + n;
    2 * s * s > 13 * t * t
}

fn m_lo(n: i64) -> i64 {
    let mut lo = n + 1;
    let mut hi = n + n / 2 + 2;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if above_a(mid, n) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn m_hi_b(n: i64) -> i64 {
    let s = (13 * n * n - 1).isqrt();
    (s + 3 * n) / 2
}

#[inline(always)]
fn quartic_le(m: i64, n: i64, bound: i64) -> bool {
    let m = m as i128;
    let n = n as i128;
    let mm = m * m;
    let nn = n * n;
    (mm + nn) * (3 * mm + 4 * m * n - 3 * nn) <= bound as i128
}

fn m_hi(n: i64, bound: i64) -> i64 {
    let cap = m_hi_b(n);
    if quartic_le(cap, n, bound) {
        return cap;
    }
    let mut lo = n;
    let mut hi = cap;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if quartic_le(mid, n, bound) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

#[inline(always)]
fn contrib(m: i64, n: i64, g: i64) -> i64 {
    let mm = m * m;
    let nn = n * n;
    let mn = m * n;
    let a = mm + nn;
    let b = -2 * mm + 6 * mn + 2 * nn;
    let c = 3 * mm + 4 * mn - 3 * nn;
    let x = a * b / g;
    let y = a * c / g;
    let z = b * c / g;
    if y <= N && z <= N && y > 0 && z > 0 {
        (x + y + z) % M
    } else {
        0
    }
}

fn max_n(bound: i64) -> i64 {
    let mut n = ((bound / 8) as u64).isqrt().isqrt() as i64;
    while n > 0 && 8 * n * n * n * n > bound {
        n -= 1;
    }
    loop {
        let np = n + 1;
        let q = 8 * np * np * np * np;
        if q < 0 || q > bound {
            break;
        }
        n = np;
    }
    n
}

fn section1(n: i64) -> i64 {
    let lo = m_lo(n);
    let hi = m_hi(n, 4 * N);
    if lo > hi {
        return 0;
    }
    let n_u = n as u32;
    let forbid = (8 * n) % 13;
    let mut ans = 0i64;
    let mut m = lo;
    while m <= hi {
        if m % 13 != forbid && gcd((m % n) as u32, n_u) == 1 {
            let g = if (m + n) & 1 == 0 { 4 } else { 1 };
            ans += contrib(m, n, g);
        }
        m += 1;
    }
    ans % M
}

fn section2(n: i64) -> i64 {
    let lo = m_lo(n);
    let hi = m_hi(n, 676 * N);
    if lo > hi {
        return 0;
    }
    let n_u = n as u32;
    let mut m = n + (7 * n) % 13;
    if m < lo {
        m += ((lo - m + 12) / 13) * 13;
    }
    let mut ans = 0i64;
    while m <= hi {
        if gcd((m % n) as u32, n_u) == 1 {
            let g = if (m + n) & 1 == 0 { 676 } else { 169 };
            ans += contrib(m, n, g);
        }
        m += 13;
    }
    ans % M
}

fn main() {
    let n1 = max_n(4 * N);
    let n2 = max_n(676 * N);

    let (ans1, ans2) = rayon::join(
        || (1..n1 + 1).into_par_iter().map(section1).sum::<i64>(),
        || (1..n2 + 1).into_par_iter().map(section2).sum::<i64>(),
    );

    println!("{}", (ans1 + ans2) % M);
}
