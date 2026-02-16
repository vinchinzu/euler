// Project Euler 391 - Hopping Game
//
// Compute sum of M(n)^3 for 1 <= n <= 1000.
// M(n) is found via iterated function composition DP.
//
// For each n, initialize f[s][x] = s+x if s+x < n+1, else 0.
// Iterate: g[s][x] = f[s][f[s+1][x]] for n+1 steps.
// M(n) = f[0][0] after iteration.

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

        // Initialize f[s][x] = s+x if s+x < n, else 0
        let fa = buf_a.as_mut_ptr();
        let fb = buf_b.as_mut_ptr();
        unsafe {
            for s in 0..rows {
                let base = s * stride;
                for x in 0..=n {
                    let v = s + x;
                    *fa.add(base + x) = if v < n { v as u16 } else { 0 };
                }
            }
        }

        let xlim = n + 1;
        let mut fi = 0usize;
        for h in 1..=n {
            let (src, dst) = if fi == 0 { (fa as *const u16, fb) } else { (fb as *const u16, fa) };
            let slim = n - h;
            unsafe {
                for s in 0..=slim {
                    let s1_base = src.add((s + 1) * stride);
                    let s_base_r = src.add(s * stride);
                    let s_base_w = dst.add(s * stride);
                    let mut x = 0usize;
                    let xlim4 = xlim & !3;
                    while x < xlim4 {
                        let y0 = *s1_base.add(x) as usize;
                        let y1 = *s1_base.add(x + 1) as usize;
                        let y2 = *s1_base.add(x + 2) as usize;
                        let y3 = *s1_base.add(x + 3) as usize;
                        *s_base_w.add(x) = *s_base_r.add(y0);
                        *s_base_w.add(x + 1) = *s_base_r.add(y1);
                        *s_base_w.add(x + 2) = *s_base_r.add(y2);
                        *s_base_w.add(x + 3) = *s_base_r.add(y3);
                        x += 4;
                    }
                    while x < xlim {
                        let y = *s1_base.add(x) as usize;
                        *s_base_w.add(x) = *s_base_r.add(y);
                        x += 1;
                    }
                }
            }
            fi = 1 - fi;
        }

        if fi == 0 {
            unsafe { *fa as i64 }
        } else {
            unsafe { *fb as i64 }
        }
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

    // Process large n first for better load balancing
    let indices: Vec<i32> = (1..=1000i32).rev().collect();
    let ans: i64 = indices.into_par_iter().map(|i| {
        let x = compute(i as usize);
        x * x * x
    }).sum();
    println!("{}", ans);
}
