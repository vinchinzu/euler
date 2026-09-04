// Problem 998: Squaring the Triangle
// T(n) = sum of perimeters of incongruent integer triangles whose
// minimum bounding square has integer side <= n. Find T(10^6).
//
// Generate Pythagorean offsets for each square side m, then close
// either along an edge (x+y <= m and xy >= m(m-x-y)) or through the
// opposite residual (third side square, exact min-square test).

use rayon::prelude::*;

const LIMIT: i32 = 1_000_000;

#[inline]
fn gcd_bin(mut u: i32, mut v: i32) -> i32 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    let shift = (u | v).trailing_zeros();
    u >>= u.trailing_zeros();
    loop {
        v >>= v.trailing_zeros();
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
        if v == 0 {
            break;
        }
    }
    u << shift
}

const SQUARE_MOD_64_MASK: u64 = 0x0202021202030213;

#[inline(always)]
fn is_square(n: i64) -> Option<i64> {
    if (SQUARE_MOD_64_MASK >> (n as u64 & 63)) & 1 == 0 {
        return None;
    }
    let r = n.isqrt();
    if r * r == n {
        Some(r)
    } else {
        None
    }
}

fn pythagorean_partners_csr(limit: i32) -> (Vec<u32>, Vec<(i32, i32)>) {
    let mut deg = vec![0u32; (limit + 2) as usize];
    let r_max = ((2 * limit as i64).isqrt() as i32) + 3;

    let mut primitives = Vec::with_capacity(200_000);
    for r in 2..=r_max {
        let rr = r * r;
        let s_start = 1 + (r & 1);
        for s in (s_start..r).step_by(2) {
            if gcd_bin(r, s) != 1 {
                continue;
            }
            let a = rr - s * s;
            let b = 2 * r * s;
            let c = rr + s * s;
            let (m, x) = if a > b { (a, b) } else { (b, a) };
            if m <= limit {
                primitives.push((m, x, c));
                let mut km = m;
                while km <= limit {
                    deg[km as usize] += 1;
                    km += m;
                }
            }
        }
    }

    let mut offsets = vec![0u32; (limit + 2) as usize];
    for i in 1..=(limit + 1) as usize {
        offsets[i] = offsets[i - 1] + deg[i - 1];
    }
    let total = offsets[(limit + 1) as usize] as usize;
    let mut data = vec![(0i32, 0i32); total];
    let mut cur_pos = offsets.clone();

    for &(m, x, c) in &primitives {
        let mut k = 1;
        let mut km = m;
        while km <= limit {
            let pos = cur_pos[km as usize] as usize;
            data[pos] = (k * x, k * c);
            cur_pos[km as usize] += 1;
            k += 1;
            km += m;
        }
    }

    (1..=limit as usize).into_par_iter().for_each(|m| {
        let start = offsets[m] as usize;
        let end = offsets[m + 1] as usize;
        if end - start > 1 {
            let slice = unsafe {
                let ptr = data.as_ptr() as *mut (i32, i32);
                std::slice::from_raw_parts_mut(ptr.add(start), end - start)
            };
            slice.sort_unstable_by_key(|&(x, _)| x);
        }
    });

    (offsets, data)
}

fn is_minimum_square(ss: [i64; 3], twice_area: i64, m: i64) -> bool {
    let m2 = m * m;
    let mut has_equal = false;

    for i in 0..3 {
        let d = ss[i];
        let e = ss[(i + 1) % 3];
        let f = ss[(i + 2) % 3];
        let den = 2 * d;
        let t_num = d * d + e * e - f * f;
        let d_num = d * den;
        let lo = 0.min(d_num).min(t_num);
        let hi = 0.max(d_num).max(t_num);
        let width_cmp = (hi - lo) - m * den;
        let height_cmp = twice_area - m * d;
        if width_cmp < 0 && height_cmp < 0 {
            return false;
        }
        if width_cmp <= 0 && height_cmp <= 0 && (width_cmp == 0 || height_cmp == 0) {
            has_equal = true;
        }
    }

    for i in 0..3 {
        let r = ss[i];
        let p = ss[(i + 1) % 3];
        let q = ss[(i + 2) % 3];
        let k_num = p * p + q * q - r * r;
        if k_num <= 0 {
            continue;
        }
        let r_den_part = p * p + q * q - 2 * twice_area;
        if r_den_part <= 0 {
            continue;
        }
        let num = k_num as i128 * k_num as i128;
        let den = 4 * r_den_part as i128;
        let p2 = p as i128 * p as i128;
        let q2 = q as i128 * q as i128;
        let d_area = twice_area as i128;
        if num < d_area * den {
            continue;
        }
        if num > p2 * den || num > q2 * den {
            continue;
        }
        if p2 * den > 2 * num || q2 * den > 2 * num {
            continue;
        }
        let target = m2 as i128 * den;
        if num < target {
            return false;
        }
        if num == target {
            has_equal = true;
        }
    }
    has_equal
}

#[inline(always)]
fn solve_m(m: i32, partners: &[(i32, i32)]) -> u64 {
    let mm = m as i64 * m as i64;
    let mut sum = 0u64;
    let count = partners.len();

    // Loop 1: strictly between partners (since (0, m) never satisfies xy >= m(m-base))
    for i in 0..count {
        let (x, hx) = partners[i];
        for j in i..count {
            let (y, hy) = partners[j];
            let base = x + y;
            if base > m {
                break;
            }
            if (x as i64) * (y as i64) < (m as i64) * ((m - base) as i64) {
                continue;
            }
            sum += (base + hx + hy) as u64;
        }
    }

    // Loop 2: includes (0, m) as first candidate
    let mut row = [(0i32, 0i32); 256];
    row[0] = (0, m);
    row[1..1 + count].copy_from_slice(partners);
    let row_len = 1 + count;

    for i in 0..row_len {
        let (u, hu) = row[i];
        let p = (m - u) as i64;
        let p2 = p * p;
        for j in i..row_len {
            let (v, hv) = row[j];
            let twice_area = mm - (u as i64) * (v as i64);
            if twice_area <= 0 {
                continue;
            }
            let q = (m - v) as i64;
            let third2 = p2 + q * q;
            if let Some(third) = is_square(third2) {
                if third > 0 {
                    let sides = [third, hu as i64, hv as i64];
                    if is_minimum_square(sides, twice_area, m as i64) {
                        sum += (third + hu as i64 + hv as i64) as u64;
                    }
                }
            }
        }
    }
    sum
}

fn solve(limit: i32) -> u64 {
    let (offsets, data) = pythagorean_partners_csr(limit);
    (1..=limit)
        .into_par_iter()
        .map(|m| {
            let start = offsets[m as usize] as usize;
            let end = offsets[(m + 1) as usize] as usize;
            solve_m(m, &data[start..end])
        })
        .sum()
}

fn main() {
    debug_assert_eq!(solve(40), 346);
    debug_assert_eq!(solve(400), 76_402);
    println!("{}", solve(LIMIT));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn samples() {
        assert_eq!(solve(40), 346);
        assert_eq!(solve(400), 76_402);
        assert_eq!(solve(2000), 3_237_036);
    }
}
