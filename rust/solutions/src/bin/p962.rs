// Project Euler 962 - Integer triangles with a specific property
// Count integer triangles (a,b,c) with a<=b<=c, a+b+c<=N, where the area squared
// has a special cube-related structure.
// Uses factorization approach: for each z up to N/3, factor z, generate candidate u values
// from z's factors, then enumerate v values and divisor pairs to find valid triangles.

use euler_utils::primes_up_to;
use rayon::prelude::*;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn factor(mut n: u64, primes: &[usize]) -> Vec<(u64, u32)> {
    let mut res = Vec::new();
    for &p in primes {
        let p = p as u64;
        if p * p > n {
            break;
        }
        if n % p == 0 {
            let mut e = 0u32;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            res.push((p, e));
        }
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

fn divisors_from_factors(factors: &[(u64, u32)]) -> Vec<u64> {
    let mut divs = vec![1u64];
    for &(p, e) in factors {
        let len = divs.len();
        let mut pe = 1u64;
        for _ in 0..e {
            pe *= p;
            for j in 0..len {
                divs.push(divs[j] * pe);
            }
        }
    }
    divs
}

fn gen_us_from_z_factor(z_factors: &[(u64, u32)]) -> Vec<u64> {
    if z_factors.is_empty() {
        return vec![1];
    }
    let bases: Vec<u64> = z_factors.iter().map(|&(p, _)| p).collect();
    let limits: Vec<u32> = z_factors.iter().map(|&(_, e)| (2 * e) / 3).collect();
    let mut us = Vec::new();

    fn backtrack(i: usize, cur: u64, bases: &[u64], limits: &[u32], us: &mut Vec<u64>) {
        if i == bases.len() {
            us.push(cur);
            return;
        }
        let mut val = 1u64;
        for _ in 0..=limits[i] {
            backtrack(i + 1, cur * val, bases, limits, us);
            val *= bases[i];
        }
    }

    backtrack(0, 1, &bases, &limits, &mut us);
    us
}

fn integer_cuberoot_floor(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = (n as f64).cbrt() as u64;
    // Ensure correctness
    while (x + 1) * (x + 1) * (x + 1) <= n {
        x += 1;
    }
    while x > 0 && x * x * x > n {
        x -= 1;
    }
    x
}

fn count_for_z(z: u64, n: u64, primes: &[usize]) -> u64 {
    let mut total = 0u64;
    let z_factors = factor(z, primes);
    let z2 = z * z;
    let u_candidates = gen_us_from_z_factor(&z_factors);
    for u in u_candidates {
        let u3 = u * u * u;
        if z2 % u3 != 0 {
            continue;
        }
        let w = z2 / u3;
        if w == 0 {
            continue;
        }
        // v_max^3 <= N^2 / w
        let v_max_cubed = n * n / w;
        if v_max_cubed == 0 {
            continue;
        }
        let v_max = integer_cuberoot_floor(v_max_cubed);
        if v_max < u {
            continue;
        }
        for v in u..=v_max {
            if gcd(u, v) != 1 {
                continue;
            }
            let t = v * w;
            let t_factors = factor(t, primes);
            let divisors = divisors_from_factors(&t_factors);
            let uv_sum = u + v;
            for &p_div in &divisors {
                let q = t / p_div;
                if p_div <= q {
                    continue;
                }
                if (p_div ^ q) & 1 != 0 {
                    continue;
                }
                let g = (p_div + q) / 2;
                let m = (p_div - q) / 2;
                if m == 0 || g <= m {
                    continue;
                }
                let a = g * u;
                let b = g * v;
                let c = m * uv_sum;
                let perimeter = a + b + c;
                if perimeter > n {
                    continue;
                }
                if !(a <= b && b <= c) {
                    continue;
                }
                if a + b <= c {
                    continue;
                }
                total += 1;
            }
        }
    }
    total
}

fn count_triangles(n: u64, primes: &[usize]) -> u64 {
    let max_z = n / 3;
    (1..=max_z)
        .into_par_iter()
        .map(|z| count_for_z(z, n, primes))
        .sum()
}

fn main() {
    let primes = primes_up_to(1_000_000);
    println!("{}", count_triangles(1_000_000, &primes));
}
