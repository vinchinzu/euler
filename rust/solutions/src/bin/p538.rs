// Project Euler 538 - Maximum Quadrilaterals
//
// Distinct u_n is small (popcount triples). Maintain multiplicities plus a
// sorted list of active unique indices; each insert only checks the <=4
// consecutive windows that contain the new element.

use rayon::prelude::*;

const N: usize = 3_000_000;

const POW3: [i64; 32] = {
    let mut a = [1i64; 32];
    let mut i = 1;
    while i < 32 {
        a[i] = a[i - 1] * 3;
        i += 1;
    }
    a
};

#[inline]
fn u_of(n: usize) -> i64 {
    let n = n as u64;
    (1i64 << (3 * n).count_ones())
        + POW3[n.count_ones() as usize]
        + (n + 1).count_ones() as i64
}

/// 256-bit product (hi, lo) of two u128 values.
#[inline]
fn mul_wide(x: u128, y: u128) -> (u128, u128) {
    const MASK: u128 = 0xFFFF_FFFF_FFFF_FFFF;
    let x_lo = x & MASK;
    let x_hi = x >> 64;
    let y_lo = y & MASK;
    let y_hi = y >> 64;
    let p0 = x_lo * y_lo;
    let p1 = x_lo * y_hi;
    let p2 = x_hi * y_lo;
    let p3 = x_hi * y_hi;
    let mid = (p0 >> 64) + (p1 & MASK) + (p2 & MASK);
    let lo = (p0 & MASK) | (mid << 64);
    let hi = p3 + (p1 >> 64) + (p2 >> 64) + (mid >> 64);
    (hi, lo)
}

fn main() {
    let mut uniq: Vec<i64> = (1..N + 1).into_par_iter().map(u_of).collect();
    uniq.par_sort_unstable();
    uniq.dedup();
    uniq.shrink_to_fit();

    let idxs: Vec<u32> = (1..N + 1)
        .into_par_iter()
        .map(|n| uniq.binary_search(&u_of(n)).unwrap() as u32)
        .collect();

    let m = uniq.len();
    let mut counts = vec![0u32; m];
    let mut active: Vec<u32> = Vec::with_capacity(m);

    let mut best_hi = 0u128;
    let mut best_lo = 0u128;
    let mut best_per = 0i64;
    let mut best_min_side = 0i64;
    let mut ans = 0i64;

    for n in 1..=N {
        // SAFETY: n in 1..=N
        let idx = unsafe { *idxs.get_unchecked(n - 1) } as usize;
        let v = unsafe { *uniq.get_unchecked(idx) };
        let c_before = unsafe { *counts.get_unchecked(idx) };

        let pos = active.partition_point(|&i| (i as usize) < idx);
        if c_before == 0 {
            active.insert(pos, idx as u32);
        }
        unsafe { *counts.get_unchecked_mut(idx) = c_before + 1; }

        if n < 4 {
            continue;
        }

        if v >= best_min_side {
            // New copy is last among equals. Collect up to 3 pred + 3 succ.
            let mut around = [0i64; 7];
            let mut alen = 0usize;

            let mut left = [0i64; 3];
            let mut nl = 0usize;
            let take_same = c_before.min(3);
            for _ in 0..take_same {
                left[nl] = v;
                nl += 1;
            }
            let mut q = pos;
            while nl < 3 && q > 0 {
                q -= 1;
                let idx2 = unsafe { *active.get_unchecked(q) } as usize;
                let v2 = unsafe { *uniq.get_unchecked(idx2) };
                let take = unsafe { *counts.get_unchecked(idx2) }.min((3 - nl) as u32);
                for _ in 0..take {
                    left[nl] = v2;
                    nl += 1;
                }
            }
            for i in (0..nl).rev() {
                around[alen] = left[i];
                alen += 1;
            }
            let pnew = alen;
            around[alen] = v;
            alen += 1;

            let mut q = pos + 1;
            let alen_cap = pnew + 1 + 3;
            while alen < alen_cap && q < active.len() {
                let idx2 = unsafe { *active.get_unchecked(q) } as usize;
                let v2 = unsafe { *uniq.get_unchecked(idx2) };
                let take = unsafe { *counts.get_unchecked(idx2) }.min((alen_cap - alen) as u32);
                for _ in 0..take {
                    around[alen] = v2;
                    alen += 1;
                }
                q += 1;
            }

            let pnew = pnew as i32;
            for delta in 0..4 {
                let s = pnew - 3 + delta;
                if s < 0 {
                    continue;
                }
                let s = s as usize;
                if s + 4 > alen {
                    continue;
                }
                let a = around[s];
                let b = around[s + 1];
                let c = around[s + 2];
                let d = around[s + 3];
                if d >= a + b + c {
                    continue;
                }
                let p = a + b + c + d;
                let w0 = (p - 2 * a) as u128;
                let w1 = (p - 2 * b) as u128;
                let w2 = (p - 2 * c) as u128;
                let w3 = (p - 2 * d) as u128;
                let (hi, lo) = mul_wide(w0 * w1, w2 * w3);
                if hi > best_hi
                    || (hi == best_hi && lo > best_lo)
                    || (hi == best_hi && lo == best_lo && p > best_per)
                {
                    best_hi = hi;
                    best_lo = lo;
                    best_per = p;
                    best_min_side = a;
                }
            }
        }

        ans += best_per;
    }

    println!("{ans}");
}
