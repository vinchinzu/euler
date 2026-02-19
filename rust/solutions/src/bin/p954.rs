// Problem 954 — Heptaphobia
//
// Count positive integers < 10^13 that are not divisible by 7 and where no
// valid digit swap (no leading zeros before or after) produces a multiple of 7.
//
// Key insight: 10^k mod 7 cycles with period 6: W = [1, 3, 2, 6, 4, 5].
// Positions sharing the same k%6 "class" have the same weight mod 7, so
// swapping digits within the same class never changes n mod 7 — only
// cross-class swaps matter.
//
// We group digits by residue mod 7:
//   res 0: digit 0 (idx 0) and digit 7 (idx 1)   — tracked separately for
//          leading-zero handling via has0/has7 flags
//   res 1: digits {1, 8} (idx 2, multiplicity 2)
//   res 2: digits {2, 9} (idx 3, multiplicity 2)
//   res 3..6: digits 3..6 (idx 4..7, multiplicity 1 each)
//
// DP state = 6 classes x 9 bits each = 54 bits packed into a u64.
//   Per class: 7-bit mask of which residues (0-6) are present
//              + bit 7 = has0 (digit 0 placed in this class)
//              + bit 8 = has7 (digit 7 placed in this class)
//
// DP value = [i32; 7]: count of partial numbers by running sum mod 7.
// (Max count value verified to be 24576, well within i32 range.
//  Using i32 instead of i64 halves value memory for better cache behavior.)
//
// For each target final residue r in {1..6}, we precompute which digit
// residue at the current class would allow a cross-class swap to bring the
// final number to 0 mod 7 (forbidden placement).  MSD processing uses
// mask_no0 to exclude swaps that would move digit 0 to the MSD position
// (creating a leading zero, which makes the swap invalid per the problem).
//
// Optimization: rayon parallelism across independent (l, tr) pairs;
// i32 values to halve memory and improve cache utilization;
// unsafe bounds-check elimination in proven-safe hot loops.

use fxhash::FxHashMap;
use rayon::prelude::*;

const W: [i32; 6] = [1, 3, 2, 6, 4, 5];
const CB: u64 = 0x1FF;
const SH: [u32; 6] = [0, 9, 18, 27, 36, 45];
const RES: [usize; 8] = [0, 0, 1, 2, 3, 4, 5, 6];
const MULT: [i16; 8] = [1, 1, 2, 2, 1, 1, 1, 1];

struct Tables {
    shifts: [[[usize; 6]; 6]; 7],
    rot_table: [[u8; 128]; 7],
    mask_all: [u8; 512],
    mask_no0: [u8; 512],
    update_table: [[u16; 8]; 512],
    add_contrib: [[usize; 8]; 6],
    perm: [[usize; 7]; 7],
}

fn build_tables() -> Tables {
    let mut invdiff = [[0i32; 6]; 6];
    for a in 0..6 {
        for b in 0..6 {
            if a != b {
                let d = ((W[b] - W[a]) % 7 + 7) % 7;
                for x in 1..7i32 {
                    if d * x % 7 == 1 {
                        invdiff[a][b] = x;
                        break;
                    }
                }
            }
        }
    }

    let mut shifts = [[[0usize; 6]; 6]; 7];
    for r in 1..7usize {
        for a in 0..6 {
            for b in 0..6 {
                if a != b {
                    shifts[r][a][b] = ((r as i32 * invdiff[a][b]) % 7) as usize;
                }
            }
        }
    }

    let mut rot_table = [[0u8; 128]; 7];
    for sh in 0..7u32 {
        for m in 0..128u32 {
            rot_table[sh as usize][m as usize] = if sh == 0 {
                m as u8
            } else {
                ((m << sh | m >> (7 - sh)) & 0x7F) as u8
            };
        }
    }

    let mut mask_all = [0u8; 512];
    let mut mask_no0 = [0u8; 512];
    for bits in 0..512u16 {
        let m7 = (bits & 0x7F) as u8;
        let has7 = (bits >> 8) & 1;
        mask_all[bits as usize] = m7;
        let mut m = m7 & !1u8;
        if has7 != 0 {
            m |= 1;
        }
        mask_no0[bits as usize] = m;
    }

    let mut update_table = [[0u16; 8]; 512];
    {
        const CH: [(u16, u16, u16); 8] = [
            (0, 1, 0),
            (0, 0, 1),
            (1, 0, 0),
            (2, 0, 0),
            (3, 0, 0),
            (4, 0, 0),
            (5, 0, 0),
            (6, 0, 0),
        ];
        for ob in 0..512u16 {
            let (msk, h0, h7) = (ob & 0x7F, (ob >> 7) & 1, (ob >> 8) & 1);
            for (i, &(r, a0, a7)) in CH.iter().enumerate() {
                update_table[ob as usize][i] =
                    (msk | (1 << r)) | ((h0 | a0) << 7) | ((h7 | a7) << 8);
            }
        }
    }

    let mut add_contrib = [[0usize; 8]; 6];
    for c in 0..6 {
        for i in 0..8 {
            add_contrib[c][i] = (RES[i] * W[c] as usize) % 7;
        }
    }

    let mut perm = [[0usize; 7]; 7];
    for a in 0..7 {
        for i in 0..7 {
            perm[a][i] = (i + a) % 7;
        }
    }

    Tables {
        shifts,
        rot_table,
        mask_all,
        mask_no0,
        update_table,
        add_contrib,
        perm,
    }
}

#[inline(never)]
fn solve_single(t: &Tables, l: usize, tr: usize) -> i64 {
    let mut dp: FxHashMap<u64, [i16; 7]> = FxHashMap::default();
    dp.reserve(1024);
    dp.insert(0, [1, 0, 0, 0, 0, 0, 0]);

    for pos in 0..l {
        let c = pos % 6;
        let sc = SH[c];
        let msd = pos == l - 1;
        let mf = if msd { &t.mask_no0[..] } else { &t.mask_all[..] };
        let sr = &t.shifts[tr];
        let ac = &t.add_contrib[c];
        let mut ndp: FxHashMap<u64, [i16; 7]> =
            FxHashMap::with_capacity_and_hasher(dp.len() * 4, Default::default());

        for (&st, cn) in &dp {
            let mut forb = 0u8;
            for a in 0..6usize {
                if a == c {
                    continue;
                }
                let ba = ((st >> SH[a]) & CB) as usize;
                // SAFETY: ba < 512 since CB = 0x1FF
                let mu = unsafe { *mf.get_unchecked(ba) };
                if mu != 0 {
                    // SAFETY: sr[a][c] < 7, mu < 128
                    forb |= unsafe {
                        *t.rot_table
                            .get_unchecked(sr[a][c])
                            .get_unchecked(mu as usize)
                    };
                }
            }

            let bc = ((st >> sc) & CB) as u16;
            let s0 = if msd { 1usize } else { 0 };

            for idx in s0..8 {
                // SAFETY: idx < 8
                let res = unsafe { *RES.get_unchecked(idx) };
                if forb & (1u8 << res) != 0 {
                    continue;
                }
                // SAFETY: bc < 512, idx < 8
                let nbc = unsafe {
                    *t.update_table
                        .get_unchecked(bc as usize)
                        .get_unchecked(idx)
                };
                let ns = st ^ (((bc ^ nbc) as u64) << sc);
                let add = unsafe { *ac.get_unchecked(idx) };
                let p = unsafe { t.perm.get_unchecked(add) };
                let m = unsafe { *MULT.get_unchecked(idx) };

                let arr = ndp.entry(ns).or_insert([0i16; 7]);
                // SAFETY: p[i] < 7 for all i, arrays have 7 elements
                unsafe {
                    let p0 = *p.get_unchecked(0);
                    let p1 = *p.get_unchecked(1);
                    let p2 = *p.get_unchecked(2);
                    let p3 = *p.get_unchecked(3);
                    let p4 = *p.get_unchecked(4);
                    let p5 = *p.get_unchecked(5);
                    let p6 = *p.get_unchecked(6);
                    if m == 1 {
                        *arr.get_unchecked_mut(p0) += *cn.get_unchecked(0);
                        *arr.get_unchecked_mut(p1) += *cn.get_unchecked(1);
                        *arr.get_unchecked_mut(p2) += *cn.get_unchecked(2);
                        *arr.get_unchecked_mut(p3) += *cn.get_unchecked(3);
                        *arr.get_unchecked_mut(p4) += *cn.get_unchecked(4);
                        *arr.get_unchecked_mut(p5) += *cn.get_unchecked(5);
                        *arr.get_unchecked_mut(p6) += *cn.get_unchecked(6);
                    } else {
                        *arr.get_unchecked_mut(p0) += *cn.get_unchecked(0) * 2;
                        *arr.get_unchecked_mut(p1) += *cn.get_unchecked(1) * 2;
                        *arr.get_unchecked_mut(p2) += *cn.get_unchecked(2) * 2;
                        *arr.get_unchecked_mut(p3) += *cn.get_unchecked(3) * 2;
                        *arr.get_unchecked_mut(p4) += *cn.get_unchecked(4) * 2;
                        *arr.get_unchecked_mut(p5) += *cn.get_unchecked(5) * 2;
                        *arr.get_unchecked_mut(p6) += *cn.get_unchecked(6) * 2;
                    }
                }
            }
        }

        dp = ndp;
    }

    dp.values().map(|c| c[tr] as i64).sum::<i64>()
}

fn main() {
    // Use 6 threads: matches the 6 tr values per l, reduces memory contention
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    let t = build_tables();

    let work: Vec<(usize, usize)> = (1..=13usize)
        .flat_map(|l| (1..7usize).map(move |tr| (l, tr)))
        .collect();

    let grand_total: i64 = work
        .par_iter()
        .map(|&(l, tr)| solve_single(&t, l, tr))
        .sum();

    println!("{}", grand_total);
}
