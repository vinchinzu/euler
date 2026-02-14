// Project Euler 591 - Best Approximations to sqrt(d) near pi
//
// For each non-square d < 100, find the closest a + b*sqrt(d) to pi
// with |a| <= N = 10^13, and sum |I_d| where I_d = a.
//
// Uses integer-scaled lattice reduction. We track exact values as
// a + b*sqrt(d) using BigInt for precision.

use num::bigint::BigInt;
use num::integer::Integer;
use num::traits::{Signed, Zero, One};

fn is_square(n: i32) -> bool {
    let r = (n as f64).sqrt() as i32;
    r * r == n || (r + 1) * (r + 1) == n
}

/// Compute floor(sqrt(n)) for BigInt
fn isqrt_big(n: &BigInt) -> BigInt {
    if n <= &BigInt::zero() { return BigInt::zero(); }
    let mut x: BigInt = BigInt::one();
    // Initial guess via doubling
    while &x * &x <= *n {
        x <<= 1;
    }
    // Newton refinement
    loop {
        let x1 = (&x + n / &x) / 2;
        if x1 >= x { break; }
        x = x1;
    }
    while &x * &x > *n { x -= 1; }
    x
}

fn main() {
    let n_limit: i64 = 10_000_000_000_000; // 10^13
    let k = 100;

    // Scale: S = 10^18 fits comfortably in i128 for intermediate calculations
    // and gives 18 digits of precision (enough for this problem since |a| <= 10^13
    // and we need ~14 digits of precision)
    //
    // Actually, we need more: the lattice reduction needs precision proportional
    // to the range of coefficients. With |a|, |b| up to ~10^13, and needing
    // to distinguish errors of ~10^{-14}, we need ~27 digits of precision.
    //
    // Use BigInt throughout.

    // S = 10^36 (using BigInt)
    let s = BigInt::from(10i64).pow(36);

    // pi * S (36 digits of pi after decimal point)
    // pi = 3.141592653589793238462643383279502884...
    let pi_s: BigInt = BigInt::from(3i64) * &s
        + "141592653589793238462643383279502884".parse::<BigInt>().unwrap();

    let mut ans: i64 = 0;

    for d in 2..k {
        if is_square(d) { continue; }

        // Compute sqrt(d) * S
        let d_big = BigInt::from(d);
        let sqrt_d_s = isqrt_big(&(&d_big * &s * &s));

        // Lattice reduction: reduce the basis
        // u = (sqrt_d_s, ai=0, bi=1) representing 0 + 1*sqrt(d)
        // v = (s, ai=1, bi=0) representing 1 + 0*sqrt(d)
        let mut u_x = sqrt_d_s.clone();
        let (mut u_ai, mut u_bi): (i64, i64) = (0, 1);
        let mut v_x = s.clone();
        let (mut v_ai, mut v_bi): (i64, i64) = (1, 0);

        let mut best_i: i64 = 0;
        let mut min_error_s: BigInt = BigInt::from(i64::MAX) * BigInt::from(i64::MAX);

        let d_search = 5i64;

        for _ in 0..200 {
            if u_ai.abs() > n_limit * 1000 { break; }
            if v_x.is_zero() { break; }

            // q = floor(u_x / v_x)
            let q_big = u_x.div_floor(&v_x);

            // r = u - q * v
            let r_x = &u_x - &q_big * &v_x;

            // Convert q to i64 (should be small enough after first few steps)
            // But in general q can be large. The ai/bi coordinates will be i64.
            let q_i64 = if let Some(q_val) = q_big.to_i64() {
                q_val
            } else {
                // q is too large for i64 - skip
                break;
            };

            let r_ai = u_ai - q_i64 * v_ai;
            let r_bi = u_bi - q_i64 * v_bi;

            u_x = v_x; u_ai = v_ai; u_bi = v_bi;
            v_x = r_x; v_ai = r_ai; v_bi = r_bi;

            // Find closest lattice point to pi
            // We want ui * u_x + vi * v_x ~ pi_s
            // Using Cramer's rule in the (ai, bi) basis:
            // det_ab = u_ai * v_bi - u_bi * v_ai (this is +-1 by properties of Euclidean algo)
            // But we need to solve in the (u, v) coordinate system.
            //
            // We want the point = ui * (u_ai, u_bi) + vi * (v_ai, v_bi)
            // such that ui * u_x + vi * v_x ~ pi_s
            //
            // Best: since v_x is small, ui ~ pi_s / u_x, then vi adjusts.
            // ui = round(pi_s / u_x)

            // Use floating point for initial estimate
            let u_xf = bi_to_f64(&u_x);
            let v_xf = bi_to_f64(&v_x);
            if v_xf.abs() < 0.5 { break; }

            let det_f = u_xf * v_ai as f64 - v_xf * u_ai as f64;
            if det_f.abs() < 1e-5 { break; }

            let pi_sf = bi_to_f64(&pi_s);
            let ui0 = (pi_sf * v_ai as f64 / det_f).round() as i64;
            let vi0 = (-(pi_sf * u_ai as f64 / det_f)).round() as i64;

            for ui in ui0 - d_search..=ui0 + d_search {
                for vi in vi0 - d_search..=vi0 + d_search {
                    let a = ui * u_ai + vi * v_ai;
                    let b = ui * u_bi + vi * v_bi;

                    if a.abs() > n_limit { continue; }

                    // error = |a*S + b*sqrt_d_s - pi_s|
                    let val = BigInt::from(a) * &s + BigInt::from(b) * &sqrt_d_s;
                    let error = (&val - &pi_s).abs();

                    if error < min_error_s {
                        best_i = a;
                        min_error_s = error;
                    }
                }
            }
        }

        ans += best_i.abs();
    }

    println!("{}", ans);
}

fn bi_to_f64(b: &BigInt) -> f64 {
    // Convert BigInt to f64 (approximate)
    let (sign, digits) = b.to_radix_le(10);
    if digits.is_empty() { return 0.0; }
    let mut result = 0.0f64;
    let mut base = 1.0f64;
    // Only use the most significant digits
    let start = if digits.len() > 18 { digits.len() - 18 } else { 0 };
    for &d in &digits[start..] {
        result += d as f64 * base;
        base *= 10.0;
    }
    if start > 0 {
        result *= 10.0f64.powi(start as i32);
    }
    match sign {
        num::bigint::Sign::Minus => -result,
        _ => result,
    }
}

use num::ToPrimitive;
