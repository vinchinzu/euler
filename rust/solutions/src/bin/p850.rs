// Project Euler 850 - Fractional parts sum S(N)
// DFS over square-full small-prime kernels; tail sums for large primes.

use rayon::prelude::*;

const MOD: i64 = 977676779;
const MOD2: i64 = 2 * MOD;
const N_VAL: i64 = 33557799775533;
const SQRT_N_MAX: usize = 5_900_000;
const SMALL_PRIME_LIMIT: usize = 32_000;
const SMALL_P2: i64 = (SMALL_PRIME_LIMIT as i64) * (SMALL_PRIME_LIMIT as i64);
const NUM_K: usize = 22;
const MOD2_I128: i128 = MOD2 as i128;

const SMALL_ODD_KS: [i32; NUM_K] = {
    let mut a = [0i32; NUM_K];
    let mut i = 0;
    let mut k = 3;
    while i < NUM_K {
        a[i] = k;
        k += 2;
        i += 1;
    }
    a
};

const T1_TABLE: [[u8; NUM_K]; 48] = {
    let mut tab = [[0u8; NUM_K]; 48];
    let mut e = 2;
    while e < 48 {
        let mut ki = 0;
        while ki < NUM_K {
            let k = SMALL_ODD_KS[ki];
            tab[e][ki] = (e as i32 - (e as i32 + k - 1) / k) as u8;
            ki += 1;
        }
        e += 1;
    }
    tab
};

const T2_TABLE: [[u8; NUM_K]; 48] = {
    let mut tab = [[0u8; NUM_K]; 48];
    let mut e = 2;
    while e < 48 {
        let mut ki = 0;
        while ki < NUM_K {
            let k = SMALL_ODD_KS[ki];
            tab[e][ki] = (e as i32 - 1 - (e as i32 + k - 2) / k) as u8;
            ki += 1;
        }
        e += 1;
    }
    tab
};

struct Ctx<'a> {
    small_primes: &'a [i64],
    is_prime: &'a [u8],
    p_sum: &'a [i64],
}

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        0
    } else {
        (n as u64).isqrt() as i64
    }
}

#[inline(always)]
fn icbrt(n: i64) -> i64 {
    if n < 8 {
        return i64::from(n > 0);
    }
    let mut x = 1i64 << (((63 - n.leading_zeros()) / 3) + 1);
    loop {
        let x2 = x.saturating_mul(x);
        let y = (x.saturating_mul(2) + n / x2) / 3;
        if y >= x {
            break;
        }
        x = y;
    }
    while x > 0 && x.saturating_mul(x).saturating_mul(x) > n {
        x -= 1;
    }
    loop {
        let x1 = x + 1;
        if x1.saturating_mul(x1).saturating_mul(x1) <= n {
            x = x1;
        } else {
            break;
        }
    }
    x
}

#[inline(always)]
fn calc_tail_sum(m: i64, is_prime: &[u8], p_sum: &[i64]) -> i64 {
    if m <= SMALL_P2 {
        return m;
    }

    let mut res = m;
    let min_p_bound = SMALL_PRIME_LIMIT;
    let mut k = 1i64;
    let mut upper_p = isqrt(m);
    loop {
        let lower_p = isqrt(m / (k + 1));
        let eff_upper = upper_p as usize;
        let eff_lower = if lower_p as usize > min_p_bound {
            lower_p as usize
        } else {
            min_p_bound
        };

        if eff_upper > eff_lower && eff_upper <= SQRT_N_MAX {
            // SAFETY: 0 <= eff_lower < eff_upper <= SQRT_N_MAX, p_sum len = SQRT_N_MAX+1.
            let term_sum = unsafe { *p_sum.get_unchecked(eff_upper) - *p_sum.get_unchecked(eff_lower) };
            res += term_sum * k;
        }

        if lower_p as usize <= min_p_bound {
            break;
        }
        upper_p = lower_p;
        k += 1;
    }

    let limit_p3 = icbrt(m);
    if limit_p3 > SMALL_PRIME_LIMIT as i64 && limit_p3 <= SQRT_N_MAX as i64 {
        for p in (SMALL_PRIME_LIMIT + 1)..=limit_p3 as usize {
            if unsafe { *is_prime.get_unchecked(p) } != 0 {
                let pi = p as i64;
                let val = pi * pi - pi;
                let term = m / (pi * pi * pi);
                res += val * term;
            }
        }
    }
    res
}

#[inline(always)]
fn contribute(
    current_d: i64,
    curr_k: &[i64; NUM_K],
    curr_inf: i64,
    ctx: &Ctx,
    out_k: &mut [i128; NUM_K],
    out_inf: &mut i128,
) {
    let m = N_VAL / current_d;
    let tm = if m <= SMALL_P2 {
        m as i128
    } else {
        (calc_tail_sum(m, ctx.is_prime, ctx.p_sum) % MOD2) as i128
    };
    let tm_u64 = tm as u64;
    for ki in 0..NUM_K {
        unsafe {
            let prod = (*curr_k.get_unchecked(ki) as u64 * tm_u64) as i128;
            *out_k.get_unchecked_mut(ki) += prod;
        }
    }
    *out_inf += (curr_inf as u64 * tm_u64) as i128;
}

#[inline(always)]
fn merge_acc(
    a_k: &mut [i128; NUM_K],
    a_inf: &mut i128,
    b_k: [i128; NUM_K],
    b_inf: i128,
) {
    for ki in 0..NUM_K {
        a_k[ki] += b_k[ki];
    }
    *a_inf += b_inf;
}

const PAR_D_MAX: i64 = 2000;

fn apply_prime(
    i: usize,
    current_d: i64,
    curr_k: &[i64; NUM_K],
    curr_inf: i64,
    ctx: &Ctx,
    out_k: &mut [i128; NUM_K],
    out_inf: &mut i128,
    par_next: bool,
) {
    let p = unsafe { *ctx.small_primes.get_unchecked(i) };
    let mlim = N_VAL / current_d;
    let mut pows = [1i64; 48];
    pows[1] = p;
    let mut pe = p;
    let mut term_inf = p - 1;
    let mut e = 2usize;
    loop {
        if p > mlim / pe {
            break;
        }
        pe *= p;
        pows[e] = pe;
        let new_d = current_d * pe;

        let mut new_k = [0i64; NUM_K];
        for ki in 0..NUM_K {
            let t1 = T1_TABLE[e][ki] as usize;
            let t2 = T2_TABLE[e][ki] as usize;
            if t1 == t2 {
                new_k[ki] = 0;
            } else {
                let term = pows[t1] - pows[t2];
                new_k[ki] = (curr_k[ki] * (term % MOD2)) % MOD2;
            }
        }
        let new_inf = (curr_inf * (term_inf % MOD2)) % MOD2;

        dfs(
            i + 1,
            new_d,
            &new_k,
            new_inf,
            ctx,
            out_k,
            out_inf,
            par_next && new_d <= PAR_D_MAX,
        );

        e += 1;
        if e >= 48 {
            break;
        }
        term_inf *= p;
    }
}

fn dfs(
    idx: usize,
    current_d: i64,
    curr_k: &[i64; NUM_K],
    curr_inf: i64,
    ctx: &Ctx,
    out_k: &mut [i128; NUM_K],
    out_inf: &mut i128,
    par: bool,
) {
    contribute(current_d, curr_k, curr_inf, ctx, out_k, out_inf);

    let nsp = ctx.small_primes.len();
    if idx >= nsp {
        return;
    }
    let max_p = isqrt(N_VAL / current_d);
    if max_p < unsafe { *ctx.small_primes.get_unchecked(idx) } {
        return;
    }
    let end = idx + ctx.small_primes[idx..].partition_point(|&p| p <= max_p);
    if end <= idx {
        return;
    }

    if par && end - idx > 16 {
        let (bk, binf) = (idx..end)
            .into_par_iter()
            .map(|i| {
                let mut lk = [0i128; NUM_K];
                let mut li = 0i128;
                apply_prime(
                    i,
                    current_d,
                    curr_k,
                    curr_inf,
                    ctx,
                    &mut lk,
                    &mut li,
                    current_d <= PAR_D_MAX,
                );
                (lk, li)
            })
            .reduce(
                || ([0i128; NUM_K], 0i128),
                |mut a, b| {
                    merge_acc(&mut a.0, &mut a.1, b.0, b.1);
                    a
                },
            );
        merge_acc(out_k, out_inf, bk, binf);
    } else {
        for i in idx..end {
            apply_prime(i, current_d, curr_k, curr_inf, ctx, out_k, out_inf, false);
        }
    }
}

fn reduce_mod2(x: i128) -> i64 {
    let r = x % MOD2_I128;
    (if r < 0 { r + MOD2_I128 } else { r }) as i64
}

fn main() {
    let mut is_prime = vec![1u8; SQRT_N_MAX + 1];
    is_prime[0] = 0;
    is_prime[1] = 0;
    let limit = isqrt(SQRT_N_MAX as i64) as usize + 1;
    for i in 2..=limit {
        if is_prime[i] != 0 {
            let mut j = i * i;
            while j <= SQRT_N_MAX {
                is_prime[j] = 0;
                j += i;
            }
        }
    }

    let mut p_sum = vec![0i64; SQRT_N_MAX + 1];
    let mut small_primes = Vec::with_capacity(3500);
    let mut cs: i64 = 0;
    for i in 2..=SQRT_N_MAX {
        if is_prime[i] != 0 {
            cs += i as i64 - 1;
            if i <= SMALL_PRIME_LIMIT {
                small_primes.push(i as i64);
            }
        }
        p_sum[i] = cs;
    }

    let ctx = Ctx {
        small_primes: &small_primes,
        is_prime: &is_prime,
        p_sum: &p_sum,
    };

    let init_k = [1i64; NUM_K];
    let mut total_k = [0i128; NUM_K];
    let mut total_inf = 0i128;
    dfs(0, 1, &init_k, 1, &ctx, &mut total_k, &mut total_inf, true);


    let mut total_sums_k = [0i64; NUM_K];
    for ki in 0..NUM_K {
        total_sums_k[ki] = reduce_mod2(total_k[ki]);
    }
    let total_sums_inf = reduce_mod2(total_inf);

    let num_odd = (N_VAL + 1) / 2;
    let n_mod = N_VAL % MOD2;
    let np1_mod = (N_VAL + 1) % MOD2;
    let half_nn = (n_mod * np1_mod / 2) % MOD2;
    let term_doubled = (num_odd % MOD2 * half_nn) % MOD2;

    let mut sum_sigma_ck = N_VAL % MOD2;
    for ki in 0..NUM_K {
        sum_sigma_ck = (sum_sigma_ck + total_sums_k[ki]) % MOD2;
    }

    let num_small = (NUM_K as i64) + 1;
    let num_large = num_odd - num_small;
    sum_sigma_ck = (sum_sigma_ck + (num_large % MOD2) * (total_sums_inf % MOD2)) % MOD2;

    let two_s = ((term_doubled - sum_sigma_ck) % MOD2 + MOD2) % MOD2;
    let ans = (two_s / 2) % MOD;

    println!("{}", ans);
}
