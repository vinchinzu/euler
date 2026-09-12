// Project Euler 565 - Divisor Sum Divisibility
//
// Find sum of all integers n <= N=10^11 such that sigma(n) % 2017 == 0.
// Uses inclusion-exclusion over prime power bases.

use euler_utils::primes_up_to;
use rayon::prelude::*;

const MAXN: i64 = 100_000_000_000;
const KK: i64 = 2017;

struct Base {
    p: i64,
    #[allow(dead_code)]
    e: i32,
    pe: i64,
}

#[inline]
fn tr(n: i64) -> i128 {
    let nn = n as i128;
    nn * (nn + 1) / 2
}

fn mod_inv_i64(a: i64, m: i64) -> i64 {
    let mut t: i64 = 0;
    let mut new_t: i64 = 1;
    let mut r: i64 = m;
    let mut new_r: i64 = a % m;
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if t < 0 {
        t += m;
    }
    t
}

fn helper(min_index: usize, parity: i128, n: i64, bases: &[Base], nn: i64) -> i128 {
    let mut ans: i128 = 0;
    if n > 1 {
        ans += parity * n as i128 * tr(nn / n);
    }
    for i in min_index..bases.len() {
        let pe = bases[i].pe;
        if pe > nn / n {
            break;
        }
        ans += helper(i + 1, -parity, n * pe, bases, nn);
        let p = bases[i].p;
        if p > 0 && pe <= (nn / n) / p {
            ans += helper(i + 1, parity, n * pe * p, bases, nn);
        }
    }
    ans
}

fn branch_at(i: usize, parity: i128, n: i64, bases: &[Base], nn: i64) -> i128 {
    let pe = bases[i].pe;
    let mut s = helper(i + 1, -parity, n * pe, bases, nn);
    let p = bases[i].p;
    if p > 0 && pe <= (nn / n) / p {
        s += helper(i + 1, parity, n * pe * p, bases, nn);
    }
    s
}

fn first_level_contrib(i: usize, bases: &[Base], nn: i64) -> i128 {
    let pe = bases[i].pe;
    if pe > nn {
        return 0;
    }
    // One extra parallel level for small pe (wide remaining range).
    let par_second = pe <= nn / 10_000_000;
    let mut s = if par_second {
        let max_pe = nn / pe;
        let end = i + 1 + bases[i + 1..].partition_point(|b| b.pe <= max_pe);
        (i + 1..end)
            .into_par_iter()
            .with_max_len(1)
            .map(|j| branch_at(j, 1, pe, bases, nn))
            .sum::<i128>()
            + pe as i128 * tr(nn / pe)
    } else {
        helper(i + 1, 1, pe, bases, nn)
    };
    let p = bases[i].p;
    if p > 0 && pe <= nn / p {
        s += helper(i + 1, -1, pe * p, bases, nn);
    }
    s
}

fn light_contrib(i: usize, bases: &[Base], nn: i64) -> i128 {
    let pe = bases[i].pe;
    let p = bases[i].p;
    let mut s = pe as i128 * tr(nn / pe);
    if p > 0 && pe <= nn / p {
        s -= (pe * p) as i128 * tr(nn / (pe * p));
    }
    s
}

fn main() {
    let nn = MAXN;
    let sieve_limit = (nn as f64).sqrt() as usize + 1;
    let primes = primes_up_to(sieve_limit);

    let sieve_size = (nn / KK + 1) as usize;
    let mut sieve2 = vec![1u8; sieve_size];

    let marks: Vec<(usize, usize, i64)> = primes
        .iter()
        .filter_map(|&p| {
            let p = p as i64;
            if p == KK {
                return None;
            }
            Some((mod_inv_i64(KK, p) as usize, p as usize, p))
        })
        .collect();

    let chunk = (sieve_size / rayon::current_num_threads().max(1)).max(1 << 16);
    sieve2.par_chunks_mut(chunk).enumerate().for_each(|(ci, slice)| {
        let start = ci * chunk;
        let end = start + slice.len();
        for &(inv, pu, _) in &marks {
            let mut i = if inv >= start {
                inv
            } else {
                let rem = (start - inv) % pu;
                if rem == 0 {
                    start
                } else {
                    start + pu - rem
                }
            };
            while i < end {
                // SAFETY: i ∈ [start, end)
                unsafe {
                    *slice.get_unchecked_mut(i - start) = 0;
                }
                i += pu;
            }
        }
    });
    // Restore sieve primes that lie on the 2017k-1 progression (unmarked in C).
    for &(inv, _, p) in &marks {
        if inv < sieve_size && inv as i64 * KK - 1 == p {
            sieve2[inv] = 1;
        }
    }

    let mut bases: Vec<Base> = Vec::with_capacity(3_000_000);

    for ii in 1..sieve_size {
        if sieve2[ii] != 0 {
            let p = ii as i64 * KK - 1;
            bases.push(Base { p, e: 1, pe: p });
        }
    }

    for &p in &primes {
        let p = p as i64;
        let mut sum_div: i64 = 1 + p;
        let mut e = 2;
        let mut pe = p * p;
        while pe <= nn {
            sum_div = sum_div * p + 1;
            if sum_div % KK == 0 {
                bases.push(Base { p, e, pe });
            }
            e += 1;
            if pe > nn / p {
                break;
            }
            pe *= p;
        }
    }

    bases.sort_by_key(|b| b.pe);

    let split = bases.partition_point(|b| b.pe <= nn / b.pe);

    let ans_heavy: i128 = (0..split)
        .into_par_iter()
        .with_max_len(1)
        .map(|i| first_level_contrib(i, &bases, nn))
        .sum();
    let ans_light: i128 = (split..bases.len())
        .into_par_iter()
        .with_min_len(4096)
        .map(|i| light_contrib(i, &bases, nn))
        .sum();

    println!("{}", (ans_heavy + ans_light) as i64);
}
