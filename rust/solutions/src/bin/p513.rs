// Project Euler 513 - Triangles with Integer Median
//
// Count triangles with integer sides a <= b <= c <= N where the median to c
// has integer length.
//
// Primitive (s,t) / (u,v) lattice regions are counted with Euclidean floor-sum
// trapezoid queries (O(log) per rational line). Full F(n) is recovered from
// the primitive count by the odd-scaling recurrence
//   F(n) = f(n) - sum_{k odd, k>=3} F(floor(n/k)).

use std::collections::HashMap;

const NN: i64 = 100_000;

#[inline(always)]
fn floor_div(a: i64, b: i64) -> i64 {
    a.div_euclid(b)
}

fn trapezoid_floor_sum(
    mut slope: i64,
    mut intercept: i64,
    mut denominator: i64,
    mut lower_x: i64,
    mut upper_x: i64,
    mut include_boundary: bool,
) -> i64 {
    let mut total = 0i64;
    loop {
        if (upper_x - lower_x).abs() <= 8 {
            let adjustment = if include_boundary { 0 } else { 1 };
            if upper_x > lower_x {
                let mut x = lower_x + 1;
                while x <= upper_x {
                    total += floor_div(slope * x + intercept - adjustment, denominator);
                    x += 1;
                }
            } else {
                let mut subtotal = 0i64;
                let mut x = upper_x + 1;
                while x <= lower_x {
                    subtotal += floor_div(slope * x + intercept - adjustment, denominator);
                    x += 1;
                }
                total -= subtotal;
            }
            return total;
        }

        let whole_intercept = floor_div(intercept, denominator);
        if whole_intercept != 0 {
            total += (upper_x - lower_x) * whole_intercept;
            intercept -= whole_intercept * denominator;
        }

        let whole_slope = floor_div(slope, denominator);
        if whole_slope != 0 {
            total += (upper_x - lower_x) * (upper_x + lower_x + 1) / 2 * whole_slope;
            slope -= whole_slope * denominator;
        }

        if slope == 0 {
            if intercept == 0 && !include_boundary {
                total -= upper_x - lower_x;
            }
            return total;
        }

        let upper_y = floor_div(slope * upper_x + intercept, denominator);
        let lower_y = floor_div(slope * lower_x + intercept, denominator);
        total += upper_x * upper_y - lower_x * lower_y;

        let (nl, nu) = (upper_y, lower_y);
        lower_x = nl;
        upper_x = nu;
        let nd = slope;
        slope = denominator;
        denominator = nd;
        intercept = -intercept;
        include_boundary = !include_boundary;
    }
}

fn trapezoid_floor_sum_mod2(
    slope: i64,
    mut intercept: i64,
    denominator: i64,
    mut lower_x: i64,
    mut upper_x: i64,
    include_boundary: bool,
    x_residue: i64,
    y_residue: i64,
) -> i64 {
    if y_residue & 1 != 0 {
        intercept += denominator;
    }
    if x_residue & 1 != 0 {
        intercept -= slope;
        lower_x += 1;
        upper_x += 1;
    }
    trapezoid_floor_sum(
        2 * slope,
        intercept,
        2 * denominator,
        lower_x.div_euclid(2),
        upper_x.div_euclid(2),
        include_boundary,
    )
}

fn primitive_count(n: i64) -> i64 {
    let mut total = 0i64;
    let three_halves_n = n + n / 2;
    let root = three_halves_n.isqrt();

    let parity_cases = [(0i64, 1i64), (1, 0), (1, 1)];
    for &(i_residue, j_residue) in &parity_cases {
        let mut max_t = 1i64;
        for s in 2..root {
            if 3 * (max_t + 1) * (max_t + 1) <= s * s {
                max_t += 1;
            }
            if i_residue == j_residue || (s & 1) == 0 {
                let start_t = ((s - 1) & 1) + 1;
                let mut t = start_t;
                while t <= max_t {
                    let v_mid = t * n / ((s - t) * (s + t));
                    let v_max = n * (s + t) / (s * s + 2 * s * t - t * t);
                    total += trapezoid_floor_sum_mod2(s, 0, t, 0, v_mid, true, j_residue, i_residue);
                    total += trapezoid_floor_sum_mod2(
                        t, n, s, v_mid, v_max, true, j_residue, i_residue,
                    );
                    total -= trapezoid_floor_sum_mod2(
                        s + 3 * t,
                        0,
                        s + t,
                        0,
                        v_max,
                        false,
                        j_residue,
                        i_residue,
                    );
                    t += 2;
                }
            }
        }

        let max_u = three_halves_n / root;
        let start_u = 1 + ((i_residue + 1) & 1);
        let mut u = start_u;
        while u <= max_u {
            let max_v_outer = (n / 2).min(u - 1);
            let start_v = 1 + ((j_residue + 1) & 1);
            let mut v = start_v;
            while v <= max_v_outer {
                let split_s = (v + n) / u;
                let residue_count = if i_residue == j_residue { 2 } else { 1 };
                for mut s_residue in 0..residue_count {
                    if i_residue != j_residue {
                        s_residue = 0;
                    }
                    let (min_s, max_s, slope0, slope1);
                    if u * u < 3 * v * v {
                        min_s = root;
                        max_s = n * (3 * v - u) / (2 * u * v + v * v - u * u);
                        slope0 = u - v;
                        slope1 = 3 * v - u;
                    } else {
                        min_s = root.max((u + v - 1) / v);
                        max_s = n * u / ((u - v) * (u + v));
                        slope0 = v;
                        slope1 = u;
                    }
                    if max_s < min_s {
                        continue;
                    }
                    let a = trapezoid_floor_sum_mod2(
                        slope0,
                        0,
                        slope1,
                        min_s - 1,
                        max_s,
                        true,
                        s_residue,
                        s_residue,
                    );
                    let mut b = 0i64;
                    if split_s < max_s {
                        b = trapezoid_floor_sum_mod2(
                            u,
                            -n,
                            v,
                            split_s.max(min_s - 1),
                            max_s,
                            false,
                            s_residue,
                            s_residue,
                        );
                    }
                    total += a - b;
                }
                v += 2;
            }
            u += 2;
        }
    }
    total
}

fn f_rec(n: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if n <= 0 {
        return 0;
    }
    if let Some(&c) = cache.get(&n) {
        return c;
    }
    let mut result = primitive_count(n);

    let mut k = 3i64;
    let mut quotient = n / k;
    while k <= quotient {
        result -= f_rec(quotient, cache);
        k += 2;
        quotient = n / k;
    }

    let mut min_k = if quotient + 1 > 0 {
        n / (quotient + 1)
    } else {
        n
    };
    while quotient > 0 {
        let max_k = n / quotient;
        let left = (min_k + 1) + (min_k & 1);
        let right = max_k - ((max_k + 1) & 1);
        if right >= left {
            result -= f_rec(quotient, cache) * ((right - left) / 2 + 1);
        }
        quotient -= 1;
        min_k = max_k;
    }

    cache.insert(n, result);
    result
}

fn f_count(n: i64) -> i64 {
    let mut cache = HashMap::with_capacity(4096);
    f_rec(n, &mut cache)
}

fn main() {
    let n = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(NN);

    println!("{}", f_count(n));
}
