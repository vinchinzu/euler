// Project Euler 562 - Maximal Triangle Perimeter
//
// Find triangle with lattice point vertices within circle of radius N=10^7,
// no other lattice points on boundary, maximum perimeter.

use euler_utils::gcd_i64;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

fn ext_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let (x1, y1) = ext_gcd(b, a % b);
    (y1, x1 - (a / b) * y1)
}

fn max_possible_perim(a: f64) -> f64 {
    a + 2.0 * (a / 2.0).hypot(1.0 / a)
}

fn main() {
    let n: i64 = 10_000_000;
    let l: i64 = 20;
    let n2 = n * n;
    let isqrt_half = (((n as f64) * (n as f64) / 2.0).sqrt()) as i64;
    let n_f = n as f64;

    // Positive f64 bit patterns are monotonic, so fetch_max is a valid relaxed prune.
    let global_max = AtomicU64::new(0);
    const CHUNK: i64 = 4096;
    let n_chunks = (isqrt_half / CHUNK) + 1;

    let (_max_perim, best_abc) = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let x1_lo = ci * CHUNK;
            let x1_hi = (x1_lo + CHUNK - 1).min(isqrt_half);
            let mut prune_max = f64::from_bits(global_max.load(Ordering::Relaxed));
            let mut max_perim = 0.0f64;
            let mut best_abc = 0.0f64;

            for x1 in x1_lo..=x1_hi {
                if x1 & 63 == 0 {
                    let g = f64::from_bits(global_max.load(Ordering::Relaxed));
                    if g > prune_max {
                        prune_max = g;
                    }
                }
                let mut y1 = ((n2 - x1 * x1) as f64).sqrt() as i64;
                while x1 * x1 + y1 * y1 > n2 {
                    y1 -= 1;
                }

                let h1 = (x1 as f64).hypot(y1 as f64);
                if max_possible_perim(n_f + h1) < prune_max {
                    continue;
                }

                let x2_lo = -x1 - l;
                let mut x2_hi = -x1 + l;
                if x2_hi >= x1 {
                    x2_hi = x1 - 1;
                }

                for x2 in x2_lo..=x2_hi {
                    let y2_lo = -y1 - l;
                    let mut y2_hi = -y1 + l;
                    if y2_hi >= y1 {
                        y2_hi = y1 - 1;
                    }

                    for y2 in y2_lo..=y2_hi {
                        if x2 * x2 + y2 * y2 > n2 {
                            continue;
                        }

                        let a = ((x1 - x2) as f64).hypot((y1 - y2) as f64);
                        if max_possible_perim(a) < prune_max {
                            continue;
                        }
                        if gcd_i64((x1 - x2).abs(), (y1 - y2).abs()) != 1 {
                            continue;
                        }

                        let (ex, ey) = ext_gcd(y1 - y2, x2 - x1);
                        let x3 = x2 + ex;
                        let y3 = y2 + ey;

                        if x3 * x3 + y3 * y3 <= n2 {
                            let b = ((x1 - x3) as f64).hypot((y1 - y3) as f64);
                            let c = ((x2 - x3) as f64).hypot((y2 - y3) as f64);
                            let perim = a + b + c;
                            if perim > max_perim {
                                max_perim = perim;
                                best_abc = a * b * c;
                            }
                            if perim > prune_max {
                                prune_max = perim;
                                global_max.fetch_max(perim.to_bits(), Ordering::Relaxed);
                            }
                        }
                    }
                }
            }
            (max_perim, best_abc)
        })
        .reduce(
            || (0.0f64, 0.0f64),
            |a, b| {
                if a.0 > b.0 {
                    a
                } else if b.0 > a.0 {
                    b
                } else if a.1 >= b.1 {
                    a
                } else {
                    b
                }
            },
        );

    let ans = (best_abc / 2.0 / n_f).round() as i64;
    println!("{}", ans);
}
