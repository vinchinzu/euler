// Project Euler 621 - Sum of three triangular numbers
// Sieve-based factorization for G(n) using divisor counts mod 4.
// Segment remaining[] / result[] so each thread walks the same primes
// over its own k-range (classic segmented sieve). Tonelli-Shanks is
// independent per prime and runs in parallel first.

use rayon::prelude::*;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

#[inline]
fn pow_mod_u64(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

struct PInfo {
    p: u32,
    r0: u32,
    r1: u32,
    nroots: u8,
    p_mod4: u8,
}

fn tonelli_roots(pp: u64, m_mod_p: u64) -> Option<PInfo> {
    let euler = pow_mod_u64(m_mod_p, (pp - 1) / 2, pp);
    if euler != 1 && m_mod_p != 0 {
        return None;
    }

    let sq = if m_mod_p == 0 {
        0u64
    } else if pp % 4 == 3 {
        pow_mod_u64(m_mod_p, (pp + 1) / 4, pp)
    } else {
        let mut q_ts = pp - 1;
        let mut s_ts = 0u32;
        while q_ts % 2 == 0 {
            q_ts /= 2;
            s_ts += 1;
        }
        let mut z = 2u64;
        while pow_mod_u64(z, (pp - 1) / 2, pp) == 1 {
            z += 1;
        }
        let mut mm = s_ts;
        let mut c = pow_mod_u64(z, q_ts, pp);
        let mut t = pow_mod_u64(m_mod_p, q_ts, pp);
        let mut r_val = pow_mod_u64(m_mod_p, (q_ts + 1) / 2, pp);
        while t != 1 {
            let mut tt = t;
            let mut ii = 0u32;
            while tt != 1 {
                tt = tt * tt % pp;
                ii += 1;
            }
            let mut b2 = c;
            for _ in 0..mm - ii - 1 {
                b2 = b2 * b2 % pp;
            }
            mm = ii;
            c = b2 * b2 % pp;
            t = t * c % pp;
            r_val = r_val * b2 % pp;
        }
        r_val
    };

    let inv2 = (pp + 1) / 2;
    let (r0, r1, nroots) = if m_mod_p == 0 {
        let r0 = ((pp - 1) * inv2 % pp) as u32;
        (r0, r0, 1u8)
    } else {
        let r0 = ((sq + pp - 1) % pp * inv2 % pp) as u32;
        let r1 = ((pp - sq + pp - 1) % pp * inv2 % pp) as u32;
        if r0 == r1 {
            (r0, r0, 1u8)
        } else {
            (r0, r1, 2u8)
        }
    };

    Some(PInfo {
        p: pp as u32,
        r0,
        r1,
        nroots,
        p_mod4: (pp % 4) as u8,
    })
}

fn main() {
    let n_val: i64 = 17_526_000_000_000;
    let q0 = 4 * n_val + 1;
    let mut l_val = ((((8.0 * n_val as f64 + 1.0).sqrt()) - 1.0) / 2.0) as i64;
    while l_val * (l_val + 1) / 2 > n_val {
        l_val -= 1;
    }
    while (l_val + 1) * (l_val + 2) / 2 <= n_val {
        l_val += 1;
    }

    let sz = (l_val + 1) as usize;
    let q0u = q0 as u64;

    let sieve_limit = isqrt(q0) as usize + 2;
    let mut is_prime = vec![true; sieve_limit + 1];
    is_prime[0] = false;
    if sieve_limit >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i <= sieve_limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= sieve_limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let m_val = (2 * q0 + 1) as u64;

    let primes: Vec<u32> = {
        let mut v = Vec::with_capacity(sieve_limit / 4);
        let mut p = 3;
        while p <= sieve_limit {
            if is_prime[p] {
                v.push(p as u32);
            }
            p += 2;
        }
        v
    };

    let infos: Vec<PInfo> = primes
        .into_par_iter()
        .filter_map(|p| {
            let pp = p as u64;
            let m_mod_p = m_val % pp;
            tonelli_roots(pp, m_mod_p)
        })
        .collect();

    let mut remaining = vec![0u64; sz];
    remaining.par_iter_mut().enumerate().for_each(|(k, slot)| {
        *slot = q0u - 2 * (k as u64) * (k as u64 + 1);
    });
    let mut result = vec![1i32; sz];

    // Segmented walk: each chunk owns a k-range of remaining/result.
    let n_threads = rayon::current_num_threads().max(1);
    let chunk_size = ((sz + n_threads - 1) / n_threads).max(1);

    remaining
        .par_chunks_mut(chunk_size)
        .zip(result.par_chunks_mut(chunk_size))
        .enumerate()
        .for_each(|(ci, (rem, res))| {
            let k_lo = ci * chunk_size;
            let k_hi = k_lo + rem.len();
            for pi in &infos {
                let p = pi.p as usize;
                let pp = pi.p as u64;
                let p_mod4 = pi.p_mod4;
                let roots = [pi.r0 as usize, pi.r1 as usize];
                for ri in 0..pi.nroots as usize {
                    let r = roots[ri];
                    let mut k = if k_lo <= r {
                        r
                    } else {
                        let remk = k_lo % p;
                        if remk <= r {
                            k_lo + (r - remk)
                        } else {
                            k_lo + (p - remk + r)
                        }
                    };
                    while k < k_hi {
                        let idx = k - k_lo;
                        // SAFETY: k ∈ [k_lo, k_hi) so idx < rem.len()
                        unsafe {
                            if *res.get_unchecked(idx) != 0 {
                                let slot = rem.get_unchecked_mut(idx);
                                let mut e = 0i32;
                                while *slot % pp == 0 {
                                    *slot /= pp;
                                    e += 1;
                                }
                                if e > 0 {
                                    let rs = res.get_unchecked_mut(idx);
                                    if p_mod4 == 1 {
                                        *rs *= e + 1;
                                    } else if e % 2 == 1 {
                                        *rs = 0;
                                    }
                                }
                            }
                        }
                        k += p;
                    }
                }
            }
        });

    let mut answer: i64 = 0;
    for k in 0..sz {
        let res = result[k];
        if res == 0 {
            continue;
        }
        let r = remaining[k];
        if r == 1 {
            answer += res as i64;
        } else if r % 4 == 1 {
            answer += res as i64 * 2;
        }
    }

    println!("{}", answer);
}
