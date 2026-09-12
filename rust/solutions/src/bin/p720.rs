// Project Euler 720 - Unpredictable Permutations
//
// Build elements[] and ranks[] for N=25 (2^25 entries), then factorial-weighted
// differences. elements/ranks fit i32; MOD^2 fits u64.

use rayon::prelude::*;

const N: usize = 25;
const MOD: i64 = 1_000_000_007;
const L: usize = 1 << N;

fn main() {
    let mut elements = vec![0i32; L];
    let mut ranks = vec![0i32; L];

    elements[0] = 1;
    elements[1] = 3;
    elements[2] = 2;
    elements[3] = 4;
    ranks[0] = 1;
    ranks[1] = 2;
    ranks[2] = 2;
    ranks[3] = 4;

    let mut i = 4usize;
    while i < L {
        if i >= 1 << 16 {
            let (el_lo, el_hi) = elements.split_at_mut(i);
            let (rk_lo, rk_hi) = ranks.split_at_mut(i);
            el_lo
                .par_iter_mut()
                .zip(el_hi.par_iter_mut())
                .zip(rk_lo.par_iter_mut())
                .zip(rk_hi.par_iter_mut())
                .with_min_len(4096)
                .for_each(|(((e, e2), r), r2)| {
                    let ev = *e;
                    *r2 = *r + ev;
                    *e2 = ev * 2;
                    *e = ev * 2 - 1;
                });
        } else {
            for j in 0..i {
                ranks[i + j] = ranks[j] + elements[j];
                elements[i + j] = 2 * elements[j];
                elements[j] = 2 * elements[j] - 1;
            }
        }
        elements[i - 1] = 2;
        elements[i] = 2 * i as i32 - 1;
        ranks[i - 1] = 2;
        ranks[i] = i as i32 + 1;
        i *= 2;
    }

    let mut factorials = vec![0i32; L];
    factorials[0] = 1;
    let mut acc = 1i64;
    for i in 1..L {
        acc = acc * (i as i64) % MOD;
        factorials[i] = acc as i32;
    }

    let sum: i64 = elements
        .par_iter()
        .zip(ranks.par_iter())
        .zip(factorials.par_iter().rev())
        .with_min_len(1 << 12)
        .map(|((&e, &r), &f)| {
            let mut t = f as i64 * (e - r) as i64 % MOD;
            if t < 0 {
                t += MOD;
            }
            t
        })
        .sum();

    println!("{}", (1 + sum) % MOD);
}
