// Project Euler 928 - Cribbage Scoring
// Enumerate all multisets of cards, compute hand score and cribbage score.
// Count hands where hand_score == cribbage_score.
// Optimized with incremental pairs, fifteens GF, and pruning.

use rayon::prelude::*;

const NRANKS: usize = 13;
const MAX_COUNT: usize = 4;
const TARGET_SUM: usize = 15;

fn rank_value(rank: usize) -> usize {
    if rank == 0 {
        return 1;
    }
    if rank >= 9 {
        return 10;
    }
    rank + 1
}

fn suit_combinations(count: usize) -> i64 {
    match count {
        0 => 1,
        1 => 4,
        2 => 6,
        3 => 4,
        4 => 1,
        _ => 0,
    }
}

fn calculate_pairs_score(counts: &[usize; NRANKS]) -> i64 {
    let mut score: i64 = 0;
    for i in 0..NRANKS {
        if counts[i] >= 2 {
            score += (counts[i] * (counts[i] - 1) / 2) as i64 * 2;
        }
    }
    score
}

fn calculate_runs_score(counts: &[usize; NRANKS]) -> i64 {
    let mut score: i64 = 0;
    let mut i = 0;
    while i < NRANKS - 2 {
        let mut run_length = 0;
        let mut j = i;
        while j < NRANKS && counts[j] > 0 {
            run_length += 1;
            j += 1;
        }
        if run_length >= 3 {
            let mut run_product: i64 = 1;
            for k in i..i + run_length {
                run_product *= counts[k] as i64;
            }
            score += run_length as i64 * run_product;
            i += run_length;
        } else {
            i += 1;
        }
    }
    score
}

fn calculate_fifteens_score(counts: &[usize; NRANKS]) -> i64 {
    let mut gf = [0i64; TARGET_SUM + 1];
    gf[0] = 1;

    for idx in 0..NRANKS {
        let value = rank_value(idx);
        let count = counts[idx];
        let mut new_gf = [0i64; TARGET_SUM + 1];

        for k in 0..=count {
            let coeff = suit_combinations(k);
            let power = k * value;
            if power > TARGET_SUM {
                break;
            }
            for s in 0..=(TARGET_SUM - power) {
                new_gf[s + power] += gf[s] * coeff;
            }
        }
        gf = new_gf;
    }

    gf[TARGET_SUM] * 2
}

fn calculate_hand_score(counts: &[usize; NRANKS]) -> i64 {
    let mut total: i64 = 0;
    for i in 0..NRANKS {
        total += counts[i] as i64 * rank_value(i) as i64;
    }
    total
}

fn calculate_num_hands(counts: &[usize; NRANKS]) -> i64 {
    let mut product: i64 = 1;
    for i in 0..NRANKS {
        product *= suit_combinations(counts[i]);
    }
    product
}

type Gf = [i64; TARGET_SUM + 1];

/// Extend the generating function with rank `idx` having count `c`.
#[inline]
fn extend_gf(gf: &Gf, idx: usize, c: usize) -> Gf {
    if c == 0 {
        return *gf;
    }
    let value = rank_value(idx);
    let mut new_gf = [0i64; TARGET_SUM + 1];
    // k=0 contribution: gf * 1
    for s in 0..=TARGET_SUM {
        new_gf[s] = gf[s];
    }
    // k=1..c contributions
    for k in 1..=c {
        let coeff = suit_combinations(k);
        let power = k * value;
        if power > TARGET_SUM {
            break;
        }
        for s in 0..=(TARGET_SUM - power) {
            new_gf[s + power] += gf[s] * coeff;
        }
    }
    new_gf
}

fn recurse(
    idx: usize,
    counts: &mut [usize; NRANKS],
    hand_score: i64,
    pairs_score: i64,
    gf: &Gf,
    has_any: bool,
    total: &mut i64,
) {
    if idx == NRANKS {
        if !has_any {
            return;
        }
        let fifteens = gf[TARGET_SUM] * 2;
        let runs = calculate_runs_score(counts);
        let cribbage_score = pairs_score + runs + fifteens;

        // Debug: verify against original calculation
        debug_assert_eq!(pairs_score, calculate_pairs_score(counts));
        debug_assert_eq!(fifteens, calculate_fifteens_score(counts));
        debug_assert_eq!(hand_score, calculate_hand_score(counts));

        if hand_score == cribbage_score {
            *total += calculate_num_hands(counts);
        }
        return;
    }

    let val = rank_value(idx) as i64;

    for c in 0..=MAX_COUNT {
        counts[idx] = c;
        let new_hand_score = hand_score + c as i64 * val;
        let new_has_any = has_any || c > 0;

        // Incremental pairs score
        let new_pairs_score = if c >= 2 {
            pairs_score + (c * (c - 1) / 2) as i64 * 2
        } else {
            pairs_score
        };

        // Extend generating function incrementally
        let new_gf = extend_gf(gf, idx, c);

        recurse(
            idx + 1,
            counts,
            new_hand_score,
            new_pairs_score,
            &new_gf,
            new_has_any,
            total,
        );
    }
    counts[idx] = 0;
}

fn main() {
    let mut counts = [0usize; NRANKS];
    let gf: Gf = [0; TARGET_SUM + 1];
    let mut gf_init = gf;
    gf_init[0] = 1;
    let mut total: i64 = 0;
    recurse(0, &mut counts, 0, 0, &gf_init, false, &mut total);
    println!("{}", total);
}
