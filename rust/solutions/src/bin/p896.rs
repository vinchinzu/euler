// Problem 896 - Divisible Ranges
//
// A range [a..a+L-1] is "divisible" if its numbers can be permuted so that
// the n-th term is a multiple of n, for n = 1..L.
//
// We compute the 36th divisible range of length 36 and print the smallest number a.

use std::collections::HashMap;

#[inline(always)]
fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline]
fn crt_merge(r1: i64, m1: i64, r2: i64, m2: i64) -> Option<(i64, i64)> {
    let g = gcd(m1, m2);
    if (r2 - r1) % g != 0 {
        return None;
    }
    let lcm = (m1 / g) * m2;
    let m1g = m1 / g;
    let m2g = m2 / g;
    let diff = (r2 - r1) / g;

    let (_, inv, _) = extended_gcd(m1g, m2g);
    let inv = ((inv % m2g) + m2g) % m2g;

    let t = ((diff as i128 * inv as i128) % m2g as i128) as i64;
    let t = ((t % m2g) + m2g) % m2g;

    let r = ((r1 as i128 + m1 as i128 * t as i128) % lcm as i128) as i64;
    let r = ((r % lcm) + lcm) % lcm;
    Some((r, lcm))
}

#[inline]
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

#[inline]
fn candidates_offsets(l: usize, unused_mask: u64, target: usize, step: usize, buf: &mut Vec<usize>) {
    buf.clear();
    let mut j = target;
    while j < l {
        if (unused_mask >> j) & 1 != 0 {
            buf.push(j);
        }
        j += step;
    }
}

#[inline]
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
    let mut cand_buf = Vec::with_capacity(l);

    for i in (1..=l).rev() {
        if (remaining_mask >> (i - 1)) & 1 == 0 {
            continue;
        }
        let g = gcd(m, i as i64) as usize;
        let target = if r == 0 { 0 } else { (g - (r as usize % g)) % g };
        candidates_offsets(l, unused_mask, target, g, &mut cand_buf);
        let c = cand_buf.len();
        if c == 0 {
            return None;
        }
        if c < best_count || (c == best_count && i > best_i) {
            best_count = c;
            best_i = i;
            std::mem::swap(&mut best_cands, &mut cand_buf);
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

    let mut residues: Vec<i64> = Vec::new();
    let mut visited: HashMap<(i64, i64, u64, u64), ()> = HashMap::new();

    fn dfs(
        l: usize,
        r: i64,
        m: i64,
        unused_mask: u64,
        remaining_mask: u64,
        residues: &mut Vec<i64>,
        visited: &mut HashMap<(i64, i64, u64, u64), ()>,
    ) {
        let r = ((r % m) + m) % m;
        let key = (r, m, unused_mask, remaining_mask);
        if visited.contains_key(&key) {
            return;
        }
        visited.insert(key, ());

        if remaining_mask == 0 {
            residues.push(r);
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

    residues.sort_unstable();
    residues.dedup();
    (residues, big_m)
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
