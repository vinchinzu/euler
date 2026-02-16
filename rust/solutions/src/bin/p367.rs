// Project Euler 367 - Bozo Sort
//
// Expected number of 3-element shuffles to sort a random permutation of 1..11.
//
// Uses representation theory of S_n:
//   E_avg = sum_{lambda != trivial} d_lambda^2 / (1 - mu_lambda)
// where d_lambda = dimension of irrep lambda, and
//   mu_lambda = 1/6 + (1/2)*(chi_lambda(trans)/d_lambda) + (1/3)*(chi_lambda(3-cyc)/d_lambda)
//
// Character values computed via the Murnaghan-Nakayama rule.
// For n=11, there are 56 partitions (irreps).

use std::collections::HashMap;

/// Generate all partitions of n in descending order
fn partitions(n: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    fn helper(n: usize, max_val: usize, current: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
        if n == 0 {
            result.push(current.clone());
            return;
        }
        for first in (1..=n.min(max_val)).rev() {
            current.push(first);
            helper(n - first, first, current, result);
            current.pop();
        }
    }
    helper(n, n, &mut current, &mut result);
    result
}

/// Dimension of irrep corresponding to partition using hook length formula
fn dimension(lam: &[usize]) -> i64 {
    let n: usize = lam.iter().sum();
    if n == 0 {
        return 1;
    }
    let mut num: i64 = 1;
    for i in 1..=n {
        num *= i as i64;
    }
    let mut denom: i64 = 1;
    for (i, &part) in lam.iter().enumerate() {
        for j in 0..part {
            let arm = part - j - 1;
            let leg = lam.iter().skip(i + 1).filter(|&&p| p > j).count();
            let hook = arm + leg + 1;
            denom *= hook as i64;
        }
    }
    num / denom
}

/// Find removable border strips of size r from partition lam
/// Returns Vec of (new_partition, height)
fn border_strips(lam: &[usize], r: usize) -> Vec<(Vec<usize>, usize)> {
    if r == 0 {
        return vec![(lam.to_vec(), 0)];
    }
    if lam.is_empty() {
        return vec![];
    }

    let n_rows = lam.len();

    // Build the rim: cells (i,j) where (i+1,j+1) is NOT in lambda
    let mut rim = Vec::new();
    for i in 0..n_rows {
        for j in 0..lam[i] {
            let in_next = i + 1 < n_rows && j + 1 < lam[i + 1];
            if !in_next {
                rim.push((i, j));
            }
        }
    }

    // Sort: column ascending, within same column row descending
    rim.sort_by_key(|&(i, j)| (j, -(i as isize)));

    let mut results = Vec::new();

    if rim.len() < r {
        return results;
    }

    for start in 0..=rim.len() - r {
        let strip = &rim[start..start + r];

        // Check connectivity
        let mut connected = true;
        for k in 0..strip.len() - 1 {
            let (r1, c1) = strip[k];
            let (r2, c2) = strip[k + 1];
            let dist = (r1 as isize - r2 as isize).unsigned_abs()
                + (c1 as isize - c2 as isize).unsigned_abs();
            if dist != 1 {
                connected = false;
                break;
            }
        }
        if !connected {
            continue;
        }

        // Count cells removed per row
        let mut removed_per_row: HashMap<usize, Vec<usize>> = HashMap::new();
        for &(i, j) in strip {
            removed_per_row.entry(i).or_default().push(j);
        }

        // Check removed cells are rightmost in each row
        let mut valid = true;
        for (&i, cols) in &mut removed_per_row {
            cols.sort();
            let expected: Vec<usize> = (lam[i] - cols.len()..lam[i]).collect();
            if *cols != expected {
                valid = false;
                break;
            }
        }
        if !valid {
            continue;
        }

        // Compute new partition
        let mut new_lam: Vec<usize> = lam.to_vec();
        for (&i, cols) in &removed_per_row {
            new_lam[i] -= cols.len();
        }

        // Check non-increasing BEFORE filtering zeros
        let mut non_incr = true;
        for k in 0..new_lam.len() - 1 {
            if new_lam[k] < new_lam[k + 1] {
                non_incr = false;
                break;
            }
        }
        if !non_incr {
            continue;
        }

        new_lam.retain(|&x| x > 0);

        let min_row = strip.iter().map(|&(i, _)| i).min().unwrap();
        let max_row = strip.iter().map(|&(i, _)| i).max().unwrap();
        let height = max_row - min_row;

        results.push((new_lam, height));
    }

    results
}

/// Murnaghan-Nakayama rule with memoization
fn murnaghan_nakayama(
    lam: &[usize],
    cycle_type: &[usize],
    cache: &mut HashMap<(Vec<usize>, Vec<usize>), i64>,
) -> i64 {
    let key = (lam.to_vec(), cycle_type.to_vec());
    if let Some(&val) = cache.get(&key) {
        return val;
    }

    let result = if cycle_type.is_empty() {
        if lam.is_empty() { 1 } else { 0 }
    } else if lam.is_empty() {
        0
    } else {
        let r = cycle_type[0];
        let remaining = &cycle_type[1..];
        let mut total: i64 = 0;
        for (new_lam, height) in border_strips(lam, r) {
            let sign: i64 = if height % 2 == 0 { 1 } else { -1 };
            total += sign * murnaghan_nakayama(&new_lam, remaining, cache);
        }
        total
    };

    cache.insert(key, result);
    result
}

fn main() {
    let n = 11;
    let parts = partitions(n);

    // Cycle types for transposition and 3-cycle in S_11
    let mut trans_ct = vec![2];
    trans_ct.extend(std::iter::repeat(1).take(n - 2));
    let mut three_ct = vec![3];
    three_ct.extend(std::iter::repeat(1).take(n - 3));

    let mut cache = HashMap::new();

    // E_avg = sum_{lambda != trivial} d^2 / (1 - mu)
    // mu = (d + 3*chi_t + 2*chi_3) / (6*d)
    // 1 - mu = (5*d - 3*chi_t - 2*chi_3) / (6*d)
    // d^2 / (1-mu) = 6*d^3 / (5*d - 3*chi_t - 2*chi_3)
    //
    // Use f64 for accumulation (answer ~4.8e7, f64 has ~15 significant digits)

    let mut total: f64 = 0.0;

    for lam in &parts {
        if lam.len() == 1 && lam[0] == n {
            continue; // Skip trivial representation
        }
        let d = dimension(lam);
        let chi_t = murnaghan_nakayama(lam, &trans_ct, &mut cache);
        let chi_3 = murnaghan_nakayama(lam, &three_ct, &mut cache);

        let d_f = d as f64;
        let denom = (5 * d - 3 * chi_t - 2 * chi_3) as f64;
        let contrib = 6.0 * d_f * d_f * d_f / denom;
        total += contrib;
    }

    println!("{}", total.round() as i64);
}
