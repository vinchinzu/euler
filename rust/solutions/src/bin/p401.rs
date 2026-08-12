// Project Euler 401: Sum of squares of divisors
use rayon::prelude::*;

fn sum_sq_mod(n: i64, m: i64) -> i64 {
    let m6 = 6 * m;
    let val = (n % m6) as i128 * ((n + 1) % m6) as i128 % m6 as i128;
    let val = val * ((2 * n + 1) % m6) as i128 % m6 as i128;
    (val / 6) as i64
}

fn main() {
    let n: i64 = 1_000_000_000_000_000;
    let m: i64 = 1_000_000_000;
    let mut l = (n as f64).sqrt() as i64;
    while (l + 1) * (l + 1) <= n {
        l += 1;
    }
    while l * l > n {
        l -= 1;
    }

    // Part 1: sum_{d=1..l} (n/d) * d^2  (mod m)
    let part1: i64 = (1..=l)
        .into_par_iter()
        .map(|d| {
            let q = n / d;
            let dmod = d % m;
            let d2mod = dmod * dmod % m;
            let qmod = q % m;
            qmod * d2mod % m
        })
        .sum::<i64>()
        % m;

    // Part 2: group by quotient t = n/d for d > l
    let tmax = n / (l + 1);
    let part2: i64 = (1..=tmax)
        .into_par_iter()
        .map(|t| {
            let d_hi = n / t;
            let mut d_lo = n / (t + 1) + 1;
            if d_lo <= l {
                d_lo = l + 1;
            }
            if d_lo > d_hi {
                return 0;
            }
            let s = (sum_sq_mod(d_hi, m) - sum_sq_mod(d_lo - 1, m) + m) % m;
            (t % m) * s % m
        })
        .sum::<i64>()
        % m;

    let mut ans = (part1 + part2) % m;
    if ans < 0 {
        ans += m;
    }
    println!("{}", ans);
}
