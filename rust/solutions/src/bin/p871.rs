// Project Euler 871 - Functional Graph D(f_n)
// For n = 10^5+1 to 10^5+100, compute D(f_n) where f(x) = (x^3 + x + 1) % n.
// Functional graph decomposition into trees + cycles, DP for max independent antecedent set.

use std::collections::VecDeque;

fn solve_cycle(p: &[i32], s: &[i32], k: usize) -> i32 {
    if k == 0 { return 0; }

    // Case 1: c0 is NOT in A
    let mut dp0 = s[0];
    let mut dp1 = -1_000_000_000i32;

    for i in 1..k {
        let next_dp0 = (dp0 + s[i]).max(dp1 + p[i] - 1);
        let next_dp1 = dp0 + p[i];
        dp0 = next_dp0;
        dp1 = next_dp1;
    }

    let res0 = dp0;
    let res1 = dp1 - s[0] + (p[0] - 1);
    let ans_case0 = res0.max(res1);

    // Case 2: c0 IS in A
    dp0 = -1_000_000_000;
    dp1 = p[0];

    for i in 1..k {
        let next_dp0 = (dp0 + s[i]).max(dp1 + p[i] - 1);
        let next_dp1 = dp0 + p[i];
        dp0 = next_dp0;
        dp1 = next_dp1;
    }

    let ans_case1 = dp0;

    ans_case0.max(ans_case1)
}

fn compute_d(n: usize) -> i32 {
    let mut f_map = vec![0usize; n];
    for x in 0..n {
        let xx = x as u64;
        let val = ((xx * xx % n as u64) * xx % n as u64 + xx + 1) % n as u64;
        f_map[x] = val as usize;
    }

    let mut in_degree = vec![0i32; n];
    for x in 0..n { in_degree[f_map[x]] += 1; }

    let mut sum_s = vec![0i32; n];
    let mut max_diff = vec![0i32; n];

    // Topological sort
    let mut queue = VecDeque::new();
    for x in 0..n {
        if in_degree[x] == 0 { queue.push_back(x); }
    }

    while let Some(u) = queue.pop_front() {
        let p_u = 1 + sum_s[u];
        let s_u = sum_s[u] + max_diff[u];
        let v = f_map[u];
        sum_s[v] += s_u;
        let diff = p_u - s_u;
        if diff > max_diff[v] { max_diff[v] = diff; }
        in_degree[v] -= 1;
        if in_degree[v] == 0 { queue.push_back(v); }
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
    let mut total_d = 0i64;
    for n in 100_001..=100_100 {
        total_d += compute_d(n) as i64;
    }
    println!("{}", total_d);
}
