// Project Euler 660 - Pandigital Triangles
// 120-degree triangles with pandigital sides in bases 9-18.
// Optimized: const-generic fast digit extraction (zero division instructions via compiler magic)
// and fine-grained rayon parallelism over (base, n) subtasks.

use rayon::prelude::*;

#[inline(always)]
fn gcd32(mut a: u32, mut b: u32) -> u32 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 { break; }
    }
    a << shift
}

#[inline(always)]
fn add_digits<const BASE: u32>(mut x: u32, mask: &mut u32) -> bool {
    if x == 0 {
        if *mask & 1 != 0 { return false; }
        *mask |= 1;
        return true;
    }
    while x > 0 {
        let d = x % BASE;
        let bit = 1u32 << d;
        if *mask & bit != 0 { return false; }
        *mask |= bit;
        x /= BASE;
    }
    true
}

#[inline(always)]
fn is_pandigital_const<const BASE: u32>(a: u32, b_val: u32, c: u32) -> bool {
    let mut mask: u32 = 0;
    add_digits::<BASE>(a, &mut mask)
        && add_digits::<BASE>(b_val, &mut mask)
        && add_digits::<BASE>(c, &mut mask)
        && mask == (1u32 << BASE) - 1
}

fn ceil_div(a: i32, b: i32) -> i32 { (a + b - 1) / b }

/// Inverse of a mod m (m > 1, gcd(a,m)=1).
#[inline(always)]
fn mod_inv(a: u32, m: u32) -> u32 {
    let mut t = 0i32;
    let mut newt = 1i32;
    let mut r = m as i32;
    let mut newr = a as i32;
    while newr != 0 {
        let q = r / newr;
        let (nt, nr) = (t - q * newt, r - q * newr);
        t = newt;
        newt = nt;
        r = newr;
        newr = nr;
    }
    if t < 0 {
        t += m as i32;
    }
    t as u32
}

fn solve_base<const BASE: u32>() -> Vec<(u32, u32, u32)> {
    let e1 = ceil_div(BASE as i32, 3);
    let mut limit = 1u64;
    for _ in 0..e1 {
        limit *= BASE as u64;
    }
    let mut limit2 = 1u64;
    for _ in 0..e1 - 1 {
        limit2 *= BASE as u64;
    }
    limit += limit2;

    let max_n = limit.isqrt();
    let b1 = BASE - 1;

    (1..max_n + 1)
        .into_par_iter()
        .flat_map_iter(move |n| {
            let mut results = Vec::new();
            let n2 = n * n;
            for m in n + 1..2 * n {
                let ls1 = m * m - m * n + n2;
                if ls1 > limit {
                    break;
                }
                if (m + n) % 3 == 0 {
                    continue;
                }
                if gcd32(m as u32, n as u32) != 1 {
                    continue;
                }

                let diff = m * m - n2;
                let b0 = m * (2 * n - m);
                // a+b+c = k*m*(m+n) ≡ sum of digits ≡ BASE*(BASE-1)/2 (mod BASE-1).
                let expected = if BASE % 2 == 1 { b1 / 2 } else { 0 };
                let prod_mod = ((m as u32) % b1) * (((m + n) as u32) % b1) % b1;
                let g = gcd32(prod_mod, b1);
                if expected % g != 0 {
                    continue;
                }
                let b1g = b1 / g;
                let k_step = b1g as u64;
                let k0 = if b1g == 1 {
                    0
                } else {
                    let inv = mod_inv((prod_mod / g) % b1g, b1g);
                    (expected / g * inv) % b1g
                };
                let mut k = if k0 == 0 { k_step } else { k0 as u64 };
                while k * ls1 <= limit {
                    let c = k * ls1;
                    let a = k * diff;
                    let b_val = k * b0;
                    if is_pandigital_const::<BASE>(a as u32, b_val as u32, c as u32) {
                        results.push((a as u32, b_val as u32, c as u32));
                    }
                    k += k_step;
                }
            }
            results
        })
        .collect()
}

fn main() {
    let parts: Vec<Vec<(u32, u32, u32)>> = [
        solve_base::<9>,
        solve_base::<10>,
        solve_base::<11>,
        solve_base::<12>,
        solve_base::<13>,
        solve_base::<14>,
        solve_base::<15>,
        solve_base::<16>,
        solve_base::<17>,
        solve_base::<18>,
    ]
    .into_par_iter()
    .map(|f| f())
    .collect();

    let mut all_results: Vec<(u32, u32, u32)> = parts.into_iter().flatten().collect();
    all_results.sort_unstable();
    all_results.dedup();
    let total: u64 = all_results.iter().map(|&(_, _, c)| c as u64).sum();
    println!("{}", total);
}
