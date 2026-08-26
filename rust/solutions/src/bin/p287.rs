// Project Euler 287: Quadtree Encoding
// Disk x^2+y^2 <= L^2 on the shifted square [-L, L)^2, L = 2^{N-1}.
// Uniform block: 2 bits; mixed: 1 + four children.
// A block is uniform iff it does not straddle the circle
// (min dist^2 > R^2 or max dist^2 <= R^2).

use rayon::join;

const N: u32 = 24;
const L: i32 = 1 << (N - 1);
const R2: i64 = {
    let l = 1i64 << (N - 1);
    l * l
};
/// Sequential DFS below this side length; larger mixed squares use rayon::join.
const SEQ_SIDE: i32 = 1 << 17;

#[inline(always)]
fn min_sq(lo: i32, hi: i32) -> i64 {
    let a = if lo > 0 {
        lo as i64
    } else if hi < 0 {
        -(hi as i64)
    } else {
        0
    };
    a * a
}

#[inline(always)]
fn max_sq(lo: i32, hi: i32) -> i64 {
    let a = lo.unsigned_abs().max(hi.unsigned_abs()) as i64;
    a * a
}

#[inline(always)]
fn uniform(x: i32, y: i32, s: i32) -> bool {
    let x1 = x + s - 1;
    let y1 = y + s - 1;
    min_sq(x, x1) + min_sq(y, y1) > R2 || max_sq(x, x1) + max_sq(y, y1) <= R2
}

fn len_seq(x0: i32, y0: i32, s0: i32) -> i64 {
    // DFS of a power-of-two quadtree: depth <= 24, so at most 1+3*24 pending nodes.
    let mut stack = [(0i32, 0i32, 0i32); 80];
    let mut sp = 1usize;
    stack[0] = (x0, y0, s0);
    let mut bits = 0i64;
    while sp > 0 {
        sp -= 1;
        // SAFETY: sp < 80; we push 4 nodes only when sp+3 < 80 (depth bound).
        let (x, y, s) = unsafe { *stack.get_unchecked(sp) };
        let x1 = x + s - 1;
        let y1 = y + s - 1;
        if min_sq(x, x1) + min_sq(y, y1) > R2 || max_sq(x, x1) + max_sq(y, y1) <= R2 {
            bits += 2;
            continue;
        }
        // Mixed 2x2 is always one split plus four 1x1 leaves.
        if s == 2 {
            bits += 9;
            continue;
        }
        bits += 1;
        let h = s >> 1;
        unsafe {
            *stack.get_unchecked_mut(sp) = (x, y, h);
            *stack.get_unchecked_mut(sp + 1) = (x + h, y, h);
            *stack.get_unchecked_mut(sp + 2) = (x, y + h, h);
            *stack.get_unchecked_mut(sp + 3) = (x + h, y + h, h);
        }
        sp += 4;
    }
    bits
}

fn len_enc(x: i32, y: i32, s: i32) -> i64 {
    if s <= SEQ_SIDE {
        return len_seq(x, y, s);
    }
    if uniform(x, y, s) {
        return 2;
    }
    let h = s >> 1;
    let (ab, cd) = join(
        || {
            let (a, b) = join(|| len_enc(x, y, h), || len_enc(x + h, y, h));
            a + b
        },
        || {
            let (c, d) = join(|| len_enc(x, y + h, h), || len_enc(x + h, y + h, h));
            c + d
        },
    );
    1 + ab + cd
}

fn main() {
    // Whole image [-L, L)^2 is mixed; four top-level quadrants are joined recursively.
    let ans = len_enc(-L, -L, L << 1);
    println!("{ans}");
}
