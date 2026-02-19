// Project Euler 928 – Cribbage Scoring
//
// For every non-empty subset of a standard 52-card deck compute
//   hand_score  = sum of card values (A=1, 2‑9 face, 10/J/Q/K=10)
//   crib_score  = pairs + runs + fifteens
// Count the number of subsets where hand_score == crib_score.
//
// BUG FIX: the fifteens GF must use C(count_i, k) — the number of
// ways to choose k cards from the count_i cards of rank i in the hand.
// The old code used C(4, k) which overcounts fifteens.

use rayon::prelude::*;

const NRANKS: usize = 13;
const MAX_COUNT: usize = 4;
const TARGET_SUM: usize = 15;

/// C(n, k) for n, k ≤ 4
const BINOM: [[i64; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 2, 1, 0, 0],
    [1, 3, 3, 1, 0],
    [1, 4, 6, 4, 1],
];

const RANK_VAL: [usize; NRANKS] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];

type Gf = [i64; TARGET_SUM + 1];

/// Multiply gf by the polynomial for a rank with given `value` and
/// `count` cards in hand: Σ_{k=0}^{count} C(count,k) · x^{k·value}
#[inline(always)]
fn extend_gf(gf: &Gf, value: usize, count: usize) -> Gf {
    let mut r = [0i64; TARGET_SUM + 1];
    for k in 0..=count {
        let coeff = BINOM[count][k];
        let pwr = k * value;
        if pwr > TARGET_SUM {
            break;
        }
        for s in 0..=(TARGET_SUM - pwr) {
            // SAFETY: s + pwr ≤ TARGET_SUM, s ≤ TARGET_SUM
            unsafe {
                *r.get_unchecked_mut(s + pwr) += *gf.get_unchecked(s) * coeff;
            }
        }
    }
    r
}

fn recurse(
    idx: usize,
    counts: &mut [usize; NRANKS],
    hs: i64,   // hand_score accumulated
    ps: i64,   // pairs_score accumulated
    ra: i64,   // runs_score accumulated (finalized runs only)
    rl: usize, // current open-run length
    rp: i64,   // current open-run product of counts
    gf: &Gf,
    has_any: bool,
    total: &mut i64,
    max_rem: &[i64; NRANKS + 1],
) {
    if idx == NRANKS {
        if !has_any {
            return;
        }
        let runs = if rl >= 3 { ra + rl as i64 * rp } else { ra };
        let deficit = hs - ps - runs;
        if deficit < 0 {
            return;
        }
        let need = gf[TARGET_SUM];
        if need * 2 == deficit {
            let mut nh = 1i64;
            for i in 0..NRANKS {
                nh *= BINOM[4][counts[i]];
            }
            *total += nh;
        }
        return;
    }

    let v = RANK_VAL[idx] as i64;

    for c in 0..=MAX_COUNT {
        let new_hs = hs + c as i64 * v;
        let new_ps = ps + if c >= 2 { (c * (c - 1) / 2) as i64 * 2 } else { 0 };

        // Incremental run tracking
        let (new_ra, new_rl, new_rp) = if c > 0 {
            if rl > 0 {
                (ra, rl + 1, rp * c as i64)
            } else {
                (ra, 1, c as i64)
            }
        } else if rl >= 3 {
            (ra + rl as i64 * rp, 0, 0)
        } else {
            (ra, 0, 0)
        };

        // Prune: pairs + finalized runs already exceed max possible hand_score
        if new_ps + new_ra > new_hs + max_rem[idx + 1] {
            continue;
        }

        counts[idx] = c;
        if c == 0 {
            recurse(
                idx + 1, counts, new_hs, new_ps, new_ra, new_rl, new_rp, gf, has_any,
                total, max_rem,
            );
        } else {
            let new_gf = extend_gf(gf, RANK_VAL[idx], c);
            recurse(
                idx + 1, counts, new_hs, new_ps, new_ra, new_rl, new_rp, &new_gf, true,
                total, max_rem,
            );
        }
    }
    counts[idx] = 0;
}

fn main() {
    // max_rem[i] = max additional hand_score from ranks i..NRANKS-1
    let mut max_rem = [0i64; NRANKS + 1];
    for i in (0..NRANKS).rev() {
        max_rem[i] = max_rem[i + 1] + MAX_COUNT as i64 * RANK_VAL[i] as i64;
    }

    // Parallelize: depth-3 split (125 tasks)
    const SPLIT: usize = 3;
    let mut tasks = Vec::new();
    for c0 in 0..=MAX_COUNT {
        for c1 in 0..=MAX_COUNT {
            for c2 in 0..=MAX_COUNT {
                tasks.push([c0, c1, c2]);
            }
        }
    }

    let total: i64 = tasks
        .par_iter()
        .map(|cs| {
            let mut counts = [0usize; NRANKS];
            let mut hs = 0i64;
            let mut ps = 0i64;
            let mut ra = 0i64;
            let mut rl = 0usize;
            let mut rp = 0i64;
            let mut has_any = false;

            let mut gf: Gf = [0; TARGET_SUM + 1];
            gf[0] = 1;

            for i in 0..SPLIT {
                let c = cs[i];
                counts[i] = c;
                hs += c as i64 * RANK_VAL[i] as i64;
                if c >= 2 {
                    ps += (c * (c - 1) / 2) as i64 * 2;
                }
                if c > 0 {
                    has_any = true;
                    gf = extend_gf(&gf, RANK_VAL[i], c);
                    if rl > 0 {
                        rl += 1;
                        rp *= c as i64;
                    } else {
                        rl = 1;
                        rp = c as i64;
                    }
                } else {
                    if rl >= 3 {
                        ra += rl as i64 * rp;
                    }
                    rl = 0;
                    rp = 0;
                }
            }

            // Early prune
            if ps + ra > hs + max_rem[SPLIT] {
                return 0;
            }

            let mut local_total = 0i64;
            recurse(
                SPLIT,
                &mut counts,
                hs,
                ps,
                ra,
                rl,
                rp,
                &gf,
                has_any,
                &mut local_total,
                &max_rem,
            );
            local_total
        })
        .sum();

    println!("{}", total);
}
