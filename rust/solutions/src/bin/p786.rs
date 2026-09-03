// Project Euler 786 - Billiard Ball Bounces
// Mobius function sieve and lattice point counting.
// Optimized: linear Mobius sieve up to exact non-zero bound, const lookup table, i64 math.

use rayon::prelude::*;

const BIG_N: i64 = 1_000_000_000;
const TAB3: [i64; 9] = [
    0, 1, 3,
    0, 2, 2,
    0, 0, 1,
];
const TAB9: [i64; 81] = [
    0, 4, 12, 15, 22, 24, 30, 31, 36,
    0, 5, 5, 9, 17, 20, 27, 29, 35,
    0, 6, 7, 12, 12, 16, 24, 27, 34,
    0, 7, 9, 15, 16, 21, 21, 25, 33,
    0, 8, 11, 18, 20, 26, 27, 32, 32,
    0, 0, 4, 12, 15, 22, 24, 30, 31,
    0, 1, 6, 6, 10, 18, 21, 28, 30,
    0, 2, 8, 9, 14, 14, 18, 26, 29,
    0, 3, 10, 12, 18, 19, 24, 24, 28,
];

#[inline(always)]
fn lattice_count_3(t: i64) -> i64 {
    if t < 8 { return 0; }
    let n = (t - 3) / 5;
    let sum_y = n * t - 5 * n * (n + 1) / 2;
    let q = n / 3;
    let r = (n % 3) as usize;
    let tm = (t % 3) as usize;
    let sum_mod = q * 3 + TAB3[tm * 3 + r];
    (sum_y - sum_mod) / 3
}

#[inline(always)]
fn lattice_count_9(t: i64) -> i64 {
    if t < 14 { return 0; }
    let n = (t - 9) / 5;
    let sum_y = n * t - 5 * n * (n + 1) / 2;
    let q = n / 9;
    let r = (n % 9) as usize;
    let tm = (t % 9) as usize;
    let sum_mod = q * 36 + TAB9[tm * 9 + r];
    (sum_y - sum_mod) / 9
}

fn main() {
    let l = (3 * BIG_N + 5) / 2;
    // For d=3 (g % 3 == 0), t >= 8 => g <= l / 8.
    // For d=9 (g % 3 != 0), t >= 14 => g <= l / 14.
    let g_limit = (l / 8) as usize;
    let g_lim_9 = (l / 14) as usize;

    let mut mobius = vec![0i8; g_limit + 1];
    let mut is_composite = vec![false; g_limit + 1];
    let mut primes: Vec<usize> = Vec::with_capacity(g_limit / 10);

    mobius[1] = 1;

    for i in 2..=g_limit {
        if !is_composite[i] {
            primes.push(i);
            unsafe { *mobius.get_unchecked_mut(i) = -1; }
        }
        let mi = unsafe { *mobius.get_unchecked(i) };
        for &p in &primes {
            let ip = i * p;
            if ip > g_limit { break; }
            is_composite[ip] = true;
            if i % p == 0 {
                break;
            } else {
                unsafe { *mobius.get_unchecked_mut(ip) = -mi; }
            }
        }
    }

    drop(is_composite);
    drop(primes);

    let mut ans: i64 = (1..=g_limit).into_par_iter().map(|g| {
        let m = unsafe { *mobius.get_unchecked(g) };
        if m == 0 { return 0; }
        let is_mod3 = g % 3 == 0;
        if !is_mod3 && g > g_lim_9 {
            return 0;
        }
        let t = l / g as i64;
        let count = if is_mod3 {
            lattice_count_3(t)
        } else {
            lattice_count_9(t)
        };
        m as i64 * count
    }).sum::<i64>();

    ans *= 4;
    ans += 2;

    println!("{}", ans);
}
