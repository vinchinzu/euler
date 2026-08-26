// Project Euler 814 - Mezzo-forte
// Count configurations of 4N people in a circle with exactly N mutual pairs.

use rayon::prelude::*;

const MOD: u64 = 998244353;
const N: usize = 1000;
const KLEN: usize = N + 3;

type Layer = [[u64; KLEN]; 4];

/// One DP step: dest[na][nb][k] gathers the 9 (da,db) transitions from prev.
/// Negative k-offsets are 0. Coefficients are the combined multiplicities.
fn dp_step(prev: &Layer, cur: &mut Layer) {
    // SAFETY: k in 0..=N, KLEN = N+3 so k, k-1, k-2 are in-bounds.
    unsafe {
        let s00 = prev[0].as_ptr();
        let s01 = prev[1].as_ptr();
        let s10 = prev[2].as_ptr();
        let s11 = prev[3].as_ptr();
        let c00 = cur[0].as_mut_ptr();
        let c01 = cur[1].as_mut_ptr();
        let c10 = cur[2].as_mut_ptr();
        let c11 = cur[3].as_mut_ptr();

        let a0 = *s00;
        let b0 = *s01;
        let c0 = *s10;
        let d0 = *s11;
        *c00 = 3 * a0 + b0 + c0;
        *c01 = 2 * a0 + 2 * b0 + c0 + d0;
        *c10 = 2 * a0 + b0 + 2 * c0 + d0;
        *c11 = a0 + b0 + c0 + d0;

        let a1 = *s00.add(1);
        let b1 = *s01.add(1);
        let c1 = *s10.add(1);
        let d1 = *s11.add(1);
        *c00.add(1) = 3 * a1 + a0 + b1 + 3 * b0 + c1 + 3 * c0 + 3 * d0;
        *c01.add(1) = 2 * a1 + 2 * b1 + c1 + c0 + d1 + d0;
        *c10.add(1) = 2 * a1 + b1 + b0 + 2 * c1 + d1 + d0;
        *c11.add(1) = a1 + b1 + c1 + d1;

        for k in 2..=N {
            let a = *s00.add(k);
            let ap = *s00.add(k - 1);
            let b = *s01.add(k);
            let bp = *s01.add(k - 1);
            let c = *s10.add(k);
            let cp = *s10.add(k - 1);
            let d = *s11.add(k);
            let dp = *s11.add(k - 1);
            let dpp = *s11.add(k - 2);
            *c00.add(k) = 3 * a + ap + b + 3 * bp + c + 3 * cp + dpp + 3 * dp;
            *c01.add(k) = 2 * a + 2 * b + c + cp + d + dp;
            *c10.add(k) = 2 * a + b + bp + 2 * c + d + dp;
            *c11.add(k) = a + b + c + d;
        }
    }
}

fn reduce(layer: &mut Layer) {
    for row in layer.iter_mut() {
        for x in &mut row[..=N] {
            *x %= MOD;
        }
    }
}

fn solve(sa: usize, sb: usize) -> u64 {
    let mut prev = Box::new([[0u64; KLEN]; 4]);
    let mut cur = Box::new([[0u64; KLEN]; 4]);
    prev[(sa << 1) | sb][0] = 1;

    for step in 1..2 * N {
        dp_step(&prev, &mut cur);
        // Coeffs per cell sum to 16, so values grow at most 16× per step.
        // 16^6 * MOD < 2^64, so reduce every 6 steps.
        if step % 6 == 0 {
            reduce(&mut cur);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    reduce(&mut prev);

    let mut ans = 0u64;
    for a in 0..2usize {
        for b in 0..2usize {
            let src = &prev[(a << 1) | b];
            for da in 0..3usize {
                for db in 0..3usize {
                    if sa == da / 2 && sb == db / 2 {
                        let mut k = N as i32;
                        if da == 1 && db == 1 {
                            k -= 1;
                        }
                        if da == 0 && b == 1 {
                            k -= 1;
                        }
                        if db == 0 && a == 1 {
                            k -= 1;
                        }
                        if k >= 0 {
                            ans += src[k as usize];
                        }
                    }
                }
            }
        }
    }
    ans % MOD
}

fn main() {
    let ans = [(0usize, 0usize), (0, 1), (1, 0), (1, 1)]
        .into_par_iter()
        .map(|(sa, sb)| solve(sa, sb))
        .sum::<u64>()
        % MOD;
    println!("{ans}");
}
