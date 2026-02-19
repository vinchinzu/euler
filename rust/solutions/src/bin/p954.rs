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
// DP state = 6 classes × 9 bits each = 54 bits packed into a u64.
//   Per class: 7-bit mask of which residues (0–6) are present
//              + bit 7 = has0 (digit 0 placed in this class)
//              + bit 8 = has7 (digit 7 placed in this class)
//
// DP value = [i64; 7]: count of partial numbers by running sum mod 7.
//
// For each target final residue r ∈ {1..6}, we precompute which digit
// residue at the current class would allow a cross-class swap to bring the
// final number to 0 mod 7 (forbidden placement).  MSD processing uses
// mask_no0 to exclude swaps that would move digit 0 to the MSD position
// (creating a leading zero, which makes the swap invalid per the problem).

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

// Fast hasher for u64 keys (avoids SipHash overhead)
#[derive(Default)]
struct FxHasher(u64);

impl Hasher for FxHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ b as u64).wrapping_mul(0x517cc1b727220a95);
        }
    }
    #[inline]
    fn write_u64(&mut self, n: u64) {
        // Fibonacci hashing — good distribution for integer keys
        self.0 = n.wrapping_mul(0x517cc1b727220a95);
    }
}

type FxBuildHasher = BuildHasherDefault<FxHasher>;
type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

fn main() {
    // W[k] = 10^k mod 7, period 6
    const W: [i32; 6] = [1, 3, 2, 6, 4, 5];
    const CB: u64 = 0x1FF; // 9-bit class-bits mask
    const SH: [u32; 6] = [0, 9, 18, 27, 36, 45];

    // invdiff[a][b] = modular inverse of (W[b] - W[a]) mod 7
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

    // shifts[r][a][c] = r * invdiff[a][c] % 7
    // Determines the rotation applied to class a's residue mask to compute
    // which residues at class c are forbidden.
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

    // rot_table[sh][m]: circular left rotation of 7-bit mask m by sh positions
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

    // mask_all[bits]: full 7-bit residue mask from 9-bit class config
    // mask_no0[bits]: residue mask excluding digit 0 but keeping digit 7
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

    // 8 abstract digit choices
    const RES: [usize; 8] = [0, 0, 1, 2, 3, 4, 5, 6];
    const MULT: [i64; 8] = [1, 1, 2, 2, 1, 1, 1, 1];

    // update_table[oldbits][idx] = new 9-bit class config after placing choice idx
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

    // add_contrib[c][idx] = (RES[idx] * W[c]) % 7
    let mut add_contrib = [[0usize; 8]; 6];
    for c in 0..6 {
        for i in 0..8 {
            add_contrib[c][i] = (RES[i] * W[c] as usize) % 7;
        }
    }

    // perm[add][i] = (i + add) % 7
    let mut perm = [[0usize; 7]; 7];
    for a in 0..7 {
        for i in 0..7 {
            perm[a][i] = (i + a) % 7;
        }
    }

    let mut grand_total: i64 = 0;

    for l in 1..=13usize {
        for tr in 1..7usize {
            let mut dp: FxHashMap<u64, [i64; 7]> = FxHashMap::default();
            dp.insert(0, [1, 0, 0, 0, 0, 0, 0]);

            for pos in 0..l {
                let c = pos % 6;
                let sc = SH[c];
                let msd = pos == l - 1;
                let mf = if msd { &mask_no0[..] } else { &mask_all[..] };
                let sr = &shifts[tr];
                let mut ndp: FxHashMap<u64, [i64; 7]> =
                    FxHashMap::with_capacity_and_hasher(dp.len() * 4, FxBuildHasher::default());

                for (&st, cn) in &dp {
                    let mut forb = 0u8;
                    for a in 0..6usize {
                        if a == c {
                            continue;
                        }
                        let ba = ((st >> SH[a]) & CB) as usize;
                        let mu = mf[ba];
                        if mu != 0 {
                            forb |= rot_table[sr[a][c]][mu as usize];
                        }
                    }

                    let bc = ((st >> sc) & CB) as u16;
                    let s0 = if msd { 1usize } else { 0 };

                    for idx in s0..8 {
                        if forb & (1u8 << RES[idx]) != 0 {
                            continue;
                        }
                        let nbc = update_table[bc as usize][idx];
                        let ns = st ^ (((bc ^ nbc) as u64) << sc);
                        let p = &perm[add_contrib[c][idx]];
                        let m = MULT[idx];

                        let arr = ndp.entry(ns).or_insert([0i64; 7]);
                        if m == 1 {
                            arr[p[0]] += cn[0];
                            arr[p[1]] += cn[1];
                            arr[p[2]] += cn[2];
                            arr[p[3]] += cn[3];
                            arr[p[4]] += cn[4];
                            arr[p[5]] += cn[5];
                            arr[p[6]] += cn[6];
                        } else {
                            arr[p[0]] += cn[0] << 1;
                            arr[p[1]] += cn[1] << 1;
                            arr[p[2]] += cn[2] << 1;
                            arr[p[3]] += cn[3] << 1;
                            arr[p[4]] += cn[4] << 1;
                            arr[p[5]] += cn[5] << 1;
                            arr[p[6]] += cn[6] << 1;
                        }
                    }
                }

                dp = ndp;
            }

            grand_total += dp.values().map(|c| c[tr]).sum::<i64>();
        }

        if l == 2 {
            assert_eq!(grand_total, 74, "C(100) should be 74");
        }
        if l == 4 {
            assert_eq!(grand_total, 3737, "C(10^4) should be 3737");
        }
    }

    println!("{}", grand_total);
}
