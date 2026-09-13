use rayon::prelude::*;

const N_MAX: usize = 10_000_000;
const P_MOD: i64 = 214_358_881; // 11^8
const PHI_MOD: i64 = 389_743_420; // 2 * phi(11^8)

fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = N_MAX;

    // Linear sieve for primes, mu, phi
    let mut mu = vec![0i8; N_MAX + 1];
    let mut phi_arr = vec![0i32; N_MAX + 1];
    let mut primes = Vec::with_capacity(700_000);

    mu[1] = 1;
    phi_arr[1] = 1;

    for i in 2..=N_MAX {
        if phi_arr[i] == 0 {
            primes.push(i as u32);
            mu[i] = -1;
            phi_arr[i] = (i - 1) as i32;
        }
        for &p_u32 in &primes {
            let p = p_u32 as usize;
            let t = i * p;
            if t > N_MAX {
                break;
            }
            if i % p == 0 {
                mu[t] = 0;
                phi_arr[t] = phi_arr[i] * p as i32;
                break;
            } else {
                mu[t] = -mu[i];
                phi_arr[t] = phi_arr[i] * (p - 1) as i32;
            }
        }
    }

    drop(primes);

    // Mertens function
    let mut mertens = vec![0i32; N_MAX + 1];
    for i in 1..=N_MAX {
        mertens[i] = mertens[i - 1] + mu[i] as i32;
    }

    // total = |M(n)| mod PHI_MOD
    let total: i64 = (1..=n)
        .into_par_iter()
        .filter_map(|i| {
            let m = mu[i];
            if m == 0 {
                return None;
            }
            let x = (n / i + 1) as i64;
            let cube = (x % PHI_MOD) * (x % PHI_MOD) % PHI_MOD * (x % PHI_MOD) % PHI_MOD;
            let cube = (cube - 1 + PHI_MOD) % PHI_MOD;
            Some((m as i64 * cube % PHI_MOD + PHI_MOD) % PHI_MOD)
        })
        .reduce(|| 0i64, |acc, val| (acc + val) % PHI_MOD);
    let total = (total % PHI_MOD + PHI_MOD) % PHI_MOD;

    drop(mu);

    // Precompute powers of 2 mod P_MOD for fast exponentiation
    const K: usize = 65536;
    let mut pow2_low = vec![0i32; K];
    pow2_low[0] = 1;
    for i in 1..K {
        pow2_low[i] = ((pow2_low[i - 1] as i64 * 2) % P_MOD) as i32;
    }
    let step = pow_mod(2, K as i64, P_MOD);
    let num_high = (PHI_MOD as usize / K) + 2;
    let mut pow2_high = vec![0i32; num_high];
    pow2_high[0] = 1;
    for i in 1..num_high {
        pow2_high[i] = ((pow2_high[i - 1] as i64 * step) % P_MOD) as i32;
    }

    let fast_pow2 = |exp: i64| -> i64 {
        let q = (exp as usize) >> 16;
        let r = (exp as usize) & 0xffff;
        (pow2_high[q] as i64 * pow2_low[r] as i64) % P_MOD
    };

    // F(s, n): compute using quotient blocks
    let f = |s: usize, n: usize| -> i64 {
        if s > n / 2 {
            return n as i64 + 1 - s as i64;
        }
        let mut ret = 0i64;
        let mut i = 1i64;
        let s_i64 = s as i64;
        let n_i64 = n as i64;
        while i * s_i64 <= n_i64 {
            let t = n_i64 / i;
            let j = n_i64 / t + 1;
            let j_idx = (j - 1) as usize;
            let dm = (mertens[j_idx] - mertens[(i - 1) as usize]) as i64;
            if dm != 0 {
                let d = t / s_i64;
                let g = (((2 * t + 2 - s_i64 * (1 + d)) * d) / 2) % PHI_MOD;
                ret += g * dm;
            }
            i = j;
        }
        (ret % PHI_MOD + PHI_MOD) % PHI_MOD
    };

    let ans_init = pow_mod(2, total, P_MOD);

    let half = (total - 1 + PHI_MOD) % PHI_MOD;
    let half = if half % 2 == 0 {
        half / 2
    } else {
        (half + PHI_MOD) / 2
    };
    let term1 = pow_mod(2, half, P_MOD);

    let sum_diff: i64 = (1..=n)
        .into_par_iter()
        .map(|b| {
            let m_val = (6i64 * phi_arr[b] as i64) % P_MOD;
            let f_val = f(b, n);
            let term2 = fast_pow2((half - f_val + PHI_MOD) % PHI_MOD);
            let diff = (term1 - term2 + P_MOD) % P_MOD;
            m_val * diff % P_MOD
        })
        .reduce(|| 0i64, |acc, val| (acc + val) % P_MOD);

    let mut ans = (ans_init - sum_diff % P_MOD + P_MOD) % P_MOD;
    ans = (ans - 1 + P_MOD) % P_MOD;
    println!("{}", ans);
}
