// Project Euler 266 - Pseudo Square Root
// Meet-in-the-middle over primes < 190: doubling-DP subset logs, par_sort,
// then parallel two-pointer (largest A-log <= log_sqrt - B-log).

use rayon::prelude::*;

const M: u64 = 10_000_000_000_000_000; // 10^16
const PRIMES: [u16; 42] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
    157, 163, 167, 173, 179, 181,
];

fn subset_sums(logs: &[f64]) -> Vec<(f64, u32)> {
    let n = logs.len();
    let mut v = vec![(0.0f64, 0u32); 1 << n];
    for i in 0..n {
        let lp = logs[i];
        let half = 1 << i;
        let bit = 1u32 << i;
        let (lo, hi) = v.split_at_mut(half);
        for s in 0..half {
            // SAFETY: s < half == lo.len() == prefix of hi
            let (lv, mask) = unsafe { *lo.get_unchecked(s) };
            unsafe {
                *hi.get_unchecked_mut(s) = (lv + lp, mask | bit);
            }
        }
    }
    v
}

fn main() {
    let n = PRIMES.len();
    let mid = n / 2;
    let mut logs = [0.0f64; 42];
    for i in 0..n {
        logs[i] = (PRIMES[i] as f64).ln();
    }
    let log_sqrt = logs.iter().sum::<f64>() * 0.5;

    let (mut pa, mut pb) = rayon::join(
        || subset_sums(&logs[..mid]),
        || subset_sums(&logs[mid..]),
    );
    rayon::join(
        || pa.par_sort_unstable_by(|a, b| a.0.total_cmp(&b.0)),
        || pb.par_sort_unstable_by(|a, b| a.0.total_cmp(&b.0)),
    );

    let nthreads = rayon::current_num_threads().max(1);
    let chunk_size = (pb.len() / nthreads).max(1);

    let (_best_log, best_a, best_b) = pb
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut idx = pa.partition_point(|e| e.0 <= log_sqrt - chunk[0].0);
            let mut best_log = -1.0f64;
            let mut best_a = 0u32;
            let mut best_b = 0u32;
            for &(b_log, b_mask) in chunk {
                let target = log_sqrt - b_log;
                while idx > 0 && unsafe { pa.get_unchecked(idx - 1).0 } > target {
                    idx -= 1;
                }
                if idx > 0 {
                    let a = unsafe { *pa.get_unchecked(idx - 1) };
                    let cand = b_log + a.0;
                    if cand > best_log {
                        best_log = cand;
                        best_a = a.1;
                        best_b = b_mask;
                    }
                }
            }
            (best_log, best_a, best_b)
        })
        .reduce(
            || (-1.0f64, 0u32, 0u32),
            |a, b| if a.0 >= b.0 { a } else { b },
        );

    // Product of selected primes fits in u64 before each reduction (p < 190, M = 10^16).
    let mut ans = 1u64;
    for i in 0..mid {
        if best_a & (1 << i) != 0 {
            ans = ans * PRIMES[i] as u64 % M;
        }
    }
    for i in 0..(n - mid) {
        if best_b & (1 << i) != 0 {
            ans = ans * PRIMES[mid + i] as u64 % M;
        }
    }
    println!("{ans}");
}
