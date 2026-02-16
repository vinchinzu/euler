// Project Euler 417: Reciprocal cycles II
//
// Compute sum of L(n) for n=3..10^8, where L(n) is the length of the
// repeating cycle in the decimal expansion of 1/n.

use rayon::prelude::*;

const NMAX: usize = 100_000_001;

#[inline(always)]
fn pow_mod_32(base: u32, mut exp: u32, modv: u32) -> u32 {
    let m = modv as u64;
    let mut result = 1u64;
    let mut b = base as u64 % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * b % m;
        }
        b = b * b % m;
        exp >>= 1;
    }
    result as u32
}

/// pow_mod for larger moduli (p^2 etc.) using u128 intermediates
#[inline(always)]
fn pow_mod_big(mut base: u64, mut exp: u64, modv: u64) -> u64 {
    let mut result = 1u64;
    base %= modv;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % modv as u128) as u64;
        }
        base = (base as u128 * base as u128 % modv as u128) as u64;
        exp >>= 1;
    }
    result
}

#[inline(always)]
fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline(always)]
fn lcm_i64(a: i64, b: i64) -> i64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    a / gcd_i64(a, b) * b
}

fn main() {
    let n: usize = 100_000_000;

    // Sieve smallest prime factors
    // spf[i] == 0 means i is prime (for i >= 2, excluding 0 and 1)
    let mut spf = vec![0u32; NMAX];
    {
        let mut i = 2usize;
        while i * i <= n {
            if spf[i] == 0 {
                let mut j = i * i;
                while j <= n {
                    if spf[j] == 0 {
                        spf[j] = i as u32;
                    }
                    j += i;
                }
            }
            i += 1;
        }
    }

    // Collect all primes != 2, 5
    let primes: Vec<usize> = (3..=n)
        .filter(|&p| spf[p] == 0 && p != 5)
        .collect();

    // Compute ord_10(p) for all primes in parallel
    let mut ord10 = vec![0u32; NMAX];
    let ord_results: Vec<(usize, u32)> = primes
        .par_iter()
        .map(|&p| {
            // Factor p-1 using spf
            let mut result = (p - 1) as u32;
            let mut temp = p - 1;
            while temp > 1 {
                // SAFETY: temp < p <= n, spf has size NMAX > n
                let q = unsafe {
                    let s = *spf.get_unchecked(temp);
                    if s == 0 { temp as u32 } else { s }
                };
                while temp % q as usize == 0 {
                    temp /= q as usize;
                }
                while result % q == 0 {
                    let pow = pow_mod_32(10, result / q, p as u32);
                    if pow == 1 {
                        result /= q;
                    } else {
                        break;
                    }
                }
            }
            (p, result)
        })
        .collect();

    for (p, ord) in ord_results {
        ord10[p] = ord;
    }

    // Compute sum of L(n) for n = 3..N in parallel chunks
    let chunk_size = 500_000usize;
    let n_chunks = (n - 2 + chunk_size - 1) / chunk_size; // chunks covering 3..=n

    let total: i64 = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let start = 3 + ci * chunk_size;
            let end = n.min(start + chunk_size - 1);
            let mut local_total: i64 = 0;

            for nn in start..=end {
                let mut temp = nn;
                while temp % 2 == 0 { temp /= 2; }
                while temp % 5 == 0 { temp /= 5; }
                if temp <= 1 { continue; }

                // SAFETY: temp <= nn <= n < NMAX
                let sp = unsafe { *spf.get_unchecked(temp) };
                if sp == 0 {
                    // temp is prime, use cached order
                    local_total += unsafe { *ord10.get_unchecked(temp) } as i64;
                    continue;
                }

                // Factor temp, compute LCM of L(p^e)
                let mut result: i64 = 0;
                let mut t = temp;
                while t > 1 {
                    let p = unsafe {
                        let s = *spf.get_unchecked(t);
                        if s == 0 { t as u32 } else { s }
                    };

                    let mut e = 0u32;
                    while t % p as usize == 0 {
                        t /= p as usize;
                        e += 1;
                    }

                    let lp = unsafe { *ord10.get_unchecked(p as usize) } as i64;
                    if lp == 0 { continue; }

                    let mut lpe = lp;
                    if e >= 2 {
                        let pp = p as u64 * p as u64;
                        if pow_mod_big(10, lp as u64, pp) == 1 {
                            let mut e0 = 2u32;
                            let mut ppow = pp;
                            for i in 3..=e {
                                ppow *= p as u64;
                                if pow_mod_big(10, lp as u64, ppow) == 1 {
                                    e0 = i;
                                } else {
                                    break;
                                }
                            }
                            for _ in e0..e {
                                lpe *= p as i64;
                            }
                        } else {
                            for _ in 1..e {
                                lpe *= p as i64;
                            }
                        }
                    }
                    result = lcm_i64(result, lpe);
                }
                local_total += result;
            }
            local_total
        })
        .sum();

    println!("{}", total);
}
