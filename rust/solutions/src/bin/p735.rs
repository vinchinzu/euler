// Project Euler 735 - Divisors of 2n^2
//
// Mobius function sieve + counting lattice points.
// Parallelized with rayon on flattened (g, t) work units for load balance.

use rayon::prelude::*;

#[inline(always)]
fn isq(n: i64) -> i64 { n * n }
#[inline(always)]
fn icb(n: i64) -> i64 { n * n * n }

fn isqrt_f(n: i64) -> i64 {
    let mut sq = (n as f64).sqrt() as i64;
    while sq * sq > n { sq -= 1; }
    while (sq + 1) * (sq + 1) <= n { sq += 1; }
    sq
}

fn compute_inner(n_val: i64) -> i64 {
    let mut res: i64 = 0;

    // x*y*z <= n_val, y > x
    {
        let mut x: i64 = 1;
        while icb(x) <= n_val {
            let nox = n_val / x;
            let sq_nox = isqrt_f(nox);
            let mut y = x + 1;
            while y <= sq_nox {
                res += n_val / (x * y) - y;
                y += 1;
            }
            x += 1;
        }

        x = 1;
        while icb(x) <= n_val {
            let nox = n_val / x;
            let sq_nox = isqrt_f(nox);
            let mut z = x + 1;
            while z <= sq_nox {
                res += n_val / (x * z) - (z - 1);
                z += 1;
            }
            x += 1;
        }

        let mut z: i64 = 1;
        while icb(z) <= n_val {
            let noz = n_val / z;
            let sq_noz = isqrt_f(noz);
            let mut x = z;
            while x <= sq_noz {
                res += n_val / (x * z) - x;
                x += 1;
            }
            z += 1;
        }
    }

    // x*y*z <= n_val, y > 2x
    {
        let mut x: i64 = 1;
        while icb(x) <= n_val {
            let nox = n_val / x;
            let sq_nox = isqrt_f(nox);
            let mut y = 2 * x + 1;
            while y <= sq_nox {
                res += n_val / (x * y) - y;
                y += 1;
            }
            x += 1;
        }

        x = 1;
        while icb(x) <= n_val {
            let nox = n_val / x;
            let sq_nox = isqrt_f(nox);
            let mut z = x + 1;
            while z <= sq_nox {
                if 2 * z * isq(x) > n_val { break; }
                let maxv = (2 * x).max(z - 1);
                res += n_val / (x * z) - maxv;
                z += 1;
            }
            x += 1;
        }

        let mut z: i64 = 1;
        while icb(z) <= n_val {
            let mut x = z;
            while 2 * z * isq(x) <= n_val {
                res += n_val / (x * z) - 2 * x;
                x += 1;
            }
            z += 1;
        }
    }

    res
}

fn main() {
    let big_n: i64 = 1_000_000_000_000;
    let mut l = (big_n as f64).sqrt() as i64;
    if l * l > big_n { l -= 1; }
    while (l + 1) * (l + 1) <= big_n { l += 1; }

    // Sieve Mobius
    let lim = l as usize;
    let mut mobius = vec![1i32; lim + 1];
    let mut is_prime = vec![true; lim + 1];
    is_prime[0] = false;
    if lim >= 1 { is_prime[1] = false; }

    for i in 2..=lim {
        if is_prime[i] {
            for j in (i..=lim).step_by(i) {
                if j != i { is_prime[j] = false; }
                mobius[j] *= -1;
            }
            let sq = i as u64 * i as u64;
            if sq <= lim as u64 {
                let mut j = sq as usize;
                while j <= lim {
                    mobius[j] = 0;
                    j += sq as usize;
                }
            }
        }
    }

    // Pre-build flattened (g, t) work units for even load distribution
    let mut work_units: Vec<(usize, u32)> = Vec::new();
    for g in 1..=lim {
        if mobius[g] == 0 { continue; }
        let g_sq = isq(g as i64);
        if g_sq >= big_n { break; }
        let mut t = 0u32;
        while t < 50 && g_sq <= (big_n >> t) {
            let n_val = (big_n / g_sq) >> t;
            if n_val >= 1 {
                work_units.push((g, t));
            }
            t += 1;
        }
    }

    let parallel_sum: i64 = work_units.par_iter()
        .map(|&(g, t)| {
            let g_sq = isq(g as i64);
            let n_val = (big_n / g_sq) >> t;
            let res = compute_inner(n_val);
            let parity = if t % 2 == 0 { 1i64 } else { -1 };
            res * parity * mobius[g] as i64
        })
        .sum();

    let ans = big_n + parallel_sum;
    println!("{}", ans);
}
