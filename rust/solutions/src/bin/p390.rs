// Project Euler 390: Triangles with Non Rational Sides and Integral Area
//
// For each even a, iterate t=2,4,6,... checking if a^2*t^2 - a^2 + t^2 is a
// perfect square. The work per `a` varies enormously (a=2 dominates), so we
// flatten all (a, t_range) pairs into uniform work units for rayon.

use rayon::prelude::*;

fn isqrt128(n: u128) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as u64;
    for _ in 0..5 {
        if x == 0 {
            break;
        }
        let x128 = x as u128;
        let next = (x128 + n / x128) / 2;
        if next >= x128 && next - x128 <= 1 {
            break;
        }
        if x128 >= next && x128 - next <= 1 {
            break;
        }
        x = next as u64;
    }
    while (x as u128) * (x as u128) > n {
        x -= 1;
    }
    while ((x + 1) as u128) * ((x + 1) as u128) <= n {
        x += 1;
    }
    x
}

fn main() {
    let big_n: u64 = 20_000_000_000;
    let t_chunk: u64 = 500_000; // each work unit processes this many t-values

    // Build work units: (a, t_start, t_end) where t iterates t_start, t_start+2, ..., t_end
    let mut work_units: Vec<(u64, u64, u64)> = Vec::new();
    let mut a: u64 = 2;
    while (a as u128) * (a as u128) + 1 <= big_n as u128 {
        let a2 = a as u128 * a as u128;
        let upper_bound = (big_n as u128 / (a2 + 1)) as u64;
        if upper_bound >= 2 {
            // Split [2, upper_bound] (step 2) into chunks of t_chunk even values
            let mut t_start: u64 = 2;
            while t_start <= upper_bound {
                let t_end = std::cmp::min(t_start + 2 * t_chunk - 2, upper_bound);
                work_units.push((a, t_start, t_end));
                t_start = t_end + 2;
            }
        }
        a += 2;
    }

    let ans: u128 = work_units
        .par_iter()
        .map(|&(a, t_start, t_end)| {
            let a2 = a as u128 * a as u128;
            let mut local_sum: u128 = 0;
            let mut t = t_start;
            while t <= t_end {
                let t128 = t as u128;
                let s = a2 * t128 * t128 - a2 + t128 * t128;
                let v = isqrt128(s);
                if v as u128 * v as u128 == s {
                    let b = a as u128 * t128 + v as u128;
                    let n_val = a as u128 * b + t128;
                    if n_val <= big_n as u128 {
                        local_sum += n_val / 2;
                    }
                }
                t += 2;
            }
            local_sum
        })
        .sum();

    println!("{ans}");
}
