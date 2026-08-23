// Project Euler 592 - Factorial Trailing Hex Digits
//
// Find the last 12 hexadecimal digits before trailing zeros in (20!)!.
// Uses baby-step/giant-step with polynomial interpolation for
// product of odd numbers mod 2^48.
//
// mulmod is wrapping u64: (a*b) mod 2^48 = (a*b mod 2^64) mod 2^48.
// f(a) samples are independent of r and are computed once, in parallel.

use rayon::prelude::*;

const NBITS: u32 = 48;
const MOD: u64 = 1u64 << NBITS;
const HALF_MOD: u64 = 1u64 << (NBITS - 1);
const MASK: u64 = MOD - 1;
const POLY_DEG: usize = 26;
const BLOCK_BITS: u32 = 22;
const BLOCK_SIZE: u64 = 1u64 << BLOCK_BITS;

/// Product of `count` consecutive odd numbers starting at 2*base+1, mod 2^48.
#[inline]
fn product_odds_range(base: u64, count: u64) -> u64 {
    let mut prod = 1u64;
    let mut odd = (base << 1).wrapping_add(1);
    let mut n = count;
    while n >= 8 {
        prod = prod.wrapping_mul(odd);
        odd = odd.wrapping_add(2);
        prod = prod.wrapping_mul(odd);
        odd = odd.wrapping_add(2);
        prod = prod.wrapping_mul(odd);
        odd = odd.wrapping_add(2);
        prod = prod.wrapping_mul(odd);
        odd = odd.wrapping_add(2);
        prod = prod.wrapping_mul(odd);
        odd = odd.wrapping_add(2);
        prod = prod.wrapping_mul(odd);
        odd = odd.wrapping_add(2);
        prod = prod.wrapping_mul(odd);
        odd = odd.wrapping_add(2);
        prod = prod.wrapping_mul(odd);
        odd = odd.wrapping_add(2);
        n -= 8;
    }
    for _ in 0..n {
        prod = prod.wrapping_mul(odd);
        odd = odd.wrapping_add(2);
    }
    prod & MASK
}

fn precompute_f_vals() -> [u64; POLY_DEG + 1] {
    let tmp: Vec<u64> = (0..POLY_DEG + 1)
        .into_par_iter()
        .with_min_len(1)
        .map(|a| product_odds_range((a as u64) * BLOCK_SIZE, BLOCK_SIZE))
        .collect();
    tmp.try_into().expect("POLY_DEG+1 samples")
}

/// One Newton step: new_d[k] = d[k] + d[k+1] (mod 2^48 via wrapping).
#[inline(always)]
fn advance_diffs(d: &mut [u64; POLY_DEG + 1]) {
    d[0] = d[0].wrapping_add(d[1]);
    d[1] = d[1].wrapping_add(d[2]);
    d[2] = d[2].wrapping_add(d[3]);
    d[3] = d[3].wrapping_add(d[4]);
    d[4] = d[4].wrapping_add(d[5]);
    d[5] = d[5].wrapping_add(d[6]);
    d[6] = d[6].wrapping_add(d[7]);
    d[7] = d[7].wrapping_add(d[8]);
    d[8] = d[8].wrapping_add(d[9]);
    d[9] = d[9].wrapping_add(d[10]);
    d[10] = d[10].wrapping_add(d[11]);
    d[11] = d[11].wrapping_add(d[12]);
    d[12] = d[12].wrapping_add(d[13]);
    d[13] = d[13].wrapping_add(d[14]);
    d[14] = d[14].wrapping_add(d[15]);
    d[15] = d[15].wrapping_add(d[16]);
    d[16] = d[16].wrapping_add(d[17]);
    d[17] = d[17].wrapping_add(d[18]);
    d[18] = d[18].wrapping_add(d[19]);
    d[19] = d[19].wrapping_add(d[20]);
    d[20] = d[20].wrapping_add(d[21]);
    d[21] = d[21].wrapping_add(d[22]);
    d[22] = d[22].wrapping_add(d[23]);
    d[23] = d[23].wrapping_add(d[24]);
    d[24] = d[24].wrapping_add(d[25]);
    d[25] = d[25].wrapping_add(d[26]);
}

fn main() {
    let n: u64 = 2_432_902_008_176_640_000;

    let mut rs = Vec::with_capacity(64);
    let mut cur = n;
    while cur > 1 {
        let r = (cur + 1) / 2;
        rs.push(r & (HALF_MOD - 1));
        cur /= 2;
    }

    let f_vals = precompute_f_vals();

    let mut deltas = [0u64; POLY_DEG + 1];
    deltas[0] = f_vals[0];
    let mut work = f_vals;
    for k in 0..POLY_DEG {
        for i in 0..POLY_DEG - k {
            work[i] = work[i + 1].wrapping_sub(work[i]);
        }
        deltas[k + 1] = work[0];
    }

    // Snapshot block-products at the q values we actually need (one eval pass).
    let mut items: Vec<(u64, usize)> = rs
        .iter()
        .enumerate()
        .filter_map(|(i, &r)| {
            if r > BLOCK_SIZE && r > POLY_DEG as u64 + 2 {
                Some((r / BLOCK_SIZE, i))
            } else {
                None
            }
        })
        .collect();
    items.sort_unstable_by_key(|&(q, _)| q);

    let mut block_prod = vec![1u64; rs.len()];
    let mut d = deltas;
    let mut prod = 1u64;
    let mut a = 0u64;
    for (q, idx) in items {
        while a < q {
            prod = prod.wrapping_mul(d[0]);
            advance_diffs(&mut d);
            a += 1;
        }
        block_prod[idx] = prod & MASK;
    }

    let odd_part = (0..rs.len())
        .into_par_iter()
        .with_min_len(1)
        .map(|i| {
            let r = rs[i];
            if r <= 1 {
                1
            } else if r <= BLOCK_SIZE || r <= POLY_DEG as u64 + 2 {
                product_odds_range(0, r)
            } else {
                let q = r / BLOCK_SIZE;
                let remainder = r % BLOCK_SIZE;
                let mut p = block_prod[i];
                if remainder > 0 {
                    p = p.wrapping_mul(product_odds_range(q * BLOCK_SIZE, remainder));
                }
                p & MASK
            }
        })
        .reduce(|| 1u64, |x, y| x.wrapping_mul(y) & MASK);

    let mut v2: u64 = 0;
    let mut t = n;
    while t > 1 {
        t /= 2;
        v2 += t;
    }

    let answer = odd_part.wrapping_mul(1u64 << (v2 % 4)) & MASK;
    println!("{:012X}", answer);
}
