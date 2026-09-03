use rayon::prelude::*;
// Problem 994: Counting Triangles
// T(1234*10^8, 2345*10^8) mod 10^9+7 with Du Jiao / totient prefix sums.
// Parallel doubling-interval Lucy recurrences, lowered 32-bit division, hyperbola splitting, interleaved AoS table.

const MOD: u64 = 1_000_000_007;
const INV2: u64 = (MOD + 1) / 2;
const INV6: u64 = 166_666_668;
const INV3: u64 = 333_333_336;

const SIEVE_LIMIT: usize = 20_000_000;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    (a * b) % MOD
}

#[inline(always)]
fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}

#[inline(always)]
fn add_mod(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn c2_mod(x: u64) -> u64 {
    let x = x % MOD;
    mul(mul(x, (x + MOD - 1) % MOD), INV2)
}

#[inline(always)]
fn c3_mod(x: u64) -> u64 {
    let x = x % MOD;
    mul(mul(mul(x, (x + MOD - 1) % MOD), (x + MOD - 2) % MOD), INV6)
}

/// `n(n+1)/2 mod MOD` with `n` already reduced into `0..MOD`.
#[inline(always)]
fn p1_small(n: u64) -> u64 {
    (n * (n + 1) / 2) % MOD
}

/// `n(n+1)(2n+1)/6 mod MOD` with `n` already reduced into `0..MOD`.
#[inline(always)]
fn p2_small(n: u64) -> u64 {
    let np1 = n + 1;
    let np1 = if np1 >= MOD { 0 } else { np1 };
    let c = 2 * n + 1;
    let c = if c >= MOD { c - MOD } else { c };
    mul(mul(n, np1), mul(c, INV6))
}

#[inline(always)]
fn p1(n: u64) -> u64 {
    p1_small(n % MOD)
}

#[inline(always)]
fn p2(n: u64) -> u64 {
    p2_small(n % MOD)
}

#[inline(always)]
fn p3(n: u64) -> u64 {
    let s = p1(n);
    mul(s, s)
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
struct PrefixItem {
    p0: u32,
    p1: u32,
    p2: u32,
}

struct TotientPrefix {
    limit: u64,
    pref: Vec<PrefixItem>,
    m_parent: u64,
    n_parent: u64,
    large_m: Vec<(u64, u64, u64)>,
    large_n: Vec<(u64, u64, u64)>,
}

impl TotientPrefix {
    fn new(limit: usize) -> Self {
        let pref = Self::build(limit);
        Self {
            limit: limit as u64,
            pref,
            m_parent: 0,
            n_parent: 0,
            large_m: Vec::new(),
            large_n: Vec::new(),
        }
    }

    fn build(limit: usize) -> Vec<PrefixItem> {
        let mut phi = vec![0u32; limit + 1];
        let mut primes = Vec::with_capacity(limit / 16);
        let mut lp = vec![0u16; limit + 1];
        phi[1] = 1;
        for i in 2..=limit {
            let lpi16 = unsafe { *lp.get_unchecked(i) };
            let (lpi, phi_i) = if lpi16 == 0 {
                primes.push(i as u32);
                let p_phi = (i - 1) as u32;
                unsafe { *phi.get_unchecked_mut(i) = p_phi; }
                (i as u32, p_phi)
            } else {
                (lpi16 as u32, unsafe { *phi.get_unchecked(i) })
            };

            for &p in &primes {
                let ip = i as u64 * p as u64;
                if ip > limit as u64 || p > lpi {
                    break;
                }
                let j = ip as usize;
                unsafe {
                    *lp.get_unchecked_mut(j) = p as u16;
                    *phi.get_unchecked_mut(j) = if p == lpi {
                        phi_i.wrapping_mul(p)
                    } else {
                        phi_i.wrapping_mul(p - 1)
                    };
                }
                if p == lpi {
                    break;
                }
            }
        }
        drop(lp);
        drop(primes);
        let mut pref = vec![PrefixItem::default(); limit + 1];

        // Parallel prefix sums over chunks
        let num_threads = rayon::current_num_threads();
        let chunk_size = (limit + num_threads - 1) / num_threads;

        let phi_ptr = phi.as_ptr() as usize;
        let pref_ptr = pref.as_mut_ptr() as usize;

        let chunk_totals: Vec<(u64, u64, u64)> = (0..num_threads)
            .into_par_iter()
            .map(|chunk_idx| {
                let start = 1 + chunk_idx * chunk_size;
                let end = (start + chunk_size).min(limit + 1);
                if start >= end {
                    return (0, 0, 0);
                }
                let mut s0 = 0u64;
                let mut s1 = 0u64;
                let mut s2 = 0u64;
                for i in start..end {
                    let ph = unsafe { *(phi_ptr as *const u32).add(i) } as u64;
                    s0 += ph;
                    if s0 >= MOD { s0 -= MOD; }
                    let im = i as u64;
                    s1 += (im * ph) % MOD;
                    if s1 >= MOD { s1 -= MOD; }
                    let im2 = (im * im) % MOD;
                    s2 += (im2 * ph) % MOD;
                    if s2 >= MOD { s2 -= MOD; }

                    unsafe {
                        let item_ptr = (pref_ptr as *mut PrefixItem).add(i);
                        (*item_ptr).p0 = s0 as u32;
                        (*item_ptr).p1 = s1 as u32;
                        (*item_ptr).p2 = s2 as u32;
                    }
                }
                (s0, s1, s2)
            })
            .collect();

        drop(phi);

        // Sequential prefix sum over chunk totals
        let mut offsets = Vec::with_capacity(num_threads);
        let mut off0 = 0u64;
        let mut off1 = 0u64;
        let mut off2 = 0u64;
        for (c0, c1, c2) in chunk_totals {
            offsets.push((off0, off1, off2));
            off0 = add_mod(off0, c0);
            off1 = add_mod(off1, c1);
            off2 = add_mod(off2, c2);
        }

        // Add offsets to chunks in parallel
        (0..num_threads).into_par_iter().for_each(|chunk_idx| {
            let (off0, off1, off2) = offsets[chunk_idx];
            if off0 == 0 && off1 == 0 && off2 == 0 {
                return;
            }
            let start = 1 + chunk_idx * chunk_size;
            let end = (start + chunk_size).min(limit + 1);
            for i in start..end {
                unsafe {
                    let item_ptr = (pref_ptr as *mut PrefixItem).add(i);
                    let v0 = (*item_ptr).p0 as u64;
                    let v1 = (*item_ptr).p1 as u64;
                    let v2 = (*item_ptr).p2 as u64;
                    (*item_ptr).p0 = add_mod(v0, off0) as u32;
                    (*item_ptr).p1 = add_mod(v1, off1) as u32;
                    (*item_ptr).p2 = add_mod(v2, off2) as u32;
                }
            }
        });
        pref
    }

    #[inline(always)]
    fn prefix_at(&self, n: u64) -> (u64, u64, u64) {
        let i = n as usize;
        let item = unsafe { self.pref.get_unchecked(i) };
        (item.p0 as u64, item.p1 as u64, item.p2 as u64)
    }

    #[inline(always)]
    fn lookup_large(
        &self,
        q: u64,
        parent: u64,
        large: &[(u64, u64, u64)],
    ) -> (u64, u64, u64) {
        if q <= self.limit {
            self.prefix_at(q)
        } else {
            // SAFETY: q > limit ⇒ parent/q <= parent/(limit+1) = max_i < large.len().
            unsafe { *large.get_unchecked((parent / q) as usize) }
        }
    }

    fn compute_one_u32(
        &self,
        x: u32,
        parent: u64,
        large: &[(u64, u64, u64)],
    ) -> (u64, u64, u64) {
        let x64 = x as u64;
        let mut f0 = p1(x64);
        let mut f1 = p2(x64);
        let mut f2 = p3(x64);
        let mut prev_p1 = 1u64;
        let mut prev_p2 = 1u64;
        let mut acc0 = 0u128;
        let mut acc1 = 0u128;
        let mut acc2 = 0u128;

        let s = (x as f64).sqrt() as u32;
        let mut l = 2u32;
        let limit_l = x / (s + 1);

        while l <= limit_l {
            let q = x / l;
            let r = x / q;
            let rm = if r >= MOD as u32 { (r as u64) % MOD } else { r as u64 };
            let pr1 = p1_small(rm);
            let c = 2 * rm + 1;
            let c = if c >= MOD { c - MOD } else { c };
            let pr2 = mul(pr1, mul(c, INV3));

            let cnt = (r - l + 1) as u64;
            let sum_0 = if cnt < MOD { cnt } else { cnt % MOD };
            let sum_1 = sub_mod(pr1, prev_p1);
            let sum_2 = sub_mod(pr2, prev_p2);

            let (sub0, sub1, sub2) = self.lookup_large(q as u64, parent, large);
            acc0 += sum_0 as u128 * sub0 as u128;
            acc1 += sum_1 as u128 * sub1 as u128;
            acc2 += sum_2 as u128 * sub2 as u128;

            prev_p1 = pr1;
            prev_p2 = pr2;
            l = r + 1;
        }

        let q_start = x / l;
        for q in (1..=q_start).rev() {
            let r = x / q;
            let rm = if r >= MOD as u32 { (r as u64) % MOD } else { r as u64 };
            let pr1 = p1_small(rm);
            let c = 2 * rm + 1;
            let c = if c >= MOD { c - MOD } else { c };
            let pr2 = mul(pr1, mul(c, INV3));

            let cnt = (r - l + 1) as u64;
            let sum_0 = if cnt < MOD { cnt } else { cnt % MOD };
            let sum_1 = sub_mod(pr1, prev_p1);
            let sum_2 = sub_mod(pr2, prev_p2);

            let (sub0, sub1, sub2) = self.prefix_at(q as u64);
            acc0 += sum_0 as u128 * sub0 as u128;
            acc1 += sum_1 as u128 * sub1 as u128;
            acc2 += sum_2 as u128 * sub2 as u128;

            prev_p1 = pr1;
            prev_p2 = pr2;
            l = r + 1;
        }

        const MODU: u128 = MOD as u128;
        f0 = sub_mod(f0, (acc0 % MODU) as u64);
        f1 = sub_mod(f1, (acc1 % MODU) as u64);
        f2 = sub_mod(f2, (acc2 % MODU) as u64);
        (f0, f1, f2)
    }

    fn compute_one_u64(
        &self,
        x: u64,
        parent: u64,
        large: &[(u64, u64, u64)],
    ) -> (u64, u64, u64) {
        let mut f0 = p1(x);
        let mut f1 = p2(x);
        let mut f2 = p3(x);
        let mut prev_p1 = 1u64;
        let mut prev_p2 = 1u64;
        let mut acc0 = 0u128;
        let mut acc1 = 0u128;
        let mut acc2 = 0u128;

        let s = (x as f64).sqrt() as u64;
        let mut l = 2u64;
        let limit_l = x / (s + 1);

        while l <= limit_l {
            let q = x / l;
            let r = x / q;
            let rm = r % MOD;
            let pr1 = p1_small(rm);
            let c = 2 * rm + 1;
            let c = if c >= MOD { c - MOD } else { c };
            let pr2 = mul(pr1, mul(c, INV3));

            let cnt = r - l + 1;
            let sum_0 = if cnt < MOD { cnt } else { cnt % MOD };
            let sum_1 = sub_mod(pr1, prev_p1);
            let sum_2 = sub_mod(pr2, prev_p2);

            let (sub0, sub1, sub2) = self.lookup_large(q, parent, large);
            acc0 += sum_0 as u128 * sub0 as u128;
            acc1 += sum_1 as u128 * sub1 as u128;
            acc2 += sum_2 as u128 * sub2 as u128;

            prev_p1 = pr1;
            prev_p2 = pr2;
            l = r + 1;
        }

        let q_start = (x / l) as u32;
        for q in (1..=q_start).rev() {
            let r = x / q as u64;
            let rm = r % MOD;
            let pr1 = p1_small(rm);
            let c = 2 * rm + 1;
            let c = if c >= MOD { c - MOD } else { c };
            let pr2 = mul(pr1, mul(c, INV3));

            let cnt = r - l + 1;
            let sum_0 = if cnt < MOD { cnt } else { cnt % MOD };
            let sum_1 = sub_mod(pr1, prev_p1);
            let sum_2 = sub_mod(pr2, prev_p2);

            let (sub0, sub1, sub2) = self.prefix_at(q as u64);
            acc0 += sum_0 as u128 * sub0 as u128;
            acc1 += sum_1 as u128 * sub1 as u128;
            acc2 += sum_2 as u128 * sub2 as u128;

            prev_p1 = pr1;
            prev_p2 = pr2;
            l = r + 1;
        }

        const MODU: u128 = MOD as u128;
        f0 = sub_mod(f0, (acc0 % MODU) as u64);
        f1 = sub_mod(f1, (acc1 % MODU) as u64);
        f2 = sub_mod(f2, (acc2 % MODU) as u64);
        (f0, f1, f2)
    }

    #[inline(always)]
    fn compute_one(
        &self,
        x: u64,
        parent: u64,
        large: &[(u64, u64, u64)],
    ) -> (u64, u64, u64) {
        if x <= u32::MAX as u64 {
            self.compute_one_u32(x as u32, parent, large)
        } else {
            self.compute_one_u64(x, parent, large)
        }
    }

    fn fill_large(&self, parent: u64) -> Vec<(u64, u64, u64)> {
        let max_i = (parent / (self.limit + 1)) as usize;
        let mut large = vec![(0u64, 0u64, 0u64); max_i + 1];
        let ptr = large.as_mut_ptr() as usize;

        let mut high = max_i;
        while high >= 1 {
            let low = (high / 2) + 1;
            (low..=high).into_par_iter().for_each(|i| {
                let x = parent / i as u64;
                let large_slice = unsafe { std::slice::from_raw_parts(ptr as *const (u64, u64, u64), max_i + 1) };
                let val = self.compute_one(x, parent, large_slice);
                unsafe {
                    *(ptr as *mut (u64, u64, u64)).add(i) = val;
                }
            });
            high = low - 1;
        }
        large
    }

    fn precompute(&mut self, m_parent: u64, n_parent: u64) {
        self.m_parent = m_parent;
        self.n_parent = n_parent;
        // Two independent Lucy fills; not nested floor-block rayon.
        let large_m = self.fill_large(m_parent);
        let large_n = self.fill_large(n_parent);
        self.large_m = large_m;
        self.large_n = large_n;
    }

    #[inline(always)]
    fn values(&self, x: u64) -> (u64, u64, u64) {
        if x <= self.limit {
            self.prefix_at(x)
        } else if x <= self.m_parent {
            let i = self.m_parent / x;
            if self.m_parent / i == x {
                // SAFETY: x > limit and x is a floor of m_parent ⇒ i <= max_i_m.
                unsafe { *self.large_m.get_unchecked(i as usize) }
            } else {
                // SAFETY: remaining queries are floors of n_parent above the sieve.
                unsafe { *self.large_n.get_unchecked((self.n_parent / x) as usize) }
            }
        } else {
            unsafe { *self.large_n.get_unchecked((self.n_parent / x) as usize) }
        }
    }
}

fn nonconcurrent_candidate_count(m: u64, n: u64) -> u64 {
    let two_same_bottom = mul(
        mul(
            mul(
                mul(mul(m % MOD, (m + MOD - 1) % MOD), n % MOD),
                (n + MOD - 1) % MOD,
            ),
            (n + 1) % MOD,
        ),
        INV6,
    );
    let distinct_bottoms = mul(c3_mod(m), sub_mod(c3_mod(n + 2), n % MOD));
    add_mod(two_same_bottom, distinct_bottoms)
}

fn weighted_gcd_sum(m: u64, n: u64, tp: &TotientPrefix) -> u64 {
    let m1 = m - 1;
    let n1 = n - 1;
    let upper = m1.min(n1);
    let mut total = 0u64;

    let mut l = 1u64;
    let mut prev_val = (0u64, 0u64, 0u64);
    while l <= upper {
        let qm = m1 / l;
        let qn = n1 / l;
        let r = (m1 / qm).min(n1 / qn).min(upper);

        let (r0, r1, r2) = tp.values(r);
        let (l0, l1, l2) = prev_val;

        let s0 = sub_mod(r0, l0);
        let s1 = sub_mod(r1, l1);
        let s2 = sub_mod(r2, l2);

        let qm_mod = qm % MOD;
        let qn_mod = qn % MOD;

        let a0m = mul(qm_mod, m % MOD);
        let a1m = mul(mul((MOD - qm_mod) % MOD, (qm + 1) % MOD), INV2);
        let a0n = mul(qn_mod, n % MOD);
        let a1n = mul(mul((MOD - qn_mod) % MOD, (qn + 1) % MOD), INV2);

        let c0 = mul(a0m, a0n);
        let c1 = add_mod(mul(a0m, a1n), mul(a1m, a0n));
        let c2 = mul(a1m, a1n);

        total = add_mod(
            total,
            add_mod(add_mod(mul(c0, s0), mul(c1, s1)), mul(c2, s2)),
        );
        prev_val = (r0, r1, r2);
        l = r + 1;
    }
    total
}

fn concurrent_triple_count(m: u64, n: u64, tp: &TotientPrefix) -> u64 {
    let gcd_part = weighted_gcd_sum(m, n, tp);
    let endpoint = mul(c2_mod(m), c2_mod(n));
    sub_mod(gcd_part, endpoint)
}

fn t_fn(m: u64, n: u64, tp: &TotientPrefix) -> u64 {
    sub_mod(
        nonconcurrent_candidate_count(m, n),
        concurrent_triple_count(m, n, tp),
    )
}

fn main() {
    let mut tp = TotientPrefix::new(SIEVE_LIMIT);
    let m = 1234 * 10u64.pow(8);
    let n = 2345 * 10u64.pow(8);
    tp.precompute(m - 1, n - 1);
    println!("{}", t_fn(m, n, &tp));
}
