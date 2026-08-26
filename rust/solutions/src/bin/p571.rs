// Project Euler 571 - Super Pandigital Numbers
//
// Sum of the 10 smallest numbers pandigital in every base 2..=12.
// Generate 12-digit base-12 permutations (no leading zero) in value order
// and test remaining bases. Parallel over the first three digits; each
// worker keeps its 10 smallest hits, then results are merged.

use rayon::prelude::*;

const BASE: u32 = 12;
const K: usize = 10;
const ALL12: u32 = (1u32 << BASE) - 1;

const POW12: [u64; 13] = {
    let mut p = [1u64; 13];
    let mut i = 1;
    while i < 13 {
        p[i] = p[i - 1] * 12;
        i += 1;
    }
    p
};

fn digit_mask_padded(n: u64, b: u64, positions: usize) -> u16 {
    let mut seen = 0u16;
    let mut val = n;
    for _ in 0..positions {
        seen |= 1 << (val % b);
        val /= b;
    }
    seen
}

fn digit_mask(mut n: u64, b: u64) -> u16 {
    let mut seen = 0u16;
    if n == 0 {
        return 1;
    }
    while n > 0 {
        seen |= 1 << (n % b);
        n /= b;
    }
    seen
}

const B11_HI_POW: u64 = 19_487_171; // 11^7
const B11_LO_POW: u64 = 14_641; // 11^4
const B11_HI_MAX: usize = 460_000;
const B11_MID_MAX: usize = 1331; // 11^3
const B11_LO_MAX: usize = 14_641; // 11^4

struct Table11 {
    hi: Vec<u16>,
    mid: Vec<u16>,
    lo: Vec<u16>,
}

fn build_table_11() -> Table11 {
    let mut hi = vec![0u16; B11_HI_MAX];
    for i in 0..B11_HI_MAX {
        hi[i] = digit_mask(i as u64, 11);
    }
    let mut mid = vec![0u16; B11_MID_MAX];
    for i in 0..B11_MID_MAX {
        mid[i] = digit_mask_padded(i as u64, 11, 3);
    }
    let mut lo = vec![0u16; B11_LO_MAX];
    for i in 0..B11_LO_MAX {
        lo[i] = digit_mask_padded(i as u64, 11, 4);
    }
    Table11 { hi, mid, lo }
}

/// Sorted list of at most K hits. DFS visits digits in increasing order, so
/// pushes are already sorted; extras beyond K are larger and dropped.
#[derive(Clone, Copy)]
struct Found {
    v: [u64; K],
    n: usize,
}

impl Found {
    fn new() -> Self {
        Self { v: [0; K], n: 0 }
    }

    #[inline(always)]
    fn push(&mut self, x: u64) {
        if self.n < K {
            self.v[self.n] = x;
            self.n += 1;
        }
    }

    fn merge(&mut self, other: &Found) {
        let mut tmp = [0u64; K];
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while k < K && i < self.n && j < other.n {
            if self.v[i] <= other.v[j] {
                tmp[k] = self.v[i];
                i += 1;
            } else {
                tmp[k] = other.v[j];
                j += 1;
            }
            k += 1;
        }
        while k < K && i < self.n {
            tmp[k] = self.v[i];
            i += 1;
            k += 1;
        }
        while k < K && j < other.n {
            tmp[k] = other.v[j];
            j += 1;
            k += 1;
        }
        self.v = tmp;
        self.n = k;
    }
}

#[inline(always)]
fn is_pandigital_11_fast(n: u64, t: &Table11) -> bool {
    let hi = (n / B11_HI_POW) as usize;
    let rem = n % B11_HI_POW;
    let mid = (rem / B11_LO_POW) as usize;
    let lo = (rem % B11_LO_POW) as usize;
    // SAFETY: n < 12^12 so hi < 457537 < B11_HI_MAX; mid < 11^3; lo < 11^4
    let seen =
        unsafe { *t.hi.get_unchecked(hi) | *t.mid.get_unchecked(mid) | *t.lo.get_unchecked(lo) };
    seen == 0x7FF
}

#[inline(always)]
fn is_pandigital_8(mut n: u64) -> bool {
    let mut seen = 0u16;
    while n >= 8 {
        seen |= 1 << (n & 7);
        n >>= 3;
        if seen == 0xFF {
            return true;
        }
    }
    (seen | (1 << n)) == 0xFF
}

#[inline(always)]
fn is_pandigital_4(mut n: u64) -> bool {
    let mut seen = 0u16;
    while n >= 4 {
        seen |= 1 << (n & 3);
        n >>= 2;
        if seen == 0xF {
            return true;
        }
    }
    (seen | (1 << n)) == 0xF
}

#[inline(always)]
fn is_pandigital_3(mut n: u64) -> bool {
    let mut seen = 0u16;
    while n >= 3 {
        seen |= 1 << (n % 3);
        n /= 3;
        if seen == 0x7 {
            return true;
        }
    }
    (seen | (1 << n)) == 0x7
}

#[inline(always)]
fn is_pandigital<const B: u32>(mut n: u64) -> bool {
    let base = B as u64;
    let target = (1u16 << B) - 1;
    let mut seen = 0u16;
    while n >= base {
        seen |= 1 << (n % base);
        n /= base;
        if seen == target {
            return true;
        }
    }
    (seen | (1u16 << n)) == target
}

/// Base 8 first (shift/mask, high reject rate), then base 11 table, then 10..3.
#[inline(always)]
fn check_all_bases(n: u64, t11: &Table11) -> bool {
    if !is_pandigital_8(n) {
        return false;
    }
    if !is_pandigital_11_fast(n, t11) {
        return false;
    }
    if !is_pandigital::<10>(n) {
        return false;
    }
    if !is_pandigital::<9>(n) {
        return false;
    }
    if !is_pandigital::<7>(n) {
        return false;
    }
    if !is_pandigital::<6>(n) {
        return false;
    }
    if !is_pandigital::<5>(n) {
        return false;
    }
    if !is_pandigital_4(n) {
        return false;
    }
    if !is_pandigital_3(n) {
        return false;
    }
    true
}

#[inline]
fn base11_feasible(n_partial: u64, remaining: u32) -> bool {
    let pow = POW12[remaining as usize];
    let lo = n_partial * pow;
    let hi = lo + pow - 1;

    let mut p11 = 1u64;
    let mut ndigits: u32 = 0;
    while p11 * 11 <= hi {
        p11 *= 11;
        ndigits += 1;
    }
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

/// Last 4 unused digits, extracted low-bit-first so nested index order is increasing.
#[inline(always)]
fn leaf4(n: u64, vis: u32, found: &mut Found, t11: &Table11) {
    let mut ds = [0u64; 4];
    let mut rem = (!vis) & ALL12;
    let mut k = 0usize;
    while rem != 0 {
        let b = rem & rem.wrapping_neg();
        ds[k] = b.trailing_zeros() as u64;
        k += 1;
        rem ^= b;
    }
    for a in 0..4 {
        let n1 = n * 12 + ds[a];
        for b in 0..4 {
            if b == a {
                continue;
            }
            let n2 = n1 * 12 + ds[b];
            for c in 0..4 {
                if c == a || c == b {
                    continue;
                }
                let d = 6 - a - b - c;
                let n4 = (n2 * 12 + ds[c]) * 12 + ds[d];
                if check_all_bases(n4, t11) {
                    found.push(n4);
                    if found.n >= K {
                        return;
                    }
                }
            }
        }
    }
}

macro_rules! def_dfs {
    ($name:ident, $next:ident) => {
        #[inline(always)]
        fn $name(n: u64, visited: u32, found: &mut Found, t11: &Table11) {
            if found.n >= K {
                return;
            }
            let mut remaining = (!visited) & ALL12;
            while remaining != 0 {
                let bit = remaining & remaining.wrapping_neg();
                $next(
                    n * 12 + bit.trailing_zeros() as u64,
                    visited | bit,
                    found,
                    t11,
                );
                if found.n >= K {
                    return;
                }
                remaining ^= bit;
            }
        }
    };
}

#[inline(always)]
fn dfs8(n: u64, visited: u32, found: &mut Found, t11: &Table11) {
    if found.n >= K {
        return;
    }
    if !base11_feasible(n, 4) {
        return;
    }
    leaf4(n, visited, found, t11);
}

#[inline(always)]
fn dfs7(n: u64, visited: u32, found: &mut Found, t11: &Table11) {
    if found.n >= K {
        return;
    }
    if !base11_feasible(n, 5) {
        return;
    }
    let mut remaining = (!visited) & ALL12;
    while remaining != 0 {
        let bit = remaining & remaining.wrapping_neg();
        dfs8(
            n * 12 + bit.trailing_zeros() as u64,
            visited | bit,
            found,
            t11,
        );
        if found.n >= K {
            return;
        }
        remaining ^= bit;
    }
}

def_dfs!(dfs3, dfs4);
def_dfs!(dfs4, dfs5);
def_dfs!(dfs5, dfs6);
def_dfs!(dfs6, dfs7);

fn search_d0_range(d0_lo: u32, d0_hi: u32, t11: &Table11) -> Found {
    let mut jobs = Vec::with_capacity(((d0_hi - d0_lo) as usize) * 110);
    for d0 in d0_lo..d0_hi {
        for d1 in 0..BASE {
            if d1 == d0 {
                continue;
            }
            for d2 in 0..BASE {
                if d2 == d0 || d2 == d1 {
                    continue;
                }
                jobs.push((d0, d1, d2));
            }
        }
    }
    jobs.into_par_iter()
        .map(|(d0, d1, d2)| {
            let n = (d0 as u64 * 12 + d1 as u64) * 12 + d2 as u64;
            let vis = (1u32 << d0) | (1u32 << d1) | (1u32 << d2);
            let mut found = Found::new();
            dfs3(n, vis, &mut found, t11);
            found
        })
        .reduce(Found::new, |mut a, b| {
            a.merge(&b);
            a
        })
}

fn main() {
    let t11 = build_table_11();
    // Search low leading-digit bands first; expand only if fewer than K hits.
    let mut found = search_d0_range(1, 6, &t11);
    if found.n < K {
        found.merge(&search_d0_range(6, BASE, &t11));
    }
    let n = found.n.min(K);
    let ans: u64 = found.v[..n].iter().sum();
    println!("{}", ans);
}
