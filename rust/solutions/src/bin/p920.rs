// Problem 920 - Tau Numbers
//
// tau(n) = number of divisors of n.
// n is a tau number if tau(n) | n.
// m(k) = smallest tau number x with tau(x) = k.
// M(n) = sum of m(k) for all k where m(k) <= 10^n.
// Compute M(16).

use std::collections::HashMap;

const LIMIT: u128 = 10_u128.pow(16);

/// Small primes up to 200.
fn small_primes() -> Vec<u64> {
    let mut is_prime = vec![true; 201];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 2;
    while p * p <= 200 {
        if is_prime[p] {
            let mut j = p * p;
            while j <= 200 {
                is_prime[j] = false;
                j += p;
            }
        }
        p += 1;
    }
    (2..=200).filter(|&i| is_prime[i]).map(|i| i as u64).collect()
}

/// Factorize n into prime factors using trial division with small primes.
fn factorize(mut n: u64, primes: &[u64]) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    for &p in primes {
        if p * p > n {
            break;
        }
        if n % p == 0 {
            let mut e = 0u32;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            factors.push((p, e));
        }
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

/// Precompute factorizations for 0..=max_n.
fn precompute_factorizations(max_n: usize, primes: &[u64]) -> Vec<Vec<(u64, u32)>> {
    let mut facs = Vec::with_capacity(max_n + 1);
    for i in 0..=max_n {
        facs.push(factorize(i as u64, primes));
    }
    facs
}

/// Generate all non-increasing exponent vectors a1>=a2>=...>=ar>=1
/// such that 2^a1 * 3^a2 * ... * p_r^ar <= limit.
fn generate_exponent_vectors(limit: u128, primes: &[u64]) -> Vec<Vec<u32>> {
    // Determine how many primes needed: smallest product of distinct primes <= limit.
    let mut primes_lb: Vec<u64> = Vec::new();
    let mut prod: u128 = 1;
    for &p in primes {
        if prod * (p as u128) > limit {
            break;
        }
        prod *= p as u128;
        primes_lb.push(p);
    }

    let mut out: Vec<Vec<u32>> = Vec::new();
    let mut exps: Vec<u32> = Vec::new();

    fn dfs(
        idx: usize,
        max_e: u32,
        current: u128,
        primes_lb: &[u64],
        limit: u128,
        exps: &mut Vec<u32>,
        out: &mut Vec<Vec<u32>>,
    ) {
        if idx >= primes_lb.len() {
            return;
        }
        let p = primes_lb[idx] as u128;
        let mut p_pow: u128 = 1;
        for e in 1..=max_e {
            p_pow = match p_pow.checked_mul(p) {
                Some(v) => v,
                None => break,
            };
            let nxt = match current.checked_mul(p_pow) {
                Some(v) => v,
                None => break,
            };
            if nxt > limit {
                break;
            }
            exps.push(e);
            out.push(exps.clone());
            dfs(idx + 1, e, nxt, primes_lb, limit, exps, out);
            exps.pop();
        }
    }

    dfs(0, 60, 1, &primes_lb, limit, &mut exps, &mut out);
    out
}

/// For a fixed exponent multiset (sorted non-increasing), minimize
/// n = prod p_i^{a_i} subject to tau(n) | n and n <= limit.
///
/// req maps prime -> exponent needed in n (from factorization of k = tau).
fn min_tau_number_for_exponents(
    exps_desc: &[u32],
    req: &HashMap<u64, u32>,
    primes_sorted: &[u64],
    limit: u128,
) -> Option<u128> {
    let r = exps_desc.len();
    if r == 0 {
        return Some(1);
    }

    let required_primes: Vec<u64> = req.keys().cloned().collect();
    let s = required_primes.len();
    if s > r {
        return None;
    }

    let max_exp = exps_desc[0];
    for (_, &need_exp) in req.iter() {
        if need_exp > max_exp {
            return None;
        }
    }

    let required_set: std::collections::HashSet<u64> =
        required_primes.iter().cloned().collect();

    // Smallest filler primes not in required_set.
    let fillers_needed = r - s;
    let mut fillers: Vec<u64> = Vec::new();
    if fillers_needed > 0 {
        for &p in primes_sorted {
            if !required_set.contains(&p) {
                fillers.push(p);
                if fillers.len() == fillers_needed {
                    break;
                }
            }
        }
    }
    fillers.sort();

    // Process larger required primes first (they prefer smaller exponents).
    let mut req_primes_sorted: Vec<u64> = required_primes.clone();
    req_primes_sorted.sort_unstable_by(|a, b| b.cmp(a));

    let exps: Vec<u32> = exps_desc.to_vec();

    // Greedy upper bound.
    let greedy_upper_bound = || -> Option<u128> {
        let mut avail: Vec<u32> = exps.clone();
        avail.sort();
        let mut prod: u128 = 1;
        for &p in &req_primes_sorted {
            let need = *req.get(&p).unwrap();
            let mut found = false;
            for i in 0..avail.len() {
                if avail[i] >= need {
                    prod = prod.checked_mul(pow128(p as u128, avail[i]))?;
                    avail.remove(i);
                    found = true;
                    break;
                }
            }
            if !found {
                return None;
            }
            if prod > limit {
                return None;
            }
        }
        avail.sort_unstable_by(|a, b| b.cmp(a));
        for (&p, &e) in fillers.iter().zip(avail.iter()) {
            prod = prod.checked_mul(pow128(p as u128, e))?;
            if prod > limit {
                return None;
            }
        }
        Some(prod)
    };

    let greedy = greedy_upper_bound();
    let mut best_int: u128 = match greedy {
        Some(v) => v,
        None => u128::MAX,
    };
    let mut best_val: Option<u128> = greedy;

    // DFS to assign exponents to required primes, with inline lower-bound pruning.
    struct DfsState {
        req_primes_sorted: Vec<u64>,
        req: HashMap<u64, u32>,
        fillers: Vec<u64>,
        exps: Vec<u32>,
        r: usize,
        limit: u128,
    }

    fn dfs2(
        state: &DfsState,
        req_idx: usize,
        mask: u64,
        current_prod: u128,
        best_int: &mut u128,
        best_val: &mut Option<u128>,
    ) {
        if current_prod >= *best_int {
            return;
        }
        if req_idx == state.req_primes_sorted.len() {
            let mut rem_exps: Vec<u32> = Vec::new();
            for i in 0..state.r {
                if (mask >> i) & 1 == 0 {
                    rem_exps.push(state.exps[i]);
                }
            }
            rem_exps.sort_unstable_by(|a, b| b.cmp(a));
            let mut total = current_prod;
            for (&p, &e) in state.fillers.iter().zip(rem_exps.iter()) {
                total = match total.checked_mul(pow128(p as u128, e)) {
                    Some(v) => v,
                    None => return,
                };
                if total >= *best_int {
                    return;
                }
            }
            if total <= state.limit && total < *best_int {
                *best_int = total;
                *best_val = Some(total);
            }
            return;
        }

        // Inline lower bound check.
        {
            let mut rem_exps: Vec<u32> = Vec::new();
            for i in 0..state.r {
                if (mask >> i) & 1 == 0 {
                    rem_exps.push(state.exps[i]);
                }
            }
            rem_exps.sort_unstable_by(|a, b| b.cmp(a));
            let mut rem_primes: Vec<u64> = state.fillers.clone();
            for i in req_idx..state.req_primes_sorted.len() {
                rem_primes.push(state.req_primes_sorted[i]);
            }
            rem_primes.sort_unstable();
            let mut lb = current_prod;
            let mut exceeded = false;
            for (&p, &e) in rem_primes.iter().zip(rem_exps.iter()) {
                lb = match lb.checked_mul(pow128(p as u128, e)) {
                    Some(v) => v,
                    None => {
                        exceeded = true;
                        break;
                    }
                };
                if lb >= *best_int {
                    exceeded = true;
                    break;
                }
            }
            if !exceeded && lb >= *best_int {
                return;
            }
            if exceeded {
                // lb overflowed or exceeded best_int => can't prune
                // Actually if lb >= best_int, we should prune!
                // The overflow case means the actual value is huge, so also prune.
                return;
            }
        }

        let p = state.req_primes_sorted[req_idx];
        let need = *state.req.get(&p).unwrap();

        let mut prev_e: Option<u32> = None;
        for i in 0..state.r {
            if (mask >> i) & 1 != 0 {
                continue;
            }
            let e = state.exps[i];
            if e < need {
                continue;
            }
            if prev_e == Some(e) {
                continue;
            }
            prev_e = Some(e);

            let nxt = match current_prod.checked_mul(pow128(p as u128, e)) {
                Some(v) => v,
                None => continue,
            };
            if nxt >= *best_int || nxt > state.limit {
                continue;
            }
            dfs2(state, req_idx + 1, mask | (1 << i), nxt, best_int, best_val);
        }
    }

    let state = DfsState {
        req_primes_sorted,
        req: req.clone(),
        fillers,
        exps,
        r,
        limit,
    };

    dfs2(&state, 0, 0, 1, &mut best_int, &mut best_val);
    best_val
}

/// Compute p^e as u128.
fn pow128(p: u128, e: u32) -> u128 {
    let mut result: u128 = 1;
    for _ in 0..e {
        result = match result.checked_mul(p) {
            Some(v) => v,
            None => return u128::MAX,
        };
    }
    result
}

fn main() {
    let primes = small_primes();

    let exponent_vectors = generate_exponent_vectors(LIMIT, &primes);

    // Maximum exponent encountered.
    let mut max_a: u32 = 1;
    for v in &exponent_vectors {
        if let Some(&first) = v.first() {
            if first > max_a {
                max_a = first;
            }
        }
    }

    let small_fac = precompute_factorizations((max_a + 2) as usize, &primes);

    let mut best: HashMap<u64, u128> = HashMap::new();
    best.insert(1, 1);

    for exps in &exponent_vectors {
        // Compute k = product(a_i + 1) and its factorization (req).
        let mut k: u64 = 1;
        let mut req: HashMap<u64, u32> = HashMap::new();
        for &a in exps {
            k = k.saturating_mul((a as u64) + 1);
            for &(p, e) in &small_fac[(a + 1) as usize] {
                *req.entry(p).or_insert(0) += e;
            }
        }

        if let Some(n) = min_tau_number_for_exponents(exps, &req, &primes, LIMIT) {
            if n <= LIMIT {
                let prev = best.get(&k).cloned();
                if prev.is_none() || n < prev.unwrap() {
                    best.insert(k, n);
                }
            }
        }
    }

    let ans: u128 = best.values().sum();
    println!("{}", ans);
}
