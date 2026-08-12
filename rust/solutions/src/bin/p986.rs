// Problem 986: Another Infinite Game
// Ported from the Python reference: CA thresholds S[n] = H(1,n), reduction for H(c,d).
// Hot path: extinction simulation. Use i32 stack buffers; wave-parallelize S[n].

use euler_utils::gcd;
use rayon::prelude::*;
use std::collections::HashMap;

const LIMIT: usize = 160;
const PREDICT_START_N: usize = 33;
// Max prediction error ~5k; keep headroom to avoid expand path.
const SEARCH_WINDOW: i64 = 8192;
const MAX_CELLS: usize = 256; // max_n = 160 + 79 = 239
const WAVE: usize = 8; // predict uses S[n-8], so waves of 8 are independent

/// Exceptions to H(c,1) = S[1 + (c-1)//2] for reduced pairs (c,1).
fn exception_h(c: usize) -> Option<i64> {
    match c {
        2 => Some(3),
        3 => Some(5),
        4 => Some(7),
        5 => Some(11),
        6 => Some(13),
        8 => Some(21),
        10 => Some(31),
        _ => None,
    }
}

/// Simulate circular floor-average until all-zero (extinct) or all-nonzero (survives).
#[inline]
fn extinct_for_k1(n: usize, k: i64) -> bool {
    if k == 0 {
        return true;
    }
    debug_assert!(n + 1 <= MAX_CELLS);
    debug_assert!(k <= i32::MAX as i64);

    let size = n + 1;
    let last = size - 1;
    let mut cells = [0i32; MAX_CELLS];
    cells[last] = k as i32;
    let mut zero_count = last as i32;
    let size_i = size as i32;

    loop {
        for i in 0..last {
            let old = cells[i];
            let nxt = (old + cells[i + 1]) >> 1;
            cells[i] = nxt;
            // branchless zero-count update
            zero_count += (nxt == 0) as i32 - (old == 0) as i32;
        }

        let old = cells[last];
        let nxt = (old + cells[0]) >> 1;
        cells[last] = nxt;
        zero_count += (nxt == 0) as i32 - (old == 0) as i32;

        if zero_count == size_i {
            return true;
        }
        if zero_count == 0 {
            return false;
        }
    }
}

fn threshold_k1_plain(n: usize) -> i64 {
    let mut lo = 0i64;
    let mut hi = 1i64;
    while extinct_for_k1(n, hi) {
        lo = hi;
        hi *= 2;
    }
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if extinct_for_k1(n, mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

fn predict_k1_from_previous(s: &[i64], n: usize) -> i64 {
    let a = s[n - 32];
    let b = s[n - 24];
    let c = s[n - 16];
    let d = s[n - 8];
    d + (d - c) + (d - 2 * c + b) + (d - 3 * c + 3 * b - a)
}

fn threshold_k1_with_guess(n: usize, guess: i64) -> i64 {
    let mut lo = (guess - SEARCH_WINDOW).max(0);
    let mut hi = guess + SEARCH_WINDOW;

    while lo > 0 && !extinct_for_k1(n, lo) {
        hi = lo;
        lo /= 2;
    }

    while extinct_for_k1(n, hi) {
        lo = hi;
        hi *= 2;
    }

    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if extinct_for_k1(n, mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

fn build_s_sequence(max_n: usize) -> Vec<i64> {
    let mut s = vec![0i64; max_n + 1];

    // Small n: independent plain binary searches.
    let plain_end = PREDICT_START_N.min(max_n + 1);
    let plain: Vec<(usize, i64)> = (1..plain_end)
        .into_par_iter()
        .map(|n| (n, threshold_k1_plain(n)))
        .collect();
    for (n, v) in plain {
        s[n] = v;
    }

    // Larger n: prediction uses S[n-8], so process in waves of WAVE (independent within wave).
    let mut start = plain_end;
    while start <= max_n {
        let end = (start + WAVE - 1).min(max_n);
        // Guesses need s[n-8].. which is filled for n >= start if WAVE <= 8 and we use n-8.
        let wave: Vec<(usize, i64)> = (start..=end)
            .into_par_iter()
            .map(|n| {
                let guess = predict_k1_from_previous(&s, n);
                (n, threshold_k1_with_guess(n, guess))
            })
            .collect();
        for (n, v) in wave {
            s[n] = v;
        }
        start = end + 1;
    }
    s
}

fn h_reduced(c: usize, d: usize, s: &[i64]) -> i64 {
    if d == 1 {
        if let Some(h) = exception_h(c) {
            return h;
        }
    }
    s[d + (c - 1) / 2]
}

fn solve(limit: usize) -> i64 {
    let max_n = limit + (limit - 1) / 2;
    let s = build_s_sequence(max_n);

    debug_assert_eq!(2 * h_reduced(2, 1, &s) + 1, 7);
    debug_assert_eq!(2 * h_reduced(1, 2, &s) + 1, 7);
    debug_assert_eq!(2 * h_reduced(3, 1, &s) + 1, 11);
    debug_assert_eq!(2 * h_reduced(1, 1, &s) + 1, 3);

    // Double loop over reduced pairs; outer c is independent.
    (1..=limit)
        .into_par_iter()
        .map(|c| {
            let mut memo: HashMap<(usize, usize), i64> = HashMap::new();
            let mut sub = 0i64;
            for d in 1..=limit {
                let g = gcd(c as u64, d as u64) as usize;
                let key = (c / g, d / g);
                let val = *memo.entry(key).or_insert_with(|| {
                    let h = h_reduced(key.0, key.1, &s);
                    2 * h + 1
                });
                sub += val;
            }
            sub
        })
        .sum()
}

fn main() {
    println!("{}", solve(LIMIT));
}
