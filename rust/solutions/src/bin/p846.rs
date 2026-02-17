// Project Euler 846 - Graph Potency
// Build graph on allowed numbers, find 2-core, enumerate cycles, sum potency.
// Optimized: flat arrays instead of HashMap/HashSet, unsafe bounds in DFS hot path.

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
            if 2 * pk <= n_val as i64 {
                is_allowed[(2 * pk) as usize] = true;
            }
            pk *= p as i64;
        }
    }

    let mut allowed_set: Vec<i32> = Vec::new();
    for i in 1..=n_val {
        if is_allowed[i] { allowed_set.push(i as i32); }
    }
    let nallowed = allowed_set.len();

    // Flat val_to_idx array
    let mut val_to_idx = vec![-1i32; MAXN];
    for (i, &v) in allowed_set.iter().enumerate() {
        val_to_idx[v as usize] = i as i32;
    }

    // Memo for sqrt(-1) mod p -- flat array, primes up to 10^6
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
        memo[p as usize] = r;
        memo_set[p as usize] = true;
        r
    };

    // Build adjacency: collect edges first, then deduplicate
    // Use edge list approach to avoid HashSet
    let mut edge_list: Vec<(i32, i32)> = Vec::new();

    for ai in 0..nallowed {
        let u = allowed_set[ai];
        let mut roots = [0i32; 4];
        let mut nroots = 0usize;

        if u == 1 {
            roots[0] = 0; nroots = 1;
        } else if u == 2 {
            roots[0] = 1; nroots = 1;
        } else {
            let mut temp = u;
            if temp % 2 == 0 { temp /= 2; }
            let p = min_prime[temp as usize];
            if p % 4 == 3 { continue; }
            let r = get_sqrt_neg1(p, &mut memo_roots, &mut memo_roots_set);
            if r < 0 { continue; }

            // Hensel lift
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
                roots[0] = r0 as i32;
                nroots = 1;
                if r0 as i32 * 2 != u {
                    roots[1] = u - r0 as i32;
                    nroots = 2;
                }
            } else {
                roots[0] = cur_r as i32;
                roots[1] = u - cur_r as i32;
                nroots = 2;
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

    // Deduplicate edges
    edge_list.sort_unstable();
    edge_list.dedup();

    // Build adjacency from deduplicated edges
    let mut adj_deg = vec![0u32; nallowed];
    for &(a, b) in &edge_list {
        adj_deg[a as usize] += 1;
        adj_deg[b as usize] += 1;
    }

    let mut adj_offset = vec![0u32; nallowed + 1];
    for i in 0..nallowed {
        adj_offset[i + 1] = adj_offset[i] + adj_deg[i];
    }
    let total_adj = adj_offset[nallowed] as usize;
    let mut adj_data = vec![0u32; total_adj];
    let mut adj_pos = adj_offset[..nallowed].to_vec();

    for &(a, b) in &edge_list {
        let ai = a as usize;
        let bi = b as usize;
        adj_data[adj_pos[ai] as usize] = b as u32;
        adj_pos[ai] += 1;
        adj_data[adj_pos[bi] as usize] = a as u32;
        adj_pos[bi] += 1;
    }

    // 2-core peeling using queue
    let mut degree = vec![0i32; nallowed];
    let mut active = vec![true; nallowed];
    let mut queue: VecDeque<usize> = VecDeque::new();

    for i in 0..nallowed {
        degree[i] = (adj_offset[i + 1] - adj_offset[i]) as i32;
        if degree[i] < 2 {
            queue.push_back(i);
        }
    }

    while let Some(i) = queue.pop_front() {
        if !active[i] { continue; }
        if degree[i] >= 2 { continue; }
        active[i] = false;
        let start = adj_offset[i] as usize;
        let end = adj_offset[i + 1] as usize;
        for idx in start..end {
            let nb = adj_data[idx] as usize;
            if active[nb] {
                degree[nb] -= 1;
                if degree[nb] < 2 {
                    queue.push_back(nb);
                }
            }
        }
    }

    // Connected components of 2-core
    let mut comp_id = vec![-1i32; nallowed];
    let mut ncomps = 0i32;

    for i in 0..nallowed {
        if !active[i] || comp_id[i] >= 0 { continue; }
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(i);
        comp_id[i] = ncomps;
        while let Some(u) = q.pop_front() {
            let start = adj_offset[u] as usize;
            let end = adj_offset[u + 1] as usize;
            for idx in start..end {
                let v = adj_data[idx] as usize;
                if active[v] && comp_id[v] < 0 {
                    comp_id[v] = ncomps;
                    q.push_back(v);
                }
            }
        }
        ncomps += 1;
    }

    // For each component, enumerate cycles via DFS
    let mut total_potency: i64 = 0;

    for ci in 0..ncomps {
        let mut nodes: Vec<usize> = Vec::new();
        for i in 0..nallowed {
            if comp_id[i] == ci { nodes.push(i); }
        }
        if nodes.len() < 3 { continue; }

        let nn = nodes.len();

        // Build local map: global idx -> local idx
        let mut local_map = vec![0u32; nallowed];
        for (li, &ni) in nodes.iter().enumerate() {
            local_map[ni] = li as u32;
        }

        // Build sub_adj as CSR
        let mut sub_deg = vec![0u32; nn];
        for li in 0..nn {
            let ni = nodes[li];
            let start = adj_offset[ni] as usize;
            let end = adj_offset[ni + 1] as usize;
            for idx in start..end {
                let nb = adj_data[idx] as usize;
                if active[nb] && comp_id[nb] == ci {
                    sub_deg[li] += 1;
                }
            }
        }

        let mut sub_offset = vec![0u32; nn + 1];
        for i in 0..nn { sub_offset[i + 1] = sub_offset[i] + sub_deg[i]; }
        let sub_total = sub_offset[nn] as usize;
        let mut sub_data = vec![0u32; sub_total];
        let mut sub_pos = sub_offset[..nn].to_vec();

        for li in 0..nn {
            let ni = nodes[li];
            let start = adj_offset[ni] as usize;
            let end = adj_offset[ni + 1] as usize;
            for idx in start..end {
                let nb = adj_data[idx] as usize;
                if active[nb] && comp_id[nb] == ci {
                    let lj = local_map[nb];
                    sub_data[sub_pos[li] as usize] = lj;
                    sub_pos[li] += 1;
                }
            }
        }

        let node_vals: Vec<i64> = nodes.iter().map(|&ni| allowed_set[ni] as i64).collect();

        let mut blk_potency: i64 = 0;
        let mut path_vis = vec![false; nn];

        // Iterative DFS cycle enumeration using explicit stack
        // Each frame: (u, current_sum, length, neighbor_index)
        // We also track start for each DFS root
        let mut stack: Vec<(u32, i64, i32, u32)> = Vec::with_capacity(nn);

        for start in 0..nn as u32 {
            // Push initial frame
            path_vis[start as usize] = true;
            let s_off = sub_offset[start as usize];
            let e_off = sub_offset[start as usize + 1];
            // Process neighbors of start
            for ei in s_off..e_off {
                let v = unsafe { *sub_data.get_unchecked(ei as usize) };
                if v == start {
                    // length 1 -- can't form cycle of length >= 3
                } else if v > start && !path_vis[v as usize] {
                    // Push frame for v
                    stack.push((v, node_vals[start as usize] + node_vals[v as usize], 2, 0));
                    path_vis[v as usize] = true;

                    // Process stack
                    while let Some(frame) = stack.last_mut() {
                        let u = frame.0;
                        let current_sum = frame.1;
                        let length = frame.2;
                        let ni = frame.3;
                        let u_s = sub_offset[u as usize];
                        let u_e = sub_offset[u as usize + 1];

                        if u_s + ni >= u_e {
                            // Done with this node
                            path_vis[u as usize] = false;
                            stack.pop();
                            continue;
                        }

                        frame.3 += 1; // advance neighbor index

                        let w = unsafe { *sub_data.get_unchecked((u_s + ni) as usize) };
                        if w == start {
                            if length >= 3 {
                                blk_potency += current_sum;
                            }
                        } else if w > start && !path_vis[w as usize] {
                            path_vis[w as usize] = true;
                            stack.push((w, current_sum + node_vals[w as usize], length + 1, 0));
                        }
                    }
                }
            }
            path_vis[start as usize] = false;
        }

        total_potency += blk_potency / 2;
    }

    println!("{}", total_potency);
}
