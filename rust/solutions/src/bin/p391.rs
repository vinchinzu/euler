// Project Euler 391 - Hopping Game
//
// Compute sum of M(n)^3 for 1 <= n <= 1000.
// M(n) is found via iterated function composition DP.
//
// For each n, initialize f[s][x] = s+x if s+x < n+1, else 0.
// Iterate: g[s][x] = f[s][f[s+1][x]] for n+1 steps.
// M(n) = f[0][0] after iteration.
//
// Optimizations:
// - Variable stride (n+1 instead of fixed 1010) for cache efficiency
// - Thread-local buffers to avoid repeated allocation
// - Rayon parallelism limited to physical cores (avoids L3 cache thrashing)

use rayon::prelude::*;
use std::cell::RefCell;

thread_local! {
    static BUFS: RefCell<(Vec<u16>, Vec<u16>)> = RefCell::new((Vec::new(), Vec::new()));
}

fn compute(i: usize) -> i64 {
    BUFS.with(|bufs| {
        let mut bufs = bufs.borrow_mut();
        let (buf_a, buf_b) = &mut *bufs;

        let n = i + 1;
        let stride = n + 1;
        let rows = n + 2;
        let size = rows * stride;

        if buf_a.len() < size { buf_a.resize(size, 0); }
        if buf_b.len() < size { buf_b.resize(size, 0); }

        for s in 0..rows {
            let base = s * stride;
            for x in 0..=n {
                buf_a[base + x] = if s + x < n { (s + x) as u16 } else { 0 };
            }
        }

        let mut fi = 0usize;
        for h in 1..=n {
            let xlim = n + 1;
            let (src, dst) = if fi == 0 {
                (buf_a.as_ptr(), buf_b.as_mut_ptr())
            } else {
                (buf_b.as_ptr(), buf_a.as_mut_ptr())
            };
            for s in 0..=(n - h) {
                unsafe {
                    let s1_base = src.add((s + 1) * stride);
                    let s_base_r = src.add(s * stride);
                    let s_base_w = dst.add(s * stride);
                    for x in 0..xlim {
                        let y = *s1_base.add(x) as usize;
                        *s_base_w.add(x) = *s_base_r.add(y);
                    }
                }
            }
            fi = 1 - fi;
        }

        if fi == 0 { buf_a[0] as i64 } else { buf_b[0] as i64 }
    })
}

fn main() {
    // Limit to physical cores to avoid L3 cache thrashing
    let n_threads = std::thread::available_parallelism()
        .map(|n| std::cmp::max(1, n.get() / 2))
        .unwrap_or(4);
    rayon::ThreadPoolBuilder::new()
        .num_threads(n_threads)
        .build_global()
        .unwrap();

    let ans: i64 = (1..=1000i32).into_par_iter().map(|i| {
        let x = compute(i as usize);
        x * x * x
    }).sum();
    println!("{}", ans);
}
