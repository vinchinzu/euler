// Project Euler 212: Combined Volume of Cuboids
use rayon::prelude::*;

const N_CUBOIDS: usize = 50000;
const L: i32 = 130;
const G: usize = 81; // section indices 0..80 (coords < 10400)
const CELLS: usize = G * G * G;

#[derive(Clone, Copy)]
struct Cuboid {
    x: i32,
    y: i32,
    z: i32,
    x2: i32,
    y2: i32,
    z2: i32,
}

#[inline(always)]
fn cell(ix: usize, iy: usize, iz: usize) -> usize {
    (ix * G + iy) * G + iz
}

/// Inclusion-exclusion over cuboids overlapping the current box.
/// Exclude-branches stay in the loop; only include-branches recurse.
#[inline(always)]
fn helper(
    cuboids: &[Cuboid],
    ids: &[u16],
    mut idx: usize,
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
    num_cuboids: i32,
) -> i64 {
    if min_x >= max_x || min_y >= max_y || min_z >= max_z {
        return 0;
    }
    let n = ids.len();
    let mut acc = 0i64;
    while idx < n {
        // SAFETY: idx < ids.len(); every id was pushed from 0..N_CUBOIDS.
        let c = unsafe { cuboids.get_unchecked(*ids.get_unchecked(idx) as usize) };
        idx += 1;
        let nx1 = min_x.max(c.x);
        let ny1 = min_y.max(c.y);
        let nz1 = min_z.max(c.z);
        let nx2 = max_x.min(c.x2);
        let ny2 = max_y.min(c.y2);
        let nz2 = max_z.min(c.z2);
        if nx1 < nx2 && ny1 < ny2 && nz1 < nz2 {
            acc += helper(
                cuboids, ids, idx, nx1, ny1, nz1, nx2, ny2, nz2, num_cuboids + 1,
            );
        }
    }
    if num_cuboids == 0 {
        return acc;
    }
    let vol = (max_x - min_x) as i64 * (max_y - min_y) as i64 * (max_z - min_z) as i64;
    acc + if num_cuboids % 2 == 0 { vol } else { -vol }
}

fn main() {
    // Generate lagged Fibonacci sequence
    let mut s_seq = vec![0i32; 6 * N_CUBOIDS];
    for k in 1..=55usize {
        let kk = k as i64;
        s_seq[k - 1] = ((100003 - 200003 * kk + 300007 * kk * kk * kk) % 1000000) as i32;
    }
    for k in 55..(6 * N_CUBOIDS) {
        s_seq[k] = (s_seq[k - 24] + s_seq[k - 55]) % 1000000;
    }

    let mut cuboids = Vec::with_capacity(N_CUBOIDS);
    for i in 0..N_CUBOIDS {
        let idx = 6 * i;
        let x = s_seq[idx] % 10000;
        let y = s_seq[idx + 1] % 10000;
        let z = s_seq[idx + 2] % 10000;
        cuboids.push(Cuboid {
            x,
            y,
            z,
            x2: x + s_seq[idx + 3] % 399 + 1,
            y2: y + s_seq[idx + 4] % 399 + 1,
            z2: z + s_seq[idx + 5] % 399 + 1,
        });
    }
    drop(s_seq);

    // Two-pass packed L×L×L sections (keys are dense in 81³). Count, prefix, fill.
    let mut counts = vec![0u16; CELLS];
    for i in 0..N_CUBOIDS {
        let c = &cuboids[i];
        let mut dx = 0;
        while dx < (c.x2 - c.x) + L {
            let mut dy = 0;
            while dy < (c.y2 - c.y) + L {
                let mut dz = 0;
                while dz < (c.z2 - c.z) + L {
                    let ix = ((c.x + dx) / L) as usize;
                    let iy = ((c.y + dy) / L) as usize;
                    let iz = ((c.z + dz) / L) as usize;
                    debug_assert!(ix < G && iy < G && iz < G);
                    // SAFETY: ix,iy,iz ∈ 0..G as coords < 10400.
                    unsafe {
                        *counts.get_unchecked_mut(cell(ix, iy, iz)) += 1;
                    }
                    dz += L;
                }
                dy += L;
            }
            dx += L;
        }
    }

    let mut offs = vec![0u32; CELLS + 1];
    let mut total = 0u32;
    let mut occupied = Vec::with_capacity(1 << 19);
    for i in 0..CELLS {
        offs[i] = total;
        if counts[i] != 0 {
            occupied.push(i as u32);
        }
        total += counts[i] as u32;
    }
    offs[CELLS] = total;

    let mut data = vec![0u16; total as usize];
    let mut cursor = offs.clone();
    for i in 0..N_CUBOIDS {
        let c = &cuboids[i];
        let id = i as u16;
        let mut dx = 0;
        while dx < (c.x2 - c.x) + L {
            let mut dy = 0;
            while dy < (c.y2 - c.y) + L {
                let mut dz = 0;
                while dz < (c.z2 - c.z) + L {
                    let ix = ((c.x + dx) / L) as usize;
                    let iy = ((c.y + dy) / L) as usize;
                    let iz = ((c.z + dz) / L) as usize;
                    let k = cell(ix, iy, iz);
                    // SAFETY: k < CELLS; cursor[k] < offs[k]+counts[k] ≤ data.len().
                    unsafe {
                        let p = *cursor.get_unchecked(k);
                        *data.get_unchecked_mut(p as usize) = id;
                        *cursor.get_unchecked_mut(k) = p + 1;
                    }
                    dz += L;
                }
                dy += L;
            }
            dx += L;
        }
    }
    drop(counts);
    drop(cursor);

    let ans: i64 = occupied
        .par_iter()
        .with_min_len(64)
        .map(|&k| {
            let k = k as usize;
            let ix = k / (G * G);
            let iy = (k / G) % G;
            let iz = k % G;
            let sx = ix as i32 * L;
            let sy = iy as i32 * L;
            let sz = iz as i32 * L;
            // SAFETY: k < CELLS; offs has length CELLS+1.
            let (a, b) = unsafe {
                (
                    *offs.get_unchecked(k) as usize,
                    *offs.get_unchecked(k + 1) as usize,
                )
            };
            -helper(&cuboids, &data[a..b], 0, sx, sy, sz, sx + L, sy + L, sz + L, 0)
        })
        .sum();

    println!("{ans}");
}
