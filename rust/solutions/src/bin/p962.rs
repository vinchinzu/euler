// Project Euler 962 - Integer triangles with a specific property
// Count integer triangles (a,b,c) with a<=b<=c, a+b+c<=N, where the area squared
// has a special cube-related structure.
// Uses factorization approach: for each z up to N/3, factor z, generate candidate u values
// from z's factors, then enumerate v values and divisor pairs to find valid triangles.

use rayon::prelude::*;
use std::cell::RefCell;

const N: u64 = 1_000_000;

#[inline(always)]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b { std::mem::swap(&mut a, &mut b); }
        b -= a;
        if b == 0 { break; }
    }
    a << shift
}

/// Build smallest-prime-factor sieve up to max_val
fn build_spf(max_val: usize) -> Vec<u32> {
    let mut spf = vec![0u32; max_val + 1];
    for i in 2..=max_val {
        if spf[i] == 0 {
            // i is prime
            let p = i as u32;
            let mut j = i;
            while j <= max_val {
                if spf[j] == 0 {
                    spf[j] = p;
                }
                j += i;
            }
        }
    }
    spf
}

/// Factor n using SPF table (for n <= MAX_Z)
#[inline]
fn factor_spf(mut n: u64, spf: &[u32], out: &mut Vec<(u64, u32)>) {
    out.clear();
    let mut ni = n as usize;
    while ni > 1 {
        let p = unsafe { *spf.get_unchecked(ni) } as u64;
        let mut e = 0u32;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        ni = n as usize;
        out.push((p, e));
    }
}

/// Factor n using trial division with primes list (for larger n)
#[inline]
fn factor_trial(mut n: u64, primes: &[u32], out: &mut Vec<(u64, u32)>) {
    out.clear();
    for &p in primes {
        let p64 = p as u64;
        if p64 * p64 > n {
            break;
        }
        if n % p64 == 0 {
            let mut e = 0u32;
            while n % p64 == 0 {
                n /= p64;
                e += 1;
            }
            out.push((p64, e));
        }
    }
    if n > 1 {
        out.push((n, 1));
    }
}

fn divisors_from_factors_buf(factors: &[(u64, u32)], divs: &mut Vec<u64>) {
    divs.clear();
    divs.push(1u64);
    for &(p, e) in factors {
        let len = divs.len();
        let mut pe = 1u64;
        for _ in 0..e {
            pe *= p;
            for j in 0..len {
                // SAFETY: j < len, and len is the original length before we started pushing
                let base = unsafe { *divs.get_unchecked(j) };
                divs.push(base * pe);
            }
        }
    }
}

fn gen_us_from_z_factor_buf(z_factors: &[(u64, u32)], us: &mut Vec<u64>) {
    us.clear();
    if z_factors.is_empty() {
        us.push(1);
        return;
    }

    // Use iterative approach instead of recursive backtrack
    us.push(1);
    for &(p, e) in z_factors {
        let limit = (2 * e) / 3;
        let base_len = us.len();
        let mut pe = 1u64;
        for _ in 0..limit {
            pe *= p;
            for j in 0..base_len {
                // SAFETY: j < base_len which is original length
                let base = unsafe { *us.get_unchecked(j) };
                us.push(base * pe);
            }
        }
    }
}

fn integer_cuberoot_floor(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = (n as f64).cbrt() as u64;
    // Ensure correctness
    while (x + 1) * (x + 1) * (x + 1) <= n {
        x += 1;
    }
    while x > 0 && x * x * x > n {
        x -= 1;
    }
    x
}

/// Build primes list up to max_val as u32
fn primes_up_to_u32(max_val: usize) -> Vec<u32> {
    let mut is_composite = vec![false; max_val + 1];
    let mut primes = Vec::new();
    for i in 2..=max_val {
        if !is_composite[i] {
            primes.push(i as u32);
            if i * i <= max_val {
                let mut j = i * i;
                while j <= max_val {
                    is_composite[j] = true;
                    j += i;
                }
            }
        }
    }
    primes
}

thread_local! {
    static TL_BUFS: RefCell<(Vec<(u64, u32)>, Vec<(u64, u32)>, Vec<u64>, Vec<u64>)> =
        RefCell::new((
            Vec::with_capacity(16),  // z_factors
            Vec::with_capacity(16),  // t_factors
            Vec::with_capacity(128), // divisors
            Vec::with_capacity(64),  // u_candidates
        ));
}

fn count_for_z(z: u64, spf: &[u32], primes: &[u32]) -> u64 {
    TL_BUFS.with(|tl| {
        let mut bufs = tl.borrow_mut();
        let (ref mut z_factors, ref mut t_factors, ref mut divisors, ref mut u_candidates) = *bufs;

        let mut total = 0u64;
        factor_spf(z, spf, z_factors);
        let z2 = z * z;
        gen_us_from_z_factor_buf(z_factors, u_candidates);

        for idx in 0..u_candidates.len() {
            // SAFETY: idx < u_candidates.len()
            let u = unsafe { *u_candidates.get_unchecked(idx) };
            let u3 = u * u * u;
            if z2 % u3 != 0 {
                continue;
            }
            let w = z2 / u3;
            if w == 0 {
                continue;
            }
            // v_max^3 <= N^2 / w
            let v_max_cubed = N * N / w;
            if v_max_cubed == 0 {
                continue;
            }
            let v_max = integer_cuberoot_floor(v_max_cubed);
            if v_max < u {
                continue;
            }
            for v in u..=v_max {
                if gcd(u, v) != 1 {
                    continue;
                }
                let t = v * w;
                if (t & 3) == 2 {
                    continue;
                }
                let uv_sum = u + v;
                let q_min = (t * uv_sum + N - 1) / N;
                let q_max_sq = (t * u) / (2 * v + u);
                let q_max = q_max_sq.isqrt();
                if q_min > q_max {
                    continue;
                }

                // Factor t: use SPF if small enough, otherwise trial division
                if (t as usize) < spf.len() {
                    factor_spf(t, spf, t_factors);
                } else {
                    factor_trial(t, primes, t_factors);
                }
                divisors_from_factors_buf(t_factors, divisors);
                for &q in divisors.iter() {
                    if q >= q_min && q <= q_max {
                        let p_div = t / q;
                        if (p_div ^ q) & 1 == 0 {
                            total += 1;
                        }
                    }
                }
            }
        }
        total
    })
}

fn count_triangles(spf: &[u32], primes: &[u32]) -> u64 {
    let max_z = N / 3;
    (1..=max_z)
        .into_par_iter()
        .map(|z| count_for_z(z, spf, primes))
        .sum()
}

fn main() {
    let spf = build_spf(1_000_000);
    let primes = primes_up_to_u32(1_000_000);
    println!("{}", count_triangles(&spf, &primes));
}
