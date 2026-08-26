// Project Euler 259: Reachable Numbers
use fxhash::FxHashSet;
use rayon::prelude::*;

const BASE: usize = 10;
const BOUND: u64 = 4_000_000_000_000_000_000;

#[derive(Clone, Copy)]
struct Frac {
    num: i64,
    den: i64,
}

struct FracSet {
    map: FxHashSet<u128>,
    list: Vec<Frac>,
}

impl FracSet {
    fn with_capacity(cap: usize) -> Self {
        Self {
            map: FxHashSet::with_capacity_and_hasher(cap, Default::default()),
            list: Vec::with_capacity(cap),
        }
    }

    #[inline(always)]
    fn insert(&mut self, f: Frac) {
        if self.map.insert(pack(f.num, f.den)) {
            self.list.push(f);
        }
    }
}

impl Default for FracSet {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

#[inline(always)]
fn cell(start: usize, end: usize) -> usize {
    start * BASE + end
}

#[inline(always)]
fn pack(num: i64, den: i64) -> u128 {
    ((num as u128) << 64) | (den as u64 as u128)
}

#[inline(always)]
fn gcd_i64(a: i64, b: i64) -> i64 {
    let mut a = a.unsigned_abs();
    let mut b = b.unsigned_abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a as i64
}

#[inline(always)]
fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn capacity_for(length: usize) -> usize {
    match length {
        0 | 1 => 4,
        2 | 3 | 4 => 512,
        5 => 2048,
        6 => 16_384,
        7 => 65_536,
        8 => 500_000,
        _ => 4_000_000,
    }
}

#[inline(always)]
fn insert_normalized(set: &mut FracSet, mut n: i64, mut d: i64) {
    if d < 0 {
        n = -n;
        d = -d;
    }
    if n == 0 {
        d = 1;
    }
    set.insert(Frac { num: n, den: d });
}

/// Reduce then insert. Bound check matches the original 4e18 cutoff.
#[inline(always)]
fn insert_reduced_i64(set: &mut FracSet, mut n: i64, mut d: i64) {
    if d == 0 {
        return;
    }
    let g = gcd_i64(n, d);
    if g > 1 {
        n /= g;
        d /= g;
    }
    if n.unsigned_abs() > BOUND || d.unsigned_abs() > BOUND {
        return;
    }
    insert_normalized(set, n, d);
}

#[inline(always)]
fn try_add(set: &mut FracSet, n: i128, d: i128) {
    if d == 0 {
        return;
    }
    if n >= i64::MIN as i128 && n <= i64::MAX as i128 && d >= i64::MIN as i128 && d <= i64::MAX as i128 {
        insert_reduced_i64(set, n as i64, d as i64);
        return;
    }
    let g = gcd_u128(n.unsigned_abs(), d.unsigned_abs());
    let (mut n, mut d) = (n, d);
    if g > 1 {
        let g = g as i128;
        n /= g;
        d /= g;
    }
    if n.unsigned_abs() > BOUND as u128 || d.unsigned_abs() > BOUND as u128 {
        return;
    }
    insert_normalized(set, n as i64, d as i64);
}

/// Mul/div of already-reduced fractions with pre-cancel is reduced; skip gcd.
#[inline(always)]
fn try_insert_reduced(set: &mut FracSet, n: i128, d: i128) {
    if d == 0 {
        return;
    }
    if n.unsigned_abs() > BOUND as u128 || d.unsigned_abs() > BOUND as u128 {
        return;
    }
    insert_normalized(set, n as i64, d as i64);
}

#[inline(always)]
fn add_ops(set: &mut FracSet, a: Frac, b: Frac) {
    try_add(
        set,
        a.num as i128 * b.den as i128 + b.num as i128 * a.den as i128,
        a.den as i128 * b.den as i128,
    );
    try_add(
        set,
        a.num as i128 * b.den as i128 - b.num as i128 * a.den as i128,
        a.den as i128 * b.den as i128,
    );
    {
        let g1 = gcd_i64(a.num, b.den);
        let g2 = gcd_i64(b.num, a.den);
        try_insert_reduced(
            set,
            (a.num / g1) as i128 * (b.num / g2) as i128,
            (a.den / g2) as i128 * (b.den / g1) as i128,
        );
    }
    if b.num != 0 {
        let g1 = gcd_i64(a.num, b.num);
        let g2 = gcd_i64(a.den, b.den);
        try_insert_reduced(
            set,
            (a.num / g1) as i128 * (b.den / g2) as i128,
            (a.den / g2) as i128 * (b.num / g1) as i128,
        );
    }
}

fn concat_digits(start: usize, end: usize) -> i64 {
    let mut concat = 0i64;
    for d in start..end {
        concat = concat * 10 + d as i64 + 1;
    }
    concat
}

fn merge(mut a: FracSet, mut b: FracSet) -> FracSet {
    if a.list.len() < b.list.len() {
        std::mem::swap(&mut a, &mut b);
    }
    for f in b.list {
        a.insert(f);
    }
    a
}

fn combine_splits(cur: &mut FracSet, start: usize, end: usize, sets: &[FracSet]) {
    let length = end - start;
    for left in 1..length {
        let mid = start + left;
        let lset = &sets[cell(start, mid)].list;
        let rset = &sets[cell(mid, end)].list;
        for &fa in lset {
            for &fb in rset {
                add_ops(cur, fa, fb);
            }
        }
    }
}

fn build_cell(start: usize, length: usize, sets: &[FracSet]) -> FracSet {
    let end = start + length;
    let mut cur = FracSet::with_capacity(capacity_for(length));
    cur.insert(Frac {
        num: concat_digits(start, end),
        den: 1,
    });
    combine_splits(&mut cur, start, end, sets);
    cur
}

/// Length-9 bottleneck: chunk left operands so rayon can balance uneven splits.
fn build_cell_parallel(start: usize, length: usize, sets: &[FracSet]) -> FracSet {
    let end = start + length;
    let nthreads = rayon::current_num_threads().max(1);

    let mut units = Vec::new();
    for left in 1..length {
        let mid = start + left;
        let n = sets[cell(start, mid)].list.len();
        if n == 0 {
            continue;
        }
        let chunk = (n / nthreads).max(1);
        let mut off = 0;
        while off < n {
            let hi = (off + chunk).min(n);
            units.push((mid, off, hi));
            off = hi;
        }
    }

    let mut cur = units
        .into_par_iter()
        .fold(
            || FracSet::with_capacity(1 << 18),
            |mut acc, (mid, lo, hi)| {
                let lset = &sets[cell(start, mid)].list;
                let rset = &sets[cell(mid, end)].list;
                for &fa in &lset[lo..hi] {
                    for &fb in rset {
                        add_ops(&mut acc, fa, fb);
                    }
                }
                acc
            },
        )
        .reduce(FracSet::default, merge);

    cur.insert(Frac {
        num: concat_digits(start, end),
        den: 1,
    });
    cur
}

fn main() {
    let mut sets: Vec<FracSet> = (0..BASE * BASE).map(|_| FracSet::with_capacity(0)).collect();

    for i in 0..BASE - 1 {
        let mut s = FracSet::with_capacity(1);
        s.insert(Frac {
            num: i as i64 + 1,
            den: 1,
        });
        sets[cell(i, i + 1)] = s;
    }

    for length in 2..BASE {
        if length == BASE - 1 {
            sets[cell(0, BASE - 1)] = build_cell_parallel(0, length, &sets);
        } else if length >= 6 {
            let nstarts = BASE - length;
            let built: Vec<FracSet> = (0..nstarts)
                .into_par_iter()
                .map(|start| build_cell(start, length, &sets))
                .collect();
            for (start, s) in built.into_iter().enumerate() {
                sets[cell(start, start + length)] = s;
            }
        } else {
            for start in 0..BASE - length {
                let end = start + length;
                sets[cell(start, end)] = build_cell(start, length, &sets);
            }
        }
    }

    let mut ans = 0i64;
    for f in &sets[cell(0, BASE - 1)].list {
        if f.den == 1 && f.num > 0 {
            ans += f.num;
        }
    }
    println!("{}", ans);
}
