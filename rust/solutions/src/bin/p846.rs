// Project Euler 846 - Graph Potency
// Build graph on allowed numbers, find 2-core, enumerate cycles, sum potency.
// Deep work decomposition for effective parallelism.

use rayon::prelude::*;
use std::collections::VecDeque;

const MAXN: usize = 1_000_001;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn dfs_cycle(
    u: u32, start: u32, current_sum: i64, length: i32,
    path_vis: &mut [bool], sub_offset: &[u32], sub_data: &[u32], node_vals: &[i64],
) -> i64 {
    let mut pot = 0i64;
    let off = unsafe { *sub_offset.get_unchecked(u as usize) };
    let end = unsafe { *sub_offset.get_unchecked(u as usize + 1) };
    for ei in off..end {
        let w = unsafe { *sub_data.get_unchecked(ei as usize) };
        if w == start {
            if length >= 3 { pot += current_sum; }
        } else if w > start {
            let ww = w as usize;
            if !unsafe { *path_vis.get_unchecked(ww) } {
                unsafe { *path_vis.get_unchecked_mut(ww) = true; }
                pot += dfs_cycle(
                    w, start,
                    current_sum + unsafe { *node_vals.get_unchecked(ww) },
                    length + 1, path_vis, sub_offset, sub_data, node_vals,
                );
                unsafe { *path_vis.get_unchecked_mut(ww) = false; }
            }
        }
    }
    pot
}

fn main() {
    let n_val = 1_000_000usize;

    // Sieve
    let mut min_prime = vec![0i32; n_val + 1];
    let mut primes = Vec::new();
    for i in 2..=n_val {
        if min_prime[i] == 0 {
            min_prime[i] = i as i32;
            primes.push(i as i32);
            let mut j = i * i;
            while j <= n_val {
                if min_prime[j] == 0 { min_prime[j] = i as i32; }
                j += i;
            }
        }
    }

    // Build allowed set
    let mut is_allowed = vec![false; MAXN];
    is_allowed[1] = true;
    is_allowed[2] = true;
    for &p in &primes {
        if p == 2 { continue; }
        let mut pk = p as i64;
        while pk <= n_val as i64 {
            is_allowed[pk as usize] = true;
            if 2 * pk <= n_val as i64 { is_allowed[(2 * pk) as usize] = true; }
            pk *= p as i64;
        }
    }

    let mut allowed_set: Vec<i32> = Vec::new();
    for i in 1..=n_val {
        if is_allowed[i] { allowed_set.push(i as i32); }
    }
    let nallowed = allowed_set.len();

    let mut val_to_idx = vec![-1i32; MAXN];
    for (i, &v) in allowed_set.iter().enumerate() { val_to_idx[v as usize] = i as i32; }

    // Memo for sqrt(-1) mod p
    let mut memo_roots = vec![0i32; MAXN];
    let mut memo_roots_set = vec![false; MAXN];
    let get_sqrt_neg1 = |p: i32, memo: &mut Vec<i32>, memo_set: &mut Vec<bool>| -> i32 {
        if p == 2 { return 1; }
        if p % 4 != 1 { return -1; }
        if memo_set[p as usize] { return memo[p as usize]; }
        let mut g = 2i64;
        loop {
            let t = pow_mod(g, (p as i64 - 1) / 2, p as i64);
            if t == p as i64 - 1 { break; }
            g += 1;
        }
        let r = pow_mod(g, (p as i64 - 1) / 4, p as i64) as i32;
        memo[p as usize] = r; memo_set[p as usize] = true; r
    };

    // Build edge list
    let mut edge_list: Vec<(i32, i32)> = Vec::new();
    for ai in 0..nallowed {
        let u = allowed_set[ai];
        let mut roots = [0i32; 4];
        let mut nroots = 0usize;

        if u == 1 { roots[0] = 0; nroots = 1; }
        else if u == 2 { roots[0] = 1; nroots = 1; }
        else {
            let mut temp = u;
            if temp % 2 == 0 { temp /= 2; }
            let p = min_prime[temp as usize];
            if p % 4 == 3 { continue; }
            let r = get_sqrt_neg1(p, &mut memo_roots, &mut memo_roots_set);
            if r < 0 { continue; }
            let mut cur_r = r as i64;
            let mut cur_mod = p as i64;
            while cur_mod < temp as i64 {
                let inv2r = pow_mod(2 * cur_r % p as i64, p as i64 - 2, p as i64);
                let val = (cur_r * cur_r + 1) / cur_mod;
                let diff = (val % p as i64 * inv2r) % p as i64;
                cur_r = cur_r - diff * cur_mod;
                cur_mod *= p as i64;
                cur_r = ((cur_r % cur_mod) + cur_mod) % cur_mod;
            }
            if u % 2 == 0 {
                let r0 = if cur_r % 2 == 0 { cur_r + temp as i64 } else { cur_r };
                roots[0] = r0 as i32; nroots = 1;
                if r0 as i32 * 2 != u { roots[1] = u - r0 as i32; nroots = 2; }
            } else {
                roots[0] = cur_r as i32; roots[1] = u - cur_r as i32; nroots = 2;
            }
        }

        let limit = ((u as f64 * n_val as f64).sqrt()) as i64 + 2;
        if u == 1 {
            for x in 0..=limit {
                let v = x * x + 1;
                if v > u as i64 && v <= n_val as i64 && is_allowed[v as usize] {
                    let ui = val_to_idx[u as usize];
                    let vi = val_to_idx[v as usize];
                    if ui >= 0 && vi >= 0 {
                        let (a, b) = if ui < vi { (ui, vi) } else { (vi, ui) };
                        edge_list.push((a, b));
                    }
                }
            }
        } else {
            for ri in 0..nroots {
                let r = roots[ri];
                let start = if r == 0 { u as i64 } else { r as i64 };
                let mut x = start;
                while x <= limit {
                    let val = x * x + 1;
                    let v = val / u as i64;
                    if v > u as i64 && v <= n_val as i64 && is_allowed[v as usize] {
                        let ui = val_to_idx[u as usize];
                        let vi = val_to_idx[v as usize];
                        if ui >= 0 && vi >= 0 {
                            let (a, b) = if ui < vi { (ui, vi) } else { (vi, ui) };
                            edge_list.push((a, b));
                        }
                    }
                    x += u as i64;
                }
            }
        }
    }

    edge_list.sort_unstable();
    edge_list.dedup();

    // Build adjacency
    let mut adj_deg = vec![0u32; nallowed];
    for &(a, b) in &edge_list { adj_deg[a as usize] += 1; adj_deg[b as usize] += 1; }
    let mut adj_offset = vec![0u32; nallowed + 1];
    for i in 0..nallowed { adj_offset[i + 1] = adj_offset[i] + adj_deg[i]; }
    let total_adj = adj_offset[nallowed] as usize;
    let mut adj_data = vec![0u32; total_adj];
    let mut adj_pos = adj_offset[..nallowed].to_vec();
    for &(a, b) in &edge_list {
        adj_data[adj_pos[a as usize] as usize] = b as u32; adj_pos[a as usize] += 1;
        adj_data[adj_pos[b as usize] as usize] = a as u32; adj_pos[b as usize] += 1;
    }

    // 2-core peeling
    let mut degree = vec![0i32; nallowed];
    let mut active = vec![true; nallowed];
    let mut queue: VecDeque<usize> = VecDeque::new();
    for i in 0..nallowed {
        degree[i] = (adj_offset[i + 1] - adj_offset[i]) as i32;
        if degree[i] < 2 { queue.push_back(i); }
    }
    while let Some(i) = queue.pop_front() {
        if !active[i] { continue; }
        if degree[i] >= 2 { continue; }
        active[i] = false;
        for idx in adj_offset[i] as usize..adj_offset[i + 1] as usize {
            let nb = adj_data[idx] as usize;
            if active[nb] { degree[nb] -= 1; if degree[nb] < 2 { queue.push_back(nb); } }
        }
    }

    // Connected components
    let mut comp_id = vec![-1i32; nallowed];
    let mut ncomps = 0i32;
    for i in 0..nallowed {
        if !active[i] || comp_id[i] >= 0 { continue; }
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(i); comp_id[i] = ncomps;
        while let Some(u) = q.pop_front() {
            for idx in adj_offset[u] as usize..adj_offset[u + 1] as usize {
                let v = adj_data[idx] as usize;
                if active[v] && comp_id[v] < 0 { comp_id[v] = ncomps; q.push_back(v); }
            }
        }
        ncomps += 1;
    }

    let mut total_potency: i64 = 0;

    for ci in 0..ncomps {
        let mut nodes: Vec<usize> = Vec::new();
        for i in 0..nallowed { if comp_id[i] == ci { nodes.push(i); } }
        if nodes.len() < 3 { continue; }
        let nn = nodes.len();

        let mut local_map = vec![0u32; nallowed];
        for (li, &ni) in nodes.iter().enumerate() { local_map[ni] = li as u32; }

        // Build sub_adj as CSR
        let mut sub_deg = vec![0u32; nn];
        for li in 0..nn {
            let ni = nodes[li];
            for idx in adj_offset[ni] as usize..adj_offset[ni + 1] as usize {
                let nb = adj_data[idx] as usize;
                if active[nb] && comp_id[nb] == ci { sub_deg[li] += 1; }
            }
        }
        let mut sub_offset = vec![0u32; nn + 1];
        for i in 0..nn { sub_offset[i + 1] = sub_offset[i] + sub_deg[i]; }
        let sub_total = sub_offset[nn] as usize;
        let mut sub_data = vec![0u32; sub_total];
        let mut sub_pos = sub_offset[..nn].to_vec();
        for li in 0..nn {
            let ni = nodes[li];
            for idx in adj_offset[ni] as usize..adj_offset[ni + 1] as usize {
                let nb = adj_data[idx] as usize;
                if active[nb] && comp_id[nb] == ci {
                    sub_data[sub_pos[li] as usize] = local_map[nb];
                    sub_pos[li] += 1;
                }
            }
        }
        let node_vals: Vec<i64> = nodes.iter().map(|&ni| allowed_set[ni] as i64).collect();

        if nn > 100 {
            // Large component: decompose work into fine-grained items for parallelism.
            // Work item: (start, current_node, current_sum, length, path_prefix)
            let mut work_items: Vec<(u32, u32, i64, i32, Vec<u32>)> = Vec::new();

            let deep3_limit = 20u32;  // starts 0..20 get depth-3 decomposition
            let deep4_limit = 2u32;   // starts 0..2 get depth-4 decomposition

            for start in 0..nn as u32 {
                let s_off = sub_offset[start as usize];
                let s_end = sub_offset[start as usize + 1];

                if start < deep3_limit {
                    // Depth 3+: enumerate (start, v, w) triples, optionally go deeper
                    for ei in s_off..s_end {
                        let v = sub_data[ei as usize];
                        if v <= start { continue; }
                        let v_off = sub_offset[v as usize];
                        let v_end = sub_offset[v as usize + 1];
                        for ei2 in v_off..v_end {
                            let w = sub_data[ei2 as usize];
                            if w == start || w <= start || w == v { continue; }

                            if start < deep4_limit {
                                // Depth 4: enumerate (start, v, w, x) quadruples
                                let w_off = sub_offset[w as usize];
                                let w_end = sub_offset[w as usize + 1];
                                let mut any_pushed = false;
                                for ei3 in w_off..w_end {
                                    let x = sub_data[ei3 as usize];
                                    if x == start {
                                        // Cycle of length 3: start→v→w→start
                                        // Handle directly (no further DFS needed)
                                        work_items.push((
                                            start, start, // current = start signals direct cycle
                                            node_vals[start as usize] + node_vals[v as usize] + node_vals[w as usize],
                                            3,
                                            vec![], // empty prefix = direct cycle contribution
                                        ));
                                        any_pushed = true;
                                    } else if x > start && x != v && x != w {
                                        work_items.push((
                                            start, x,
                                            node_vals[start as usize] + node_vals[v as usize] + node_vals[w as usize] + node_vals[x as usize],
                                            4,
                                            vec![start, v, w, x],
                                        ));
                                        any_pushed = true;
                                    }
                                }
                                let _ = any_pushed;
                            } else {
                                work_items.push((
                                    start, w,
                                    node_vals[start as usize] + node_vals[v as usize] + node_vals[w as usize],
                                    3,
                                    vec![start, v, w],
                                ));
                            }
                        }
                    }
                } else {
                    // Depth 2: (start, v) pairs
                    for ei in s_off..s_end {
                        let v = sub_data[ei as usize];
                        if v > start {
                            work_items.push((
                                start, v,
                                node_vals[start as usize] + node_vals[v as usize],
                                2,
                                vec![start, v],
                            ));
                        }
                    }
                }
            }

            let blk_potency: i64 = work_items.par_iter().map(|(start, current, sum, len, prefix)| {
                if prefix.is_empty() {
                    // Direct cycle found during decomposition (length-3 cycle)
                    return *sum;
                }
                let mut path_vis = vec![false; nn];
                for &p in prefix { path_vis[p as usize] = true; }
                dfs_cycle(
                    *current, *start, *sum, *len,
                    &mut path_vis, &sub_offset, &sub_data, &node_vals,
                )
            }).sum();

            total_potency += blk_potency / 2;
        } else {
            // Small component: sequential
            let mut blk_potency: i64 = 0;
            let mut path_vis = vec![false; nn];
            for start in 0..nn as u32 {
                path_vis[start as usize] = true;
                let s_off = sub_offset[start as usize];
                let s_end = sub_offset[start as usize + 1];
                for ei in s_off..s_end {
                    let v = sub_data[ei as usize];
                    if v > start && !path_vis[v as usize] {
                        path_vis[v as usize] = true;
                        blk_potency += dfs_cycle(
                            v, start, node_vals[start as usize] + node_vals[v as usize], 2,
                            &mut path_vis, &sub_offset, &sub_data, &node_vals,
                        );
                        path_vis[v as usize] = false;
                    }
                }
                path_vis[start as usize] = false;
            }
            total_potency += blk_potency / 2;
        }
    }

    println!("{}", total_potency);
}
