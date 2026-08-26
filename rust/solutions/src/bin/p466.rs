// Project Euler 466: Distinct products
// For m from 1 to K=64, count how many n <= N=10^16 have a unique
// representation as n*m among {m*1, m*2, ..., m*K}.
// Inclusion-exclusion with divisor-minimal factor sets and coprime splitting.

use rayon::prelude::*;

const N: i64 = 10_000_000_000_000_000; // 10^16
const K: usize = 64;
const PRIMES: [u8; 18] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61];

struct Tables {
    gcd: [[u8; K + 1]; K + 1],
    pm: [u32; K + 1],
}

fn gcd_i32(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn build_tables() -> Tables {
    let mut gcd = [[0u8; K + 1]; K + 1];
    for i in 0..=K {
        for j in 0..=K {
            gcd[i][j] = gcd_i32(i as i32, j as i32) as u8;
        }
    }
    let mut pm = [0u32; K + 1];
    for v in 1..=K {
        let mut x = v;
        let mut m = 0u32;
        for (i, &p) in PRIMES.iter().enumerate() {
            if x % (p as usize) == 0 {
                m |= 1u32 << i;
                while x % (p as usize) == 0 {
                    x /= p as usize;
                }
            }
        }
        pm[v] = m;
    }
    Tables { gcd, pm }
}

/// Keep only divisor-minimal elements of a set encoded as bits 0..63 = values 1..=64.
#[inline(always)]
fn minimize(mut mask: u64) -> u64 {
    let mut bits = mask;
    while bits != 0 {
        let d = bits.trailing_zeros() + 1;
        bits &= bits - 1;
        let mut m = d << 1;
        while m <= K as u32 {
            mask &= !(1u64 << (m - 1));
            m += d;
        }
    }
    mask
}

/// Count n' in 1..=n not divisible by any value whose bit is set in `mask`.
fn num_not_divisible_by(n: i64, mask: u64, t: &Tables, par: bool) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mask = minimize(mask);
    if mask == 0 {
        return n;
    }
    // Divisible by 1 is everything.
    if mask & 1 != 0 {
        return 0;
    }
    let min_f = (mask.trailing_zeros() + 1) as i64;
    if min_f > n {
        return n;
    }

    // Split on a prime that divides at least two remaining factors.
    let mut once = 0u32;
    let mut twice = 0u32;
    let mut bits = mask;
    while bits != 0 {
        let v = bits.trailing_zeros() as usize + 1;
        bits &= bits - 1;
        let pm = t.pm[v];
        twice |= once & pm;
        once |= pm;
    }

    if twice != 0 {
        let p = PRIMES[twice.trailing_zeros() as usize] as i32;
        let mut mask1 = 0u64;
        let mut mask2 = 1u64 << (p as u32 - 1);
        let mut bits = mask;
        while bits != 0 {
            let f = bits.trailing_zeros() as i32 + 1;
            bits &= bits - 1;
            if f % p == 0 {
                let f1 = f / p;
                mask1 |= 1u64 << (f1 as u32 - 1);
            } else {
                let bit = 1u64 << (f as u32 - 1);
                mask1 |= bit;
                mask2 |= bit;
            }
        }
        let n1 = n / p as i64;
        if par {
            let (a, b) = rayon::join(
                || num_not_divisible_by(n1, mask1, t, false),
                || num_not_divisible_by(n, mask2, t, false),
            );
            return a + b;
        }
        return num_not_divisible_by(n1, mask1, t, false)
            + num_not_divisible_by(n, mask2, t, false);
    }

    // Pairwise coprime: inclusion-exclusion via doubling (product = lcm).
    let mut fac = [0i64; 18];
    let mut nf = 0usize;
    let mut bits = mask;
    while bits != 0 {
        fac[nf] = bits.trailing_zeros() as i64 + 1;
        nf += 1;
        bits &= bits - 1;
    }
    ie_coprime(n, &fac[..nf], par)
}

fn ie_coprime(n: i64, fac: &[i64], par: bool) -> i64 {
    let nf = fac.len();
    if nf == 0 {
        return n;
    }
    // Fan the largest coprime sets across threads, then finish sequentially.
    if par && nf >= 12 {
        let k = (nf - 8).min(5); // 32-way, remaining >= 8
        return (0u32..(1 << k))
            .into_par_iter()
            .map(|subset| {
                let mut count = n;
                let mut sign = 1i64;
                for i in 0..k {
                    if subset & (1 << i) != 0 {
                        count /= fac[i];
                        sign = -sign;
                    }
                }
                sign * ie_coprime_seq(count, &fac[k..])
            })
            .sum();
    }
    ie_coprime_seq(n, fac)
}

fn ie_coprime_seq(n: i64, fac: &[i64]) -> i64 {
    let nf = fac.len();
    if nf == 0 {
        return n;
    }
    let mut term = Vec::with_capacity(1 << nf);
    term.push(n);
    let mut result = n;
    for &f in fac {
        let den = -f;
        let len = term.len();
        for i in 0..len {
            let v = term[i] / den;
            term.push(v);
            result += v;
        }
    }
    result
}

fn factors_mask(m: usize, t: &Tables) -> u64 {
    let mut mask = 0u64;
    for i in (m + 1)..=K {
        let f = i / t.gcd[i][m] as usize;
        mask |= 1u64 << (f - 1);
    }
    mask
}

fn main() {
    let t = build_tables();

    // Independent per m; first-level split of heavy m is handled inside the DFS.
    let ans: i64 = (1..=K)
        .into_par_iter()
        .map(|m| num_not_divisible_by(N, factors_mask(m, &t), &t, true))
        .sum();

    println!("{}", ans);
}
