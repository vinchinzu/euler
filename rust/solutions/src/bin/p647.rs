// Project Euler 647 - Linear Transformations of Polygonal Numbers
// For positive odd k, find all (A,B) with A,B positive integers such that
// A * X_n + B is always a k-gonal number. Sum A+B over all such pairs.

use rayon::prelude::*;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn main() {
    let n: i64 = 1_000_000_000_000; // 10^12
    let l = isqrt(n);
    let k_max = ((l - 1) / 2) as usize;

    let ans: i64 = (0..k_max + 1).into_par_iter().map(|k| {
        let sqrt_a = 2 * k as i64 + 1;
        let a = sqrt_a * sqrt_a;
        let d_max = (sqrt_a - 1) / 2;
        if d_max == 0 {
            return 0;
        }

        let mut local = 0i64;
        let mut d = 1i64;
        while d * d <= d_max {
            if d_max % d == 0 {
                if d % 2 == 1 {
                    let b = ((a - 1) / (8 * d)) * ((d - 2) * (d - 2));
                    if b >= 1 && b <= n {
                        local += a + b;
                    }
                }
                let d2 = d_max / d;
                if d2 != d && d2 % 2 == 1 {
                    let b = ((a - 1) / (8 * d2)) * ((d2 - 2) * (d2 - 2));
                    if b >= 1 && b <= n {
                        local += a + b;
                    }
                }
            }
            d += 1;
        }
        local
    }).sum();

    println!("{}", ans);
}
