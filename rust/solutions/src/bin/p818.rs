// Project Euler 818 - SET
// Sum of S(C)^4 over all 12-card subsets of the 81-card SET deck

use rayon::prelude::*;

const B: usize = 3;
const D: usize = 4;
const NCARDS: usize = 81;
const NN: usize = 12;
const KK: usize = 4;

#[inline(always)]
fn ncr(n: i64, r: i64) -> i64 {
    if r < 0 || r > n {
        return 0;
    }
    let r = r.min(n - r);
    let mut v = 1i64;
    for i in 0..r {
        v = v * (n - i) / (i + 1);
    }
    v
}

#[inline(always)]
fn add_hist(mut a: [i64; NN + 1], b: [i64; NN + 1]) -> [i64; NN + 1] {
    for i in 0..=NN {
        a[i] += b[i];
    }
    a
}

fn main() {
    let mut cards = [[0u8; D]; NCARDS];
    for i in 0..NCARDS {
        let mut v = i;
        for d in (0..D).rev() {
            cards[i][d] = (v % B) as u8;
            v /= B;
        }
    }

    // Each SET as a 81-bit mask (bits 0..80)
    let mut masks = Vec::with_capacity(1080);
    for i in 0..NCARDS {
        for j in (i + 1)..NCARDS {
            let mut k = 0usize;
            let mut p = 1usize;
            for d in (0..D).rev() {
                let kd = (B * 2 - cards[i][d] as usize - cards[j][d] as usize) % B;
                k += kd * p;
                p *= B;
            }
            // card_index of digits stored high-to-low matches i itself
            if k > j {
                masks.push((1u128 << i) | (1u128 << j) | (1u128 << k));
            }
        }
    }
    let num_sets = masks.len();
    let masks = masks.as_slice();
    let m0 = masks[0];

    // stirling2(4,e)*e! / e  for e=1..=4
    let coeff: [i64; 5] = [0, 1, 7, 12, 6];
    let mut ncr_k = [0i64; NN + 1];
    for k in 0..=NN {
        ncr_k[k] = ncr((NCARDS - k) as i64, (NN - k) as i64);
    }

    // e=1: distinguished SET covers 3 cards
    let mut hist1 = [0i64; NN + 1];
    hist1[B] = 1;

    // Fix SET 0; enumerate the other e-1 SETs. 3e <= 12 so the union always fits.
    let (hist2, hist3, hist4) = (1..num_sets)
        .into_par_iter()
        .fold(
            || ([0i64; NN + 1], [0i64; NN + 1], [0i64; NN + 1]),
            |(mut c2, mut c3, mut c4), si| {
                // SAFETY: si in 1..num_sets; union of 2/3/4 SETs has 3..=12 points
                unsafe {
                    let mi = *masks.get_unchecked(si);
                    let occ1 = m0 | mi;
                    *c2.get_unchecked_mut(occ1.count_ones() as usize) += 1;
                    for sj in si + 1..num_sets {
                        let occ2 = occ1 | *masks.get_unchecked(sj);
                        let base = occ2.count_ones() as usize;
                        *c3.get_unchecked_mut(base) += 1;
                        for sk in sj + 1..num_sets {
                            let k = base + (*masks.get_unchecked(sk) & !occ2).count_ones() as usize;
                            *c4.get_unchecked_mut(k) += 1;
                        }
                    }
                }
                (c2, c3, c4)
            },
        )
        .reduce(
            || ([0i64; NN + 1], [0i64; NN + 1], [0i64; NN + 1]),
            |(a2, a3, a4), (b2, b3, b4)| (add_hist(a2, b2), add_hist(a3, b3), add_hist(a4, b4)),
        );

    let hists = [hist1, hist2, hist3, hist4];
    let mut ans = 0i64;
    for e in 1..=KK {
        let h = &hists[e - 1];
        for k in 0..=NN {
            if h[k] != 0 {
                ans += coeff[e] * num_sets as i64 * ncr_k[k] * h[k];
            }
        }
    }

    println!("{}", ans);
}
