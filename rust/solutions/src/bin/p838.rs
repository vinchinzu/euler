// Project Euler Problem 838: Not Coprime
// Compute ln(f(10^6)) rounded to 6 decimal places

use std::collections::{HashSet, VecDeque};

fn sieve_primes(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b)
        .map(|(i, _)| i)
        .collect()
}

fn iroot3_floor(n: usize) -> usize {
    let mut x = (n as f64).powf(1.0 / 3.0).round() as usize;
    while (x + 1).pow(3) <= n {
        x += 1;
    }
    while x.pow(3) > n {
        x -= 1;
    }
    x
}

struct Dinic {
    n: usize,
    g: Vec<Vec<[usize; 3]>>,
}

impl Dinic {
    fn new(n: usize) -> Self {
        Dinic {
            n,
            g: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        let fwd_idx = self.g[v].len();
        let rev_idx = self.g[u].len();
        self.g[u].push([v, cap as usize, fwd_idx]);
        self.g[v].push([u, 0, rev_idx]);
    }

    fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0i64;
        let n = self.n;
        let mut level = vec![-1i32; n];

        loop {
            // BFS
            level.fill(-1);
            let mut q = VecDeque::new();
            q.push_back(s);
            level[s] = 0;

            while let Some(u) = q.pop_front() {
                for edge in &self.g[u] {
                    let v = edge[0];
                    let cap = edge[1];
                    if cap > 0 && level[v] < 0 {
                        level[v] = level[u] + 1;
                        q.push_back(v);
                    }
                }
            }

            if level[t] < 0 {
                break;
            }

            // DFS
            let mut it = vec![0usize; n];

            fn dfs(
                g: &mut Vec<Vec<[usize; 3]>>,
                level: &[i32],
                it: &mut [usize],
                u: usize,
                t: usize,
                f: i64,
            ) -> i64 {
                if u == t {
                    return f;
                }
                for i in it[u]..g[u].len() {
                    it[u] = i;
                    let edge = g[u][i];
                    let v = edge[0];
                    let cap = edge[1] as i64;
                    let rev = edge[2];
                    if cap > 0 && level[v] == level[u] + 1 {
                        let pushed = dfs(g, level, it, v, t, f.min(cap));
                        if pushed > 0 {
                            g[u][i][1] -= pushed as usize;
                            g[v][rev][1] += pushed as usize;
                            return pushed;
                        }
                    }
                }
                0
            }

            loop {
                let pushed = dfs(&mut self.g, &level, &mut it, s, t, i64::MAX);
                if pushed == 0 {
                    break;
                }
                flow += pushed;
            }
        }

        flow
    }

    fn reachable_from(&self, s: usize) -> Vec<bool> {
        let mut vis = vec![false; self.n];
        let mut q = VecDeque::new();
        q.push_back(s);
        vis[s] = true;

        while let Some(u) = q.pop_front() {
            for edge in &self.g[u] {
                let v = edge[0];
                let cap = edge[1];
                if cap > 0 && !vis[v] {
                    vis[v] = true;
                    q.push_back(v);
                }
            }
        }

        vis
    }
}

fn forced_primes(primes: &[usize], n: usize) -> HashSet<usize> {
    let mut forced = HashSet::new();

    // Primes ending in 3
    for &p in primes {
        if p % 10 == 3 {
            forced.insert(p);
        }
    }

    // Primes p^3 ends in 3
    let c = iroot3_floor(n);
    for &p in primes {
        if p % 10 == 7 && p <= c {
            forced.insert(p);
        }
    }

    forced
}

fn build_prefix_bipartite(
    primes: &[usize],
    n: usize,
    forced: &HashSet<usize>,
) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let left: Vec<usize> = primes
        .iter()
        .filter(|&&p| p % 10 == 7 && !forced.contains(&p) && p * 19 <= n)
        .cloned()
        .collect();

    if left.is_empty() {
        return (Vec::new(), Vec::new(), Vec::new());
    }

    let pmin = left[0];
    let right: Vec<usize> = primes
        .iter()
        .filter(|&&q| q % 10 == 9 && q <= n / pmin)
        .cloned()
        .collect();

    let mut pref_len = Vec::new();
    for &p in &left {
        let lim = n / p;
        let len = right.iter().take_while(|&&q| q <= lim).count();
        pref_len.push(len);
    }

    (left, right, pref_len)
}

fn min_weight_vertex_cover_prefix_bipartite(
    left: &[usize],
    right: &[usize],
    pref_len: &[usize],
) -> (HashSet<usize>, HashSet<usize>) {
    let scale = 1_000_000_000_000i64;

    let wl: Vec<i64> = left
        .iter()
        .map(|&p| ((p as f64).ln() * scale as f64).round() as i64)
        .collect();
    let wr: Vec<i64> = right
        .iter()
        .map(|&q| ((q as f64).ln() * scale as f64).round() as i64)
        .collect();

    let total: i64 = wl.iter().sum::<i64>() + wr.iter().sum::<i64>();
    let inf = total + 1;

    let nl = left.len();
    let nr = right.len();
    let s = 0;
    let off_l = 1;
    let off_r = 1 + nl;
    let t = 1 + nl + nr;

    let mut dinic = Dinic::new(t + 1);

    for (i, &w) in wl.iter().enumerate() {
        dinic.add_edge(s, off_l + i, w);
    }
    for (j, &w) in wr.iter().enumerate() {
        dinic.add_edge(off_r + j, t, w);
    }

    for i in 0..nl {
        let u = off_l + i;
        let k = pref_len[i];
        for j in 0..k {
            dinic.add_edge(u, off_r + j, inf);
        }
    }

    dinic.max_flow(s, t);
    let vis = dinic.reachable_from(s);

    let mut cover_l = HashSet::new();
    let mut cover_r = HashSet::new();

    for (i, &p) in left.iter().enumerate() {
        if !vis[off_l + i] {
            cover_l.insert(p);
        }
    }
    for (j, &q) in right.iter().enumerate() {
        if vis[off_r + j] {
            cover_r.insert(q);
        }
    }

    (cover_l, cover_r)
}

fn ln_f(n: usize) -> f64 {
    let primes = sieve_primes(n);
    let forced = forced_primes(&primes, n);

    let (left, right, pref_len) = build_prefix_bipartite(&primes, n, &forced);
    let (cover_l, cover_r) = if left.is_empty() || right.is_empty() {
        (HashSet::new(), HashSet::new())
    } else {
        min_weight_vertex_cover_prefix_bipartite(&left, &right, &pref_len)
    };

    let chosen: HashSet<_> = forced.union(&cover_l).chain(cover_r.iter()).collect();
    let mut sum = 0.0f64;
    for &&p in &chosen {
        sum += (p as f64).ln();
    }
    sum
}

fn main() {
    // Test: ln(f(40)) ≈ 6.799056
    let ln40 = ln_f(40);
    assert!((ln40 - 6.799056).abs() < 0.000001);

    // Test: ln(f(2800)) ≈ 715.019337
    let ln2800 = ln_f(2800);
    assert!((ln2800 - 715.019337).abs() < 0.000001);

    let result = ln_f(1_000_000);
    println!("{:.6}", result);
}
