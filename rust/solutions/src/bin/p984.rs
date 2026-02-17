// Problem 984: Knights and Horses
// Port of the reference constructive-count + interpolation approach.
// Compute f(10^18) mod 1_000_000_007.

use num::{BigInt, One, ToPrimitive, Zero};
use num_integer::Integer;

const MOD: i64 = 1_000_000_007;
const TARGET_N: i64 = 1_000_000_000_000_000_000;
const SMALL_LIMIT: usize = 20;

const KNIGHT_DIRS: [(i32, i32); 8] = [
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
];

fn build_neighbors(width: usize, height: usize) -> Vec<Vec<(usize, u32)>> {
    let n = width * height;
    let mut neighbors = vec![Vec::new(); n];

    for x in 0..width as i32 {
        for y in 0..height as i32 {
            let idx = x as usize * height + y as usize;
            let nbrs = &mut neighbors[idx];
            for &(dx, dy) in &KNIGHT_DIRS {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    nbrs.push((nx as usize, 1u32 << ny));
                }
            }
        }
    }

    neighbors
}

fn is_connected(
    occ_rows: &[u32],
    occ_count: u32,
    neighbors: &[Vec<(usize, u32)>],
    height: usize,
) -> bool {
    if occ_count <= 1 {
        return true;
    }

    let mut start_x = 0usize;
    while start_x < occ_rows.len() && occ_rows[start_x] == 0 {
        start_x += 1;
    }
    if start_x == occ_rows.len() {
        return false;
    }

    let start_y = occ_rows[start_x].trailing_zeros() as usize;
    let start_idx = start_x * height + start_y;

    let mut seen_rows = vec![0u32; occ_rows.len()];
    seen_rows[start_x] |= 1u32 << start_y;
    let mut seen = 1u32;

    let mut stack = vec![start_idx];
    while let Some(idx) = stack.pop() {
        for &(nx, bit) in &neighbors[idx] {
            if (occ_rows[nx] & bit) != 0 && (seen_rows[nx] & bit) == 0 {
                seen_rows[nx] |= bit;
                seen += 1;
                let ny = bit.trailing_zeros() as usize;
                stack.push(nx * height + ny);
            }
        }
    }

    seen == occ_count
}

fn count_connected_canonical(width: usize, height: usize) -> u64 {
    let neighbors = build_neighbors(width, height);
    let full = width as i32 + height as i32 - 2;
    let mlim = (width - 1).min(height - 1) as i32;
    let mut connected_count = 0u64;

    for p in 0..=mlim {
        for q in 0..=mlim {
            let s_upper = full - q;
            for c in 0..=mlim {
                let d_lower = -(height as i32 - 1) + c;
                for d in 0..=mlim {
                    let d_upper = (width as i32 - 1) - d;

                    // Side-touch feasibility checks (canonical box definition).
                    let mut lo = 0.max(p).max(-d_upper);
                    let mut hi = (height as i32 - 1).min(s_upper).min(-d_lower);
                    if lo > hi {
                        continue;
                    }

                    lo = 0
                        .max(p - (width as i32 - 1))
                        .max((width as i32 - 1) - d_upper);
                    hi = (height as i32 - 1)
                        .min(s_upper - (width as i32 - 1))
                        .min((width as i32 - 1) - d_lower);
                    if lo > hi {
                        continue;
                    }

                    lo = 0.max(p).max(d_lower);
                    hi = (width as i32 - 1).min(s_upper).min(d_upper);
                    if lo > hi {
                        continue;
                    }

                    lo = 0
                        .max(p - (height as i32 - 1))
                        .max(d_lower + (height as i32 - 1));
                    hi = (width as i32 - 1)
                        .min(s_upper - (height as i32 - 1))
                        .min(d_upper + (height as i32 - 1));
                    if lo > hi {
                        continue;
                    }

                    let mut occ_rows = vec![0u32; width];
                    let mut occ_count = 0u32;

                    for x in 0..width as i32 {
                        let mut y_lo = p - x;
                        let t = x - d_upper;
                        if t > y_lo {
                            y_lo = t;
                        }
                        if y_lo < 0 {
                            y_lo = 0;
                        }

                        let mut y_hi = s_upper - x;
                        let t = x - d_lower;
                        if t < y_hi {
                            y_hi = t;
                        }
                        if y_hi > height as i32 - 1 {
                            y_hi = height as i32 - 1;
                        }

                        if y_lo <= y_hi {
                            let lo = y_lo as u32;
                            let hi = y_hi as u32;
                            let mut row_mask = (1u32 << (hi + 1)) - 1;
                            if lo > 0 {
                                row_mask ^= (1u32 << lo) - 1;
                            }
                            occ_rows[x as usize] = row_mask;
                            occ_count += row_mask.count_ones();
                        }
                    }

                    if is_connected(&occ_rows, occ_count, &neighbors, height) {
                        connected_count += 1;
                    }
                }
            }
        }
    }

    connected_count
}

fn compute_f_upto(limit: usize) -> Vec<i128> {
    let mut canonical_counts = vec![vec![0u64; limit + 1]; limit + 1];

    for width in 1..=limit {
        for height in 1..=width {
            let val = count_connected_canonical(width, height);
            canonical_counts[width][height] = val;
            canonical_counts[height][width] = val;
        }
    }

    let mut f = vec![0i128; limit + 1];
    for n in 1..=limit {
        let mut total = 0i128;
        for width in 1..=n {
            let wx = (n - width + 1) as i128;
            for height in 1..=n {
                total += wx * (n - height + 1) as i128 * canonical_counts[width][height] as i128;
            }
        }
        f[n] = total;
    }

    f
}

fn mod_norm(x: i64, modulus: i64) -> i64 {
    let v = x % modulus;
    if v < 0 { v + modulus } else { v }
}

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    base = mod_norm(base, modulus);
    let mut out = 1i64;
    while exp > 0 {
        if (exp & 1) == 1 {
            out = (out as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    out
}

fn mod_inv(a: i64, modulus: i64) -> i64 {
    mod_pow(a, modulus - 2, modulus)
}

fn lagrange_eval_mod(xs: &[i64], ys: &[i64], x: i64, modulus: i64) -> i64 {
    let x = mod_norm(x, modulus);
    let mut ans = 0i64;

    for i in 0..xs.len() {
        let xi = mod_norm(xs[i], modulus);
        let mut num = 1i64;
        let mut den = 1i64;

        for j in 0..xs.len() {
            if i == j {
                continue;
            }
            let xj = mod_norm(xs[j], modulus);
            num = (num as i128 * mod_norm(x - xj, modulus) as i128 % modulus as i128) as i64;
            den = (den as i128 * mod_norm(xi - xj, modulus) as i128 % modulus as i128) as i64;
        }

        let term = (ys[i] as i128
            * num as i128
            % modulus as i128
            * mod_inv(den, modulus) as i128
            % modulus as i128) as i64;
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

fn solve() -> i64 {
    let f = compute_f_upto(SMALL_LIMIT);
    assert_eq!(f[3], 9);
    assert_eq!(f[5], 903);

    let even_xs = [4i64, 6, 8, 10, 12, 14, 16, 18, 20];
    let even_ys: Vec<i128> = even_xs.iter().map(|&n| f[n as usize]).collect();

    assert_eq!(lagrange_eval_int(&even_xs, &even_ys, 100), 8_658_918_531_876);

    let even_ys_mod: Vec<i64> = even_ys
        .iter()
        .map(|&y| (y % MOD as i128) as i64)
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
