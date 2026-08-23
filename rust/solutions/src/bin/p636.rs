// Project Euler 636 - Restricted Factorisations
// Partition enumeration + coin-change DP over prime exponents of N!
//
// Profiles whose coins cannot make exponent 1 (no jump of 1) contribute 0
// because v_p(N!) = 1 for many primes. Remaining profiles share sorted-jump
// prefixes, so a trie reuses DP along common coins.

use rayon::prelude::*;

const N_FACT: usize = 1_000_000;
const MOD: u64 = 1_000_000_007;
const MOD32: u32 = 1_000_000_007;

#[inline(always)]
fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn add_mod_u32(a: u32, b: u32) -> u32 {
    let v = a + b;
    let t = v.wrapping_sub(MOD32);
    v.min(t)
}

#[cfg(target_arch = "x86_64")]
fn simd_level() -> u8 {
    static L: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    *L.get_or_init(|| {
        if is_x86_feature_detected!("avx2") {
            2
        } else if is_x86_feature_detected!("sse4.1") {
            1
        } else {
            0
        }
    })
}

/// DP after coins {1, c}: `1/(1-x) * 1/(1-x^c)` has coefficients `1 + i/c < MOD`.
fn init_one_and(dp: &mut [u32], c: usize) {
    let n = dp.len();
    match c {
        2 => {
            for i in 0..n {
                dp[i] = 1 + (i as u32 >> 1);
            }
        }
        3 => {
            for i in 0..n {
                dp[i] = 1 + (i as u32) / 3;
            }
        }
        4 => {
            for i in 0..n {
                dp[i] = 1 + (i as u32 >> 2);
            }
        }
        c => {
            let mut v = 1u32;
            let mut nxt = c;
            for i in 0..n {
                if i == nxt {
                    v += 1;
                    nxt += c;
                }
                dp[i] = v;
            }
        }
    }
}

fn apply_coin(dp: &mut [u32], c: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        let lvl = simd_level();
        if c >= 8 && lvl >= 2 {
            unsafe { apply_coin_avx2(dp, c) };
            return;
        }
        if c >= 4 && lvl >= 1 {
            unsafe { apply_coin_sse41(dp, c) };
            return;
        }
    }
    apply_coin_scalar(dp, c);
}

fn apply_coin_scalar(dp: &mut [u32], c: usize) {
    let n = dp.len();
    // SAFETY: i starts at c and stays < n, so i-c is in-bounds.
    unsafe {
        let p = dp.as_mut_ptr();
        if c >= 4 {
            let mut i = c;
            while i + 4 <= n {
                let v0 = add_mod_u32(*p.add(i), *p.add(i - c));
                let v1 = add_mod_u32(*p.add(i + 1), *p.add(i + 1 - c));
                let v2 = add_mod_u32(*p.add(i + 2), *p.add(i + 2 - c));
                let v3 = add_mod_u32(*p.add(i + 3), *p.add(i + 3 - c));
                *p.add(i) = v0;
                *p.add(i + 1) = v1;
                *p.add(i + 2) = v2;
                *p.add(i + 3) = v3;
                i += 4;
            }
            while i < n {
                *p.add(i) = add_mod_u32(*p.add(i), *p.add(i - c));
                i += 1;
            }
        } else {
            let mut i = c;
            while i + 2 <= n {
                let v0 = add_mod_u32(*p.add(i), *p.add(i - c));
                let v1 = add_mod_u32(*p.add(i + 1), *p.add(i + 1 - c));
                *p.add(i) = v0;
                *p.add(i + 1) = v1;
                i += 2;
            }
            while i < n {
                *p.add(i) = add_mod_u32(*p.add(i), *p.add(i - c));
                i += 1;
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn apply_coin_avx2(dp: &mut [u32], c: usize) {
    use std::arch::x86_64::*;
    let n = dp.len();
    let p = dp.as_mut_ptr();
    let modv = _mm256_set1_epi32(MOD32 as i32);
    let mut i = c;
    // SAFETY: c >= 8 so 8-wide stores do not overlap the reads at i-c; i+16 <= n.
    unsafe {
        while i + 16 <= n {
            let a0 = _mm256_loadu_si256(p.add(i) as *const __m256i);
            let b0 = _mm256_loadu_si256(p.add(i - c) as *const __m256i);
            let s0 = _mm256_add_epi32(a0, b0);
            let r0 = _mm256_min_epu32(s0, _mm256_sub_epi32(s0, modv));
            _mm256_storeu_si256(p.add(i) as *mut __m256i, r0);

            let a1 = _mm256_loadu_si256(p.add(i + 8) as *const __m256i);
            let b1 = _mm256_loadu_si256(p.add(i + 8 - c) as *const __m256i);
            let s1 = _mm256_add_epi32(a1, b1);
            let r1 = _mm256_min_epu32(s1, _mm256_sub_epi32(s1, modv));
            _mm256_storeu_si256(p.add(i + 8) as *mut __m256i, r1);
            i += 16;
        }
        while i + 8 <= n {
            let a = _mm256_loadu_si256(p.add(i) as *const __m256i);
            let b = _mm256_loadu_si256(p.add(i - c) as *const __m256i);
            let s = _mm256_add_epi32(a, b);
            _mm256_storeu_si256(
                p.add(i) as *mut __m256i,
                _mm256_min_epu32(s, _mm256_sub_epi32(s, modv)),
            );
            i += 8;
        }
        while i < n {
            *p.add(i) = add_mod_u32(*p.add(i), *p.add(i - c));
            i += 1;
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.1")]
unsafe fn apply_coin_sse41(dp: &mut [u32], c: usize) {
    use std::arch::x86_64::*;
    let n = dp.len();
    let p = dp.as_mut_ptr();
    let modv = _mm_set1_epi32(MOD32 as i32);
    let mut i = c;
    // SAFETY: c >= 4 so 4-wide stores do not overlap the reads at i-c; i+8 <= n.
    unsafe {
        while i + 8 <= n {
            let a0 = _mm_loadu_si128(p.add(i) as *const __m128i);
            let b0 = _mm_loadu_si128(p.add(i - c) as *const __m128i);
            let s0 = _mm_add_epi32(a0, b0);
            _mm_storeu_si128(
                p.add(i) as *mut __m128i,
                _mm_min_epu32(s0, _mm_sub_epi32(s0, modv)),
            );
            let a1 = _mm_loadu_si128(p.add(i + 4) as *const __m128i);
            let b1 = _mm_loadu_si128(p.add(i + 4 - c) as *const __m128i);
            let s1 = _mm_add_epi32(a1, b1);
            _mm_storeu_si128(
                p.add(i + 4) as *mut __m128i,
                _mm_min_epu32(s1, _mm_sub_epi32(s1, modv)),
            );
            i += 8;
        }
        while i + 4 <= n {
            let a = _mm_loadu_si128(p.add(i) as *const __m128i);
            let b = _mm_loadu_si128(p.add(i - c) as *const __m128i);
            let s = _mm_add_epi32(a, b);
            _mm_storeu_si128(
                p.add(i) as *mut __m128i,
                _mm_min_epu32(s, _mm_sub_epi32(s, modv)),
            );
            i += 4;
        }
        while i < n {
            *p.add(i) = add_mod_u32(*p.add(i), *p.add(i - c));
            i += 1;
        }
    }
}

fn factorial_val(n: usize) -> i64 {
    (2..=n as i64).product::<i64>()
}

type GC = [i32; 4];

fn init_group_comps() -> Vec<GC> {
    let mut gcs = Vec::new();
    for n1 in 0..=1i32 {
        for n2 in 0..=2 {
            for n3 in 0..=3 {
                for n4 in 0..=4 {
                    if n1 + n2 + n3 + n4 > 0 {
                        gcs.push([n1, n2, n3, n4]);
                    }
                }
            }
        }
    }
    gcs
}

fn multinomial(n: usize, groups: &[i32]) -> i64 {
    let mut r = factorial_val(n);
    for &g in groups {
        r /= factorial_val(g as usize);
    }
    r
}

struct TrieNode {
    children: Vec<(u8, u32)>,
    coeff: i64,
    edges: u32,
}

fn eval_leaf(coeff: i64, dp: &[u32], distinct: &[(u32, u32)]) -> u64 {
    let mut prod = 1u64;
    for &(e, mult) in distinct {
        // SAFETY: e <= max_e = dp.len()-1
        let base = unsafe { *dp.get_unchecked(e as usize) } as u64;
        if base == 0 {
            return 0;
        }
        if base != 1 {
            prod = prod * pow_mod(base, mult as u64) % MOD;
        }
    }
    let c = coeff.rem_euclid(MOD as i64) as u64;
    prod * c % MOD
}

fn dfs(idx: usize, dp: &mut [u32], nodes: &[TrieNode], distinct: &[(u32, u32)]) -> u64 {
    let children = nodes[idx].children.as_slice();
    if children.is_empty() {
        return eval_leaf(nodes[idx].coeff, dp, distinct);
    }
    if let [(c, ch)] = children {
        apply_coin(dp, *c as usize);
        return dfs(*ch as usize, dp, nodes, distinct);
    }
    // Clone+steal only on large subtrees; tiny branches stay on one buffer.
    if nodes[idx].edges > 20 {
        let dp_shared: &[u32] = dp;
        return children
            .par_iter()
            .map(|&(c, ch)| {
                let mut d2 = dp_shared.to_vec();
                apply_coin(&mut d2, c as usize);
                dfs(ch as usize, &mut d2, nodes, distinct)
            })
            .sum::<u64>()
            % MOD;
    }
    let mut acc = 0u64;
    let last = children.len() - 1;
    for &(c, ch) in &children[..last] {
        let mut d2 = dp.to_vec();
        apply_coin(&mut d2, c as usize);
        acc += dfs(ch as usize, &mut d2, nodes, distinct);
    }
    apply_coin(dp, children[last].0 as usize);
    acc += dfs(children[last].1 as usize, dp, nodes, distinct);
    acc % MOD
}

fn main() {
    let gcs = init_group_comps();

    let mut all_parts: Vec<Vec<GC>> = Vec::new();

    fn enum_parts(
        rem: [i32; 4],
        min_idx: usize,
        cur: &mut Vec<GC>,
        gcs: &[GC],
        all_parts: &mut Vec<Vec<GC>>,
    ) {
        if rem == [0, 0, 0, 0] {
            let mut p = cur.clone();
            p.sort();
            if !all_parts.contains(&p) {
                all_parts.push(p);
            }
            return;
        }
        for idx in min_idx..gcs.len() {
            let gc = &gcs[idx];
            if gc[0] <= rem[0] && gc[1] <= rem[1] && gc[2] <= rem[2] && gc[3] <= rem[3] {
                let new_rem = [rem[0] - gc[0], rem[1] - gc[1], rem[2] - gc[2], rem[3] - gc[3]];
                cur.push(*gc);
                enum_parts(new_rem, idx, cur, gcs, all_parts);
                cur.pop();
            }
        }
    }

    let mut cur = Vec::new();
    enum_parts([1, 2, 3, 4], 0, &mut cur, &gcs, &mut all_parts);

    let mut jump_map: Vec<(Vec<i32>, i64)> = Vec::new();
    for p in &all_parts {
        let m = p.len();
        let sign: i64 = if (10 - m) % 2 == 0 { 1 } else { -1 };
        let bf: i64 = p
            .iter()
            .map(|g| factorial_val((g[0] + g[1] + g[2] + g[3]) as usize - 1))
            .product();

        let mut ways = 1i64;
        let tot = [1, 2, 3, 4];
        for t in 0..4 {
            let gs: Vec<i32> = p.iter().map(|g| g[t]).collect();
            ways *= multinomial(tot[t] as usize, &gs);
        }

        let mut i = 0;
        while i < m {
            let mut cnt = 1;
            while i + cnt < m && p[i + cnt] == p[i] {
                cnt += 1;
            }
            ways /= factorial_val(cnt);
            i += cnt;
        }

        let coeff = sign * bf * ways;
        let mut jumps: Vec<i32> = p.iter().map(|g| g[0] + 2 * g[1] + 3 * g[2] + 4 * g[3]).collect();
        jumps.sort();

        if let Some(pos) = jump_map.iter().position(|(k, _)| k == &jumps) {
            jump_map[pos].1 += coeff;
        } else {
            jump_map.push((jumps, coeff));
        }
    }

    // Sieve primes, compute exponents in N_FACT!
    let mut is_prime = vec![true; N_FACT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= N_FACT {
        if is_prime[i] {
            let mut j = i * i;
            while j <= N_FACT {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut max_e = 0usize;
    let mut exp_count: Vec<u32> = Vec::new();

    for p in 2..=N_FACT {
        if !is_prime[p] {
            continue;
        }
        let mut e = 0usize;
        let mut pk = p as u64;
        while pk <= N_FACT as u64 {
            e += N_FACT / pk as usize;
            if pk > N_FACT as u64 / p as u64 {
                break;
            }
            pk *= p as u64;
        }
        if e > max_e {
            max_e = e;
        }
        if e >= exp_count.len() {
            exp_count.resize(e + 1, 0);
        }
        exp_count[e] += 1;
    }

    let distinct: Vec<(u32, u32)> = (1..=max_e)
        .filter(|&e| e < exp_count.len() && exp_count[e] > 0)
        .map(|e| (e as u32, exp_count[e]))
        .collect();

    // Trie of profiles that contain a jump of 1. Leading 1 is applied as dp.fill(1).
    let mut nodes = vec![TrieNode {
        children: Vec::new(),
        coeff: 0,
        edges: 0,
    }];
    for (jumps, coeff) in jump_map {
        if coeff == 0 || jumps.first() != Some(&1) {
            continue;
        }
        let mut idx = 0usize;
        for &j in jumps.iter().skip(1) {
            let j = j as u8;
            if let Some(&(_, ch)) = nodes[idx].children.iter().find(|(c, _)| *c == j) {
                idx = ch as usize;
            } else {
                let ch = nodes.len() as u32;
                nodes[idx].children.push((j, ch));
                nodes.push(TrieNode {
                    children: Vec::new(),
                    coeff: 0,
                    edges: 0,
                });
                idx = ch as usize;
            }
        }
        nodes[idx].coeff += coeff;
    }
    fn fill_edges(idx: usize, nodes: &mut [TrieNode]) -> u32 {
        let ch: Vec<u32> = nodes[idx].children.iter().map(|&(_, c)| c).collect();
        let mut e = 0u32;
        for c in ch {
            e += 1 + fill_edges(c as usize, nodes);
        }
        nodes[idx].edges = e;
        e
    }
    fill_edges(0, &mut nodes);

    let dp_len = max_e + 1;
    let answer = if nodes[0].children.is_empty() {
        eval_leaf(nodes[0].coeff, &vec![1u32; dp_len], &distinct)
    } else {
        let nodes = &nodes;
        let distinct = &distinct;
        nodes[0]
            .children
            .par_iter()
            .map(|&(c, ch)| {
                let mut dp = vec![0u32; dp_len];
                init_one_and(&mut dp, c as usize);
                dfs(ch as usize, &mut dp, nodes, distinct)
            })
            .sum::<u64>()
            % MOD
    };

    // Divide by 1!*2!*3!*4! = 288
    let answer = answer * pow_mod(288, MOD - 2) % MOD;
    println!("{}", answer);
}
