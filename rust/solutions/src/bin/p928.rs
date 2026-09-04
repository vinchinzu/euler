// Project Euler 928 – Cribbage Scoring
//
// For every non-empty subset of a standard 52-card deck compute
//   hand_score  = sum of card values (A=1, 2‑9 face, 10/J/Q/K=10)
//   crib_score  = pairs + runs + fifteens
// Count the number of subsets where hand_score == crib_score.

use rayon::prelude::*;

const NRANKS: usize = 13;
const MAX_COUNT: usize = 4;
const TARGET_SUM: usize = 15;

/// C(n, k) for n, k ≤ 4
const BINOM: [[u32; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 2, 1, 0, 0],
    [1, 3, 3, 1, 0],
    [1, 4, 6, 4, 1],
];

const CHOOSE_4: [i64; 5] = [1, 4, 6, 4, 1];
const PAIR_SCORE: [i64; 5] = [0, 0, 2, 6, 12];
const RANK_VAL: [usize; NRANKS] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];
const MAX_REM: [i64; NRANKS + 1] = [
    340, 336, 328, 316, 300, 280, 256, 228, 196, 160, 120, 80, 40, 0,
];

type Gf = [u32; TARGET_SUM + 1];

/// Multiply gf by the polynomial for a rank with given `value` and
/// `count` cards in hand: Σ_{k=0}^{count} C(count,k) · x^{k·value}
#[inline(always)]
fn extend_gf(gf: &Gf, value: usize, count: usize) -> Gf {
    let mut r = *gf;
    for (k, &coeff) in BINOM[count].iter().enumerate().take(count + 1).skip(1) {
        let pwr = k * value;
        if pwr > TARGET_SUM {
            break;
        }
        for s in 0..=(TARGET_SUM - pwr) {
            unsafe {
                *r.get_unchecked_mut(s + pwr) += *gf.get_unchecked(s) * coeff;
            }
        }
    }
    r
}

#[allow(clippy::too_many_arguments)]
fn recurse_tail(
    idx: usize,
    hs: i64,
    ps: i64,
    ra: i64,
    rl: usize,
    rp: i64,
    f15: i64,
    g7: i64,
    g6: i64,
    g5: i64,
    has_any: bool,
    nh: i64,
    total: &mut i64,
) {
    if idx == NRANKS {
        if !has_any {
            return;
        }
        let runs = if rl >= 3 { ra + rl as i64 * rp } else { ra };
        let deficit = hs - ps - runs;
        if deficit >= 0 && f15 * 2 == deficit {
            *total += nh;
        }
        return;
    }

    let v = RANK_VAL[idx] as i64;
    let mult = match idx {
        7 => g7,
        8 => g6,
        _ => g5,
    };

    for c in 0..=MAX_COUNT {
        let new_hs = hs + c as i64 * v;
        let new_ps = ps + PAIR_SCORE[c];

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

        let min_runs = new_ra + if new_rl >= 3 { new_rl as i64 * new_rp } else { 0 };
        let new_f15 = f15 + c as i64 * mult;
        if new_ps + min_runs + 2 * new_f15 > new_hs + MAX_REM[idx + 1] {
            continue;
        }

        let new_nh = nh * CHOOSE_4[c];
        recurse_tail(
            idx + 1,
            new_hs,
            new_ps,
            new_ra,
            new_rl,
            new_rp,
            new_f15,
            g7,
            g6,
            g5,
            has_any || c > 0,
            new_nh,
            total,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn recurse(
    idx: usize,
    hs: i64,
    ps: i64,
    ra: i64,
    rl: usize,
    rp: i64,
    gf: &Gf,
    has_any: bool,
    nh: i64,
    total: &mut i64,
) {
    if idx == 7 {
        recurse_tail(
            7,
            hs,
            ps,
            ra,
            rl,
            rp,
            gf[15] as i64,
            gf[7] as i64,
            gf[6] as i64,
            gf[5] as i64,
            has_any,
            nh,
            total,
        );
        return;
    }

    let v = RANK_VAL[idx] as i64;

    for c in 0..=MAX_COUNT {
        let new_hs = hs + c as i64 * v;
        let new_ps = ps + PAIR_SCORE[c];

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

        let min_runs = new_ra + if new_rl >= 3 { new_rl as i64 * new_rp } else { 0 };
        if new_ps + min_runs + 2 * (gf[TARGET_SUM] as i64) > new_hs + MAX_REM[idx + 1] {
            continue;
        }

        let new_nh = nh * CHOOSE_4[c];
        if c == 0 {
            recurse(
                idx + 1, new_hs, new_ps, new_ra, new_rl, new_rp, gf, has_any, new_nh,
                total,
            );
        } else {
            let new_gf = extend_gf(gf, RANK_VAL[idx], c);
            if new_ps + min_runs + 2 * (new_gf[TARGET_SUM] as i64) > new_hs + MAX_REM[idx + 1] {
                continue;
            }
            recurse(
                idx + 1, new_hs, new_ps, new_ra, new_rl, new_rp, &new_gf, true, new_nh,
                total,
            );
        }
    }
}

struct Task {
    hs: i64,
    ps: i64,
    ra: i64,
    rl: usize,
    rp: i64,
    gf: Gf,
    has_any: bool,
    nh: i64,
}

fn main() {
    const SPLIT: usize = 4;
    let mut tasks = Vec::with_capacity(504);
    for c0 in 0..=MAX_COUNT {
        for c1 in 0..=MAX_COUNT {
            for c2 in 0..=MAX_COUNT {
                for c3 in 0..=MAX_COUNT {
                    let cs = [c0, c1, c2, c3];
                    let mut hs = 0i64;
                    let mut ps = 0i64;
                    let mut ra = 0i64;
                    let mut rl = 0usize;
                    let mut rp = 0i64;
                    let mut has_any = false;
                    let mut nh = 1i64;

                    let mut gf: Gf = [0; TARGET_SUM + 1];
                    gf[0] = 1;

                    for i in 0..SPLIT {
                        let c = cs[i];
                        nh *= CHOOSE_4[c];
                        hs += c as i64 * RANK_VAL[i] as i64;
                        ps += PAIR_SCORE[c];
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

                    let min_runs = ra + if rl >= 3 { rl as i64 * rp } else { 0 };
                    if ps + min_runs + 2 * (gf[TARGET_SUM] as i64) > hs + MAX_REM[SPLIT] {
                        continue;
                    }

                    tasks.push(Task {
                        hs,
                        ps,
                        ra,
                        rl,
                        rp,
                        gf,
                        has_any,
                        nh,
                    });
                }
            }
        }
    }

    let total: i64 = tasks
        .par_iter()
        .map(|task| {
            let mut local_total = 0i64;
            recurse(
                SPLIT,
                task.hs,
                task.ps,
                task.ra,
                task.rl,
                task.rp,
                &task.gf,
                task.has_any,
                task.nh,
                &mut local_total,
            );
            local_total
        })
        .sum();

    println!("{}", total);
}
