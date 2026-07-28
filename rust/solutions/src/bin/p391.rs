// Project Euler 391 - Hopping Game
//
// M(n) = largest winning first move. Equivalent to scanning a popcount-derived
// sequence with accumulator s <- s+v if s+v <= n else 0; final s is M(n).
//
// Popcounts on [0, 2^k) have structure P(k) = P(k-1) || (P(k-1)+1). Scanning
// descending gives block transforms F(k,off) = F(k-1,off) ∘ F(k-1,off+1).
// Represent each transform as a table on {0..n}; compose in O(n). Once F(k,0)
// is constant, larger k stay constant (early saturation) — for n<=1000 this
// happens by k≈25, so each M(n) is O(n * k_sat) rather than O(n^3).

use rayon::prelude::*;

/// Mapping s |-> f(s) on {0..=n}.
enum Map {
    Id,
    Const(u16),
    Table(Vec<u16>),
}

#[inline]
fn apply(m: &Map, s: u16) -> u16 {
    match m {
        Map::Id => s,
        Map::Const(c) => *c,
        Map::Table(t) => t[s as usize],
    }
}

fn compose(a: &Map, b: &Map, n: usize) -> Map {
    // A ∘ B: s -> A(B(s))
    match (a, b) {
        (Map::Const(c), _) => Map::Const(*c),
        (_, Map::Const(c)) => Map::Const(apply(a, *c)),
        (Map::Id, other) => match other {
            Map::Id => Map::Id,
            Map::Const(c) => Map::Const(*c),
            Map::Table(t) => Map::Table(t.clone()),
        },
        (other, Map::Id) => match other {
            Map::Id => Map::Id,
            Map::Const(c) => Map::Const(*c),
            Map::Table(t) => Map::Table(t.clone()),
        },
        (Map::Table(ta), Map::Table(tb)) => {
            let mut out = vec![0u16; n + 1];
            for s in 0..=n {
                out[s] = ta[tb[s] as usize];
            }
            // Collapse to Const if uniform (helps early saturation detection
            // and shrinks later compositions).
            let c0 = out[0];
            if out.iter().all(|&x| x == c0) {
                Map::Const(c0)
            } else {
                Map::Table(out)
            }
        }
    }
}

fn m_of(n: usize) -> i64 {
    // Base k=0: sequence length 1 with value = off (popcount structure offset).
    // off=0 is identity (value 0 never changes accumulator via +0, but the
    // recurrence uses off as the additive offset in the reset rule).
    //
    // For offset `off`, one step: s -> s+off if s+off <= n else 0.
    // Need offsets up to MAX_K+1 for the recurrence's off+1 look-ahead.
    const MAX_K: usize = 40;
    let width = MAX_K + 2;

    let mut prev: Vec<Map> = Vec::with_capacity(width);
    for off in 0..width {
        if off == 0 {
            prev.push(Map::Id);
        } else if off > n {
            // Adding off always overflows => constant 0
            prev.push(Map::Const(0));
        } else {
            let mut t = vec![0u16; n + 1];
            let lim = n - off;
            for s in 0..=lim {
                t[s] = (s + off) as u16;
            }
            // s > lim already 0
            prev.push(Map::Table(t));
        }
    }

    for _k in 1..=MAX_K {
        let mut curr: Vec<Map> = Vec::with_capacity(width);
        for off in 0..width - 1 {
            // F(k,off) = F(k-1,off) ∘ F(k-1,off+1)
            // Scan order: upper half (off+1) first, then lower (off).
            // Composition A∘B means apply B first: so B = upper = off+1, A = off.
            curr.push(compose(&prev[off], &prev[off + 1], n));
        }
        curr.push(Map::Const(0)); // unused boundary

        match &curr[0] {
            Map::Const(c) => return *c as i64,
            Map::Id => return 0, // identity at root would mean M=0 only if start at 0; shouldn't happen
            Map::Table(t) => {
                let c0 = t[0];
                if t.iter().all(|&x| x == c0) {
                    return c0 as i64;
                }
            }
        }
        prev = curr;
    }
    panic!("no saturation for n={n}; increase MAX_K");
}

fn main() {
    let ans: i64 = (1..=1000usize)
        .into_par_iter()
        .map(|n| {
            let x = m_of(n);
            x * x * x
        })
        .sum();
    println!("{}", ans);
}
