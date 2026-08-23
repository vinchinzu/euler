// Project Euler 757 - Stealthy Numbers
// A stealthy number is n = x(x+1)*g(g+1) for some x,g >= 1. Count distinct <= N.

use rayon::prelude::*;

fn main() {
    const N: i64 = 100_000_000_000_000; // 10^14

    let mut xmax = 1i64;
    while xmax * (xmax + 1) * xmax * (xmax + 1) <= N {
        xmax += 1;
    }
    xmax -= 1;

    let mut stealthies: Vec<i64> = (1..=xmax)
        .into_par_iter()
        .flat_map_iter(|x| {
            let xa = x * (x + 1);
            let gmax = {
                let mut lo = x;
                let mut hi = (N / xa).isqrt() + 2;
                while lo < hi {
                    let mid = (lo + hi + 1) / 2;
                    if xa.saturating_mul(mid).saturating_mul(mid + 1) <= N {
                        lo = mid;
                    } else {
                        hi = mid - 1;
                    }
                }
                lo
            };
            (x..=gmax).map(move |g| xa * g * (g + 1))
        })
        .collect();

    stealthies.par_sort_unstable();
    stealthies.dedup();
    println!("{}", stealthies.len());
}
