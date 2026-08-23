// Problem 984: Knights and Horses
// Port of the reference constructive-count + interpolation approach.
// Compute f(10^18) mod 1_000_000_007.

use num::{BigInt, One, ToPrimitive, Zero};
use num_integer::Integer;
use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const TARGET_N: i64 = 1_000_000_000_000_000_000;
const SMALL_LIMIT: usize = 20;

#[inline]
fn is_connected(occ: &[u32; SMALL_LIMIT], width: usize, occ_count: u32) -> bool {
    if occ_count <= 1 {
        return true;
    }

    let mut start_x = 0usize;
    while start_x < width && occ[start_x] == 0 {
        start_x += 1;
    }

    let mut seen = [0u32; SMALL_LIMIT];
    let mut frontier = [0u32; SMALL_LIMIT];
    let start_bit = occ[start_x] & occ[start_x].wrapping_neg();
    seen[start_x] = start_bit;
    frontier[start_x] = start_bit;
    let mut seen_count = 1u32;

    loop {
        let mut changed = false;
        for x in 0..width {
            let bits = frontier[x];
            if bits == 0 {
                continue;
            }
            frontier[x] = 0;
            let shift2 = (bits << 2) | (bits >> 2);
            let shift1 = (bits << 1) | (bits >> 1);

            if x + 1 < width {
                let reach = shift2 & occ[x + 1] & !seen[x + 1];
                if reach != 0 {
                    seen[x + 1] |= reach;
                    frontier[x + 1] |= reach;
                    seen_count += reach.count_ones();
                    changed = true;
                }
            }
            if x > 0 {
                let reach = shift2 & occ[x - 1] & !seen[x - 1];
                if reach != 0 {
                    seen[x - 1] |= reach;
                    frontier[x - 1] |= reach;
                    seen_count += reach.count_ones();
                    changed = true;
                }
            }
            if x + 2 < width {
                let reach = shift1 & occ[x + 2] & !seen[x + 2];
                if reach != 0 {
                    seen[x + 2] |= reach;
                    frontier[x + 2] |= reach;
                    seen_count += reach.count_ones();
                    changed = true;
                }
            }
            if x >= 2 {
                let reach = shift1 & occ[x - 2] & !seen[x - 2];
                if reach != 0 {
                    seen[x - 2] |= reach;
                    frontier[x - 2] |= reach;
                    seen_count += reach.count_ones();
                    changed = true;
                }
            }
        }
        if !changed || seen_count == occ_count {
            break;
        }
    }

    seen_count == occ_count
}

fn count_connected_for_p(width: usize, height: usize, p: i32, mlim: i32) -> u64 {
    let w = width as i32;
    let h = height as i32;
    let full = w + h - 2;
    let mut connected_count = 0u64;
    let mut occ = [0u32; SMALL_LIMIT];

    for q in 0..=mlim {
        let s_upper = full - q;
        for c in 0..=mlim {
            let d_lower = -(h - 1) + c;
            for d in 0..=mlim {
                let d_upper = (w - 1) - d;

                // Side-touch feasibility checks (canonical box definition).
                let mut lo = 0.max(p).max(-d_upper);
                let mut hi = (h - 1).min(s_upper).min(-d_lower);
                if lo > hi {
                    continue;
                }

                lo = 0.max(p - (w - 1)).max((w - 1) - d_upper);
                hi = (h - 1).min(s_upper - (w - 1)).min((w - 1) - d_lower);
                if lo > hi {
                    continue;
                }

                lo = 0.max(p).max(d_lower);
                hi = (w - 1).min(s_upper).min(d_upper);
                if lo > hi {
                    continue;
                }

                lo = 0.max(p - (h - 1)).max(d_lower + (h - 1));
                hi = (w - 1).min(s_upper - (h - 1)).min(d_upper + (h - 1));
                if lo > hi {
                    continue;
                }

                let mut occ_count = 0u32;
                for x in 0..width {
                    let xi = x as i32;
                    let y_lo = 0.max(p - xi).max(xi - d_upper);
                    let y_hi = (h - 1).min(s_upper - xi).min(xi - d_lower);
                    let row_mask = if y_lo <= y_hi {
                        (1u32 << (y_hi as u32 + 1)) - (1u32 << (y_lo as u32))
                    } else {
                        0
                    };
                    occ[x] = row_mask;
                    occ_count += row_mask.count_ones();
                }

                if is_connected(&occ, width, occ_count) {
                    connected_count += 1;
                }
            }
        }
    }

    connected_count
}

fn compute_f_upto(limit: usize) -> Vec<i128> {
    debug_assert!(limit <= SMALL_LIMIT);

    // Flatten (width, height, p) so large boards split into stealable units.
    let mut units = Vec::new();
    for width in 1..=limit {
        for height in 1..=width {
            let mlim = (width - 1).min(height - 1);
            for p in 0..=mlim {
                units.push((width, height, p as i32));
            }
        }
    }
    units.sort_unstable_by_key(|&(w, h, _p)| std::cmp::Reverse(h.pow(5) * w));

    let parts: Vec<(usize, usize, u64)> = units
        .into_par_iter()
        .map(|(width, height, p)| {
            let mlim = (width - 1).min(height - 1) as i32;
            (
                width,
                height,
                count_connected_for_p(width, height, p, mlim),
            )
        })
        .collect();

    let stride = limit + 1;
    let mut canonical_counts = vec![0u64; stride * stride];
    for (width, height, val) in parts {
        canonical_counts[width * stride + height] += val;
        if width != height {
            canonical_counts[height * stride + width] += val;
        }
    }

    let mut f = vec![0i128; limit + 1];
    for n in 1..=limit {
        let mut total = 0i128;
        for width in 1..=n {
            let wx = (n - width + 1) as i128;
            let row = width * stride;
            for height in 1..=n {
                total += wx
                    * (n - height + 1) as i128
                    * canonical_counts[row + height] as i128;
            }
        }
        f[n] = total;
    }

    f
}

fn mod_norm(x: i64, modulus: u64) -> u64 {
    let m = modulus as i64;
    let v = x % m;
    if v < 0 {
        (v + m) as u64
    } else {
        v as u64
    }
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    base %= modulus;
    let mut out = 1u64;
    while exp > 0 {
        if (exp & 1) == 1 {
            out = out * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    out
}

fn mod_inv(a: u64, modulus: u64) -> u64 {
    mod_pow(a, modulus - 2, modulus)
}

fn lagrange_eval_mod(xs: &[i64], ys: &[u64], x: i64, modulus: u64) -> u64 {
    let x = mod_norm(x, modulus);
    let mut ans = 0u64;

    for i in 0..xs.len() {
        let xi = mod_norm(xs[i], modulus);
        let mut num = 1u64;
        let mut den = 1u64;

        for j in 0..xs.len() {
            if i == j {
                continue;
            }
            let xj = mod_norm(xs[j], modulus);
            num = num * (x + modulus - xj) % modulus;
            den = den * (xi + modulus - xj) % modulus;
        }

        let term = ys[i] * num % modulus * mod_inv(den, modulus) % modulus;
        ans += term;
        if ans >= modulus {
            ans -= modulus;
        }
    }

    ans
}

fn lagrange_eval_int(xs: &[i64], ys: &[i128], x: i64) -> i128 {
    let mut total_num = BigInt::zero();
    let mut total_den = BigInt::one();
    let x_big = BigInt::from(x);

    for i in 0..xs.len() {
        let mut term_num = BigInt::from(ys[i]);
        let mut term_den = BigInt::one();
        let xi = BigInt::from(xs[i]);

        for j in 0..xs.len() {
            if i == j {
                continue;
            }
            let xj = BigInt::from(xs[j]);
            term_num *= &x_big - &xj;
            term_den *= &xi - &xj;
            let g = term_num.gcd(&term_den);
            term_num /= &g;
            term_den /= g;
        }

        total_num = total_num * &term_den + term_num * &total_den;
        total_den *= term_den;
        let g = total_num.gcd(&total_den);
        total_num /= &g;
        total_den /= g;
    }

    assert_eq!(total_den, BigInt::one());
    total_num
        .to_i128()
        .expect("integer interpolation value should fit i128")
}

fn solve() -> u64 {
    let f = compute_f_upto(SMALL_LIMIT);
    assert_eq!(f[3], 9);
    assert_eq!(f[5], 903);

    let even_xs = [4i64, 6, 8, 10, 12, 14, 16, 18, 20];
    let even_ys: Vec<i128> = even_xs.iter().map(|&n| f[n as usize]).collect();

    assert_eq!(lagrange_eval_int(&even_xs, &even_ys, 100), 8_658_918_531_876);

    let even_ys_mod: Vec<u64> = even_ys
        .iter()
        .map(|&y| y.rem_euclid(MOD as i128) as u64)
        .collect();
    assert_eq!(
        lagrange_eval_mod(&even_xs, &even_ys_mod, 10_000, MOD),
        377_956_308
    );

    lagrange_eval_mod(&even_xs, &even_ys_mod, TARGET_N, MOD)
}

fn main() {
    println!("{}", solve());
}
