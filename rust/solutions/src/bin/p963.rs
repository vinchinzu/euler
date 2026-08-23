// Project Euler 963 - Base-3 combinatorics
// F(N): count (L, parity, B) records for 1..=N, pair them, sum v^2 / 4.
// L tuples are short (len <= 9, values <= 8 for N=1e5); pack into u64 and intern.

use fxhash::FxHashMap;
use rayon::prelude::*;

const LPACK_LEN_BITS: u32 = 8;

#[inline(always)]
fn pack_l(vals: &[u8], n: usize) -> u64 {
    let mut p = n as u64;
    for i in 0..n {
        p |= (vals[i] as u64) << (LPACK_LEN_BITS + 4 * i as u32);
    }
    p
}

#[inline(always)]
fn merge_pack(a: u64, b: u64) -> u128 {
    let na = (a & 0xff) as usize;
    let nb = (b & 0xff) as usize;
    let mut buf = [0u8; 20];
    let mut xa = a >> LPACK_LEN_BITS;
    for i in 0..na {
        buf[i] = (xa & 0xf) as u8;
        xa >>= 4;
    }
    let mut xb = b >> LPACK_LEN_BITS;
    for i in 0..nb {
        buf[na + i] = (xb & 0xf) as u8;
        xb >>= 4;
    }
    let n = na + nb;
    buf[..n].sort_unstable();
    let mut p = n as u128;
    for i in 0..n {
        p |= (buf[i] as u128) << (LPACK_LEN_BITS + 4 * i as u32);
    }
    p
}

// merged_id: 15 bits, parity: 1 bit, b_sum: 16 bits (N=1e5: b_sum <= 45056)
#[inline(always)]
fn d_key(mid: u16, parity: u8, bsum: u32) -> u32 {
    ((mid as u32) << 17) | ((parity as u32) << 16) | bsum
}

#[derive(Clone, Copy)]
struct Ent {
    l: usize,
    p: u8,
    b: u32,
    c: u64,
}

fn compute_f(n: usize) -> u128 {
    let mut m_exp = 0u32;
    {
        let mut val = 1usize;
        while val < n.max(1) {
            val *= 3;
            m_exp += 1;
        }
    }
    let m = 1usize << m_exp;
    let scale = 1i64 << m_exp;

    let mut b_int = vec![0i64; m + 1];
    for i in 1..=m {
        let h = b_int[i >> 1];
        if i & 1 == 1 {
            b_int[i] = h + scale;
        } else if h < 2 * scale {
            b_int[i] = h / 2;
        } else {
            b_int[i] = h - scale;
        }
    }

    let mut counter: FxHashMap<u64, u64> =
        FxHashMap::with_capacity_and_hasher(4096, Default::default());
    let mut digits = [0u8; 16];
    let mut lvals = [0u8; 16];

    for n_val in 1..=n {
        let mut nd = 0usize;
        let mut x = n_val;
        while x > 0 {
            digits[nd] = (x % 3) as u8;
            nd += 1;
            x /= 3;
        }
        digits[..nd].reverse();

        let mut last_zero: i32 = -1;
        for j in 0..nd {
            if digits[j] == 1 {
                break;
            }
            if digits[j] == 0 {
                last_zero = j as i32;
            }
        }

        let mut ln = 0usize;
        if last_zero >= 0 {
            let mut k = 0u8;
            for j in (0..last_zero as usize).rev() {
                if digits[j] == 0 {
                    k += 1;
                } else {
                    lvals[ln] = k;
                    ln += 1;
                }
            }
        }

        let mut u_val = 0usize;
        let mut count2 = 0u8;
        for &c in &digits[..nd] {
            if c < 2 {
                u_val = (u_val << 1) | c as usize;
            } else {
                count2 += 1;
            }
        }

        let lpack = pack_l(&lvals, ln);
        let parity = count2 & 1;
        let b = b_int[u_val] as u32;
        // lpack < 2^44 for N=1e5; parity 1 bit; b < 2^15
        let key = lpack | ((parity as u64) << 44) | ((b as u64) << 45);
        *counter.entry(key).or_insert(0) += 1;
    }

    let mut l_intern: FxHashMap<u64, u16> =
        FxHashMap::with_capacity_and_hasher(512, Default::default());
    let mut l_packs: Vec<u64> = Vec::with_capacity(512);
    let mut entries: Vec<Ent> = Vec::with_capacity(counter.len());
    for (&key, &cnt) in &counter {
        let lpack = key & ((1u64 << 44) - 1);
        let p = ((key >> 44) & 1) as u8;
        let b = (key >> 45) as u32;
        let lid = if let Some(&id) = l_intern.get(&lpack) {
            id
        } else {
            let id = l_packs.len() as u16;
            l_intern.insert(lpack, id);
            l_packs.push(lpack);
            id
        };
        entries.push(Ent {
            l: lid as usize,
            p,
            b,
            c: cnt,
        });
    }

    let n_l = l_packs.len();
    let mut merge_intern: FxHashMap<u128, u16> =
        FxHashMap::with_capacity_and_hasher(20_000, Default::default());
    let mut merge_ids = vec![0u16; n_l * n_l];
    for i in 0..n_l {
        let base = i * n_l;
        let pa = l_packs[i];
        for j in 0..n_l {
            let packed = merge_pack(pa, l_packs[j]);
            let id = if let Some(&id) = merge_intern.get(&packed) {
                id
            } else {
                let id = merge_intern.len() as u16;
                merge_intern.insert(packed, id);
                id
            };
            merge_ids[base + j] = id;
        }
    }

    let nent = entries.len();
    let mut pairs = Vec::with_capacity(nent * (nent + 1) / 2);
    for i in 0..nent {
        let ei = entries[i];
        let base = ei.l * n_l;
        // SAFETY: interned L ids are in 0..n_l
        let mid_ii = unsafe { *merge_ids.get_unchecked(base + ei.l) };
        pairs.push((d_key(mid_ii, 0, ei.b + ei.b), ei.c.wrapping_mul(ei.c + 1)));
        for j in (i + 1)..nent {
            let ej = entries[j];
            let mid = unsafe { *merge_ids.get_unchecked(base + ej.l) };
            pairs.push((
                d_key(mid, ei.p ^ ej.p, ei.b + ej.b),
                ei.c.wrapping_mul(ej.c) << 1,
            ));
        }
    }

    pairs.par_sort_unstable();

    let mut result = 0u128;
    let mut i = 0usize;
    let plen = pairs.len();
    while i < plen {
        let k = pairs[i].0;
        let mut s = 0u64;
        while i < plen && pairs[i].0 == k {
            s = s.wrapping_add(pairs[i].1);
            i += 1;
        }
        let sq = s as u128;
        result += sq * sq;
    }
    result / 4
}

fn main() {
    debug_assert_eq!(compute_f(5), 21);
    println!("{}", compute_f(100_000));
}
