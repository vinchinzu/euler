// Problem 958 - Euclid's Labour
//
// Subtraction-only Euclidean algorithm distance. Parallelize top-level
// search branches with rayon; iterative deepening from fib-derived depth.

use std::sync::atomic::{AtomicI64, Ordering};

/// (n * p) rem_euclid y with y > 0, result in 0..y.
#[inline(always)]
fn residue_a(n: i64, p: i64, y: i64) -> i64 {
    let nm = n % y;
    let mut pm = p % y;
    if pm < 0 {
        pm += y;
    }
    // y <= 2^31-1 => nm*pm fits in i64
    if y <= 2_147_483_647 {
        nm * pm % y
    } else {
        ((nm as u128) * (pm as u128) % (y as u128)) as i64
    }
}

fn f(n: i64) -> i64 {
    assert!(n >= 2);

    let mut fib = [0i64; 202];
    fib[1] = 1;
    for i in 2..202 {
        fib[i] = fib[i - 1].saturating_add(fib[i - 2]);
    }

    let mut depth: usize = 0;
    while fib[depth + 3] < n {
        depth += 1;
    }

    fn prune(
        x: i64,
        y: i64,
        p: i64,
        rem: usize,
        n: i64,
        fib: &[i64; 202],
        best_m: &AtomicI64,
    ) -> bool {
        // true => skip this node
        if y >= best_m.load(Ordering::Relaxed) {
            return true;
        }
        if rem == 0 || x > n {
            return true;
        }
        if x.saturating_add((rem as i64).saturating_mul(y)) > n {
            return true;
        }
        let upper = fib[rem + 1]
            .saturating_mul(x)
            .saturating_add(fib[rem].saturating_mul(y));
        if upper < n {
            return true;
        }
        let a = residue_a(n, p, y);
        a.saturating_mul(x) > n
    }

    fn search_seq(
        x: i64,
        y: i64,
        p: i64,
        q: i64,
        rem: usize,
        n: i64,
        fib: &[i64; 202],
        best_m: &AtomicI64,
    ) {
        if y >= best_m.load(Ordering::Relaxed) {
            return;
        }
        if x == n {
            best_m.fetch_min(y, Ordering::Relaxed);
            return;
        }
        if prune(x, y, p, rem, n, fib, best_m) {
            return;
        }
        let xp = x + y;
        let rem1 = rem - 1;
        search_seq(xp, y, p, q - p, rem1, n, fib, best_m);
        search_seq(xp, x, q, p - q, rem1, n, fib, best_m);
    }

    fn search_par(
        x: i64,
        y: i64,
        p: i64,
        q: i64,
        rem: usize,
        par: usize,
        n: i64,
        fib: &[i64; 202],
        best_m: &AtomicI64,
    ) {
        if y >= best_m.load(Ordering::Relaxed) {
            return;
        }
        if x == n {
            best_m.fetch_min(y, Ordering::Relaxed);
            return;
        }
        if prune(x, y, p, rem, n, fib, best_m) {
            return;
        }
        let xp = x + y;
        let rem1 = rem - 1;
        if par == 0 {
            search_seq(xp, y, p, q - p, rem1, n, fib, best_m);
            search_seq(xp, x, q, p - q, rem1, n, fib, best_m);
        } else {
            rayon::join(
                || search_par(xp, y, p, q - p, rem1, par - 1, n, fib, best_m),
                || search_par(xp, x, q, p - q, rem1, par - 1, n, fib, best_m),
            );
        }
    }

    loop {
        let best_m = AtomicI64::new(i64::MAX);
        let par_levels = 10usize.min(depth);
        search_par(2, 1, 0, 1, depth, par_levels, n, &fib, &best_m);
        let found = best_m.load(Ordering::Relaxed);
        if found != i64::MAX {
            return found;
        }
        depth += 1;
    }
}

fn main() {
    assert_eq!(f(7), 2);
    assert_eq!(f(89), 34);
    assert_eq!(f(8191), 1856);

    let n: i64 = 1_000_000_000_000 + 39;
    println!("{}", f(n));
}
