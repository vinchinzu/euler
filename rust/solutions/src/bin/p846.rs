// Project Euler 846 - Graph Potency
// Build graph on allowed numbers, find 2-core, enumerate cycles, sum potency.

use std::collections::{HashMap, HashSet, VecDeque};

const MAXN: usize = 1_000_001;

fn sieve(n: usize) -> (Vec<i32>, Vec<i32>) {
    let mut min_prime = vec![0i32; n + 1];
    let mut primes = Vec::new();
    for i in 2..=n {
        if min_prime[i] == 0 {
            min_prime[i] = i as i32;
            primes.push(i as i32);
            let mut j = i * i;
            while j <= n {
                if min_prime[j] == 0 {
                    min_prime[j] = i as i32;
                }
                j += i;
            }
        }
    }
    (min_prime, primes)
}

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

fn get_sqrt_neg1(p: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if p == 2 { return 1; }
    if p % 4 != 1 { return -1; }
    if let Some(&r) = memo.get(&p) { return r; }

    let mut g = 2i64;
    loop {
        let t = pow_mod(g, (p as i64 - 1) / 2, p as i64);
        if t == p as i64 - 1 { break; }
        g += 1;
    }
    let r = pow_mod(g, (p as i64 - 1) / 4, p as i64) as i32;
    memo.insert(p, r);
    r
}

fn main() {
    let n_val = 1_000_000usize;
    let (min_prime, primes) = sieve(n_val);

    let mut is_allowed = vec![false; MAXN];
    let mut allowed_set: Vec<i32> = Vec::new();

    is_allowed[1] = true;
    is_allowed[2] = true;

    for &p in &primes {
        if p == 2 { continue; }
        let mut pk = p as i64;
        while pk <= n_val as i64 {
            if !is_allowed[pk as usize] {
                is_allowed[pk as usize] = true;
            }
            if 2 * pk <= n_val as i64 && !is_allowed[(2 * pk) as usize] {
                is_allowed[(2 * pk) as usize] = true;
            }
            pk *= p as i64;
        }
    }

    for i in 1..=n_val {
        if is_allowed[i] {
            allowed_set.push(i as i32);
        }
    }
    allowed_set.sort();

    let mut val_to_idx: HashMap<i32, usize> = HashMap::new();
    for (i, &v) in allowed_set.iter().enumerate() {
        val_to_idx.insert(v, i);
    }
    let nallowed = allowed_set.len();

    let mut memo_roots: HashMap<i32, i32> = HashMap::new();

    // Build adjacency using sets to deduplicate
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); nallowed];

    for ai in 0..nallowed {
        let u = allowed_set[ai];
        let mut roots: Vec<i32> = Vec::new();

        if u == 1 {
            roots.push(0);
        } else if u == 2 {
            roots.push(1);
        } else {
            let mut temp = u;
            if temp % 2 == 0 { temp /= 2; }
            let p = min_prime[temp as usize];
            if p % 4 == 3 { continue; }
            let r = get_sqrt_neg1(p, &mut memo_roots);
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
                roots.push(r0 as i32);
                if r0 as i32 * 2 != u {
                    roots.push(u - r0 as i32);
                }
            } else {
                roots.push(cur_r as i32);
                roots.push(u - cur_r as i32);
            }
        }

        let limit = ((u as f64 * n_val as f64).sqrt()) as i64 + 2;

        if u == 1 {
            for x in 0..=limit {
                let v = x * x + 1;
                if v > u as i64 && v <= n_val as i64 && is_allowed[v as usize] {
                    if let (Some(&ui), Some(&vi)) = (val_to_idx.get(&u), val_to_idx.get(&(v as i32))) {
                        adj[ui].insert(vi);
                        adj[vi].insert(ui);
                    }
                }
            }
        } else {
            for &r in &roots {
                let start = if r == 0 { u as i64 } else { r as i64 };
                let mut x = start;
                while x <= limit {
                    let val = x * x + 1;
                    let v = val / u as i64;
                    if v > u as i64 && v <= n_val as i64 && is_allowed[v as usize] {
                        if let (Some(&ui), Some(&vi)) = (val_to_idx.get(&u), val_to_idx.get(&(v as i32))) {
                            adj[ui].insert(vi);
                            adj[vi].insert(ui);
                        }
                    }
                    x += u as i64;
                }
            }
        }
    }

    // 2-core peeling
    let mut degree: Vec<i32> = adj.iter().map(|s| s.len() as i32).collect();
    let mut active = vec![true; nallowed];

    loop {
        let mut changed = false;
        for i in 0..nallowed {
            if !active[i] { continue; }
            if degree[i] < 2 {
                active[i] = false;
                changed = true;
                for &nb in &adj[i] {
                    if active[nb] {
                        degree[nb] -= 1;
                    }
                }
            }
        }
        if !changed { break; }
    }

    // Build clean adjacency for 2-core
    let mut adj2: Vec<Vec<usize>> = vec![Vec::new(); nallowed];
    for i in 0..nallowed {
        if !active[i] { continue; }
        for &nb in &adj[i] {
            if active[nb] {
                adj2[i].push(nb);
            }
        }
    }

    // Connected components of 2-core
    let mut comp_id = vec![-1i32; nallowed];
    let mut ncomps = 0i32;

    for i in 0..nallowed {
        if !active[i] || comp_id[i] >= 0 { continue; }
        let mut queue = VecDeque::new();
        queue.push_back(i);
        comp_id[i] = ncomps;
        while let Some(u) = queue.pop_front() {
            for &v in &adj2[u] {
                if comp_id[v] < 0 {
                    comp_id[v] = ncomps;
                    queue.push_back(v);
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
            if comp_id[i] == ci {
                nodes.push(i);
            }
        }
        if nodes.len() < 3 { continue; }

        let nn = nodes.len();
        let mut local_map: HashMap<usize, usize> = HashMap::new();
        for (li, &ni) in nodes.iter().enumerate() {
            local_map.insert(ni, li);
        }

        let mut sub_adj: Vec<Vec<usize>> = vec![Vec::new(); nn];
        for li in 0..nn {
            let ni = nodes[li];
            for &nb in &adj2[ni] {
                if let Some(&lj) = local_map.get(&nb) {
                    sub_adj[li].push(lj);
                }
            }
        }

        let node_vals: Vec<i64> = nodes.iter().map(|&ni| allowed_set[ni] as i64).collect();

        let mut blk_potency: i64 = 0;
        let mut path_vis = vec![false; nn];

        // DFS cycle enumeration
        fn dfs_blk(
            start: usize, u: usize, current_sum: i64, length: i32,
            sub_adj: &[Vec<usize>], node_vals: &[i64],
            path_vis: &mut [bool], potency: &mut i64,
        ) {
            path_vis[u] = true;
            for &v in &sub_adj[u] {
                if v == start {
                    if length >= 3 {
                        *potency += current_sum;
                    }
                } else if v > start && !path_vis[v] {
                    dfs_blk(start, v, current_sum + node_vals[v], length + 1,
                            sub_adj, node_vals, path_vis, potency);
                }
            }
            path_vis[u] = false;
        }

        for i in 0..nn {
            dfs_blk(i, i, node_vals[i], 1, &sub_adj, &node_vals, &mut path_vis, &mut blk_potency);
        }

        total_potency += blk_potency / 2;
    }

    println!("{}", total_potency);
}
