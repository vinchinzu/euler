// Project Euler 931 - Totient Graph
//
// For positive integer n, build graph on divisors: edge a->b when b|a and a/b is prime,
// weight = phi(a) - phi(b). t(n) = total weight. T(N) = sum_{n=1}^N t(n).
// Find T(10^12) mod 715827883.
//
// Key derivation:
// T(N) = sum_{m=1}^N g(m)*floor(N/m) where g(m) = sum_{p|m,p prime} (phi(m)-phi(m/p)).
// Using Dirichlet convolution: T(N) = sum_{n=1}^N h(n) where h = g*1.
//
// After simplification: T(N) = S_main + D - C_ge2 where:
// S_main = sum_{p prime, p<=N} (p-2) * f(floor(N/p))   with f(x) = x*(x+1)/2
// D = sum_{p prime, p<=sqrt(N)} p * f(floor(N/p^2))
// C_ge2 = sum_{a>=2} sum_{p: p^a<=N} [f(floor(N/p^a)) - p*f(floor(N/p^{a+1}))]
//
// Optimized Lucy DP:
// - Two dense u32 arrays (small and large) fitting in L3 cache (16MB total vs 64MB)
// - Eliminates quotient vector, idx_small, idx_large, and hash maps
// - Loop-invariant prime term hoisting
// - Branch-free loop split with stride addition in large-large updates
// - 32-bit division and piecewise constant block updates for small quotient range

const MOD: u64 = 715827883;
const LIMIT: u64 = 1_000_000_000_000;
const K: usize = 1_000_000;

#[inline(always)]
unsafe fn sub_mod_slice(ptr: *mut u32, len: usize, sub: u32, m: u32) {
    if sub == 0 {
        return;
    }
    let diff = m - sub;
    unsafe {
        for i in 0..len {
            let cur = *ptr.add(i);
            *ptr.add(i) = if cur >= sub { cur - sub } else { cur + diff };
        }
    }
}

fn main() {
    let inv2 = (MOD + 1) / 2;

    // Sieve primes up to K
    let mut is_prime = vec![true; K + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= K {
        if is_prime[i] {
            let mut j = i * i;
            while j <= K {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<u32> = (2..=K).filter(|&x| is_prime[x]).map(|x| x as u32).collect();

    // Allocate s0 and s1 for small and large (4 x 4MB = 16MB)
    let mut s0_small = vec![0u32; K + 1];
    let mut s1_small = vec![0u32; K + 1];
    let mut s0_large = vec![0u32; K + 1];
    let mut s1_large = vec![0u32; K + 1];

    for v in 1..=K {
        let qm = v as u64 % MOD;
        s1_small[v] = ((qm * (qm + 1) % MOD * inv2 % MOD + MOD - 1) % MOD) as u32;
        s0_small[v] = ((qm + MOD - 1) % MOD) as u32;
    }

    for k in 1..=K {
        let q = LIMIT / k as u64;
        let qm = q % MOD;
        s1_large[k] = ((qm * ((qm + 1) % MOD) % MOD * inv2 % MOD + MOD - 1) % MOD) as u32;
        s0_large[k] = ((qm + MOD - 1) % MOD) as u32;
    }

    let mod_u32 = MOD as u32;

    // Lucy DP
    for &p_u32 in &primes {
        let p = p_u32 as u64;
        let p2 = p * p;
        if p2 > LIMIT {
            break;
        }
        let pm = p % MOD;
        let p_u = p_u32 as usize;

        let s1_pm1 = s1_small[p_u - 1] as u64;
        let s0_pm1 = s0_small[p_u - 1] as u64;

        let max_k = K.min((LIMIT / p2) as usize);
        let lim_div_p = LIMIT / p;
        let split_k = (K / p_u).min(max_k);

        // Branch-free loop 1: kp <= K (access s_large)
        let mut kp = p_u;
        for k in 1..=split_k {
            // SAFETY: kp <= K
            let idx_s1 = unsafe { *s1_large.get_unchecked(kp) } as u64;
            let idx_s0 = unsafe { *s0_large.get_unchecked(kp) } as u64;

            let sub1 = if idx_s1 >= s1_pm1 { idx_s1 - s1_pm1 } else { idx_s1 + MOD - s1_pm1 };
            let prod = pm * sub1 % MOD;
            let cur_s1 = unsafe { *s1_large.get_unchecked(k) } as u64;
            let new_s1 = if cur_s1 >= prod { cur_s1 - prod } else { cur_s1 + MOD - prod };
            unsafe { *s1_large.get_unchecked_mut(k) = new_s1 as u32; }

            let sub0 = if idx_s0 >= s0_pm1 { idx_s0 - s0_pm1 } else { idx_s0 + MOD - s0_pm1 };
            let cur_s0 = unsafe { *s0_large.get_unchecked(k) } as u64;
            let new_s0 = if cur_s0 >= sub0 { cur_s0 - sub0 } else { cur_s0 + MOD - sub0 };
            unsafe { *s0_large.get_unchecked_mut(k) = new_s0 as u32; }
            kp += p_u;
        }

        // Branch-free loop 2: kp > K (access s_small)
        let s1_l_ptr = s1_large.as_mut_ptr();
        let s0_l_ptr = s0_large.as_mut_ptr();
        let s1_s_ptr = s1_small.as_ptr();
        let s0_s_ptr = s0_small.as_ptr();

        if lim_div_p <= u32::MAX as u64 {
            let m_u32 = lim_div_p as u32;
            let isqrt_m = (m_u32 as f64).sqrt() as usize;
            let mid_k = isqrt_m.min(max_k);

            let mut k = split_k + 1;
            while k <= mid_k {
                let v = (m_u32 / k as u32) as usize;
                let idx_s1 = unsafe { *s1_s_ptr.add(v) } as u64;
                let idx_s0 = unsafe { *s0_s_ptr.add(v) } as u64;

                let sub1 = if idx_s1 >= s1_pm1 { idx_s1 - s1_pm1 } else { idx_s1 + MOD - s1_pm1 };
                let prod = pm * sub1 % MOD;
                let cur_s1 = unsafe { *s1_l_ptr.add(k) } as u64;
                let new_s1 = if cur_s1 >= prod { cur_s1 - prod } else { cur_s1 + MOD - prod };
                unsafe { *s1_l_ptr.add(k) = new_s1 as u32; }

                let sub0 = if idx_s0 >= s0_pm1 { idx_s0 - s0_pm1 } else { idx_s0 + MOD - s0_pm1 };
                let cur_s0 = unsafe { *s0_l_ptr.add(k) } as u64;
                let new_s0 = if cur_s0 >= sub0 { cur_s0 - sub0 } else { cur_s0 + MOD - sub0 };
                unsafe { *s0_l_ptr.add(k) = new_s0 as u32; }
                k += 1;
            }

            if k <= max_k {
                let v_start = (m_u32 / k as u32) as usize;
                let v_end = ((m_u32 / max_k as u32) as usize).max(1);
                for v in (v_end..=v_start).rev() {
                    let k_end = ((m_u32 / v as u32) as usize).min(max_k);
                    if k > k_end {
                        continue;
                    }
                    let idx_s1 = unsafe { *s1_s_ptr.add(v) } as u64;
                    let idx_s0 = unsafe { *s0_s_ptr.add(v) } as u64;

                    let sub1 = if idx_s1 >= s1_pm1 { idx_s1 - s1_pm1 } else { idx_s1 + MOD - s1_pm1 };
                    let prod = (pm * sub1 % MOD) as u32;
                    let sub0 = if idx_s0 >= s0_pm1 { (idx_s0 - s0_pm1) as u32 } else { (idx_s0 + MOD - s0_pm1) as u32 };

                    let len = k_end - k + 1;
                    unsafe {
                        sub_mod_slice(s1_l_ptr.add(k), len, prod, mod_u32);
                        sub_mod_slice(s0_l_ptr.add(k), len, sub0, mod_u32);
                    }
                    k = k_end + 1;
                }
            }
        } else {
            let isqrt_m = (lim_div_p as f64).sqrt() as usize;
            let mid_k = isqrt_m.min(max_k);

            let mut k = split_k + 1;
            while k <= mid_k {
                let v = (lim_div_p / k as u64) as usize;
                let idx_s1 = unsafe { *s1_s_ptr.add(v) } as u64;
                let idx_s0 = unsafe { *s0_s_ptr.add(v) } as u64;

                let sub1 = if idx_s1 >= s1_pm1 { idx_s1 - s1_pm1 } else { idx_s1 + MOD - s1_pm1 };
                let prod = pm * sub1 % MOD;
                let cur_s1 = unsafe { *s1_l_ptr.add(k) } as u64;
                let new_s1 = if cur_s1 >= prod { cur_s1 - prod } else { cur_s1 + MOD - prod };
                unsafe { *s1_l_ptr.add(k) = new_s1 as u32; }

                let sub0 = if idx_s0 >= s0_pm1 { idx_s0 - s0_pm1 } else { idx_s0 + MOD - s0_pm1 };
                let cur_s0 = unsafe { *s0_l_ptr.add(k) } as u64;
                let new_s0 = if cur_s0 >= sub0 { cur_s0 - sub0 } else { cur_s0 + MOD - sub0 };
                unsafe { *s0_l_ptr.add(k) = new_s0 as u32; }
                k += 1;
            }

            if k <= max_k {
                let v_start = (lim_div_p / k as u64) as usize;
                let v_end = ((lim_div_p / max_k as u64) as usize).max(1);
                for v in (v_end..=v_start).rev() {
                    let k_end = ((lim_div_p / v as u64) as usize).min(max_k);
                    if k > k_end {
                        continue;
                    }
                    let idx_s1 = unsafe { *s1_s_ptr.add(v) } as u64;
                    let idx_s0 = unsafe { *s0_s_ptr.add(v) } as u64;

                    let sub1 = if idx_s1 >= s1_pm1 { idx_s1 - s1_pm1 } else { idx_s1 + MOD - s1_pm1 };
                    let prod = (pm * sub1 % MOD) as u32;
                    let sub0 = if idx_s0 >= s0_pm1 { (idx_s0 - s0_pm1) as u32 } else { (idx_s0 + MOD - s0_pm1) as u32 };

                    let len = k_end - k + 1;
                    unsafe {
                        sub_mod_slice(s1_l_ptr.add(k), len, prod, mod_u32);
                        sub_mod_slice(s0_l_ptr.add(k), len, sub0, mod_u32);
                    }
                    k = k_end + 1;
                }
            }
        }

        if p2 <= K as u64 {
            let max_qp = K / p_u;
            let min_qp = p_u;
            let s1_ptr = s1_small.as_mut_ptr();
            let s0_ptr = s0_small.as_mut_ptr();
            for qp in (min_qp..=max_qp).rev() {
                let idx_s1 = unsafe { *s1_ptr.add(qp) } as u64;
                let idx_s0 = unsafe { *s0_ptr.add(qp) } as u64;

                let sub1 = if idx_s1 >= s1_pm1 { idx_s1 - s1_pm1 } else { idx_s1 + MOD - s1_pm1 };
                let prod = (pm * sub1 % MOD) as u32;
                let sub0 = if idx_s0 >= s0_pm1 { (idx_s0 - s0_pm1) as u32 } else { (idx_s0 + MOD - s0_pm1) as u32 };

                let start = qp * p_u;
                let end = (start + p_u - 1).min(K);
                let len = end - start + 1;
                unsafe {
                    sub_mod_slice(s1_ptr.add(start), len, prod, mod_u32);
                    sub_mod_slice(s0_ptr.add(start), len, sub0, mod_u32);
                }
            }
        }
    }

    // f(x) = x*(x+1)/2 mod MOD
    let f = |x: u64| -> u64 {
        let xm = x % MOD;
        xm * ((xm + 1) % MOD) % MOD * inv2 % MOD
    };

    let mut s_main: u64 = 0;
    let mut k = 1u64;
    while k <= LIMIT {
        let v = LIMIT / k;
        let k_end = LIMIT / v;

        let hi = k_end;
        let lo = if v + 1 <= LIMIT { LIMIT / (v + 1) } else { 0 };

        let (sp_hi, cp_hi) = if hi <= K as u64 {
            (s1_small[hi as usize] as u64, s0_small[hi as usize] as u64)
        } else {
            let idx = (LIMIT / hi) as usize;
            (s1_large[idx] as u64, s0_large[idx] as u64)
        };

        let (sp_lo, cp_lo) = if lo == 0 {
            (0, 0)
        } else if lo <= K as u64 {
            (s1_small[lo as usize] as u64, s0_small[lo as usize] as u64)
        } else {
            let idx = (LIMIT / lo) as usize;
            (s1_large[idx] as u64, s0_large[idx] as u64)
        };

        let sp_range = (sp_hi + MOD - sp_lo) % MOD;
        let cp_range = (cp_hi + MOD - cp_lo) % MOD;
        let sum_p_minus_2 = (sp_range + 2 * MOD - 2 * cp_range % MOD) % MOD;

        s_main = (s_main + f(v) * sum_p_minus_2 % MOD) % MOD;

        k = k_end + 1;
    }

    let mut d_sum: u64 = 0;
    for &p_u32 in &primes {
        let p = p_u32 as u64;
        if p * p > LIMIT { break; }
        d_sum = (d_sum + (p % MOD) * f(LIMIT / (p * p)) % MOD) % MOD;
    }

    let mut c_ge2: u64 = 0;
    for &p_u32 in &primes {
        let p = p_u32 as u64;
        let mut pa = p * p;
        if pa > LIMIT { break; }
        loop {
            let q1 = LIMIT / pa;
            let q2 = q1 / p;
            let term = (f(q1) + MOD - (p % MOD) * f(q2) % MOD) % MOD;
            c_ge2 = (c_ge2 + term) % MOD;
            if pa > LIMIT / p { break; }
            pa *= p;
            if pa > LIMIT { break; }
        }
    }

    println!("{}", (s_main + d_sum + MOD - c_ge2) % MOD);
}
