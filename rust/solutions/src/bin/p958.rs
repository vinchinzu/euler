// Problem 958 - Euclid's Labour
//
// Subtraction-only Euclidean algorithm distance. Parallelize top-level
// search branches with rayon; iterative deepening from fib-derived depth.

use rayon::prelude::*;

fn f(n: i64) -> i64 {
    assert!(n >= 2);

    let mut fib = [0i64; 202];
    fib[1] = 1;
    for i in 2..202 {
        fib[i] = fib[i - 1].saturating_add(fib[i - 2]);
    }

    // Smallest depth such that fib[depth+3] >= n
    let mut depth: usize = 0;
    while fib[depth + 3] < n {
        depth += 1;
    }

    let n128 = n as i128;

    // Search from a single root (2,1) at given rem, exploring children in parallel
    // at the top few levels for load balance.
    fn search_seq(
        x: i64,
        y: i64,
        p: i64,
        q: i64,
        rem: usize,
        n: i64,
        n128: i128,
        fib: &[i64; 202],
        best_m: &mut i64,
    ) {
        if y >= *best_m {
            return;
        }
        if x == n {
            *best_m = y;
            return;
        }
        if rem == 0 || x > n {
            return;
        }
        if x + (rem as i64) * y > n {
            return;
        }
        let upper = (fib[rem + 1] as i128) * (x as i128) + (fib[rem] as i128) * (y as i128);
        if upper < n128 {
            return;
        }
        let a = ((n128 * p as i128) % y as i128 + y as i128) % y as i128;
        if a * x as i128 > n128 {
            return;
        }

        let xp = x + y;
        let rem1 = rem - 1;
        // Prefer (x+y, y) first (slow growth / smaller y branch tends to find good m sooner)
        search_seq(xp, y, p, q - p, rem1, n, n128, fib, best_m);
        search_seq(xp, x, q, p - q, rem1, n, n128, fib, best_m);
    }

    loop {
        // Expand first few levels into a work queue, then parallelize.
        // Depth of expansion: enough tasks for rayon without too much overhead.
        let expand_levels = 12usize.min(depth);
        let mut frontier: Vec<(i64, i64, i64, i64, usize)> = Vec::with_capacity(1 << 14);
        frontier.push((2, 1, 0, 1, depth));

        for _ in 0..expand_levels {
            let mut next = Vec::with_capacity(frontier.len() * 2);
            for (x, y, p, q, rem) in frontier {
                if rem == 0 || x > n || x == n {
                    next.push((x, y, p, q, rem));
                    continue;
                }
                if x + (rem as i64) * y > n {
                    continue;
                }
                let upper =
                    (fib[rem + 1] as i128) * (x as i128) + (fib[rem] as i128) * (y as i128);
                if upper < n128 {
                    continue;
                }
                let a = ((n128 * p as i128) % y as i128 + y as i128) % y as i128;
                if a * x as i128 > n128 {
                    continue;
                }
                let xp = x + y;
                let rem1 = rem - 1;
                next.push((xp, y, p, q - p, rem1));
                next.push((xp, x, q, p - q, rem1));
            }
            frontier = next;
            if frontier.len() > 4096 {
                break;
            }
        }

        // Parallel search from frontier; each task returns best m found (or MAX).
        let best_m: i64 = frontier
            .par_iter()
            .map(|&(x, y, p, q, rem)| {
                let mut local_best = i64::MAX;
                if x == n {
                    return y;
                }
                search_seq(x, y, p, q, rem, n, n128, &fib, &mut local_best);
                local_best
            })
            .min()
            .unwrap_or(i64::MAX);

        if best_m != i64::MAX {
            return best_m;
        }

        depth += 1;
    }
}

fn main() {
    debug_assert_eq!(f(7), 2);
    debug_assert_eq!(f(89), 34);
    debug_assert_eq!(f(8191), 1856);

    let n: i64 = 1_000_000_000_000 + 39;
    println!("{}", f(n));
}
