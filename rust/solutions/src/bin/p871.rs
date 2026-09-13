// Project Euler 871 - Functional Graph D(f_n)
// For n = 10^5+1 to 10^5+100, compute D(f_n) where f(x) = (x^3 + x + 1) % n.
// Functional graph decomposition into trees + cycles, DP for max independent antecedent set.

use rayon::prelude::*;
use std::collections::VecDeque;

fn solve_cycle(p: &[i32], s: &[i32], k: usize) -> i32 {
    if k == 0 { return 0; }

    let mut dp0 = unsafe { *s.get_unchecked(0) };
    let mut dp1 = -1_000_000_000i32;

    for i in 1..k {
        // SAFETY: i < k guaranteed by loop bounds; p and s have length k
        let (si, pi) = unsafe { (*s.get_unchecked(i), *p.get_unchecked(i)) };
        let next_dp0 = (dp0 + si).max(dp1 + pi - 1);
        let next_dp1 = dp0 + pi;
        dp0 = next_dp0;
        dp1 = next_dp1;
    }

    let res0 = dp0;
    // SAFETY: p and s have length k >= 1
    let (s0, p0) = unsafe { (*s.get_unchecked(0), *p.get_unchecked(0)) };
    let res1 = dp1 - s0 + (p0 - 1);
    let ans_case0 = res0.max(res1);

    dp0 = -1_000_000_000;
    dp1 = p0;

    for i in 1..k {
        // SAFETY: i < k guaranteed by loop bounds; p and s have length k
        let (si, pi) = unsafe { (*s.get_unchecked(i), *p.get_unchecked(i)) };
        let next_dp0 = (dp0 + si).max(dp1 + pi - 1);
        let next_dp1 = dp0 + pi;
        dp0 = next_dp0;
        dp1 = next_dp1;
    }

    let ans_case1 = dp0;

    ans_case0.max(ans_case1)
}

fn compute_d(n: usize) -> i32 {
    let mut f_map = vec![0usize; n];
    let mut in_degree = vec![0i32; n];
    
    let n64 = n as u64;
    for x in 0..n {
        let xx = x as u64;
        let val = ((xx * xx % n64) * xx % n64 + xx + 1) % n64;
        let val_usize = val as usize;
        // SAFETY: x < n guaranteed by loop bounds; f_map has length n
        unsafe { *f_map.get_unchecked_mut(x) = val_usize; }
        // SAFETY: val < n64 = n, so val_usize < n; in_degree has length n
        unsafe { *in_degree.get_unchecked_mut(val_usize) += 1; }
    }

    let mut sum_s = vec![0i32; n];
    let mut max_diff = vec![0i32; n];

    // Topological sort
    let mut queue = VecDeque::new();
    for x in 0..n {
        if in_degree[x] == 0 { queue.push_back(x); }
    }

    while let Some(u) = queue.pop_front() {
        // SAFETY: u comes from queue which only contains valid indices < n
        let (sum_s_u, max_diff_u, f_u) = unsafe {
            (*sum_s.get_unchecked(u), *max_diff.get_unchecked(u), *f_map.get_unchecked(u))
        };
        let p_u = 1 + sum_s_u;
        let s_u = sum_s_u + max_diff_u;
        let v = f_u;
        
        // SAFETY: v = f_map[u] < n, arrays have length n
        unsafe {
            *sum_s.get_unchecked_mut(v) += s_u;
            let diff = p_u - s_u;
            let md = max_diff.get_unchecked_mut(v);
            if diff > *md { *md = diff; }
            *in_degree.get_unchecked_mut(v) -= 1;
            if *in_degree.get_unchecked(v) == 0 { queue.push_back(v); }
        }
    }

    // Process cycles
    let mut visited = vec![false; n];
    let mut total_max = 0i32;

    for i in 0..n {
        if in_degree[i] > 0 && !visited[i] {
            let mut cycle_nodes = Vec::new();
            let mut curr = i;
            while !visited[curr] {
                visited[curr] = true;
                cycle_nodes.push(curr);
                curr = f_map[curr];
            }

            let k = cycle_nodes.len();
            let cycle_p: Vec<i32> = cycle_nodes.iter().map(|&node| 1 + sum_s[node]).collect();
            let cycle_s: Vec<i32> = cycle_nodes.iter().map(|&node| sum_s[node] + max_diff[node]).collect();

            total_max += solve_cycle(&cycle_p, &cycle_s, k);
        }
    }

    total_max
}

fn main() {
    let total_d: i64 = (100_001..=100_100)
        .into_par_iter()
        .map(|n| compute_d(n) as i64)
        .sum();
    println!("{}", total_d);
}
