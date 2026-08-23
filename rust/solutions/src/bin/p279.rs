// Project Euler 279: Triangles with integral sides and integral angle
use rayon::prelude::*;

#[inline(always)]
fn coprime(mut u: u32, mut v: u32) -> bool {
    if (u | v) & 1 == 0 {
        return false;
    }
    u >>= u.trailing_zeros();
    while v != 0 {
        v >>= v.trailing_zeros();
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
    }
    u == 1
}

fn type60_i(n: i64, m_hi: u32) -> i64 {
    (2..m_hi).into_par_iter().map(|m| {
        let m_i = m as i64;
        let mm2 = 2 * m_i * m_i;
        let m5 = 5 * m_i;
        if mm2 + m5 + 2 > n {
            return 0;
        }
        let bad3 = m % 3;
        let mut s = 0i64;
        for ni in 1..m {
            let ni_i = ni as i64;
            let p = mm2 + 2 * ni_i * ni_i + m5 * ni_i;
            if p > n {
                break;
            }
            if ni % 3 != bad3 && coprime(m, ni) {
                s += n / p;
            }
        }
        s
    }).sum()
}

fn type60_ii(n: i64, m_hi: u32) -> i64 {
    (2..m_hi).into_par_iter().map(|m| {
        let m_i = m as i64;
        let m3 = 3 * m_i;
        if m3 * (m_i + 1) > n {
            return 0;
        }
        let bad3 = m % 3;
        let mut s = 0i64;
        for ni in 1..m {
            let p = m3 * (m_i + ni as i64);
            if p > n {
                break;
            }
            if ni % 3 != bad3 && coprime(m, ni) {
                s += n / p;
            }
        }
        s
    }).sum()
}

fn type120(n: i64, m_hi: u32) -> i64 {
    (2..m_hi).into_par_iter().map(|m| {
        let m_i = m as i64;
        let mm2 = 2 * m_i * m_i;
        let m3 = 3 * m_i;
        if mm2 + m3 + 1 > n {
            return 0;
        }
        let bad3 = m % 3;
        let mut s = 0i64;
        for ni in 1..m {
            let ni_i = ni as i64;
            let p = mm2 + ni_i * ni_i + m3 * ni_i;
            if p > n {
                break;
            }
            if ni % 3 != bad3 && coprime(m, ni) {
                s += n / p;
            }
        }
        s
    }).sum()
}

fn type90(n: i64, m_hi: u32) -> i64 {
    (2..m_hi).into_par_iter().map(|m| {
        let m_i = m as i64;
        let m2 = 2 * m_i;
        if m2 * (m_i + 1) > n {
            return 0;
        }
        let mut s = 0i64;
        let mut ni = if m & 1 == 1 { 2u32 } else { 1u32 };
        while ni < m {
            let p = m2 * (m_i + ni as i64);
            if p > n {
                break;
            }
            if coprime(m, ni) {
                s += n / p;
            }
            ni += 2;
        }
        s
    }).sum()
}

fn main() {
    const N: i64 = 100_000_000;
    // original: m in 2..=sqrt(N/2)+2  ==  2..m_hi
    let m_hi = (N as u64 / 2).isqrt() as u32 + 3;

    let ((t60i, t60ii), (t120, t90)) = rayon::join(
        || rayon::join(|| type60_i(N, m_hi), || type60_ii(N, m_hi)),
        || rayon::join(|| type120(N, m_hi), || type90(N, m_hi)),
    );

    println!("{}", N / 3 + t60i + t60ii + t120 + t90);
}
