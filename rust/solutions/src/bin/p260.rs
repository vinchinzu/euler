// Project Euler 260: Stone Game
// Losing positions x <= y <= z <= N. Occupancy of pair-lines, difference
// diags, and space diagonals is bit-packed (16 u64s / row, ~125KB / table).

const N: usize = 1000;
const W: usize = 16; // 16 * 64 = 1024 >= N + 1

#[inline(always)]
unsafe fn test(row: &[u64; W], j: usize) -> bool {
    unsafe { *row.get_unchecked(j >> 6) & (1u64 << (j & 63)) != 0 }
}

#[inline(always)]
unsafe fn set(bits: &mut [[u64; W]], i: usize, j: usize) {
    unsafe {
        *bits.get_unchecked_mut(i).get_unchecked_mut(j >> 6) |= 1u64 << (j & 63);
    }
}

/// dest = src shifted left by `sh` bits (bit i -> bit i+sh).
#[inline(always)]
unsafe fn shl(src: &[u64; W], sh: usize, dest: &mut [u64; W]) {
    dest.fill(0);
    let ww = sh >> 6;
    let b = sh & 63;
    if ww >= W {
        return;
    }
    unsafe {
        if b == 0 {
            for i in 0..W - ww {
                *dest.get_unchecked_mut(i + ww) = *src.get_unchecked(i);
            }
        } else {
            let rb = 64 - b;
            let mut prev = *src.get_unchecked(0);
            *dest.get_unchecked_mut(ww) = prev << b;
            for i in 1..W - ww {
                let cur = *src.get_unchecked(i);
                *dest.get_unchecked_mut(i + ww) = (cur << b) | (prev >> rb);
                prev = cur;
            }
        }
    }
}

/// dest = (src_a | src_b) shifted left by `sh` bits.
#[inline(always)]
unsafe fn shl_or(src_a: &[u64; W], src_b: &[u64; W], sh: usize, dest: &mut [u64; W]) {
    dest.fill(0);
    let ww = sh >> 6;
    let b = sh & 63;
    if ww >= W {
        return;
    }
    unsafe {
        if b == 0 {
            for i in 0..W - ww {
                *dest.get_unchecked_mut(i + ww) = src_a.get_unchecked(i) | src_b.get_unchecked(i);
            }
        } else {
            let rb = 64 - b;
            let mut prev = src_a.get_unchecked(0) | src_b.get_unchecked(0);
            *dest.get_unchecked_mut(ww) = prev << b;
            for i in 1..W - ww {
                let cur = src_a.get_unchecked(i) | src_b.get_unchecked(i);
                *dest.get_unchecked_mut(i + ww) = (cur << b) | (prev >> rb);
                prev = cur;
            }
        }
    }
}

fn main() {
    let mut lines = vec![[0u64; W]; N + 1];
    let mut diags = vec![[0u64; W]; N + 1];
    let mut diags_t = vec![[0u64; W]; N + 1];
    let mut space = vec![[0u64; W]; N + 1];

    let mut ans: i64 = 0;
    let n_word = N >> 6;
    let n_keep = u64::MAX >> (63 - (N & 63));

    for x in 0..=N {
        for y in x..=N {
            // SAFETY: x,y,z and the diffs z-y, z-x, y-x are in 0..=N by the
            // loop bounds (x <= y <= z <= N). Each row has W words covering
            // 1024 bits, so word index j>>6 and bit j&63 are in range.
            unsafe {
                if test(lines.get_unchecked(x), y) {
                    continue;
                }

                let ymx = y - x;
                let lx = *lines.get_unchecked(x);
                let ly = *lines.get_unchecked(y);
                let dx = *diags.get_unchecked(x);
                let dy = *diags.get_unchecked(y);
                let sp = *space.get_unchecked(ymx);
                let dt = *diags_t.get_unchecked(ymx);

                // Align row-wise occupancy onto bit-index z:
                //   diags[x][z-y] | space[y-x][z-y]  ->  << y
                //   diags[y][z-x]                    ->  << x
                //   diags[z][y-x]                    ->  diags_t[y-x][z]
                let mut sh_dxsp = [0u64; W];
                let mut sh_dy = [0u64; W];
                shl_or(&dx, &sp, y, &mut sh_dxsp);
                shl(&dy, x, &mut sh_dy);

                let y_word = y >> 6;
                let y_mask = u64::MAX << (y & 63);

                for wi in y_word..=n_word {
                    let mut open = !(lx.get_unchecked(wi)
                        | ly.get_unchecked(wi)
                        | sh_dxsp.get_unchecked(wi)
                        | sh_dy.get_unchecked(wi)
                        | dt.get_unchecked(wi));
                    if wi == y_word {
                        open &= y_mask;
                    }
                    if wi == n_word {
                        open &= n_keep;
                    }
                    if open != 0 {
                        let z = (wi << 6) + open.trailing_zeros() as usize;
                        let dxy = z - y;
                        let dyx = z - x;
                        set(&mut lines, x, y);
                        set(&mut lines, x, z);
                        set(&mut lines, y, z);
                        set(&mut diags, x, dxy);
                        set(&mut diags, y, dyx);
                        set(&mut diags, z, ymx);
                        set(&mut diags_t, ymx, z);
                        set(&mut space, ymx, dxy);
                        ans += (x + y + z) as i64;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
