// Problem 904 - Pythagorean Angle
//
// For each angle alpha, find the integer right triangle (hypotenuse <= L)
// whose "median angle" theta is closest to alpha (degrees). Ties resolved
// by choosing the triangle with the largest area.
//
// The median angle theta satisfies tan(theta) = 3ab / (2(a^2+b^2)) where
// a,b are the legs. Using Euclid parametrization (m,n) with t = n/m, we get
// g(t) = tan(theta) = 3t(1-t^2)/(1+t^2)^2 for t in (0,1).
//
// We use continued fractions to find rational approximations n/m to the roots
// of g(t) = tan(alpha), subject to m^2+n^2 <= L (the hypotenuse constraint
// on the primitive triple).

use std::f64::consts::PI;

const DEG2RAD: f64 = PI / 180.0;
const T0: f64 = std::f64::consts::SQRT_2 - 1.0; // sqrt(2) - 1, maximizer of g

fn g_of_t(t: f64) -> f64 {
    let tt = t * t;
    let denom = 1.0 + tt;
    3.0 * t * (1.0 - tt) / (denom * denom)
}

fn root_left(y: f64) -> f64 {
    let mut lo = 0.0_f64;
    let mut hi = T0;
    for _ in 0..80 {
        let mid = (lo + hi) * 0.5;
        if g_of_t(mid) < y {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) * 0.5
}

fn root_right(y: f64) -> f64 {
    let mut lo = T0;
    let mut hi = 1.0_f64;
    for _ in 0..80 {
        let mid = (lo + hi) * 0.5;
        if g_of_t(mid) > y {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) * 0.5
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn isqrt_i128(n: i128) -> i128 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i128;
    // Correct for floating-point inaccuracy
    if x < 0 {
        x = 0;
    }
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// Continued-fraction candidates for a real x in (0,1), under the constraint
/// p^2 + q^2 <= L. Returns a vec of (p, q) pairs (Euclid parameters n, m).
fn cf_candidates_circle(x: f64, l: i64) -> Vec<(i64, i64)> {
    let mut cands: Vec<(i64, i64)> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let mut p0: i64 = 0;
    let mut q0: i64 = 1;
    let mut p1: i64 = 1;
    let mut q1: i64 = 0;
    let mut frac = x;

    for _ in 0..80 {
        let a = frac as i64;
        let p2 = a * p1 + p0;
        let q2 = a * q1 + q0;

        if p2 * p2 + q2 * q2 > l {
            // Find the largest k (<= a-1) such that:
            // (p0+k*p1)^2 + (q0+k*q1)^2 <= L
            // Use i128 to avoid overflow in discriminant computation
            let big_a = (p1 as i128) * (p1 as i128) + (q1 as i128) * (q1 as i128);
            let big_b = 2 * ((p0 as i128) * (p1 as i128) + (q0 as i128) * (q1 as i128));
            let big_c = (p0 as i128) * (p0 as i128) + (q0 as i128) * (q0 as i128) - (l as i128);

            let mut kmax: i64 = 0;
            if big_a > 0 {
                let disc = big_b * big_b - 4 * big_a * big_c;
                if disc > 0 {
                    let s = isqrt_i128(disc);
                    // positive root of A*k^2 + B*k + C = 0
                    kmax = ((-big_b + s) / (2 * big_a)) as i64;
                }
            }

            kmax = kmax.min(a - 1);

            let lo = 1_i64.max(kmax - 3);
            let hi = (a - 1).min(kmax + 3);
            for k in lo..=hi {
                let ps = k * p1 + p0;
                let qs = k * q1 + q0;
                if ps > 0 && qs > 0 && ps * ps + qs * qs <= l {
                    if seen.insert((ps, qs)) {
                        cands.push((ps, qs));
                    }
                }
            }
            break;
        }

        if p2 > 0 && q2 > 0 {
            if seen.insert((p2, q2)) {
                cands.push((p2, q2));
            }
        }

        let a_f64 = a as f64;
        if (frac - a_f64).abs() < 1e-18 {
            break;
        }
        frac = 1.0 / (frac - a_f64);
        p0 = p1;
        q0 = q1;
        p1 = p2;
        q1 = q2;
    }

    // Also include the last convergent if admissible
    if p1 > 0 && q1 > 0 && p1 * p1 + q1 * q1 <= l {
        if seen.insert((p1, q1)) {
            cands.push((p1, q1));
        }
    }

    cands
}

fn triangle_from_mn(m: i64, n: i64) -> (i64, i64, i64) {
    let a = m * m - n * n;
    let b = 2 * m * n;
    let c = m * m + n * n;
    let g = gcd(a, gcd(b, c));
    (a / g, b / g, c / g)
}

fn tan_theta_from_legs(a: i64, b: i64) -> f64 {
    let aa = a as f64 * a as f64;
    let bb = b as f64 * b as f64;
    (3.0 * a as f64 * b as f64) / (2.0 * (aa + bb))
}

fn f_single(alpha_deg: f64, l: i64) -> i64 {
    let alpha_rad = alpha_deg * DEG2RAD;
    let y = alpha_rad.tan();

    let r1 = root_left(y);
    let r2 = root_right(y);

    let mut best_diff = f64::INFINITY;
    let mut best_area_key: i128 = -1; // use i128 to avoid overflow: k^2*a*b can be huge
    let mut best_perim: i64 = 0;

    for r in &[r1, r2] {
        let cands = cf_candidates_circle(*r, l);
        for &(n, m) in &cands {
            if !(0 < n && n < m) {
                continue;
            }

            let (a, b, c) = triangle_from_mn(m, n);
            if a <= 0 || b <= 0 {
                continue;
            }
            if c > l {
                continue;
            }

            // Scale to the largest allowed triangle (theta is scale-invariant)
            let k = l / c;
            if k <= 0 {
                continue;
            }

            let theta = tan_theta_from_legs(a, b).atan();
            let diff = (theta - alpha_rad).abs();

            // Use i128 for area_key since k can be up to 10^10 and a*b can be large
            let area_key = (k as i128) * (k as i128) * (a as i128) * (b as i128);
            let perim = k * (a + b + c);

            if diff + 1e-16 < best_diff {
                best_diff = diff;
                best_area_key = area_key;
                best_perim = perim;
            } else if (diff - best_diff).abs() <= 1e-16 {
                if area_key > best_area_key {
                    best_area_key = area_key;
                    best_perim = perim;
                }
            }
        }
    }

    best_perim
}

fn f_big(n: i64, l: i64) -> i64 {
    let one_third = 1.0 / 3.0;
    let mut total: i64 = 0;
    for i in 1..=n {
        let alpha = (i as f64).powf(one_third);
        total += f_single(alpha, l);
    }
    total
}

fn main() {
    // Self-test with the examples from the problem statement
    debug_assert_eq!(f_single(30.0, 100), 198);
    debug_assert_eq!(f_single(10.0, 1_000_000), 1_600_158);
    debug_assert_eq!(f_big(10, 1_000_000), 16_684_370);

    println!("{}", f_big(45_000, 10_000_000_000));
}
