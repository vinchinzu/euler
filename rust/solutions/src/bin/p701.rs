// Project Euler 701 - Random Connected Area
//
// Expected maximum area of contiguous black cells in an NxN grid.
// Cell-by-cell profile DP with states packed into u64, stored in sharded
// Vec open-addressing tables and updated in parallel.

use rayon::prelude::*;

const N: usize = 7;
const PBITS: usize = 3 * N;
const PSPACE: usize = 1 << PBITS;
const EMPTY: u32 = u32::MAX;
const SHARDS: usize = 32;

// key: profile_idx (13) | areas (42) | max_area (6)
const AREA_SHIFT: u32 = 13;
const MAX_SHIFT: u32 = 13 + 42;

#[inline(always)]
fn mix64(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb);
    x ^= x >> 31;
    x
}

#[inline(always)]
fn shard_of(key: u64) -> usize {
    (mix64(key) >> 59) as usize
}

#[inline(always)]
fn hash64(key: u64) -> usize {
    mix64(key) as usize
}

struct Table {
    keys: Vec<u64>,
    vals: Vec<i64>,
    key_slot: Vec<u32>,
    slots: Vec<u32>,
}

impl Table {
    fn new(bits: usize) -> Self {
        Table {
            keys: Vec::with_capacity(1 << 14),
            vals: Vec::with_capacity(1 << 14),
            key_slot: Vec::with_capacity(1 << 14),
            slots: vec![EMPTY; 1 << bits],
        }
    }

    fn grow(&mut self) {
        let new_len = self.slots.len() * 2;
        self.slots.clear();
        self.slots.resize(new_len, EMPTY);
        let mask = new_len - 1;
        for (i, &key) in self.keys.iter().enumerate() {
            let mut h = hash64(key) & mask;
            loop {
                // SAFETY: h < slots.len()
                unsafe {
                    if *self.slots.get_unchecked(h) == EMPTY {
                        *self.slots.get_unchecked_mut(h) = i as u32;
                        *self.key_slot.get_unchecked_mut(i) = h as u32;
                        break;
                    }
                }
                h = (h + 1) & mask;
            }
        }
    }

    #[inline(always)]
    fn add(&mut self, key: u64, count: i64) {
        if self.keys.len() * 2 >= self.slots.len() {
            self.grow();
        }
        let mask = self.slots.len() - 1;
        let mut h = hash64(key) & mask;
        loop {
            // SAFETY: h < slots.len()
            let idx = unsafe { *self.slots.get_unchecked(h) };
            if idx == EMPTY {
                let i = self.keys.len() as u32;
                unsafe {
                    *self.slots.get_unchecked_mut(h) = i;
                }
                self.keys.push(key);
                self.vals.push(count);
                self.key_slot.push(h as u32);
                return;
            }
            // SAFETY: idx is a live key index
            unsafe {
                if *self.keys.get_unchecked(idx as usize) == key {
                    *self.vals.get_unchecked_mut(idx as usize) += count;
                    return;
                }
            }
            h = (h + 1) & mask;
        }
    }

    fn clear(&mut self) {
        for &s in &self.key_slot {
            // SAFETY: s is a slot this table occupied
            unsafe {
                *self.slots.get_unchecked_mut(s as usize) = EMPTY;
            }
        }
        self.keys.clear();
        self.vals.clear();
        self.key_slot.clear();
    }
}

fn canonicalize_packed(p: u32) -> u32 {
    let mut mapping = [0u8; 8];
    let mut idx = 0u8;
    let mut out = 0u32;
    for i in 0..N {
        let num = ((p >> (3 * i)) & 7) as usize;
        if num > 0 && mapping[num] == 0 {
            idx += 1;
            mapping[num] = idx;
        }
        out |= (mapping[num] as u32) << (3 * i);
    }
    out
}

#[inline(always)]
fn trans_white(prof: u64, areas: u64, ma: u64, prof_to_idx: &[u16]) -> u64 {
    let np = prof >> 3;
    let na = areas >> 6;
    // SAFETY: np has at most 18 bits
    let npidx = unsafe { *prof_to_idx.get_unchecked(np as usize) } as u64;
    npidx | (na << AREA_SHIFT) | (ma << MAX_SHIFT)
}

#[inline(always)]
fn trans_black(prof: u64, areas: u64, ma: u64, has_left: bool, prof_to_idx: &[u16]) -> u64 {
    let above_g = prof & 7;
    let above_a = areas & 63;
    let mut new_area = 1 + above_a;
    let left_g = if has_left {
        let g = (prof >> 18) & 7;
        if above_g != g {
            new_area += (areas >> 36) & 63;
        }
        g
    } else {
        0
    };

    let mut np = prof >> 3;
    let mut na = areas >> 6;
    for c in 0..6u32 {
        let g = (np >> (3 * c)) & 7;
        if g != 0 && (g == above_g || g == left_g) {
            np = (np & !(7u64 << (3 * c))) | (7u64 << (3 * c));
            na = (na & !(63u64 << (6 * c))) | (new_area << (6 * c));
        }
    }
    np |= 7u64 << 18;
    na |= new_area << 36;

    // SAFETY: np is a 21-bit profile
    let npidx = unsafe { *prof_to_idx.get_unchecked(np as usize) } as u64;
    let nma = if new_area > ma { new_area } else { ma };
    npidx | (na << AREA_SHIFT) | (nma << MAX_SHIFT)
}

fn main() {
    let mut canon_packed = vec![0u32; PSPACE];
    for p in 0..PSPACE {
        canon_packed[p] = canonicalize_packed(p as u32);
    }

    let mut idx_to_prof: Vec<u32> = Vec::with_capacity(4200);
    let mut prof_to_idx = vec![0u16; PSPACE];
    for p in 0..PSPACE {
        if canon_packed[p] == p as u32 {
            prof_to_idx[p] = idx_to_prof.len() as u16;
            idx_to_prof.push(p as u32);
        }
    }
    for p in 0..PSPACE {
        if canon_packed[p] != p as u32 {
            prof_to_idx[p] = prof_to_idx[canon_packed[p] as usize];
        }
    }
    drop(canon_packed);
    let prof_to_idx = prof_to_idx;
    let idx_to_prof = idx_to_prof;

    let mut cur: Vec<Table> = (0..SHARDS).map(|_| Table::new(18)).collect();
    let mut nxt: Vec<Table> = (0..SHARDS).map(|_| Table::new(18)).collect();
    cur[shard_of(0)].add(0, 1);

    let mut out: Vec<Vec<Vec<(u64, i64)>>> = (0..SHARDS)
        .map(|_| (0..SHARDS).map(|_| Vec::with_capacity(8192)).collect())
        .collect();

    for _row in 0..N {
        for col in 0..N {
            let has_left = col > 0;
            let p2i = prof_to_idx.as_slice();
            let i2p = idx_to_prof.as_slice();

            cur.par_iter()
                .zip(out.par_iter_mut())
                .for_each(|(src, dests)| {
                    for d in dests.iter_mut() {
                        d.clear();
                    }
                    let nst = src.keys.len();
                    for i in 0..nst {
                        // SAFETY: i < keys.len() == vals.len()
                        let key = unsafe { *src.keys.get_unchecked(i) };
                        let count = unsafe { *src.vals.get_unchecked(i) };
                        let pidx = (key & 0x1FFF) as usize;
                        let areas = (key >> AREA_SHIFT) & ((1u64 << 42) - 1);
                        let ma = key >> MAX_SHIFT;
                        // SAFETY: pidx is a canonical profile index
                        let prof = unsafe { *i2p.get_unchecked(pidx) } as u64;

                        let wk = trans_white(prof, areas, ma, p2i);
                        dests[shard_of(wk)].push((wk, count));
                        let bk = trans_black(prof, areas, ma, has_left, p2i);
                        dests[shard_of(bk)].push((bk, count));
                    }
                });

            nxt.par_iter_mut().enumerate().for_each(|(dst, table)| {
                table.clear();
                for src in 0..SHARDS {
                    for &(k, c) in &out[src][dst] {
                        table.add(k, c);
                    }
                }
            });

            std::mem::swap(&mut cur, &mut nxt);
        }
    }

    let numer: i64 = cur
        .iter()
        .map(|t| {
            let mut s = 0i64;
            for i in 0..t.keys.len() {
                let key = unsafe { *t.keys.get_unchecked(i) };
                let count = unsafe { *t.vals.get_unchecked(i) };
                s += count * ((key >> MAX_SHIFT) as i64);
            }
            s
        })
        .sum();
    let ans = numer as f64 / ((1u64 << (N * N)) as f64);
    println!("{:.8}", ans);
}
