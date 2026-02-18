// Problem 896 - Divisible Ranges
//
// A range [a..a+L-1] is "divisible" if its numbers can be permuted so that
// the n-th term is a multiple of n, for n = 1..L.
//
// We compute the 36th divisible range of length 36 and print the smallest number a.

use std::collections::HashSet;

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// CRT merge: a ≡ r1 (mod m1) and a ≡ r2 (mod m2).
/// Returns Some((r, lcm(m1,m2))) or None if inconsistent.
/// Uses i128 internally to avoid overflow since m1*m2 can be up to ~48+48 = 96 bits.
fn crt_merge(r1: i64, m1: i64, r2: i64, m2: i64) -> Option<(i64, i64)> {
    let g = gcd(m1, m2);
    if (r2 - r1) % g != 0 {
        return None;
    }
    let lcm = (m1 / g) * m2;
    let m1g = m1 / g;
    let m2g = m2 / g;
    let diff = (r2 - r1) / g;

    // Extended gcd to find inverse of m1g mod m2g
    let (_, inv, _) = extended_gcd(m1g, m2g);
    let inv = ((inv % m2g) + m2g) % m2g;

    // t = (diff * inv) % m2g, but use i128 to avoid overflow
    let t = ((diff as i128 * inv as i128) % m2g as i128) as i64;
    let t = ((t % m2g) + m2g) % m2g;

    // r = (r1 + m1 * t) % lcm
    let r = ((r1 as i128 + m1 as i128 * t as i128) % lcm as i128) as i64;
    let r = ((r % lcm) + lcm) % lcm;
    Some((r, lcm))
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut x0, mut y0, mut x1, mut y1) = (1i64, 0i64, 0i64, 1i64);
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let q = a / b;
        let tmp = b;
        b = a - q * b;
        a = tmp;
        let tmp = x1;
        x1 = x0 - q * x1;
        x0 = tmp;
        let tmp = y1;
        y1 = y0 - q * y1;
        y0 = tmp;
    }
    (a, x0, y0)
}

fn lcm_upto(n: i64) -> i64 {
    let mut m: i64 = 1;
    for i in 1..=n {
        m = (m / gcd(m, i)) * i;
    }
    m
}

/// Return offsets j in [0..L-1] such that j ≡ target (mod step) and bit j is set in unused_mask.
fn candidates_offsets(l: usize, unused_mask: u64, target: usize, step: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut j = target;
    while j < l {
        if (unused_mask >> j) & 1 != 0 {
            res.push(j);
        }
        j += step;
    }
    res
}

/// Pick the remaining index i (1..=L) with fewest feasible offsets (MRV heuristic).
/// Tie-break towards larger i.
/// Returns None if any remaining index has 0 candidates (dead end).
fn pick_index_mrv(
    l: usize,
    r: i64,
    m: i64,
    unused_mask: u64,
    remaining_mask: u64,
) -> Option<(usize, Vec<usize>)> {
    let mut best_i: usize = 0;
    let mut best_cands: Vec<usize> = Vec::new();
    let mut best_count: usize = usize::MAX;

    // Iterate indices descending for good tie-breaking
    for i in (1..=l).rev() {
        if (remaining_mask >> (i - 1)) & 1 == 0 {
            continue;
        }
        let g = gcd(m, i as i64) as usize;
        let target = if r == 0 { 0 } else { (g - (r as usize % g)) % g };
        let cands = candidates_offsets(l, unused_mask, target, g);
        let c = cands.len();
        if c == 0 {
            return None;
        }
        if c < best_count || (c == best_count && i > best_i) {
            best_count = c;
            best_i = i;
            best_cands = cands;
        }
    }

    if best_i == 0 {
        return None;
    }
    Some((best_i, best_cands))
}

fn enumerate_valid_residues(l: usize) -> (Vec<i64>, i64) {
    let big_m = lcm_upto(l as i64);
    let all_offsets_mask: u64 = (1u64 << l) - 1;
    let all_indices_mask: u64 = (1u64 << l) - 1;

    let mut residues: HashSet<i64> = HashSet::new();
    let mut visited: HashSet<(i64, i64, u64, u64)> = HashSet::new();

    fn dfs(
        l: usize,
        r: i64,
        m: i64,
        unused_mask: u64,
        remaining_mask: u64,
        residues: &mut HashSet<i64>,
        visited: &mut HashSet<(i64, i64, u64, u64)>,
    ) {
        let r = ((r % m) + m) % m;
        let key = (r, m, unused_mask, remaining_mask);
        if visited.contains(&key) {
            return;
        }
        visited.insert(key);

        if remaining_mask == 0 {
            residues.insert(r);
            return;
        }

        let pick = pick_index_mrv(l, r, m, unused_mask, remaining_mask);
        if pick.is_none() {
            return;
        }
        let (i, cands) = pick.unwrap();
        let remaining2 = remaining_mask & !(1u64 << (i - 1));

        for j in cands {
            let rhs = (-(j as i64)).rem_euclid(i as i64);
            let merged = crt_merge(r, m, rhs, i as i64);
            if let Some((r2, m2)) = merged {
                dfs(l, r2, m2, unused_mask & !(1u64 << j), remaining2, residues, visited);
            }
        }
    }

    dfs(
        l,
        0,
        1,
        all_offsets_mask,
        all_indices_mask,
        &mut residues,
        &mut visited,
    );

    let res: Vec<i64> = residues.into_iter().collect();
    (res, big_m)
}

fn nth_divisible_range_start(l: usize, n: usize) -> i64 {
    let (residues, big_m) = enumerate_valid_residues(l);
    let mut starts: Vec<i64> = residues
        .into_iter()
        .map(|r| if r > 0 { r } else { big_m })
        .collect();
    starts.sort();
    starts[n - 1]
}

fn main() {
    // Self-test with length 4
    debug_assert_eq!(nth_divisible_range_start(4, 1), 1);
    debug_assert_eq!(nth_divisible_range_start(4, 2), 2);
    debug_assert_eq!(nth_divisible_range_start(4, 3), 3);
    debug_assert_eq!(nth_divisible_range_start(4, 4), 6);

    println!("{}", nth_divisible_range_start(36, 36));
}
