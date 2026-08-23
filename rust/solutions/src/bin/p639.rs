// Project Euler 639 - Summing a multiplicative function
// Powerful number iteration with Lagrange interpolation for power sums

use rayon::prelude::*;

const N_VAL: u64 = 1_000_000_000_000;
const K_VAL: usize = 50;
const MOD: u64 = 1_000_000_007;
const MOD32: u32 = 1_000_000_007;

#[inline(always)]
fn mul(a: u32, b: u32) -> u32 {
    ((a as u64) * (b as u64) % MOD) as u32
}

#[inline(always)]
fn add(a: u32, b: u32) -> u32 {
    let s = a + b;
    if s >= MOD32 { s - MOD32 } else { s }
}

#[inline(always)]
fn sub(a: u32, b: u32) -> u32 {
    if a >= b { a - b } else { a + MOD32 - b }
}

fn powmod(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

#[inline(always)]
fn isqrt(n: u64) -> u64 {
    let mut x = (n as f64).sqrt() as u64;
    if x > 0 && x * x > n {
        x -= 1;
    }
    if (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

#[inline(always)]
fn icbrt(n: u64) -> u64 {
    let mut x = (n as f64).cbrt() as u64;
    while x > 0 && x * x * x > n {
        x -= 1;
    }
    loop {
        let x1 = x + 1;
        if x1 * x1 * x1 > n {
            break;
        }
        x = x1;
    }
    x
}

fn sum_kth_powers(n: u64, k: usize, sp: &[u32], denoms: &[u32], l: u64) -> u32 {
    if n <= l {
        // SAFETY: n <= l and sp.len() == l+1
        return unsafe { *sp.get_unchecked(n as usize) };
    }
    let pts = k + 2;
    let nmod = n % MOD;
    let mut prefix = [0u32; 64];
    let mut suffix = [0u32; 64];
    prefix[0] = 1;
    for j in 0..pts {
        let t = if nmod >= j as u64 {
            nmod - j as u64
        } else {
            nmod + MOD - j as u64
        };
        prefix[j + 1] = (prefix[j] as u64 * t % MOD) as u32;
    }
    suffix[pts] = 1;
    for j in (0..pts).rev() {
        let t = if nmod >= j as u64 {
            nmod - j as u64
        } else {
            nmod + MOD - j as u64
        };
        suffix[j] = (suffix[j + 1] as u64 * t % MOD) as u32;
    }
    let mut result = 0u64;
    for i in 0..pts {
        // SAFETY: i < pts <= 52, sp covers 0..=l, denoms.len() >= pts
        let (numer_pref, numer_suf, yi, denom) = unsafe {
            (
                *prefix.get_unchecked(i),
                *suffix.get_unchecked(i + 1),
                *sp.get_unchecked(i),
                *denoms.get_unchecked(i),
            )
        };
        let numer = (numer_pref as u64 * numer_suf as u64 % MOD) * yi as u64 % MOD;
        result += numer * denom as u64 % MOD;
        if result >= MOD {
            result -= MOD;
        }
    }
    result as u32
}

struct Entry {
    min_idx: u32,
    d: u64,
    mult: u32,
    prev_e: i32,
}

fn process_k(
    k: usize,
    l: usize,
    primes: &[u32],
    is_prime: &[u8],
    spf: &[u32],
    inv_fact: &[u32],
) -> u32 {
    let mut nth_pows = vec![0u32; l + 1];
    nth_pows[1] = 1;
    for i in 2..=l {
        let p = spf[i] as usize;
        if p == i {
            nth_pows[i] = powmod(i as u64, k as u64) as u32;
        } else {
            nth_pows[i] = mul(nth_pows[p], nth_pows[i / p]);
        }
    }

    let mut sum_powers = vec![0u32; l + 1];
    let mut sum_coeffs = vec![0u32; l + 1];
    for i in 1..=l {
        sum_powers[i] = add(sum_powers[i - 1], nth_pows[i]);
        let coeff = if is_prime[i] != 0 {
            mul(nth_pows[i], sub(1, nth_pows[i]))
        } else {
            0
        };
        sum_coeffs[i] = add(sum_coeffs[i - 1], coeff);
    }

    let m = k + 1;
    let pts = k + 2;
    let mut denoms = [0u32; 64];
    for i in 0..pts {
        let mut d = mul(inv_fact[i], inv_fact[m - i]);
        if (m - i) % 2 == 1 {
            d = if d == 0 { 0 } else { MOD32 - d };
        }
        denoms[i] = d;
    }

    let l64 = l as u64;
    let mut ans: u64 = 0;
    let mut stack: Vec<Entry> = Vec::with_capacity(1 << 18);
    stack.push(Entry { min_idx: 0, d: 1, mult: 1, prev_e: 0 });

    while let Some(e) = stack.pop() {
        let n = N_VAL / e.d;

        if e.prev_e != 2 {
            let sp = sum_kth_powers(n, k, &sum_powers, &denoms, l64);
            ans += sp as u64 * e.mult as u64 % MOD;
            if ans >= MOD {
                ans -= MOD;
            }
        }

        let lim = icbrt(n);
        let threshold = if lim > 0 { n / lim } else { n };

        for i in e.min_idx as usize..primes.len() {
            let p = primes[i] as u64;
            let pp = p * p;
            if pp > threshold {
                break;
            }
            let q = n / pp;
            let sp_q = sum_kth_powers(q, k, &sum_powers, &denoms, l64);
            let pu = p as usize;
            let coeff = sub(sum_coeffs[pu], sum_coeffs[pu - 1]);
            ans += (sp_q as u64 * e.mult as u64 % MOD) * coeff as u64 % MOD;
            if ans >= MOD {
                ans -= MOD;
            }
        }

        let p_min = if (e.min_idx as usize) < primes.len() {
            primes[e.min_idx as usize] as u64
        } else {
            l64 + 1
        };
        let pmin_m1 = p_min - 1;

        if lim > 1 {
            let mut high = isqrt(n);
            for q in 1..lim {
                let nq1 = n / (q + 1);
                let low_raw = isqrt(nq1);
                let mut low = low_raw;
                if low < pmin_m1 {
                    low = pmin_m1;
                }
                let mut hi = high;
                if hi > l64 {
                    hi = l64;
                }
                if hi >= low {
                    // SAFETY: 1 <= q < lim <= 10^4 < l; 0 <= low <= hi <= l
                    let (coeff_sum, spq) = unsafe {
                        let cs = sub(
                            *sum_coeffs.get_unchecked(hi as usize),
                            *sum_coeffs.get_unchecked(low as usize),
                        );
                        (cs, *sum_powers.get_unchecked(q as usize))
                    };
                    ans += (spq as u64 * e.mult as u64 % MOD) * coeff_sum as u64 % MOD;
                    if ans >= MOD {
                        ans -= MOD;
                    }
                } else {
                    break;
                }
                high = low_raw;
            }
        }

        for i in e.min_idx as usize..primes.len() {
            let p = primes[i] as u64;
            if p > 1 && e.d > N_VAL / p / p / p {
                break;
            }
            let mut new_d = e.d * p;
            let pk = nth_pows[p as usize];
            let new_mult = mul(e.mult, mul(pk, sub(1, pk)));
            let mut ee = 1;
            while new_d <= N_VAL / p {
                new_d *= p;
                ee += 1;
                stack.push(Entry {
                    min_idx: (i + 1) as u32,
                    d: new_d,
                    mult: new_mult,
                    prev_e: ee,
                });
            }
        }
    }

    ans as u32
}

fn main() {
    let l = {
        let mut x = (N_VAL as f64).sqrt() as u64;
        while x * x > N_VAL {
            x -= 1;
        }
        while (x + 1) * (x + 1) <= N_VAL {
            x += 1;
        }
        x as usize
    };

    let mut spf = vec![0u32; l + 1];
    let mut is_prime = vec![0u8; l + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(l / 10);
    for i in 2..=l {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
            is_prime[i] = 1;
        }
        for &p in &primes {
            if p as usize > l / i || p > spf[i] {
                break;
            }
            spf[i * p as usize] = p;
        }
    }

    let max_deg = K_VAL + 2;
    let mut fact = vec![1u32; max_deg + 1];
    for i in 1..=max_deg {
        fact[i] = mul(fact[i - 1], i as u32);
    }
    let mut inv_fact = vec![1u32; max_deg + 1];
    inv_fact[max_deg] = powmod(fact[max_deg] as u64, MOD - 2) as u32;
    for i in (0..max_deg).rev() {
        inv_fact[i] = mul(inv_fact[i + 1], (i as u32) + 1);
    }

    let ans: u64 = (1..K_VAL + 1)
        .into_par_iter()
        .map(|k| process_k(k, l, &primes, &is_prime, &spf, &inv_fact) as u64)
        .sum();

    println!("{}", ans % MOD);
}
