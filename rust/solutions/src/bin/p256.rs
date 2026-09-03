// Project Euler 256: Tatami-Free Rooms
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static BEST: AtomicUsize = AtomicUsize::new(usize::MAX);

fn solve_chunk(s_min: usize, s_max: usize, sq: usize) -> Option<usize> {
    if s_min >= BEST.load(Ordering::Relaxed) {
        return None;
    }
    const TARGET: i16 = 200;
    let len = s_max - s_min;
    let mut counts = vec![0i16; len];

    let max_a = sq.min(((s_max - 1) as f64).sqrt() as usize);

    for a in 5..=max_a {
        if s_min >= BEST.load(Ordering::Relaxed) {
            return None;
        }
        let b_min_req = a.max((s_min + a - 1) / a);
        let b_max_req = (s_max - 1) / a;
        if b_min_req > b_max_req || b_max_req < 2 {
            continue;
        }

        let max_k = (a - 5) / 2;
        let k_start = (b_min_req + a) / (a - 1) - 1;
        let k_end = ((b_max_req - 2) / (a + 1)).min(max_k);

        if k_start > k_end {
            continue;
        }

        let a_plus_1 = a + 1;
        let a_minus_1 = a - 1;
        let mut lo_raw = k_start * a_plus_1 + 2;
        let mut hi_raw = (k_start + 1) * a_minus_1 - 2;

        for _ in k_start..=k_end {
            let lo = lo_raw.max(b_min_req);
            let hi = hi_raw.min(b_max_req);
            if lo <= hi {
                let mut idx = a * lo - s_min;
                let end_idx = a * hi - s_min;
                while idx <= end_idx {
                    unsafe {
                        *counts.get_unchecked_mut(idx) += 1;
                    }
                    idx += a;
                }
            }
            lo_raw += a_plus_1;
            hi_raw += a_minus_1;
        }
    }

    let mut found = None;
    for (i, &c) in counts.iter().enumerate() {
        if c == TARGET {
            let ans = s_min + i;
            BEST.fetch_min(ans, Ordering::Relaxed);
            found = Some(ans);
            break;
        }
    }
    found
}

fn main() {
    const LIMIT: usize = 100_000_000;
    const CHUNK_SIZE: usize = 2_000_000;

    let sq = (LIMIT as f64).sqrt() as usize;

    let mut chunks = Vec::new();
    let mut s = 1;
    while s <= LIMIT {
        let next_s = (s + CHUNK_SIZE).min(LIMIT + 1);
        chunks.push((s, next_s));
        s = next_s;
    }

    let result = chunks
        .into_par_iter()
        .filter_map(|(s_min, s_max)| solve_chunk(s_min, s_max, sq))
        .min();

    if let Some(ans) = result {
        println!("{}", ans);
    } else {
        println!("0");
    }
}
