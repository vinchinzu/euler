// Project Euler 292: Pythagorean Polygons
// Meet-in-the-middle over primitive integer-length directions.
// Angular halves are 180° opposites, so only one half is DP'd.
// Many-scale directions are prefix-sums along each displacement ray.

use rayon::prelude::*;

const PERIM: i32 = 120;
const P: usize = 120;
const SPAN: usize = 241;
const GSZ: usize = SPAN * SPAN * 4;
const SLOT_CAP: usize = 1 << 20;

struct Dir {
    a: i32,
    b: i32,
    c: i32,
    angle: f64,
}

struct Map {
    /// Power-of-two open-address table. 0 = empty, else 1-based index into `entries`.
    slots: Vec<u32>,
    mask: usize,
    entries: Vec<(u32, i64)>,
}

impl Map {
    fn new() -> Self {
        Self {
            slots: vec![0; SLOT_CAP],
            mask: SLOT_CAP - 1,
            entries: Vec::with_capacity(1 << 18),
        }
    }

    #[inline(always)]
    fn hash(key: u32) -> usize {
        key.wrapping_mul(0x9E3779B9) as usize
    }

    fn grow(&mut self) {
        let new_cap = self.slots.len() * 2;
        let mut new_slots = vec![0u32; new_cap];
        let new_mask = new_cap - 1;
        for (idx, &(key, _)) in self.entries.iter().enumerate() {
            let mut i = Self::hash(key) & new_mask;
            loop {
                if new_slots[i] == 0 {
                    new_slots[i] = idx as u32 + 1;
                    break;
                }
                i = (i + 1) & new_mask;
            }
        }
        self.slots = new_slots;
        self.mask = new_mask;
    }

    fn rebuild_slots(&mut self) {
        if self.entries.len() * 2 >= self.slots.len() {
            self.grow();
            return;
        }
        self.slots.fill(0);
        for (idx, &(key, _)) in self.entries.iter().enumerate() {
            let mut i = Self::hash(key) & self.mask;
            loop {
                // SAFETY: i is masked to an in-range slot
                if unsafe { *self.slots.get_unchecked(i) } == 0 {
                    unsafe { *self.slots.get_unchecked_mut(i) = idx as u32 + 1 };
                    break;
                }
                i = (i + 1) & self.mask;
            }
        }
    }

    #[inline(always)]
    fn add(&mut self, key: u32, val: i64) {
        let mut i = Self::hash(key) & self.mask;
        loop {
            // SAFETY: i is masked to an in-range slot index
            let s = unsafe { *self.slots.get_unchecked(i) };
            if s == 0 {
                if self.entries.len() * 2 >= self.slots.len() {
                    self.grow();
                    self.add(key, val);
                    return;
                }
                let idx = self.entries.len() as u32 + 1;
                unsafe { *self.slots.get_unchecked_mut(i) = idx };
                self.entries.push((key, val));
                return;
            }
            // SAFETY: s != 0 means a valid 1-based entries index
            let e = unsafe { self.entries.get_unchecked_mut((s - 1) as usize) };
            if e.0 == key {
                e.1 += val;
                return;
            }
            i = (i + 1) & self.mask;
        }
    }
}

#[inline(always)]
fn pack(sx: i32, sy: i32, peri: i32, ne: i32) -> u32 {
    (((sx + PERIM) as u32) << 17) | (((sy + PERIM) as u32) << 9) | ((peri as u32) << 2) | (ne as u32)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn isqrt(n: i32) -> i32 {
    let mut r = (n as f64).sqrt() as i32;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

fn generate_directions() -> Vec<Dir> {
    let mut dirs = Vec::new();
    for a in -PERIM..=PERIM {
        for b in -PERIM..=PERIM {
            if a == 0 && b == 0 {
                continue;
            }
            let c2 = a * a + b * b;
            let c = isqrt(c2);
            if c * c != c2 || c > PERIM {
                continue;
            }
            if gcd(a.abs(), b.abs()) != 1 {
                continue;
            }
            dirs.push(Dir {
                a,
                b,
                c,
                angle: (b as f64).atan2(a as f64),
            });
        }
    }
    dirs.sort_by(|x, y| x.angle.partial_cmp(&y.angle).unwrap());
    dirs
}

/// Apply direction (a,b,c) as skip + take-k via prefix sums on each invariant ray.
fn apply_dir(entries: &[(u32, i64)], a: i32, b: i32, c: i32) -> Vec<(u32, i64)> {
    let mut items: Vec<(i32, i32, u8, u8, i64)> = Vec::with_capacity(entries.len());
    for &(key, cnt) in entries {
        let ne = (key & 3) as u8;
        let peri = ((key >> 2) & 127) as u8;
        let sy = ((key >> 9) & 255) as i32 - PERIM;
        let sx = (key >> 17) as i32 - PERIM;
        let g1 = b * sx - a * sy;
        let g2 = if a != 0 {
            c * sx - a * peri as i32
        } else {
            c * sy - b * peri as i32
        };
        items.push((g1, g2, ne, peri, cnt));
    }
    items.par_sort_unstable();

    let mut out = Vec::with_capacity(entries.len() + entries.len() / 2);
    let mut i = 0;
    let n = items.len();
    while i < n {
        let g1 = items[i].0;
        let g2 = items[i].1;
        let r = items[i].3 as i32 % c;
        let mut old = [[0i64; 121]; 4];
        while i < n && items[i].0 == g1 && items[i].1 == g2 {
            old[items[i].2 as usize][items[i].3 as usize] += items[i].4;
            i += 1;
        }

        let mut newc = old;
        let mut pref = [0i64; 4];
        let mut p = r;
        while p <= PERIM {
            let pu = p as usize;
            newc[1][pu] += pref[0];
            newc[2][pu] += pref[1];
            newc[3][pu] += pref[2] + pref[3];
            pref[0] += old[0][pu];
            pref[1] += old[1][pu];
            pref[2] += old[2][pu];
            pref[3] += old[3][pu];
            p += c;
        }

        p = r;
        while p <= PERIM {
            let pu = p as usize;
            for ne in 0..4 {
                let cv = newc[ne][pu];
                if cv != 0 {
                    let peri = p;
                    let (sx, sy) = if a != 0 {
                        let sx = (g2 + a * peri) / c;
                        let sy = (b * sx - g1) / a;
                        (sx, sy)
                    } else {
                        let sy = (g2 + b * peri) / c;
                        let sx = g1 / b;
                        (sx, sy)
                    };
                    out.push((pack(sx, sy, peri, ne as i32), cv));
                }
            }
            p += c;
        }
    }
    out
}

fn main() {
    let dirs = generate_directions();
    let mid = dirs.len() / 2;

    let mut dir_opts: Vec<(i32, i32, i32, Vec<(i32, i32, i32)>)> = Vec::with_capacity(mid);
    for d in &dirs[..mid] {
        let mut opts = Vec::new();
        let mut k = 1;
        while k * d.c <= PERIM {
            opts.push((k * d.a, k * d.b, k * d.c));
            k += 1;
        }
        dir_opts.push((d.a, d.b, d.c, opts));
    }

    let mut map = Map::new();
    map.add(pack(0, 0, 0, 0), 1);

    let mut snapshot: Vec<(u32, i64)> = Vec::with_capacity(1 << 18);
    let n_dirs = dir_opts.len();
    for (di, (a, b, c, opts)) in dir_opts.iter().enumerate() {
        if opts.is_empty() {
            continue;
        }
        if *c == 1 || opts.len() >= 16 {
            map.entries = apply_dir(&map.entries, *a, *b, *c);
            if di + 1 < n_dirs {
                map.rebuild_slots();
            }
            continue;
        }
        snapshot.clear();
        snapshot.extend_from_slice(&map.entries);
        for &(key, cnt) in &snapshot {
            let ne = (key & 3) as i32;
            let peri = ((key >> 2) & 127) as i32;
            let sy = ((key >> 9) & 255) as i32 - PERIM;
            let sx = (key >> 17) as i32 - PERIM;
            let new_ne = if ne < 3 { ne + 1 } else { 3 };
            let rem = PERIM - peri;
            for &(dx, dy, dl) in opts {
                if dl > rem {
                    break;
                }
                map.add(pack(sx + dx, sy + dy, peri + dl, new_ne), cnt);
            }
        }
    }

    let mut idx = vec![u32::MAX; GSZ];
    let mut data: Vec<[i64; 121]> = Vec::with_capacity(map.entries.len() / 8 + 8);
    for &(key, cnt) in &map.entries {
        let ne = (key & 3) as usize;
        let peri = ((key >> 2) & 127) as usize;
        let syb = ((key >> 9) & 255) as usize;
        let sxb = (key >> 17) as usize;
        let gi = (sxb * SPAN + syb) * 4 + ne;
        let mut slot = idx[gi];
        if slot == u32::MAX {
            slot = data.len() as u32;
            idx[gi] = slot;
            data.push([0; 121]);
        }
        data[slot as usize][peri] += cnt;
    }
    for row in &mut data {
        for p in 1..=P {
            row[p] += row[p - 1];
        }
    }

    let mut total = 0i64;
    for &(key, cnt1) in &map.entries {
        let ne1 = (key & 3) as i32;
        let p1 = ((key >> 2) & 127) as usize;
        let syb = ((key >> 9) & 255) as usize;
        let sxb = (key >> 17) as usize;
        let remaining = P - p1;
        let min_ne2 = (3 - ne1).max(0) as usize;
        let base = (sxb * SPAN + syb) * 4;
        for ne2 in min_ne2..4 {
            let slot = idx[base + ne2];
            if slot != u32::MAX {
                total += cnt1 * data[slot as usize][remaining];
            }
        }
    }

    println!("{}", total);
}
