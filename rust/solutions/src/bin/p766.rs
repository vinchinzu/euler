// Project Euler 766 - Sliding Block Puzzle
// DFS over reachable configurations; same-shape pieces are indistinguishable.
// Pieces/occupancy as 30-bit masks. Visited key packs empty + non-monomino origins into u64.

use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

const H: usize = 5;
const W: usize = 6;
const MAX_PIECES: usize = 14;
const BOARD: u32 = (1 << (H * W)) - 1;
const COL0: u32 = 0x0104_1041;
const COL5: u32 = 0x2082_0820;
const ROW0: u32 = 0x0000_003F;
const ROW4: u32 = 0x3F00_0000;

const HASH_BITS: usize = 22;
const HASH_SIZE: usize = 1 << HASH_BITS;
const HASH_MASK: usize = HASH_SIZE - 1;

const GRID: [&[u8]; H] = [
    b".AABCC",
    b".ABBCD",
    b"EFGGHD",
    b"IJGGHK",
    b"LMNNKK",
];

#[derive(Clone, Copy)]
struct Frame {
    ti: u8,
    di: u8,
    ek: u8,
    moved: bool,
    prev_key: u64,
}

#[derive(Clone, Copy)]
struct Snap {
    occ: u32,
    masks: [u32; MAX_PIECES],
    owner: [u8; 32],
    key: u64,
}

struct Vis {
    keys: Box<[AtomicU64]>,
}

impl Vis {
    fn new() -> Self {
        let raw = vec![0u64; HASH_SIZE].into_boxed_slice();
        let ptr = Box::into_raw(raw);
        // SAFETY: AtomicU64 has the same layout as u64.
        let keys = unsafe { Box::from_raw(ptr as *mut [AtomicU64]) };
        Self { keys }
    }

    #[inline(always)]
    fn insert(&self, key: u64) -> bool {
        let keys = &self.keys;
        let mut i = mix(key);
        loop {
            // SAFETY: i is always masked into HASH_SIZE.
            let slot = unsafe { keys.get_unchecked(i) };
            let cur = slot.load(Ordering::Relaxed);
            if cur == 0 {
                match slot.compare_exchange_weak(0, key, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => return true,
                    Err(v) => {
                        if v == key {
                            return false;
                        }
                    }
                }
            } else if cur == key {
                return false;
            }
            i = (i + 1) & HASH_MASK;
        }
    }
}

#[inline(always)]
fn mix(x: u64) -> usize {
    (x.wrapping_mul(0x9e3779b97f4a7c15) >> (64 - HASH_BITS)) as usize
}

#[inline(always)]
fn pack_pair(a: u32, b: u32) -> u64 {
    let za = a.trailing_zeros();
    let zb = b.trailing_zeros();
    let (lo, hi) = if za < zb { (za, zb) } else { (zb, za) };
    (lo as u64) | ((hi as u64) << 5)
}

#[inline(always)]
fn pack_key(occ: u32, m: &[u32; MAX_PIECES], pairs: &[[u8; 2]; 4], npair: usize, singles: &[u8; 4], nsingle: usize) -> u64 {
    let mut empty = (!occ) & BOARD;
    let e0 = empty.trailing_zeros() as u64;
    empty &= empty - 1;
    let e1 = empty.trailing_zeros() as u64;
    let mut key = e0 | (e1 << 5);
    let mut sh = 10u32;
    for i in 0..npair {
        let p = pairs[i];
        key |= pack_pair(
            unsafe { *m.get_unchecked(p[0] as usize) },
            unsafe { *m.get_unchecked(p[1] as usize) },
        ) << sh;
        sh += 10;
    }
    for i in 0..nsingle {
        let pi = singles[i] as usize;
        key |= (unsafe { m.get_unchecked(pi) }.trailing_zeros() as u64) << sh;
        sh += 5;
    }
    key
}

#[inline(always)]
fn update_key(
    mut key: u64,
    occ: u32,
    m: &[u32; MAX_PIECES],
    ti: usize,
    pairs: &[[u8; 2]; 4],
    pair_of: &[u8; MAX_PIECES],
    single_of: &[u8; MAX_PIECES],
) -> u64 {
    let mut empty = (!occ) & BOARD;
    let e0 = empty.trailing_zeros() as u64;
    empty &= empty - 1;
    let e1 = empty.trailing_zeros() as u64;
    key = (key & !0x3ff) | e0 | (e1 << 5);
    let s = unsafe { *single_of.get_unchecked(ti) };
    if s != 0xff {
        let sh = 40 + 5 * s as u32;
        key &= !(0x1fu64 << sh);
        key |= (unsafe { m.get_unchecked(ti) }.trailing_zeros() as u64) << sh;
        return key;
    }
    let p = unsafe { *pair_of.get_unchecked(ti) };
    if p != 0xff {
        let sh = 10 + 10 * p as u32;
        key &= !(0x3ffu64 << sh);
        let pr = unsafe { *pairs.get_unchecked(p as usize) };
        key |= pack_pair(
            unsafe { *m.get_unchecked(pr[0] as usize) },
            unsafe { *m.get_unchecked(pr[1] as usize) },
        ) << sh;
    }
    key
}

#[inline(always)]
fn neighbor_of_empty(e: u32, di: u8) -> i32 {
    match di {
        0 => {
            if e >= 24 {
                -1
            } else {
                (e + 6) as i32
            }
        }
        1 => {
            if e < 6 {
                -1
            } else {
                (e - 6) as i32
            }
        }
        2 => {
            if e % 6 == 5 {
                -1
            } else {
                (e + 1) as i32
            }
        }
        _ => {
            if e % 6 == 0 {
                -1
            } else {
                (e - 1) as i32
            }
        }
    }
}

#[inline(always)]
fn reown(owner: &mut [u8; 32], mut delta: u32, newm: u32, pid: u8) {
    while delta != 0 {
        let c = delta.trailing_zeros();
        delta &= delta - 1;
        owner[c as usize] = if newm & (1 << c) != 0 { pid } else { 0xff };
    }
}

#[inline(always)]
fn shifted(mask: u32, di: u8) -> u32 {
    match di {
        0 => {
            if mask & ROW0 != 0 {
                0
            } else {
                mask >> W
            }
        }
        1 => {
            if mask & ROW4 != 0 {
                0
            } else {
                mask << W
            }
        }
        2 => {
            if mask & COL0 != 0 {
                0
            } else {
                mask >> 1
            }
        }
        3 => {
            if mask & COL5 != 0 {
                0
            } else {
                mask << 1
            }
        }
        _ => 0,
    }
}

fn gen_children(
    mut occ: u32,
    mut masks: [u32; MAX_PIECES],
    mut owner: [u8; 32],
    key: u64,
    vis: &Vis,
    pairs: &[[u8; 2]; 4],
    pair_of: &[u8; MAX_PIECES],
    single_of: &[u8; MAX_PIECES],
) -> Vec<Snap> {
    let mut out = Vec::with_capacity(8);
    let mut em = (!occ) & BOARD;
    let e0 = em.trailing_zeros();
    em &= em - 1;
    let e1 = em.trailing_zeros();
    for ek in 0..2u8 {
        let e = if ek == 0 { e0 } else { e1 };
        for di in 0..4u8 {
            let nb = neighbor_of_empty(e, di);
            if nb < 0 {
                continue;
            }
            let pid = unsafe { *owner.get_unchecked(nb as usize) };
            if pid == 0xff {
                continue;
            }
            let ti = pid as usize;
            let old = unsafe { *masks.get_unchecked(ti) };
            let newm = shifted(old, di);
            if newm == 0 || newm & (occ ^ old) != 0 {
                continue;
            }
            let delta = old ^ newm;
            occ ^= delta;
            unsafe {
                *masks.get_unchecked_mut(ti) = newm;
            }
            let new_key = update_key(key, occ, &masks, ti, pairs, pair_of, single_of);
            if vis.insert(new_key) {
                reown(&mut owner, delta, newm, pid);
                out.push(Snap {
                    occ,
                    masks,
                    owner,
                    key: new_key,
                });
                reown(&mut owner, delta, old, pid);
            }
            occ ^= delta;
            unsafe {
                *masks.get_unchecked_mut(ti) = old;
            }
        }
    }
    out
}

fn dfs(
    mut occ: u32,
    mut masks: [u32; MAX_PIECES],
    mut owner: [u8; 32],
    mut key: u64,
    vis: &Vis,
    pairs: &[[u8; 2]; 4],
    pair_of: &[u8; MAX_PIECES],
    single_of: &[u8; MAX_PIECES],
) -> usize {
    let mut added = 0usize;
    let mut stack = Vec::with_capacity(4096);
    stack.push(Frame {
        ti: 0,
        di: 0,
        ek: 0,
        moved: false,
        prev_key: 0,
    });
    let mut top = 1usize;

    while top > 0 {
        let mut ek: u8;
        let mut di: u8;
        {
            let f = unsafe { stack.get_unchecked_mut(top - 1) };
            if f.moved {
                let ti = f.ti as usize;
                let cur = unsafe { *masks.get_unchecked(ti) };
                let old = shifted(cur, f.di ^ 1);
                let delta = cur ^ old;
                occ ^= delta;
                unsafe {
                    *masks.get_unchecked_mut(ti) = old;
                }
                reown(&mut owner, delta, old, ti as u8);
                key = f.prev_key;
                f.moved = false;
                f.di += 1;
                if f.di >= 4 {
                    f.di = 0;
                    f.ek += 1;
                }
            }
            ek = f.ek;
            di = f.di;
        }

        let mut em = (!occ) & BOARD;
        let e0 = em.trailing_zeros();
        em &= em - 1;
        let e1 = em.trailing_zeros();

        let mut found = false;
        'search: while ek < 2 {
            let e = if ek == 0 { e0 } else { e1 };
            while di < 4 {
                let nb = neighbor_of_empty(e, di);
                if nb >= 0 {
                    let pid = unsafe { *owner.get_unchecked(nb as usize) };
                    if pid != 0xff {
                        let ti = pid as usize;
                        let old = unsafe { *masks.get_unchecked(ti) };
                        let newm = shifted(old, di);
                        if newm != 0 && newm & (occ ^ old) == 0 {
                            let delta = old ^ newm;
                            occ ^= delta;
                            unsafe {
                                *masks.get_unchecked_mut(ti) = newm;
                            }
                            let new_key =
                                update_key(key, occ, &masks, ti, pairs, pair_of, single_of);
                            if vis.insert(new_key) {
                                added += 1;
                                reown(&mut owner, delta, newm, pid);
                                let f = unsafe { stack.get_unchecked_mut(top - 1) };
                                f.ti = pid;
                                f.ek = ek;
                                f.di = di;
                                f.moved = true;
                                f.prev_key = key;
                                key = new_key;
                                if top == stack.len() {
                                    stack.push(Frame {
                                        ti: 0,
                                        di: 0,
                                        ek: 0,
                                        moved: false,
                                        prev_key: 0,
                                    });
                                } else {
                                    stack[top] = Frame {
                                        ti: 0,
                                        di: 0,
                                        ek: 0,
                                        moved: false,
                                        prev_key: 0,
                                    };
                                }
                                top += 1;
                                found = true;
                                break 'search;
                            } else {
                                occ ^= delta;
                                unsafe {
                                    *masks.get_unchecked_mut(ti) = old;
                                }
                            }
                        }
                    }
                }
                di += 1;
            }
            di = 0;
            ek += 1;
        }

        if !found {
            top -= 1;
        }
    }
    added
}

fn main() {
    let mut cell_count = [0usize; 26];
    let mut cell_pos = [[0u8; 4]; 26];
    for y in 0..H {
        for x in 0..W {
            let ch = GRID[y][x];
            if ch != b'.' {
                let ci = (ch - b'A') as usize;
                cell_pos[ci][cell_count[ci]] = (y * W + x) as u8;
                cell_count[ci] += 1;
            }
        }
    }

    let mut npieces = 0usize;
    let mut masks = [0u32; MAX_PIECES];
    let mut types = [0u8; MAX_PIECES];

    let mut shape_n = [0u8; MAX_PIECES];
    let mut shape_sig = [0u64; MAX_PIECES];
    let mut nshapes = 0usize;

    for ci in 0..26 {
        if cell_count[ci] == 0 {
            continue;
        }
        let pi = npieces;
        npieces += 1;
        let nc = cell_count[ci];
        let mut mask = 0u32;
        let mut min_y = H as i32;
        let mut min_x = W as i32;
        for j in 0..nc {
            let p = cell_pos[ci][j] as usize;
            mask |= 1 << p;
            let y = (p / W) as i32;
            let x = (p % W) as i32;
            if y < min_y {
                min_y = y;
            }
            if x < min_x {
                min_x = x;
            }
        }
        masks[pi] = mask;

        let mut rel = [0u16; 4];
        for j in 0..nc {
            let p = cell_pos[ci][j] as usize;
            let dy = (p / W) as i32 - min_y;
            let dx = (p % W) as i32 - min_x;
            rel[j] = ((dy as u16) << 8) | (dx as u16);
        }
        rel[..nc].sort_unstable();
        let mut sig = 0u64;
        for j in 0..nc {
            sig |= (rel[j] as u64) << (16 * j);
        }

        let mut found = nshapes;
        for s in 0..nshapes {
            if shape_n[s] == nc as u8 && shape_sig[s] == sig {
                found = s;
                break;
            }
        }
        if found == nshapes {
            shape_n[nshapes] = nc as u8;
            shape_sig[nshapes] = sig;
            nshapes += 1;
        }
        types[pi] = (found + 1) as u8;
    }

    let mut by_type = [[0u8; 6]; 8];
    let mut n_of_type = [0u8; 8];
    for i in 0..npieces {
        let t = types[i] as usize;
        let k = n_of_type[t] as usize;
        by_type[t][k] = i as u8;
        n_of_type[t] += 1;
    }

    let mut pairs = [[0u8; 2]; 4];
    let mut npair = 0usize;
    let mut singles = [0u8; 4];
    let mut nsingle = 0usize;
    for t in 1..=nshapes {
        let n = n_of_type[t] as usize;
        if n == 2 {
            pairs[npair] = [by_type[t][0], by_type[t][1]];
            npair += 1;
        } else if n == 1 {
            singles[nsingle] = by_type[t][0];
            nsingle += 1;
        }
    }

    let mut pair_of = [0xffu8; MAX_PIECES];
    let mut single_of = [0xffu8; MAX_PIECES];
    for i in 0..npair {
        pair_of[pairs[i][0] as usize] = i as u8;
        pair_of[pairs[i][1] as usize] = i as u8;
    }
    for i in 0..nsingle {
        single_of[singles[i] as usize] = i as u8;
    }

    let mut occ = 0u32;
    let mut owner = [0xffu8; 32];
    for i in 0..npieces {
        occ |= masks[i];
        let mut m = masks[i];
        while m != 0 {
            let c = m.trailing_zeros() as usize;
            m &= m - 1;
            owner[c] = i as u8;
        }
    }

    let key = pack_key(occ, &masks, &pairs, npair, &singles, nsingle);
    let vis = Vis::new();
    vis.insert(key);
    let mut nstates = 1usize;

    let mut layer = vec![Snap {
        occ,
        masks,
        owner,
        key,
    }];
    while layer.len() < 48 {
        let mut next = Vec::new();
        for s in &layer {
            next.extend(gen_children(
                s.occ,
                s.masks,
                s.owner,
                s.key,
                &vis,
                &pairs,
                &pair_of,
                &single_of,
            ));
        }
        if next.is_empty() {
            println!("{}", nstates);
            return;
        }
        nstates += next.len();
        layer = next;
    }

    nstates += layer
        .into_par_iter()
        .map(|s| dfs(s.occ, s.masks, s.owner, s.key, &vis, &pairs, &pair_of, &single_of))
        .sum::<usize>();

    println!("{}", nstates);
}
