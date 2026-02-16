// Project Euler 571 - Super Pandigital Numbers
//
// Find the sum of the smallest 10 numbers that are pandigital in all bases
// from 2 to 12.
//
// Optimizations:
// - Skip first digit = 0 (can't be pandigital in base 12 with leading zero)
// - Use u16 bitmask for visited tracking and pandigitality checks
// - At depth 8, do early base-11 feasibility check to prune subtrees
// - Precomputed 3-part lookup table for base-11 pandigitality
//   (split into hi/mid/lo parts, OR their digit masks)
// - Check most restrictive bases first

const BASE: u32 = 12;
const K: usize = 10;

const POW12: [i64; 13] = {
    let mut p = [1i64; 13];
    let mut i = 1;
    while i < 13 { p[i] = p[i - 1] * 12; i += 1; }
    p
};

/// Compute the digit mask for a chunk of exactly `positions` base-b digits.
/// Accounts for leading zeros in the chunk.
fn digit_mask_padded(n: i64, b: i64, positions: usize) -> u16 {
    let mut seen = 0u16;
    let mut val = n;
    for _ in 0..positions {
        seen |= 1 << (val % b);
        val /= b;
    }
    seen
}

/// Compute the bitmask of base-b digits present (no padding - for top chunk).
fn digit_mask(mut n: i64, b: i64) -> u16 {
    let mut seen = 0u16;
    if n == 0 { return 1; }
    while n > 0 { seen |= 1 << (n % b); n /= b; }
    seen
}

// 3-part split for base 11: n = hi * 11^7 + mid * 11^4 + lo
// hi: positions 7+ (variable # digits), mid: positions 4-6 (3 digits), lo: positions 0-3 (4 digits)
const B11_HI_POW: i64 = 19487171; // 11^7
const B11_LO_POW: i64 = 14641;    // 11^4
const B11_HI_MAX: usize = 460000;
const B11_MID_MAX: usize = 1331;  // 11^3
const B11_LO_MAX: usize = 14641;  // 11^4

struct Table11 {
    hi: Vec<u16>,
    mid: Vec<u16>,
    lo: Vec<u16>,
}

fn build_table_11() -> Table11 {
    let mut hi = vec![0u16; B11_HI_MAX];
    for i in 0..B11_HI_MAX { hi[i] = digit_mask(i as i64, 11); }
    let mut mid = vec![0u16; B11_MID_MAX];
    for i in 0..B11_MID_MAX { mid[i] = digit_mask_padded(i as i64, 11, 3); }
    let mut lo = vec![0u16; B11_LO_MAX];
    for i in 0..B11_LO_MAX { lo[i] = digit_mask_padded(i as i64, 11, 4); }
    Table11 { hi, mid, lo }
}

/// Fast base-11 pandigitality check: 3-part split, 3 table lookups, 2 divisions.
#[inline(always)]
fn is_pandigital_11_fast(n: i64, t: &Table11) -> bool {
    let hi = (n / B11_HI_POW) as usize;
    let rem = n % B11_HI_POW;
    let mid = (rem / B11_LO_POW) as usize;
    let lo = (rem % B11_LO_POW) as usize;
    // SAFETY: hi < B11_HI_MAX, mid < B11_MID_MAX, lo < B11_LO_MAX by construction
    let seen = unsafe {
        *t.hi.get_unchecked(hi) | *t.mid.get_unchecked(mid) | *t.lo.get_unchecked(lo)
    };
    seen == 0x7FF
}

/// Check if n is pandigital in the given base using bitmask.
#[inline(always)]
fn is_pandigital(mut n: i64, base: i64) -> bool {
    let target = (1u16 << base) - 1;
    let mut seen = 0u16;
    while n > 0 {
        seen |= 1 << (n % base);
        n /= base;
        if seen == target { return true; }
    }
    seen == target
}

/// After placing all 12 base-12 digits, check pandigitality in bases 2-11.
#[inline]
fn check_all_bases(n: i64, t11: &Table11) -> bool {
    if !is_pandigital_11_fast(n, t11) { return false; }
    if !is_pandigital(n, 10) { return false; }
    if !is_pandigital(n, 9) { return false; }
    if !is_pandigital(n, 8) { return false; }
    if !is_pandigital(n, 7) { return false; }
    if !is_pandigital(n, 6) { return false; }
    if !is_pandigital(n, 5) { return false; }
    if !is_pandigital(n, 4) { return false; }
    if !is_pandigital(n, 3) { return false; }
    true
}

/// Fast base-11 feasibility check at recursion depth 8.
#[inline]
fn base11_feasible(n_partial: i64, remaining: u32) -> bool {
    let pow = POW12[remaining as usize];
    let lo = n_partial * pow;
    let hi = lo + pow - 1;

    let mut p11 = 1i64;
    let mut ndigits: u32 = 0;
    while p11 * 11 <= hi { p11 *= 11; ndigits += 1; }
    ndigits += 1;

    let mut fixed_seen: u16 = 0;
    let mut l = lo;
    let mut h = hi;
    let mut unfixed = ndigits;

    while p11 > 0 {
        let dl = l / p11;
        let dh = h / p11;
        if dl == dh {
            fixed_seen |= 1u16 << dl;
            l -= dl * p11;
            h -= dh * p11;
            unfixed -= 1;
        } else {
            break;
        }
        p11 /= 11;
    }

    let missing = (0x7FFu16 & !fixed_seen).count_ones();
    missing <= unfixed
}

fn helper(index: u32, n: i64, visited: u16, count: &mut usize, ans: &mut i64, t11: &Table11) {
    if index == BASE {
        if check_all_bases(n, t11) {
            *count += 1;
            *ans += n;
        }
        return;
    }
    if *count >= K {
        return;
    }

    if index == 8 {
        if !base11_feasible(n, BASE - 8) {
            return;
        }
    }

    let mut remaining = (!visited) & ((1u16 << BASE) - 1);
    while remaining != 0 {
        let bit = remaining & remaining.wrapping_neg();
        let i = bit.trailing_zeros();

        if index == 0 && i == 0 {
            remaining ^= bit;
            continue;
        }

        helper(index + 1, n * BASE as i64 + i as i64, visited | bit, count, ans, t11);
        if *count >= K {
            return;
        }
        remaining ^= bit;
    }
}

fn main() {
    let t11 = build_table_11();
    let mut count = 0usize;
    let mut ans = 0i64;
    helper(0, 0, 0u16, &mut count, &mut ans, &t11);
    println!("{}", ans);
}
