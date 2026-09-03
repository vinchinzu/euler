// Project Euler 861 - Bi-unitary divisors
//
// We need sum_{k=2}^{10} Q_k(10^12), where Q_k(N) counts n <= N with
// P(n) = n^k (P(n) = product of all bi-unitary divisors of n).
//
// For n = prod p_i^a_i:
//   D_bu(n) = prod f(a_i), with f(a) = a (a even), a+1 (a odd)
// and P(n) = n^(D_bu(n)/2), so we need D_bu(n) = 2k.
//
// Steps:
// 1) Generate exponent signatures [a_i] such that prod f(a_i) = 2k.
// 2) For each signature, enumerate unique exponent permutations in
//    increasing-prime order (p1 < p2 < ...).
// 3) Count tuples with DFS + pi(x) for the final prime.
//    This removes expensive cross-group collision checks from prior version.

use rayon::prelude::*;
use std::collections::BTreeSet;

const N_LIMIT: u64 = 1_000_000_000_000;
const SIEVE_LIM: usize = 1_000_000;
const MAX_EXP: usize = 20;

fn sieve_primes(limit: usize) -> Vec<u32> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2usize;
    while p * p <= limit {
        if is_prime[p] {
            let mut j = p * p;
            while j <= limit {
                is_prime[j] = false;
                j += p;
            }
        }
        p += 1;
    }

    (2..=limit).filter(|&x| is_prime[x]).map(|x| x as u32).collect()
}

struct PrimePi {
    n: u64,
    isqrt: u64,
    s_small: Vec<i32>,
    s_large: Vec<i64>,
}

impl PrimePi {
    fn new(n: u64, primes: &[u32]) -> Self {
        let mut isqrt = (n as f64).sqrt() as u64;
        while (isqrt + 1).saturating_mul(isqrt + 1) <= n {
            isqrt += 1;
        }
        while isqrt.saturating_mul(isqrt) > n {
            isqrt -= 1;
        }

        let m = isqrt as usize;
        let mut s_small = vec![0i32; m + 1];
        let mut s_large = vec![0i64; m + 1];

        for v in 0..=m {
            s_small[v] = v as i32 - 1;
        }
        for k in 1..=m {
            s_large[k] = (n / k as u64) as i64 - 1;
        }

        let s_ptr = s_small.as_mut_ptr();
        let l_ptr = s_large.as_mut_ptr();

        for &p32 in primes {
            let p = p32 as u64;
            if p > isqrt {
                break;
            }
            let p2 = p * p;
            if p2 > n {
                break;
            }

            let p_usize = p as usize;
            let sp_1 = unsafe { *s_ptr.add(p_usize - 1) } as i64;
            let k_limit = ((n / p2) as usize).min(m);
            let i_split = (m / p_usize).min(k_limit);

            let mut kp = p_usize;
            unsafe {
                for k in 1..=i_split {
                    let s_target = *l_ptr.add(kp);
                    *l_ptr.add(k) -= s_target - sp_1;
                    kp += p_usize;
                }
            }

            let n_div_p = n / p;
            let mut k = i_split + 1;

            unsafe {
                if n_div_p <= u32::MAX as u64 {
                    let nd = n_div_p as u32;
                    let isqrt_m = ((nd as f32).sqrt() as usize).min(k_limit);
                    while k + 1 <= isqrt_m {
                        let q0 = (nd / (k as u32)) as usize;
                        let q1 = (nd / ((k + 1) as u32)) as usize;
                        let s_target0 = *s_ptr.add(q0) as i64;
                        let s_target1 = *s_ptr.add(q1) as i64;
                        *l_ptr.add(k) -= s_target0 - sp_1;
                        *l_ptr.add(k + 1) -= s_target1 - sp_1;
                        k += 2;
                    }
                    if k <= isqrt_m {
                        let q = (nd / (k as u32)) as usize;
                        let s_target = *s_ptr.add(q) as i64;
                        *l_ptr.add(k) -= s_target - sp_1;
                        k += 1;
                    }
                    while k <= k_limit {
                        let q = nd / (k as u32);
                        let k_last = ((nd / q) as usize).min(k_limit);
                        let diff = *s_ptr.add(q as usize) as i64 - sp_1;
                        let count = k_last - k + 1;
                        let ptr = l_ptr.add(k);
                        for offset in 0..count {
                            *ptr.add(offset) -= diff;
                        }
                        k = k_last + 1;
                    }
                } else {
                    let isqrt_m = ((n_div_p as f64).sqrt() as usize).min(k_limit);
                    while k + 1 <= isqrt_m {
                        let q0 = (n_div_p / (k as u64)) as usize;
                        let q1 = (n_div_p / ((k + 1) as u64)) as usize;
                        let s_target0 = *s_ptr.add(q0) as i64;
                        let s_target1 = *s_ptr.add(q1) as i64;
                        *l_ptr.add(k) -= s_target0 - sp_1;
                        *l_ptr.add(k + 1) -= s_target1 - sp_1;
                        k += 2;
                    }
                    if k <= isqrt_m {
                        let q = (n_div_p / (k as u64)) as usize;
                        let s_target = *s_ptr.add(q) as i64;
                        *l_ptr.add(k) -= s_target - sp_1;
                        k += 1;
                    }
                    while k <= k_limit {
                        let q = (n_div_p / (k as u64)) as usize;
                        let k_last = ((n_div_p / (q as u64)) as usize).min(k_limit);
                        let diff = *s_ptr.add(q) as i64 - sp_1;
                        let count = k_last - k + 1;
                        let ptr = l_ptr.add(k);
                        for offset in 0..count {
                            *ptr.add(offset) -= diff;
                        }
                        k = k_last + 1;
                    }
                }
            }

            if p2 <= isqrt {
                let mut v = m;
                let sp_1_i32 = sp_1 as i32;
                unsafe {
                    while v >= p2 as usize {
                        let q = v / p_usize;
                        let diff = *s_ptr.add(q) - sp_1_i32;
                        let start = (q * p_usize).max(p2 as usize);
                        let count = v - start + 1;
                        let ptr = s_ptr.add(start);
                        for offset in 0..count {
                            *ptr.add(offset) -= diff;
                        }
                        v = start - 1;
                    }
                }
            }
        }

        Self {
            n,
            isqrt,
            s_small,
            s_large,
        }
    }

    fn pi(&self, x: u64) -> i64 {
        if x <= 1 {
            return 0;
        }
        if x <= self.isqrt {
            self.s_small[x as usize] as i64
        } else {
            self.s_large[(self.n / x) as usize]
        }
    }
}

fn integer_root(n: u64, k: u8) -> u64 {
    if k == 1 {
        return n;
    }
    if k == 2 {
        let mut r = (n as f64).sqrt() as u64;
        while (r + 1).saturating_mul(r + 1) <= n {
            r += 1;
        }
        while r.saturating_mul(r) > n {
            r -= 1;
        }
        return r;
    }
    if k == 3 {
        let mut r = (n as f64).cbrt() as u64;
        while (r + 1)
            .saturating_mul(r + 1)
            .saturating_mul(r + 1)
            <= n
        {
            r += 1;
        }
        while r.saturating_mul(r).saturating_mul(r) > n {
            r -= 1;
        }
        return r;
    }
    if n <= 1 {
        return n;
    }

    let k_u32 = k as u32;
    let mut lo = 1u64;
    let mut hi = (n as f64).powf(1.0 / k as f64) as u64 + 2;
    let mut ans = 1u64;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let mut p = 1u64;
        let mut over = false;
        for _ in 0..k_u32 {
            if p > n / mid {
                over = true;
                break;
            }
            p *= mid;
        }
        if !over && p <= n {
            ans = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    ans
}

fn factor_partitions(
    target: u32,
    count: u32,
    min_val: u32,
    current: &mut Vec<u8>,
    out: &mut Vec<Vec<u8>>,
) {
    if count == 1 {
        if target >= min_val {
            let mut v = current.clone();
            v.push(target as u8);
            out.push(v);
        }
        return;
    }

    for f in min_val..=target {
        if target % f == 0 {
            current.push(f as u8);
            factor_partitions(target / f, count - 1, f, current, out);
            current.pop();
        }
    }
}

fn generate_signatures(k: u32) -> Vec<Vec<u8>> {
    let mut sigs: BTreeSet<Vec<u8>> = BTreeSet::new();
    let mut r = 1u32;

    loop {
        let pow2 = 1u32 << (r - 1);
        if pow2 > k {
            break;
        }
        if k % pow2 == 0 {
            let target = k / pow2;
            let mut parts = Vec::new();
            let mut current = Vec::new();
            factor_partitions(target, r, 1, &mut current, &mut parts);

            for part in parts {
                let choices = 1u32 << r;
                for mask in 0..choices {
                    let mut exps = Vec::with_capacity(r as usize);
                    for i in 0..r as usize {
                        let y = part[i] as u16;
                        let a = if ((mask >> i) & 1) == 1 {
                            2 * y
                        } else {
                            2 * y - 1
                        };
                        exps.push(a as u8);
                    }
                    exps.sort_unstable();
                    sigs.insert(exps);
                }
            }
        }
        r += 1;
    }

    sigs.into_iter().collect()
}

fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    if a.len() < 2 {
        return false;
    }

    let mut i = a.len() - 2;
    loop {
        if a[i] < a[i + 1] {
            break;
        }
        if i == 0 {
            return false;
        }
        i -= 1;
    }

    let mut j = a.len() - 1;
    while a[j] <= a[i] {
        j -= 1;
    }
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}

fn unique_permutations(mut vals: Vec<u8>) -> Vec<Vec<u8>> {
    vals.sort_unstable();
    let mut out = Vec::new();
    loop {
        out.push(vals.clone());
        if !next_permutation(&mut vals) {
            break;
        }
    }
    out
}

struct Counter {
    n: u64,
    primes: Vec<u32>,
    pi_table: PrimePi,
    plen: usize,
    // Flat pow_table[exp * plen + idx]
    pow_table: Vec<u64>,
}

impl Counter {
    fn new(n: u64, primes: Vec<u32>, pi_table: PrimePi) -> Self {
        let plen = primes.len();
        let mut pow_table = vec![n + 1; (MAX_EXP + 1) * plen];
        for i in 0..plen {
            pow_table[i] = 1;
        }
        for e in 1..=MAX_EXP {
            let prev_offset = (e - 1) * plen;
            let cur_offset = e * plen;
            for (i, &p32) in primes.iter().enumerate() {
                let prev = pow_table[prev_offset + i];
                if prev > n {
                    continue;
                }
                let p = p32 as u64;
                let val = prev.saturating_mul(p);
                pow_table[cur_offset + i] = if val > n { n + 1 } else { val };
            }
        }

        Self {
            n,
            primes,
            pi_table,
            plen,
            pow_table,
        }
    }

    #[inline(always)]
    fn pow_at(&self, exp: usize, idx: usize) -> u64 {
        unsafe { *self.pow_table.get_unchecked(exp * self.plen + idx) }
    }

    fn count_permutation(&self, exps: &[u8]) -> u64 {
        if exps.len() == 1 {
            return self.count_last(exps[0], 0, self.n);
        }

        let exp0 = exps[0] as usize;
        let mut end = 0usize;
        while end < self.primes.len() {
            let p_pow = self.pow_at(exp0, end);
            if p_pow > self.n {
                break;
            }
            let rem = self.n / p_pow;
            if !self.can_fill_nonlast(exps, 1, end + 1, rem) {
                break;
            }
            end += 1;
        }

        let mut total = 0u64;
        for idx in 0..end {
            let rem = self.n / self.pow_at(exp0, idx);
            total += self.dfs(exps, 1, idx + 1, rem);
        }
        total
    }

    fn dfs(&self, exps: &[u8], pos: usize, start_idx: usize, rem: u64) -> u64 {
        let last = exps.len() - 1;
        if pos == last {
            return self.count_last(exps[pos], start_idx, rem);
        }

        let exp = exps[pos] as usize;
        let last_exp = exps[last] as usize;
        let mut total = 0u64;
        let mut idx = start_idx;

        while idx < self.primes.len() {
            let p_pow = self.pow_at(exp, idx);
            if p_pow > rem {
                break;
            }
            let rem2 = rem / p_pow;

            if !self.can_fill_nonlast(exps, pos + 1, idx + 1, rem2) {
                break;
            }

            if pos == last - 1 {
                if idx + 1 < self.primes.len() {
                    if self.pow_at(last_exp, idx + 1) > rem2 {
                        break;
                    }
                }
            }

            total += self.dfs(exps, pos + 1, idx + 1, rem2);
            idx += 1;
        }

        total
    }

    fn can_fill_nonlast(
        &self,
        exps: &[u8],
        from_pos: usize,
        mut start_idx: usize,
        mut rem: u64,
    ) -> bool {
        let last = exps.len() - 1;
        if from_pos >= last {
            return true;
        }

        for &e in &exps[from_pos..last] {
            if start_idx >= self.primes.len() {
                return false;
            }
            let need = self.pow_at(e as usize, start_idx);
            if need > rem {
                return false;
            }
            rem /= need;
            start_idx += 1;
        }
        true
    }

    fn count_last(&self, exp: u8, start_idx: usize, rem: u64) -> u64 {
        let limit = integer_root(rem, exp);
        if start_idx < self.primes.len() && limit < self.primes[start_idx] as u64 {
            return 0;
        }

        let lower_pi = start_idx as i64;
        let cnt = self.pi_table.pi(limit) - lower_pi;
        if cnt > 0 { cnt as u64 } else { 0 }
    }
}

fn main() {
    let primes = sieve_primes(SIEVE_LIM);
    let pi_table = PrimePi::new(N_LIMIT, &primes);
    let counter = Counter::new(N_LIMIT, primes, pi_table);

    // Collect all (sig, perm) work items across all k values
    let mut work: Vec<Vec<u8>> = Vec::new();
    for k in 2..=10u32 {
        let sigs = generate_signatures(k);
        for sig in sigs {
            for perm in unique_permutations(sig) {
                work.push(perm);
            }
        }
    }

    let total_sum: u64 = work
        .par_iter()
        .map(|perm| counter.count_permutation(perm))
        .sum();

    println!("{}", total_sum);
}
