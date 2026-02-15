// Project Euler 513 - Triangles with Integer Median
//
// Count triangles with integer sides a <= b <= c <= N where the median to c
// has integer length.
//
// Key identity: 2a^2 + 2b^2 - c^2 = 4m^2. Let s=(a+b)/2, d=(b-a)/2, t=c/2.
// Then s^2 + d^2 = t^2 + m^2. Let u = s - t >= 1. Then:
//   P = u(2t+u) = (m-d)(m+d) = v*w
// where v and w have same parity as u. For each (u,v) with v > u, same parity,
// and v | P for some valid t, the values of t form an arithmetic progression
// with step = v' = v/gcd(u,v) (or related via CRT when v divisible by 4 and u even).
//
// The answer is f_count(N) -- no Mobius inversion needed.

use rayon::prelude::*;

const NN: i64 = 100_000;

#[inline]
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// For a given u, count all valid triangles by iterating over v.
/// Each (u,v) pair with v > u, same parity, gives an arithmetic progression
/// of valid t values. We count the number of t in [t_min, half_n] with step.
fn count_for_u(u: i64, half_n: i64) -> i64 {
    let mut count: i64 = 0;
    let p_max = u * (2 * half_n + u);

    // v_max = floor(sqrt(P_max))
    let mut v_max = (p_max as f64).sqrt() as i64 + 1;
    while v_max * v_max > p_max {
        v_max -= 1;
    }

    let u_odd = u & 1 == 1;
    // v must have same parity as u. Start at u+1 or u+2 to match parity.
    let v_start = if u_odd {
        if (u + 1) & 1 == 1 { u + 1 } else { u + 2 }
    } else {
        if (u + 1) & 1 == 0 { u + 1 } else { u + 2 }
    };

    let mut v = v_start;
    while v <= v_max {
        // Compute gcd(u, v) and v' = v / gcd(u,v)
        let g = gcd(u, v);
        let vp = v / g;

        // Solve for arithmetic progression of t values where v' | (2t + u).
        // 2t ≡ -u (mod v')
        let (mut step, mut t0);
        if vp == 1 {
            step = 1;
            t0 = 1;
        } else if vp & 1 == 1 {
            // vp odd: inv2 = (vp+1)/2
            let inv2 = (vp + 1) / 2;
            let r = ((-u).rem_euclid(vp)) as i64;
            t0 = (r * inv2) % vp;
            if t0 == 0 { t0 = vp; }
            step = vp;
        } else {
            // vp even: need u even (guaranteed by parity)
            let half_vp = vp / 2;
            let hu = u / 2;
            t0 = ((-hu).rem_euclid(half_vp)) as i64;
            if t0 == 0 { t0 = half_vp; }
            step = half_vp;
        }

        // Extra CRT condition when u even and v divisible by 4
        if !u_odd {
            let v1 = v / 2;
            if v1 % 2 == 0 {
                // Need v1 | k(t+k) where k = u/2
                // Let g2 = gcd(k, v1). Need (v1/g2) | (t + k).
                let k = u / 2;
                let g2 = gcd(k, v1);
                let r2 = v1 / g2;
                if r2 > 1 {
                    // t ≡ -k (mod r2)
                    let t0_2 = ((-k).rem_euclid(r2)) as i64;
                    // CRT: combine t ≡ t0 (mod step) with t ≡ t0_2 (mod r2)
                    let g3 = gcd(step, r2);
                    if (t0_2 - t0).rem_euclid(g3) != 0 {
                        v += 2;
                        continue;
                    }
                    let lcm = step / g3 * r2;
                    let m1g = step / g3;
                    let m2g = r2 / g3;
                    let diff = (t0_2 - t0) / g3;
                    // Extended GCD to find inverse of m1g mod m2g
                    let (mut x, mut x1) = (0i64, 1i64);
                    let (mut tm, mut ta) = (m2g, m1g);
                    while ta > 0 {
                        let q = tm / ta;
                        let tmp = ta; ta = tm - q * ta; tm = tmp;
                        let tmp = x1; x1 = x - q * x1; x = tmp;
                    }
                    let kk = ((diff % m2g) * (x % m2g) % m2g + m2g).rem_euclid(m2g);
                    t0 = t0 + kk * step;
                    t0 = t0.rem_euclid(lcm);
                    if t0 == 0 { t0 = lcm; }
                    step = lcm;
                }
            }
        }

        if t0 == 0 { t0 = step; }

        // Lower bounds on t:
        // 1. t >= u (from a >= 1: a = s - d >= 1, need s >= d + 1, s = t + u, so t + u >= d + 1)
        //    Actually more precisely: a = s - d >= 1 requires checking the triangle inequality.
        //    The three bounds below encode a >= 1, b <= c, and a + b > c.
        let mut t_min = u;

        // 2. From b <= c: (s+d) <= 2t, need v^2 - u^2 <= 2u*t (when v^2 > u^2)
        let vv_uu = v * v - u * u;
        if vv_uu > 0 {
            let t_min2 = (vv_uu + 2 * u - 1) / (2 * u);
            if t_min2 > t_min { t_min = t_min2; }
        }

        // 3. From a + b > c: 2s > 2t => s > t always. But also need s - d >= 1 and
        //    a + b > c: (s-d) + (s+d) > 2t => 2s > 2t => s > t, always true since u >= 1.
        //    Actually need a + b > c, which is 2s > c = 2t. s = t + u > t always. OK.
        //    But also a >= 1: s - d >= 1. d can be up to m-1.
        //    w = P/v. d = (v-w)/2 when v > w. Then s - d = (t+u) - (v-w)/2.
        //    Constraint: s - d >= 1. This is hard to express in terms of just t.
        //
        //    Let's use: a >= 1 means s - d >= 1. s = t+u, d = |m - (v+w)/2| ... no.
        //    Actually m = (v+w)/2, d = (v-w)/2 (assuming v >= w).
        //    a = s - d = (t+u) - (v-w)/2.
        //    Need a >= 1: t + u - (v - w)/2 >= 1. Since w = P/v = u(2t+u)/v:
        //    t + u - (v - u(2t+u)/v)/2 >= 1. This is complex; the t_min bounds above
        //    approximate this. The exact check from the original C code uses:
        //    num = (u+v)^2 - 2v^2 = u^2 + 2uv - v^2.
        //    If num > 0: t_min3 = ceil(num / (2*(v-u))).
        let num = (u + v) * (u + v) - 2 * v * v;
        if num > 0 {
            let denom = 2 * (v - u);
            let t_min3 = (num + denom - 1) / denom;
            if t_min3 > t_min { t_min = t_min3; }
        }

        // Align t_min to the arithmetic progression
        if t_min <= t0 {
            t_min = t0;
        } else {
            let k = (t_min - t0 + step - 1) / step;
            t_min = t0 + k * step;
        }

        if t_min <= half_n {
            count += (half_n - t_min) / step + 1;
        }

        v += 2;
    }

    count
}

fn f_count(n: i64) -> i64 {
    let half_n = n / 2;
    // Parallel sum over u from 1 to half_n
    (1..=half_n)
        .into_par_iter()
        .map(|u| count_for_u(u, half_n))
        .sum()
}

fn main() {
    let n = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(NN);

    println!("{}", f_count(n));
}
