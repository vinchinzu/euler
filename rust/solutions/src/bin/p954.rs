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
// DP value = [u16; 7]: count of partial numbers by running sum mod 7.
// Max stored count is 8192 (fits u16). States interned to dense Vec ids;
// open-addressing epoch table replaces HashMap in the inner loop.

use rayon::prelude::*;

const W: [i32; 6] = [1, 3, 2, 6, 4, 5];
const CB: u64 = 0x1FF;
const SH: [u32; 6] = [0, 9, 18, 27, 36, 45];
const RES: [usize; 8] = [0, 0, 1, 2, 3, 4, 5, 6];
const MULT: [u16; 8] = [1, 1, 2, 2, 1, 1, 1, 1];

const HBITS: usize = 22;
const HSIZE: usize = 1 << HBITS;
const HMASK: usize = HSIZE - 1;
const DENSE_CAP: usize = 1_500_000;

struct Tables {
    shifts: [[[usize; 6]; 6]; 7],
    forb_all: [[u8; 512]; 7],
    forb_no0: [[u8; 512]; 7],
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

    let mut forb_all = [[0u8; 512]; 7];
    let mut forb_no0 = [[0u8; 512]; 7];
    for bits in 0..512u16 {
        let m7 = (bits & 0x7F) as u8;
        let has7 = (bits >> 8) & 1;
        let mut m_no0 = m7 & !1u8;
        if has7 != 0 {
            m_no0 |= 1;
        }
        for sh in 0..7 {
            forb_all[sh][bits as usize] = rot_table[sh][m7 as usize];
            forb_no0[sh][bits as usize] = rot_table[sh][m_no0 as usize];
        }
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
        forb_all,
        forb_no0,
        update_table,
        add_contrib,
        perm,
    }
}

struct Intern {
    keys: Vec<u64>,
    ids: Vec<u32>,
    stamp: Vec<u32>,
    epoch: u32,
}

impl Intern {
    fn new() -> Self {
        let mut keys = Vec::<u64>::with_capacity(HSIZE);
        let mut ids = Vec::<u32>::with_capacity(HSIZE);
        unsafe {
            keys.set_len(HSIZE);
            ids.set_len(HSIZE);
        }
        Self {
            keys,
            ids,
            stamp: vec![0u32; HSIZE],
            epoch: 1,
        }
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.epoch = self.epoch.wrapping_add(1);
        if self.epoch == 0 {
            self.stamp.fill(0);
            self.epoch = 1;
        }
    }

    #[inline(always)]
    fn intern(&mut self, key: u64, out_keys: &mut Vec<u64>, out_vals: &mut Vec<[u16; 7]>) -> usize {
        let mut idx = (key.wrapping_mul(0x517cc1b727220a95) >> (64 - HBITS)) as usize;
        loop {
            if unsafe { *self.stamp.get_unchecked(idx) } != self.epoch {
                let id = out_keys.len();
                out_keys.push(key);
                out_vals.push([0u16; 7]);
                unsafe {
                    *self.keys.get_unchecked_mut(idx) = key;
                    *self.ids.get_unchecked_mut(idx) = id as u32;
                    *self.stamp.get_unchecked_mut(idx) = self.epoch;
                }
                return id;
            }
            if unsafe { *self.keys.get_unchecked(idx) } == key {
                return unsafe { *self.ids.get_unchecked(idx) as usize };
            }
            idx = (idx + 1) & HMASK;
        }
    }
}

#[inline(always)]
fn fold_forb(st: u64, c: usize, sr: &[[usize; 6]; 6], tab: &[[u8; 512]; 7]) -> u8 {
    let mut forb = 0u8;
    unsafe {
        for a in 0..6usize {
            if a != c {
                let ba = ((st >> *SH.get_unchecked(a)) & CB) as usize;
                let sh = *sr.get_unchecked(a).get_unchecked(c);
                forb |= *tab.get_unchecked(sh).get_unchecked(ba);
            }
        }
    }
    forb
}

#[inline(always)]
fn acc_cnt(arr: &mut [u16; 7], cn: &[u16; 7], p: &[usize; 7], m: u16) {
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
            *arr.get_unchecked_mut(p0) += *cn.get_unchecked(0) << 1;
            *arr.get_unchecked_mut(p1) += *cn.get_unchecked(1) << 1;
            *arr.get_unchecked_mut(p2) += *cn.get_unchecked(2) << 1;
            *arr.get_unchecked_mut(p3) += *cn.get_unchecked(3) << 1;
            *arr.get_unchecked_mut(p4) += *cn.get_unchecked(4) << 1;
            *arr.get_unchecked_mut(p5) += *cn.get_unchecked(5) << 1;
            *arr.get_unchecked_mut(p6) += *cn.get_unchecked(6) << 1;
        }
    }
}

#[inline(never)]
fn solve_all_lengths_for_tr(t: &Tables, max_l: usize, tr: usize) -> i64 {
    let mut intern = Intern::new();
    let mut keys_a: Vec<u64> = Vec::with_capacity(DENSE_CAP);
    let mut vals_a: Vec<[u16; 7]> = Vec::with_capacity(DENSE_CAP);
    let mut keys_b: Vec<u64> = Vec::with_capacity(DENSE_CAP);
    let mut vals_b: Vec<[u16; 7]> = Vec::with_capacity(DENSE_CAP);
    keys_a.push(0);
    vals_a.push([1, 0, 0, 0, 0, 0, 0]);

    let mut total_count = 0i64;

    for pos in 0..max_l {
        let c = pos % 6;
        let sc = SH[c];
        let sr = &t.shifts[tr];
        let ac = &t.add_contrib[c];
        let n = keys_a.len();

        // Part 1: Branch for MSD (treat pos as the final MSD digit)
        for i in 0..n {
            let st = unsafe { *keys_a.get_unchecked(i) };
            let cn = unsafe { *vals_a.get_unchecked(i) };
            let forb = fold_forb(st, c, sr, &t.forb_no0);

            for idx in 1..8 {
                let res = unsafe { *RES.get_unchecked(idx) };
                if forb & (1u8 << res) != 0 {
                    continue;
                }
                let add = unsafe { *ac.get_unchecked(idx) };
                let m = unsafe { *MULT.get_unchecked(idx) } as i64;
                let s_needed = (tr + 7 - add) % 7;
                total_count += m * (unsafe { *cn.get_unchecked(s_needed) } as i64);
            }
        }

        // Part 2: If pos + 1 < max_l, advance dp with non-MSD transitions
        if pos + 1 < max_l {
            intern.clear();
            keys_b.clear();
            vals_b.clear();

            for i in 0..n {
                let st = unsafe { *keys_a.get_unchecked(i) };
                let cn = unsafe { *vals_a.get_unchecked(i) };
                let forb = fold_forb(st, c, sr, &t.forb_all);

                let bc = ((st >> sc) & CB) as u16;
                for idx in 0..8 {
                    let res = unsafe { *RES.get_unchecked(idx) };
                    if forb & (1u8 << res) != 0 {
                        continue;
                    }
                    let nbc = unsafe {
                        *t.update_table
                            .get_unchecked(bc as usize)
                            .get_unchecked(idx)
                    };
                    let ns = st ^ (((bc ^ nbc) as u64) << sc);
                    let add = unsafe { *ac.get_unchecked(idx) };
                    let p = unsafe { t.perm.get_unchecked(add) };
                    let m = unsafe { *MULT.get_unchecked(idx) };
                    let id = intern.intern(ns, &mut keys_b, &mut vals_b);
                    let arr = unsafe { vals_b.get_unchecked_mut(id) };
                    acc_cnt(arr, &cn, p, m);
                }
            }
            std::mem::swap(&mut keys_a, &mut keys_b);
            std::mem::swap(&mut vals_a, &mut vals_b);
        }
    }

    total_count
}

fn main() {
    let t = build_tables();

    let grand_total: i64 = (1..7usize)
        .into_par_iter()
        .map(|tr| solve_all_lengths_for_tr(&t, 13, tr))
        .sum();

    println!("{}", grand_total);
}
