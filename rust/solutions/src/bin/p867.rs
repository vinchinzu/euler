// Project Euler 867 - Dodecagon Tilings
// Profile DP for hex/trap tilings, then dodecagon memoization. N = 10.

use rayon::prelude::*;

const MOD: i32 = 1_000_000_007;
const N: i32 = 10;
const PAR_WINDOW: usize = 14;

#[inline(always)]
fn add_mod(a: i32, b: i32) -> i32 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn pow6(t: i64) -> i64 {
    let m = MOD as i64;
    let t2 = t * t % m;
    t2 * t2 % m * t2 % m
}

/// One DP layer: dp_curr[prev] from dp_next via (prev<<1) transitions.
/// High half of prev shares the same next-indices as the low half.
fn fill_layer(dp_next: &[i32], dp_curr: &mut [i32], conflict: u32, window_len: usize) {
    if window_len == 0 {
        dp_curr[0] = add_mod(dp_next[0], dp_next[0]);
        return;
    }

    let half = 1usize << (window_len - 1);
    let conflict_low = conflict & (half as u32 - 1);
    let high_blocked = (conflict & half as u32) != 0;
    let (low, high) = dp_curr.split_at_mut(half);

    if window_len >= PAR_WINDOW {
        fill_layer_par(dp_next, low, high, conflict_low, high_blocked, half);
    } else {
        fill_layer_seq(dp_next, low, high, conflict_low, high_blocked, half);
    }
}

fn fill_layer_seq(
    dp_next: &[i32],
    low: &mut [i32],
    high: &mut [i32],
    conflict_low: u32,
    high_blocked: bool,
    half: usize,
) {
    // SAFETY: prev < half, next0 = prev*2 < 2*half = mask_mod; slices have length half / mask_mod.
    unsafe {
        let src = dp_next.as_ptr();
        let lo = low.as_mut_ptr();
        let hi = high.as_mut_ptr();
        if conflict_low == 0 {
            if high_blocked {
                for prev in 0..half {
                    let v0 = *src.add(prev << 1);
                    let v1 = *src.add((prev << 1) | 1);
                    *lo.add(prev) = add_mod(v0, v1);
                    *hi.add(prev) = v0;
                }
            } else {
                for prev in 0..half {
                    let v0 = *src.add(prev << 1);
                    let v1 = *src.add((prev << 1) | 1);
                    let v = add_mod(v0, v1);
                    *lo.add(prev) = v;
                    *hi.add(prev) = v;
                }
            }
        } else if high_blocked {
            for prev in 0..half {
                let v0 = *src.add(prev << 1);
                if (prev as u32 & conflict_low) == 0 {
                    let v1 = *src.add((prev << 1) | 1);
                    *lo.add(prev) = add_mod(v0, v1);
                } else {
                    *lo.add(prev) = v0;
                }
                *hi.add(prev) = v0;
            }
        } else {
            for prev in 0..half {
                let v0 = *src.add(prev << 1);
                let v = if (prev as u32 & conflict_low) == 0 {
                    add_mod(v0, *src.add((prev << 1) | 1))
                } else {
                    v0
                };
                *lo.add(prev) = v;
                *hi.add(prev) = v;
            }
        }
    }
}

fn fill_layer_par(
    dp_next: &[i32],
    low: &mut [i32],
    high: &mut [i32],
    conflict_low: u32,
    high_blocked: bool,
    half: usize,
) {
    let nt = rayon::current_num_threads().max(1);
    let chunk_size = (half / nt).max(2048);
    low.par_chunks_mut(chunk_size)
        .zip(high.par_chunks_mut(chunk_size))
        .enumerate()
        .for_each(|(ci, (lc, hc))| {
            let start = ci * chunk_size;
            // SAFETY: start+i < half; next0 = (start+i)*2 < mask_mod; chunk lens match.
            unsafe {
                let src = dp_next.as_ptr();
                let n = lc.len();
                if conflict_low == 0 {
                    if high_blocked {
                        for i in 0..n {
                            let prev = start + i;
                            let v0 = *src.add(prev << 1);
                            let v1 = *src.add((prev << 1) | 1);
                            *lc.get_unchecked_mut(i) = add_mod(v0, v1);
                            *hc.get_unchecked_mut(i) = v0;
                        }
                    } else {
                        for i in 0..n {
                            let prev = start + i;
                            let v0 = *src.add(prev << 1);
                            let v1 = *src.add((prev << 1) | 1);
                            let v = add_mod(v0, v1);
                            *lc.get_unchecked_mut(i) = v;
                            *hc.get_unchecked_mut(i) = v;
                        }
                    }
                } else if high_blocked {
                    for i in 0..n {
                        let prev = start + i;
                        let v0 = *src.add(prev << 1);
                        if (prev as u32 & conflict_low) == 0 {
                            let v1 = *src.add((prev << 1) | 1);
                            *lc.get_unchecked_mut(i) = add_mod(v0, v1);
                        } else {
                            *lc.get_unchecked_mut(i) = v0;
                        }
                        *hc.get_unchecked_mut(i) = v0;
                    }
                } else {
                    for i in 0..n {
                        let prev = start + i;
                        let v0 = *src.add(prev << 1);
                        let v = if (prev as u32 & conflict_low) == 0 {
                            add_mod(v0, *src.add((prev << 1) | 1))
                        } else {
                            v0
                        };
                        *lc.get_unchecked_mut(i) = v;
                        *hc.get_unchecked_mut(i) = v;
                    }
                }
            }
        });
}

fn tilings_with_tri_hex_iter(px: &[i32], py: &[i32], window_len: usize) -> i64 {
    let npts = px.len();
    if npts == 0 {
        return 1;
    }
    if window_len > 20 {
        return 0;
    }

    let mut conflicts = vec![0u32; npts];
    for index in 0..npts {
        let pxi = px[index];
        let pyi = py[index];
        let lim = window_len.min(index);
        let mut mask = 0u32;
        for i in 0..lim {
            let qi = index - i - 1;
            let dy = (pyi - py[qi]).unsigned_abs();
            let dx = (pxi - px[qi]).unsigned_abs();
            if dy <= 1 && dx + dy <= 2 {
                mask |= 1 << i;
            }
        }
        conflicts[index] = mask;
    }

    let mask_mod = 1usize << window_len;
    let mut dp_next = vec![1i32; mask_mod];
    let mut dp_curr = vec![0i32; mask_mod];

    for index in (0..npts).rev() {
        fill_layer(&dp_next, &mut dp_curr, conflicts[index], window_len);
        std::mem::swap(&mut dp_next, &mut dp_curr);
    }

    dp_next[0] as i64
}

fn compute_hexagon(size: i32) -> i64 {
    if size <= 0 {
        return 1;
    }
    let mut px = Vec::with_capacity(300);
    let mut py = Vec::with_capacity(300);
    for y in (-(size - 1))..size {
        let ay = y.abs();
        let mut x = -2 * size + ay + 2;
        while x < 2 * size - ay {
            px.push(x);
            py.push(y);
            x += 2;
        }
    }
    tilings_with_tri_hex_iter(&px, &py, (2 * size - 1) as usize)
}

fn compute_trapezoid(base: i32, height: i32) -> i64 {
    if height <= 0 || base <= 0 {
        return 1;
    }
    let mut px = Vec::with_capacity(128);
    let mut py = Vec::with_capacity(128);
    for y in (base - height)..(base - 1) {
        let mut x = 1 - y;
        while x < y {
            px.push(x);
            py.push(y);
            x += 2;
        }
    }
    tilings_with_tri_hex_iter(&px, &py, (base - 1) as usize)
}

fn tilings_for_dodecagon(
    a: i32,
    b: i32,
    allow_a: bool,
    allow_b: bool,
    hex: &[i64],
    trap: &[[i64; 12]; 12],
    dodec_cache: &mut [Option<i64>; 2000],
) -> i64 {
    if a == 0 {
        return hex[b as usize];
    }
    if b == 0 {
        return hex[a as usize];
    }

    let key = a as usize * 100 + b as usize * 4 + (allow_a as usize) * 2 + allow_b as usize;
    if let Some(v) = dodec_cache[key] {
        return v;
    }

    let mut res = 0i64;
    let m = MOD as i64;

    if allow_a {
        for h in 1..=b {
            let t6 = pow6(trap[b as usize][h as usize]);
            let sub = tilings_for_dodecagon(a, b - h, false, true, hex, trap, dodec_cache);
            res = (res + t6 * sub) % m;
        }
    }

    if allow_b {
        for h in 1..=a {
            let t6 = pow6(trap[a as usize][h as usize]);
            let sub = tilings_for_dodecagon(a - h, b, true, false, hex, trap, dodec_cache);
            res = (res + t6 * sub) % m;
        }
    }

    if a == 1 && b == 1 {
        res = (res + 1) % m;
    }

    dodec_cache[key] = Some(res);
    res
}

fn main() {
    let n = N as usize;
    let hex: Vec<i64> = (0..n + 1)
        .into_par_iter()
        .map(|s| compute_hexagon(s as i32))
        .collect();

    let mut trap = [[1i64; 12]; 12];
    let trap_vals: Vec<(usize, usize, i64)> = (1..=n)
        .into_par_iter()
        .flat_map(|base| {
            (1..=n).into_par_iter().map(move |height| {
                (base, height, compute_trapezoid(base as i32, height as i32))
            })
        })
        .collect();
    for (base, height, val) in trap_vals {
        trap[base][height] = val;
    }

    let mut dodec_cache = [None; 2000];
    let ans = tilings_for_dodecagon(N, N, true, true, &hex, &trap, &mut dodec_cache);
    println!("{}", ans);
}
