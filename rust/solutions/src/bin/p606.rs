// Project Euler 606 - Gozinta Chains
//
// Lucy DP for sum of p^3 for primes p <= n, then pair-product sums.
// Optimized:
// - Dense u32 arrays for small_s and large_s (8 MB total, cache resident).
// - Precomputed prime sieve up to sqrt(L).
// - Incremental sum-of-cubes for small_s and Rayon parallel initialization for large_s.
// - Branch-free linear-stride quotient updates (elimination of division for k <= sqrt(L)/p).
// - 32-bit division lowering for k > sqrt(L)/p.
// - Piecewise constant quotient updates for large k ranges.
// - Sieve updates restricted to p <= sqrt(sqrt(L)) for small_s.
// - Rayon parallel reduction for final pair-sum computation.

use rayon::prelude::*;

const M: u32 = 1_000_000_000;
const L: i64 = 1_000_000_000_000;
const SQRT_L: usize = 1_000_000;

#[inline(always)]
fn mul_mod(a: u32, b: u32) -> u32 {
    ((a as u64 * b as u64) % (M as u64)) as u32
}

#[inline(always)]
fn sum_cubes(n: i64) -> u32 {
    let m = M as i64;
    let n_red = (n % (2 * m)) as u64;
    let t = if n_red % 2 == 0 {
        let h = (n_red / 2) % (M as u64);
        let hp1 = (n_red + 1) % (M as u64);
        (h * hp1) % (M as u64)
    } else {
        let nm = n_red % (M as u64);
        let hp1 = ((n_red + 1) / 2) % (M as u64);
        (nm * hp1) % (M as u64)
    };
    ((t * t) % (M as u64)) as u32
}

fn main() {
    let mut is_prime = vec![true; SQRT_L + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::with_capacity(78500);
    for i in 2..=SQRT_L {
        if is_prime[i] {
            primes.push(i as u32);
            let mut j = i * i;
            while j <= SQRT_L {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let mut small_s = vec![0u32; SQRT_L + 2];
    let mut large_s = vec![0u32; SQRT_L + 2];

    // Incremental sum of cubes for small_s
    let mut running_sum = 0u64;
    for v in 1..=SQRT_L {
        running_sum += v as u64;
        if running_sum >= M as u64 {
            running_sum %= M as u64;
        }
        let c = (running_sum * running_sum) % (M as u64);
        small_s[v] = if c == 0 { M - 1 } else { (c - 1) as u32 };
    }

    // Parallel large_s initialization
    large_s[1..=SQRT_L].par_iter_mut().enumerate().for_each(|(idx, val)| {
        let k = (idx + 1) as i64;
        let v = L / k;
        let sc = sum_cubes(v);
        *val = if sc == 0 { M - 1 } else { sc - 1 };
    });

    let b_ptr = large_s.as_mut_ptr();
    let s_ptr = small_s.as_mut_ptr();
    let r = SQRT_L;

    // Lucy DP
    for &p in &primes {
        let p_i64 = p as i64;
        let p2 = p_i64 * p_i64;
        if p2 > L {
            break;
        }
        let p_usize = p as usize;
        let p3 = mul_mod(p, mul_mod(p, p));
        let sp = unsafe { *s_ptr.add(p_usize - 1) };

        let max_i = ((L / p2) as usize).min(r);
        let lim1 = max_i.min(r / p_usize);

        let mut ip = p_usize;
        unsafe {
            for i in 1..=lim1 {
                let sv = *b_ptr.add(ip);
                let diff = if sv >= sp { sv - sp } else { sv + M - sp };
                let sub = mul_mod(p3, diff);
                let cur = *b_ptr.add(i);
                *b_ptr.add(i) = if cur >= sub { cur - sub } else { cur + M - sub };
                ip += p_usize;
            }
        }

        let m = L / p_i64;
        if p2 <= r as i64 {
            let isqrt_m = ((m as f64).sqrt() as usize).min(max_i);
            let mut i = lim1 + 1;
            unsafe {
                while i + 1 <= isqrt_m {
                    let q0 = (m / (i as i64)) as usize;
                    let q1 = (m / ((i + 1) as i64)) as usize;
                    let sv0 = *s_ptr.add(q0);
                    let sv1 = *s_ptr.add(q1);
                    let d0 = if sv0 >= sp { sv0 - sp } else { sv0 + M - sp };
                    let d1 = if sv1 >= sp { sv1 - sp } else { sv1 + M - sp };
                    let sub0 = mul_mod(p3, d0);
                    let sub1 = mul_mod(p3, d1);
                    let c0 = *b_ptr.add(i);
                    let c1 = *b_ptr.add(i + 1);
                    *b_ptr.add(i) = if c0 >= sub0 { c0 - sub0 } else { c0 + M - sub0 };
                    *b_ptr.add(i + 1) = if c1 >= sub1 { c1 - sub1 } else { c1 + M - sub1 };
                    i += 2;
                }
                if i <= isqrt_m {
                    let q0 = (m / (i as i64)) as usize;
                    let sv0 = *s_ptr.add(q0);
                    let d0 = if sv0 >= sp { sv0 - sp } else { sv0 + M - sp };
                    let sub0 = mul_mod(p3, d0);
                    let c0 = *b_ptr.add(i);
                    *b_ptr.add(i) = if c0 >= sub0 { c0 - sub0 } else { c0 + M - sub0 };
                    i += 1;
                }
                while i <= max_i {
                    let q = (m / (i as i64)) as usize;
                    let mut i_last = (m / (q as i64)) as usize;
                    if i_last > max_i {
                        i_last = max_i;
                    }
                    let sv = *s_ptr.add(q);
                    let diff = if sv >= sp { sv - sp } else { sv + M - sp };
                    let sub = mul_mod(p3, diff);
                    let count = i_last - i + 1;
                    let ptr = b_ptr.add(i);
                    for offset in 0..count {
                        let cur = *ptr.add(offset);
                        *ptr.add(offset) = if cur >= sub { cur - sub } else { cur + M - sub };
                    }
                    i = i_last + 1;
                }
            }
        } else {
            let m_u32 = m as u32;
            let mut i = lim1 + 1;
            unsafe {
                while i + 1 <= max_i {
                    let q0 = (m_u32 / (i as u32)) as usize;
                    let q1 = (m_u32 / ((i + 1) as u32)) as usize;
                    let sv0 = *s_ptr.add(q0);
                    let sv1 = *s_ptr.add(q1);
                    let d0 = if sv0 >= sp { sv0 - sp } else { sv0 + M - sp };
                    let d1 = if sv1 >= sp { sv1 - sp } else { sv1 + M - sp };
                    let sub0 = mul_mod(p3, d0);
                    let sub1 = mul_mod(p3, d1);
                    let c0 = *b_ptr.add(i);
                    let c1 = *b_ptr.add(i + 1);
                    *b_ptr.add(i) = if c0 >= sub0 { c0 - sub0 } else { c0 + M - sub0 };
                    *b_ptr.add(i + 1) = if c1 >= sub1 { c1 - sub1 } else { c1 + M - sub1 };
                    i += 2;
                }
                if i <= max_i {
                    let q0 = (m_u32 / (i as u32)) as usize;
                    let sv0 = *s_ptr.add(q0);
                    let d0 = if sv0 >= sp { sv0 - sp } else { sv0 + M - sp };
                    let sub0 = mul_mod(p3, d0);
                    let c0 = *b_ptr.add(i);
                    *b_ptr.add(i) = if c0 >= sub0 { c0 - sub0 } else { c0 + M - sub0 };
                }
            }
        }

        // Update small_s only when p^2 <= r
        if p2 <= r as i64 {
            let max_k = r / p_usize;
            unsafe {
                let sv = *s_ptr.add(max_k);
                let diff = if sv >= sp { sv - sp } else { sv + M - sp };
                let sub = mul_mod(p3, diff);
                for v in (max_k * p_usize)..=r {
                    let cur = *s_ptr.add(v);
                    *s_ptr.add(v) = if cur >= sub { cur - sub } else { cur + M - sub };
                }
                match p {
                    2 => {
                        for k in (2..max_k).rev() {
                            let sv = *s_ptr.add(k);
                            let diff = if sv >= sp { sv - sp } else { sv + M - sp };
                            let sub = mul_mod(p3, diff);
                            let base = s_ptr.add(k * 2);
                            let c0 = *base;
                            let c1 = *base.add(1);
                            *base = if c0 >= sub { c0 - sub } else { c0 + M - sub };
                            *base.add(1) = if c1 >= sub { c1 - sub } else { c1 + M - sub };
                        }
                    }
                    3 => {
                        for k in (3..max_k).rev() {
                            let sv = *s_ptr.add(k);
                            let diff = if sv >= sp { sv - sp } else { sv + M - sp };
                            let sub = mul_mod(p3, diff);
                            let base = s_ptr.add(k * 3);
                            let c0 = *base;
                            let c1 = *base.add(1);
                            let c2 = *base.add(2);
                            *base = if c0 >= sub { c0 - sub } else { c0 + M - sub };
                            *base.add(1) = if c1 >= sub { c1 - sub } else { c1 + M - sub };
                            *base.add(2) = if c2 >= sub { c2 - sub } else { c2 + M - sub };
                        }
                    }
                    _ => {
                        for k in (p_usize..max_k).rev() {
                            let sv = *s_ptr.add(k);
                            let diff = if sv >= sp { sv - sp } else { sv + M - sp };
                            let sub = mul_mod(p3, diff);
                            let base = s_ptr.add(k * p_usize);
                            for offset in 0..p_usize {
                                let cur = *base.add(offset);
                                *base.add(offset) = if cur >= sub { cur - sub } else { cur + M - sub };
                            }
                        }
                    }
                }
            }
        }
    }

    let get2 = |v: i64| -> u32 {
        if v <= SQRT_L as i64 {
            small_s[v as usize]
        } else {
            large_s[(L / v) as usize]
        }
    };

    let ans: u64 = primes
        .par_iter()
        .map(|&p| {
            let p_i64 = p as i64;
            let q_max = L / p_i64;
            if q_max > p_i64 {
                let p3 = mul_mod(p, mul_mod(p, p));
                let sum_qmax = get2(q_max);
                let sum_p = get2(p_i64);
                let sum_q = if sum_qmax >= sum_p {
                    sum_qmax - sum_p
                } else {
                    sum_qmax + M - sum_p
                };
                mul_mod(p3, sum_q) as u64
            } else {
                0u64
            }
        })
        .sum::<u64>()
        % (M as u64);

    println!("{}", ans);
}
