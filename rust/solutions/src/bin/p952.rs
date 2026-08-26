// Project Euler Problem 952
// Order Modulo Factorial.
// R(p, n) = multiplicative order of p mod n!.
// Find R(10^9+7, 10^7) mod (10^9+7).

use rayon::prelude::*;

const N: usize = 10_000_000;
const P: u64 = 1_000_000_007;
const MOD_ANS: u32 = 1_000_000_007;

#[inline(always)]
fn mod_pow_u32(mut base: u32, mut exp: u32, m: u32) -> u32 {
    let mut result = 1u32;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u64 * base as u64) % (m as u64)) as u32;
        }
        base = ((base as u64 * base as u64) % (m as u64)) as u32;
        exp >>= 1;
    }
    result
}

#[inline(always)]
fn mod_pow_u64(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m <= u32::MAX as u64 {
        return mod_pow_u32((base % m) as u32, exp as u32, m as u32) as u64;
    }
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % (m as u128)) as u64;
        }
        base = ((base as u128 * base as u128) % (m as u128)) as u64;
        exp >>= 1;
    }
    result
}

#[inline]
fn legendre(n: u32, p: u32) -> i32 {
    let n64 = n as u64;
    let p64 = p as u64;
    if p64 * p64 > n64 {
        return (n / p) as i32;
    }
    let mut count = 0u32;
    let mut pp = p64;
    while pp <= n64 {
        count += (n64 / pp) as u32;
        match pp.checked_mul(p64) {
            Some(next) => pp = next,
            None => break,
        }
    }
    count as i32
}

/// Multiplicative order of `p_mod` (already reduced) modulo odd prime `q`.
/// Factors q-1 via SPF and peels prime powers.
#[inline]
fn order_mod_q(p_mod: u32, q: u32, spf: &[u32]) -> u32 {
    let mut curr_ord = q - 1;
    let mut temp = q - 1;
    while temp > 1 {
        // SAFETY: 1 < temp <= q-1 <= N, spf has length N+1.
        let f = unsafe { *spf.get_unchecked(temp as usize) };
        while curr_ord % f == 0 {
            let next = curr_ord / f;
            if mod_pow_u32(p_mod, next, q) == 1 {
                curr_ord = next;
            } else {
                break;
            }
        }
        while temp % f == 0 {
            temp /= f;
        }
    }
    curr_ord
}

/// v_q(p^{d} - 1), starting from v_q >= 1 (d = ord_q(p)).
/// Lifts by multiplying the running modulus by q (incremental, not a full
/// restart of the exponent from a fresh modulus each conceptually-new power:
/// we only re-evaluate when the modulus actually grows).
#[inline]
fn lift_vq(p: u64, d: u32, q: u32, k_q: i32) -> i32 {
    let q64 = q as u64;
    let mut v = 1i32;
    let mut curr_mod = q64;
    loop {
        if v >= k_q {
            return v;
        }
        if curr_mod > u64::MAX / q64 {
            return v;
        }
        let next_mod = curr_mod * q64;
        let rem = if next_mod <= u32::MAX as u64 {
            mod_pow_u32((p % next_mod) as u32, d, next_mod as u32) as u64
        } else {
            mod_pow_u64(p, d as u64, next_mod)
        };
        if rem != 1 {
            return v;
        }
        v += 1;
        curr_mod = next_mod;
    }
}

fn process_q(q: u32, n: u32, spf: &[u32], acc: &mut Vec<(u32, i32)>) {
    let p_mod = (P % (q as u64)) as u32;
    let d_q = order_mod_q(p_mod, q, spf);

    let mut temp = d_q;
    while temp > 1 {
        // SAFETY: 1 < temp <= q-1 <= N.
        let ell = unsafe { *spf.get_unchecked(temp as usize) };
        let mut cnt = 0i32;
        while temp % ell == 0 {
            cnt += 1;
            temp /= ell;
        }
        acc.push((ell, cnt));
    }

    // k_q <= 1 ⇒ a_q = 0, skip the q^2 lift (all q > n/2).
    let k_q = if (q as u64) * 2 > n as u64 {
        1
    } else {
        legendre(n, q)
    };
    if k_q <= 1 {
        return;
    }

    let v_q = lift_vq(P, d_q, q, k_q);
    let a_q = k_q - v_q;
    if a_q > 0 {
        acc.push((q, a_q));
    }
}

fn main() {
    let n = N;
    let n32 = n as u32;

    let mut spf = vec![0u32; n + 1];
    let mut primes = Vec::with_capacity(n / 10);
    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
            if (i as u64) * (i as u64) <= n as u64 {
                let mut j = i * i;
                while j <= n {
                    if spf[j] == 0 {
                        spf[j] = i as u32;
                    }
                    j += i;
                }
            }
        }
    }

    let mut exponents = vec![0i32; n + 1];

    let k2 = legendre(n32, 2);
    if k2 >= 4 {
        exponents[2] = k2 - 3;
    } else if k2 >= 2 {
        exponents[2] = 1;
    }

    // Each odd prime q is independent: emit (ell, cnt) updates, merge with max.
    let updates = primes[1..]
        .par_iter()
        .with_min_len(64)
        .fold(
            || Vec::with_capacity(1 << 17),
            |mut acc, &q| {
                process_q(q, n32, &spf, &mut acc);
                acc
            },
        )
        .reduce(Vec::new, |mut a, mut b| {
            a.append(&mut b);
            a
        });

    for (ell, cnt) in updates {
        let slot = unsafe { exponents.get_unchecked_mut(ell as usize) };
        if *slot < cnt {
            *slot = cnt;
        }
    }

    let mut ans = 1u32;
    for ell in 2..=n {
        let e = unsafe { *exponents.get_unchecked(ell) };
        if e > 0 {
            let term = mod_pow_u32(ell as u32, e as u32, MOD_ANS);
            ans = ((ans as u64 * term as u64) % (MOD_ANS as u64)) as u32;
        }
    }

    println!("{}", ans);
}
