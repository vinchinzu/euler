// Project Euler 583 - Heron Envelopes
//
// A Heron envelope is a pentagon ABCDE = rectangle ABDE + isosceles triangle BCD,
// with all sides and diagonals integral, and flap height < rectangle height.
//
// Setting up coordinates with half-width b, rectangle height h, flap height f:
//   Condition 1: b^2 + f^2 = s^2  (triangle side BC = CD = s)
//   Condition 2: (2b)^2 + h^2 = d^2  (rectangle diagonal)
//   Condition 3: b^2 + (h+f)^2 = t^2  (pentagon diagonal AC = CE)
//   f < h, perimeter = 2b + 2h + 2s <= N
//
// Scaling conditions 1,3 by 2 gives common leg w=2b:
//   w^2 + (2f)^2 = (2s)^2
//   w^2 + h^2 = d^2
//   w^2 + (2h+2f)^2 = (2t)^2
//
// For each even w, other_legs come from factor pairs of w^2. Then find
// (A_leg=2f, C_leg=h) such that A_leg + 2*C_leg is also an other_leg,
// A's hypotenuse is even, and f < h. Perimeter = A_hyp + w + 2*C_leg.

use rayon::prelude::*;

const LIMIT: i32 = 10_000_000;

fn sieve_spf(n: usize) -> Vec<u32> {
    let mut spf = vec![0u32; n + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(n / 10);
    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
        }
        let spi = spf[i];
        for &p in &primes {
            let j = i as u64 * p as u64;
            if j > n as u64 {
                break;
            }
            spf[j as usize] = p;
            if p == spi {
                break;
            }
        }
    }
    spf
}

fn process_group(pairs: &[(i32, i32)], w: i64) -> i64 {
    let n = pairs.len();
    if n < 3 {
        return 0;
    }
    let mut ans = 0i64;
    for i in 0..n {
        let a_leg = pairs[i].0;
        let a_hyp = pairs[i].1;
        if a_hyp & 1 != 0 {
            continue;
        }
        let f = a_leg >> 1;
        let remain = LIMIT as i64 - a_hyp as i64 - w;
        if remain <= 0 {
            continue;
        }
        let h_max = (remain >> 1) as i32;
        if h_max <= f {
            continue;
        }
        let mut j = pairs.partition_point(|p| p.0 <= f);
        let mut k = 0usize;
        while j < n {
            let h = pairs[j].0;
            if h > h_max {
                break;
            }
            let target = a_leg + h + h;
            while k < n && pairs[k].0 < target {
                k += 1;
            }
            if k < n && pairs[k].0 == target {
                ans += a_hyp as i64 + w + ((h as i64) << 1);
            }
            j += 1;
        }
    }
    ans
}

fn fill_pairs(
    w: u32,
    spf: &[u32],
    fac: &mut Vec<(u32, u32)>,
    divs: &mut Vec<u32>,
    pairs: &mut Vec<(i32, i32)>,
) {
    fac.clear();
    divs.clear();
    pairs.clear();

    let tz = w.trailing_zeros();
    let mut x = w >> tz;
    fac.push((2, tz << 1));
    while x > 1 {
        // SAFETY: x decreases from w <= LIMIT; spf.len() == LIMIT+1
        let p = unsafe { *spf.get_unchecked(x as usize) };
        let mut e = 0u32;
        while x > 1 && unsafe { *spf.get_unchecked(x as usize) } == p {
            x /= p;
            e += 1;
        }
        fac.push((p, e << 1));
    }

    divs.push(1);
    let w64 = w as u64;
    for &(p, exp) in fac.iter() {
        let n0 = divs.len();
        let mut mul = 1u64;
        let p64 = p as u64;
        for _ in 0..exp {
            mul *= p64;
            for i in 0..n0 {
                let v = unsafe { *divs.get_unchecked(i) } as u64 * mul;
                if v < w64 {
                    divs.push(v as u32);
                }
            }
        }
    }

    let w2 = w64 * w64;
    let lim = LIMIT as u64;
    for &u in divs.iter() {
        let uu = u as u64;
        let v = w2 / uu;
        if (uu ^ v) & 1 != 0 {
            continue;
        }
        let d = (v + uu) >> 1;
        if d > lim {
            continue;
        }
        let h = (v - uu) >> 1;
        if h != 0 {
            pairs.push((h as i32, d as i32));
        }
    }
}

fn main() {
    let n = LIMIT as usize;
    let spf = sieve_spf(n);
    let n_even = n / 2;

    let ans: i64 = (1..n_even + 1)
        .into_par_iter()
        .with_min_len(8)
        .with_max_len(64)
        .map_init(
            || {
                (
                    Vec::<(u32, u32)>::with_capacity(16),
                    Vec::<u32>::with_capacity(64),
                    Vec::<(i32, i32)>::with_capacity(64),
                )
            },
            |(fac, divs, pairs), i| {
                let w = (i as u32) * 2;
                fill_pairs(w, &spf, fac, divs, pairs);
                if pairs.len() >= 3 {
                    pairs.sort_unstable_by_key(|p| p.0);
                    process_group(pairs, w as i64)
                } else {
                    0
                }
            },
        )
        .sum();

    println!("{}", ans);
}
