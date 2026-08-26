// Project Euler 732 - Standing on the Shoulders of Trolls
//
// For each arm-troll i, min IQ of a subset of the others with height
// >= D - h_i - l_i. Prefix/suffix knapsacks, flat layers, no clones.

use rayon::prelude::*;

const NTROLLS: usize = 1000;
const MOD_VAL: i64 = 1_000_000_007;
const INF: i32 = 1_000_000_000;

struct Troll {
    h: i32,
    l: i32,
    q: i32,
}

fn generate_trolls() -> Vec<Troll> {
    let mut trolls = Vec::with_capacity(NTROLLS);
    let mut r: i64 = 1;
    for _ in 0..NTROLLS {
        let h = ((r % 101) + 101) % 101 + 50;
        r = r * 5 % MOD_VAL;
        let l = ((r % 101) + 101) % 101 + 50;
        r = r * 5 % MOD_VAL;
        let q = ((r % 101) + 101) % 101 + 50;
        r = r * 5 % MOD_VAL;
        trolls.push(Troll {
            h: h as i32,
            l: l as i32,
            q: q as i32,
        });
    }
    trolls
}

/// layers[k * cols + j] = min IQ to reach height >= j using the first k items.
/// Sequential 0-1 knapsack; writes each new row from the previous (no clone).
fn build_layers(items: &[(i32, i32)], d: usize) -> Vec<i32> {
    let cols = d + 1;
    let n_layers = items.len() + 1;
    let mut dp = Vec::with_capacity(n_layers * cols);
    unsafe {
        dp.set_len(n_layers * cols);
        let p = dp.as_mut_ptr();
        *p = 0;
        for j in 1..cols {
            *p.add(j) = INF;
        }
        for k in 0..items.len() {
            let h = items[k].0 as usize;
            let q = items[k].1;
            let src = p.add(k * cols);
            let dst = p.add((k + 1) * cols);
            // SAFETY: src/dst are adjacent non-overlapping rows of length `cols`.
            // Row 0 is initialized; each later row is fully written before it is read.
            std::ptr::copy_nonoverlapping(src, dst, h);
            let mut j = h;
            while j + 8 <= cols {
                for t in 0..8 {
                    let v = *src.add(j + t);
                    let u = *src.add(j + t - h) + q;
                    *dst.add(j + t) = if u < v { u } else { v };
                }
                j += 8;
            }
            while j < cols {
                let v = *src.add(j);
                let u = *src.add(j - h) + q;
                *dst.add(j) = if u < v { u } else { v };
                j += 1;
            }
            let mut m = *dst.add(cols - 1);
            let mut i = cols - 1;
            while i > 0 {
                i -= 1;
                let x = *dst.add(i);
                if m < x {
                    *dst.add(i) = m;
                } else {
                    m = x;
                }
            }
        }
    }
    dp
}

/// min_j left[j] + right[dist - j]
#[inline(always)]
fn min_pair_sum(left: &[i32], right: &[i32], dist: usize) -> i32 {
    unsafe {
        let lp = left.as_ptr();
        let mut rp = right.as_ptr().add(dist);
        let mut best = INF;
        let mut j = 0usize;
        while j + 8 <= dist + 1 {
            for t in 0..8 {
                let s = *lp.add(j + t) + *rp.sub(t);
                if s < best {
                    best = s;
                }
            }
            j += 8;
            rp = rp.sub(8);
        }
        while j <= dist {
            let s = *lp.add(j) + *rp;
            if s < best {
                best = s;
            }
            j += 1;
            rp = rp.sub(1);
        }
        best
    }
}

fn main() {
    let trolls = generate_trolls();

    let mut total_h: i32 = 0;
    let mut total_iq: i32 = 0;
    for t in &trolls {
        total_h += t.h;
        total_iq += t.q;
    }
    let d = (total_h as f64 / std::f64::consts::SQRT_2).ceil() as usize;
    let cols = d + 1;

    let mut left_items = Vec::with_capacity(NTROLLS - 1);
    for t in trolls.iter().take(NTROLLS - 1) {
        left_items.push((t.h, t.q));
    }
    let mut right_items = Vec::with_capacity(NTROLLS - 1);
    for t in trolls.iter().skip(1).rev() {
        right_items.push((t.h, t.q));
    }

    let (left, right) = rayon::join(
        || build_layers(&left_items, d),
        || build_layers(&right_items, d),
    );

    let ans = (0..NTROLLS)
        .into_par_iter()
        .with_min_len(8)
        .map(|i| {
            let dist_raw = d as i32 - trolls[i].h - trolls[i].l;
            if dist_raw < 0 {
                return 0;
            }
            let dist = dist_raw as usize;
            let lrow = &left[i * cols..i * cols + cols];
            let rk = NTROLLS - 1 - i;
            let rrow = &right[rk * cols..rk * cols + cols];
            let used = min_pair_sum(lrow, rrow, dist);
            if used >= INF {
                0
            } else {
                total_iq - used
            }
        })
        .max()
        .unwrap_or(0);

    println!("{}", ans);
}
