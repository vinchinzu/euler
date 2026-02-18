// Project Euler 902 - Permutation rank sum (cycle-structure approach)
//
// Build pi = tau_inv o sigma o tau on n = m*(m+1)/2 = 5050 elements.
// sigma has one cycle of each length 1..m, so pi (conjugate to sigma)
// has the same cycle structure. Order d = lcm(1..m).
//
// Instead of iterating over all d powers (astronomically large for m=100),
// use the cycle structure to compute sum_{k=1}^{d} rank(pi^k) analytically.
//
// rank(perm) = 1 + sum_i lehmer[i] * (n-1-i)!
// lehmer[i] = |{j > i : perm[j] < perm[i]}|
//
// sum_{k=0}^{d-1} rank(pi^k) = d + sum_{i<j} fact[n-1-i] * C(i,j)
// where C(i,j) = total count of k in 0..d-1 where perm^k(j) < perm^k(i).
//
// C(i,j) depends on the cycles containing i and j, and can be computed
// from precomputed per-period counts.

const MOD: u64 = 1_000_000_007;

fn power_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: u64, m: u64) -> u64 {
    power_mod(a, m - 2, m)
}

fn triangular(k: usize) -> usize {
    k * (k + 1) / 2
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Compute lcm(1..=m) mod p, using sieve to find highest prime powers <= m.
fn compute_lcm_mod(m: usize, p: u64) -> u64 {
    let mut is_prime = vec![true; m + 1];
    is_prime[0] = false;
    if m >= 1 {
        is_prime[1] = false;
    }
    let mut res = 1u64;
    for i in 2..=m {
        if is_prime[i] {
            // Find highest power of i that is <= m
            let mut pp = i as u64;
            while pp * (i as u64) <= m as u64 {
                pp *= i as u64;
            }
            res = (res as u128 * pp as u128 % p as u128) as u64;
            // Sieve composites
            let mut j = i * i;
            while j <= m {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    res
}

fn main() {
    let m = 100usize;
    let n = triangular(m); // 5050

    // Build sigma: cyclic shift within each block k (length k)
    // Block k spans positions [tri(k-1) .. tri(k)-1] (0-indexed).
    // sigma(j) = j+1 for non-last, sigma(last) = first.
    let mut sigma = vec![0usize; n];
    for k in 1..=m {
        let start = triangular(k - 1); // first position of block k (0-indexed)
        let end = triangular(k) - 1;   // last position of block k (0-indexed)
        for j in start..end {
            sigma[j] = j + 1; // shift forward
        }
        sigma[end] = start; // wrap last -> first
    }

    // Build tau (0-indexed): tau[i] = (a * (i+1)) % n  (0-indexed)
    let a_val = 1_000_000_007u64;
    let mut tau = vec![0usize; n];
    for i in 0..n {
        tau[i] = ((a_val * (i as u64 + 1)) % n as u64) as usize;
    }

    // Build tau inverse (0-indexed)
    let mut tau_inv = vec![0usize; n];
    for i in 0..n {
        tau_inv[tau[i]] = i;
    }

    // pi = tau_inv o sigma o tau (0-indexed)
    // pi[i] = tau_inv[sigma[tau[i]]]
    let mut pi = vec![0usize; n];
    for i in 0..n {
        pi[i] = tau_inv[sigma[tau[i]]];
    }

    // Find cycles of pi (0-indexed)
    let mut visited = vec![false; n];
    let mut cycle_id = vec![0usize; n];   // which cycle each position belongs to
    let mut offset_in_cycle = vec![0usize; n]; // offset within cycle
    let mut cycles: Vec<Vec<usize>> = Vec::new(); // cycles[c] = list of positions in cycle order

    for start in 0..n {
        if visited[start] {
            continue;
        }
        let cid = cycles.len();
        let mut cycle = Vec::new();
        let mut pos = start;
        loop {
            visited[pos] = true;
            cycle_id[pos] = cid;
            offset_in_cycle[pos] = cycle.len();
            cycle.push(pos);
            pos = pi[pos];
            if pos == start {
                break;
            }
        }
        cycles.push(cycle);
    }

    let num_cycles = cycles.len();

    // For each cycle, store the values (0-indexed) in cycle order
    // In cycle c, position cycles[c][t] maps to cycles[c][(t+1) % l] under pi.
    // Under perm^k, position cycles[c][t] has value cycles[c][(t+k) % l].
    // The "value" at a position in the identity is just the position index itself.
    // So perm^k maps position p to the position that is k steps ahead in the cycle.
    // But for rank computation, we need the two-line notation value.
    // perm^k[pos] = the value (position) that pos maps to after k applications.
    // If pos = cycles[c][t], then perm^k(pos) = cycles[c][(t+k) % l].
    // So the value at position pos in permutation perm^k is cycles[c][(t+k) % l].

    // Precompute factorials mod MOD
    let mut fact = vec![0u64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }

    // Compute d mod MOD = lcm(1..m) mod MOD
    // The cycle lengths of pi are exactly the cycle lengths of sigma.
    // sigma has cycles of lengths 1, 2, ..., m (one block of size l for each l).
    // Actually, let's verify: sigma acts on blocks. Block l has positions
    // tri(l-1)..tri(l)-1 (0-indexed), i.e., l positions. Within block l,
    // sigma is identity except the last position maps to the first.
    // So sigma cyclically shifts each block, giving a cycle of length l.
    // Since pi is conjugate to sigma, it has the same cycle lengths.
    //
    // Verify cycle lengths:
    let mut cycle_lengths: Vec<usize> = cycles.iter().map(|c| c.len()).collect();
    cycle_lengths.sort();
    // Should be 1, 2, 3, ..., 100
    debug_assert_eq!(cycle_lengths, (1..=m).collect::<Vec<_>>());

    let d_mod = compute_lcm_mod(m, MOD);
    let inv_d = mod_inv(d_mod, MOD);

    // For same-cycle pairs:
    // f_c[delta] = count over one period l of [vals[(s+delta) % l] < vals[s]]
    // where vals[t] = cycles[c][t] (the 0-indexed position = value in identity)
    //
    // For cross-cycle pairs (a, b):
    // G_ab[delta] for delta = 0..gcd(l_a, l_b)-1
    // G_ab[delta] = sum_{s_a=0}^{l_a-1} sum_{s_b: s_b ≡ s_a+delta (mod g)} [vals_b[s_b] < vals_a[s_a]]
    // Then C(i,j) = G_ab[delta_ij] * (d / lcm(l_a, l_b)) for i in a, j in b, i < j
    // where delta_ij = (offset_b[j] - offset_a[i]) mod g

    // Precompute same-cycle f tables
    // f_same[c][delta] for c = 0..num_cycles-1, delta = 0..len-1
    let mut f_same: Vec<Vec<u64>> = Vec::with_capacity(num_cycles);
    for c in 0..num_cycles {
        let l = cycles[c].len();
        let vals = &cycles[c]; // vals[t] = position (= value) at offset t
        let mut f = vec![0u64; l];
        for delta in 0..l {
            let mut count = 0u64;
            for s in 0..l {
                if vals[(s + delta) % l] < vals[s] {
                    count += 1;
                }
            }
            f[delta] = count;
        }
        f_same.push(f);
    }

    // Precompute cross-cycle G tables
    // G_cross[a][b] = Vec of g values, for a < b only (by cycle id)
    // Also need G_cross[b][a] for when i is in b, j in a.
    // Note: G_ab counts [val_b < val_a], G_ba counts [val_a < val_b].
    // G_ba[delta'] for the reverse direction.
    //
    // Actually, let's think carefully. For i in cycle a, j in cycle b (i < j):
    // C(i,j) = count of k where perm^k(j) < perm^k(i)
    //        = count of k where val at j < val at i
    // val at i = cycles[a][(offset_a + k) % l_a]
    // val at j = cycles[b][(offset_b + k) % l_b]
    //
    // Over one period p = lcm(l_a, l_b), this count = G(a->b, delta)
    // where delta = (offset_b - offset_a) mod gcd(l_a, l_b)
    // and G(a->b, delta) = sum_{s_a} sum_{s_b ≡ s_a+delta (mod g)} [vals_b[s_b] < vals_a[s_a]]
    //
    // For i in cycle b, j in cycle a (i < j):
    // C(i,j) = count of k where val at j < val at i
    // val at i = cycles[b][(offset_b + k) % l_b]
    // val at j = cycles[a][(offset_a + k) % l_a]
    // = count of k where vals_a[(offset_a+k)%l_a] < vals_b[(offset_b+k)%l_b]
    // = G(b->a, delta) where delta = (offset_a - offset_b) mod g
    // and G(b->a, delta) = sum_{s_b} sum_{s_a ≡ s_b+delta (mod g)} [vals_a[s_a] < vals_b[s_b]]
    //
    // Note G(a->b, delta) + G(b->a, delta') may not sum to p because delta and delta' differ.
    // But for a given k: [vals_b < vals_a] + [vals_a < vals_b] = 1 (distinct values).
    // Over the full period: G(a->b, delta) counted at (offset_a, offset_b) +
    //   G(b->a, delta') counted at (offset_b, offset_a) where delta' is derived from the same k.
    // Actually for the same (offset_a, offset_b), the delta for a->b is (offset_b - offset_a) mod g,
    // and if we look at b->a with the same offsets, delta = (offset_a - offset_b) mod g = g - delta (if delta > 0).
    // And G(a->b, delta) + G(b->a, g-delta) = p (since for each k, exactly one of the two comparisons holds).
    //
    // So we only need G(a->b, delta) and can derive G(b->a, delta') = p - G(a->b, g - delta') (with appropriate mod).

    // Store G_cross as a flat structure indexed by cycle pair
    // For efficiency, use a 2D array indexed by (cycle_a_len, cycle_b_len) -> G values
    // But cycles have unique lengths 1..100, so we can index by (length_a, length_b).
    // Actually, let's just compute on the fly for each pair of cycles.

    // We need to iterate over all pairs (i, j) with i < j anyway (O(n^2)).
    // For each pair, look up the precomputed count.
    // Let's precompute G_cross tables for all cycle pairs.

    // Index by (min_cid, max_cid) -> (g, Vec<u64> of length g for "a->b" direction)
    // where a = min_cid, b = max_cid.

    // Actually, the direction matters. Let me store for each ordered pair (a, b) with a != b.
    // G_ab[delta] = sum_{s_a=0}^{l_a-1} sum_{s_b ≡ s_a+delta (mod g)} [vals_b[s_b] < vals_a[s_a]]
    // This counts: val in cycle b < val in cycle a.
    // Used when position i is in cycle a, position j is in cycle b.

    // Store as HashMap or 2D Vec. Since num_cycles <= 100, use 2D Vec.
    // g_cross[a][b] = Vec<u64> of length gcd(l_a, l_b)
    let mut g_cross: Vec<Vec<Vec<u64>>> = vec![Vec::new(); num_cycles];
    for a in 0..num_cycles {
        g_cross[a] = vec![Vec::new(); num_cycles];
    }

    for a in 0..num_cycles {
        let la = cycles[a].len();
        let vals_a = &cycles[a];
        for b in 0..num_cycles {
            if a == b {
                continue;
            }
            let lb = cycles[b].len();
            let vals_b = &cycles[b];
            let g = gcd(la, lb);

            // G_ab[delta] = sum_{s_a=0}^{la-1} sum_{s_b ≡ s_a+delta (mod g), s_b in 0..lb-1} [vals_b[s_b] < vals_a[s_a]]
            let mut g_vals = vec![0u64; g];

            // For each s_a, group s_b by (s_b - s_a) mod g
            // Precompute for each s_a: for each residue class of s_b, count vals_b[s_b] < vals_a[s_a]
            // Actually, let's precompute for cycle b: for each residue r mod g, sort the values in that class
            // Then for each s_a, binary search to count.

            // Group vals_b by s_b mod g
            let mut b_by_residue: Vec<Vec<usize>> = vec![Vec::new(); g];
            for s_b in 0..lb {
                b_by_residue[s_b % g].push(vals_b[s_b]);
            }
            // Sort each residue class
            for r in 0..g {
                b_by_residue[r].sort();
            }

            for s_a in 0..la {
                let va = vals_a[s_a];
                let sa_mod_g = s_a % g;
                for delta in 0..g {
                    // s_b ≡ s_a + delta (mod g), i.e., s_b mod g = (sa_mod_g + delta) % g
                    let target_residue = (sa_mod_g + delta) % g;
                    // Count vals_b[s_b] < va in this residue class
                    let class = &b_by_residue[target_residue];
                    let cnt = class.partition_point(|&v| v < va);
                    g_vals[delta] += cnt as u64;
                }
            }

            g_cross[a][b] = g_vals;
        }
    }

    // Now compute the rank sum.
    // rank_sum = d + sum_{i=0}^{n-1} fact[n-1-i] * W[i]
    // where W[i] = sum_{j>i} C(i,j)
    // C(i,j) = count_per_period(i,j) * (d / period)
    //
    // For same cycle: C(i,j) = f_same[c][delta] * (d / l) mod MOD
    // For cross cycle (i in a, j in b): C(i,j) = G_cross[a][b][delta] * (d / lcm(la,lb)) mod MOD
    //
    // We compute everything mod MOD.

    // Precompute (d / l) mod MOD for each cycle length l
    // d / l mod MOD = d_mod * mod_inv(l, MOD)
    let mut d_over_l: Vec<u64> = vec![0; m + 1];
    for l in 1..=m {
        d_over_l[l] = d_mod % MOD * mod_inv(l as u64, MOD) % MOD;
    }

    // Precompute (d / lcm(la, lb)) mod MOD for each pair of cycle lengths
    // d / lcm(la,lb) = d * gcd(la,lb) / (la*lb)
    // mod MOD: d_mod * gcd(la,lb) % MOD * mod_inv(la * lb % MOD, MOD) % MOD
    // But la*lb might overflow u64? No, max is 100*100 = 10000. Fine.
    let mut d_over_lcm: Vec<Vec<u64>> = vec![vec![0; m + 1]; m + 1];
    for la in 1..=m {
        for lb in 1..=m {
            if la == lb {
                continue; // same cycle, handled differently
            }
            let g = gcd(la, lb);
            let lcm_val = la * lb / g;
            d_over_lcm[la][lb] = d_mod % MOD * mod_inv(lcm_val as u64, MOD) % MOD;
        }
    }

    // Sort positions within each cycle by their position index (for efficient iteration)
    // cycles[c] is in cycle order, but we also need to know the position indices sorted.
    // Let's build for each cycle: sorted list of (position_index, offset_in_cycle)
    let mut cycle_sorted: Vec<Vec<(usize, usize)>> = Vec::with_capacity(num_cycles);
    for c in 0..num_cycles {
        let mut sorted: Vec<(usize, usize)> = cycles[c]
            .iter()
            .enumerate()
            .map(|(offset, &pos)| (pos, offset))
            .collect();
        sorted.sort();
        cycle_sorted.push(sorted);
    }

    // Main computation: for each position i from 0 to n-1, compute W[i] and accumulate
    // T += fact[n-1-i] * W[i]
    //
    // This is O(n^2) in the straightforward approach: for each i, iterate over all j > i.
    // With n = 5050, this is ~12.7M operations, each O(1). Fast enough.
    //
    // Optimization: instead of iterating over all j > i for each i, we can process
    // cycle by cycle. For each position i (in cycle a), the contributions come from:
    // 1. Same-cycle positions j > i in cycle a
    // 2. Cross-cycle positions j > i in each other cycle b

    // Build an array: for each position pos, store (cycle_id, offset)
    // Already have cycle_id[] and offset_in_cycle[].

    // For efficient counting, for each cycle c, build a sorted list of positions.
    // cycle_sorted[c] is already sorted by position.

    let mut t_sum = 0u64; // This will hold sum_{i} fact[n-1-i] * W[i] mod MOD

    // Process each position i
    for i in 0..n {
        let ca = cycle_id[i];
        let la = cycles[ca].len();
        let oa = offset_in_cycle[i];
        let fi = fact[n - 1 - i];

        let mut wi = 0u64; // W[i] mod MOD

        // Same-cycle contribution: positions j > i in the same cycle ca
        for &(pos_j, off_j) in &cycle_sorted[ca] {
            if pos_j <= i {
                continue;
            }
            // delta = (off_j - oa) mod la
            let delta = (off_j + la - oa) % la;
            // C(i,j) = f_same[ca][delta] * d_over_l[la] mod MOD
            let c_ij = f_same[ca][delta] % MOD * d_over_l[la] % MOD;
            wi = (wi + c_ij) % MOD;
        }

        // Cross-cycle contribution: for each other cycle cb, positions j > i
        for cb in 0..num_cycles {
            if cb == ca {
                continue;
            }
            let lb = cycles[cb].len();
            let g = gcd(la, lb);
            let g_table = &g_cross[ca][cb];
            let dol = d_over_lcm[la][lb];

            for &(pos_j, off_j) in &cycle_sorted[cb] {
                if pos_j <= i {
                    continue;
                }
                // delta = (off_j - oa) mod g
                let delta = (off_j % g + g - oa % g) % g;
                let c_ij = g_table[delta] % MOD * dol % MOD;
                wi = (wi + c_ij) % MOD;
            }
        }

        t_sum = (t_sum + fi % MOD * wi % MOD) % MOD;
    }

    // rank_sum = d + T (mod MOD)
    let rank_sum = (d_mod + t_sum) % MOD;

    // answer = (m! / d) * rank_sum mod MOD = fact[m] * inv_d * rank_sum mod MOD
    let factorial_m = fact[m];
    let answer = factorial_m % MOD * inv_d % MOD * rank_sum % MOD;

    println!("{}", answer);
}
