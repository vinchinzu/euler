// Project Euler 754 - Product of Gauss Factorials
// Compute product of g(i) for i=1..N using Mobius function and factorial grouping.

fn main() {
    const MOD: u64 = 1_000_000_007;
    const N: usize = 100_000_000;

    fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
        let mut result = 1u64;
        base %= m;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result as u128 * base as u128 % m as u128) as u64;
            }
            exp >>= 1;
            base = (base as u128 * base as u128 % m as u128) as u64;
        }
        result
    }

    fn mod_inv(a: u64, m: u64) -> u64 {
        pow_mod(a, m - 2, m)
    }

    fn tr(n: u64) -> u64 {
        n.wrapping_mul(n + 1) / 2
    }

    // Compute Mobius function
    let mut mobius = vec![1i8; N + 1];
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    if N >= 1 { is_prime[1] = false; }

    for i in 2..=N {
        if is_prime[i] {
            for j in (i..=N).step_by(i) {
                if j > i { is_prime[j] = false; }
                if (j / i) % i == 0 {
                    mobius[j] = 0;
                } else {
                    mobius[j] = -mobius[j];
                }
            }
        }
    }
    drop(is_prime);

    // Precompute factorials mod MOD
    let mut fact = vec![1u64; N + 1];
    for i in 1..=N {
        fact[i] = (fact[i - 1] as u128 * i as u128 % MOD as u128) as u64;
    }

    // Precompute prefix product of factorials
    let mut prod_fact = vec![1u64; N + 1];
    for i in 1..=N {
        prod_fact[i] = (prod_fact[i - 1] as u128 * fact[i] as u128 % MOD as u128) as u64;
    }

    let l = (N as f64).sqrt() as usize;

    let mut res_neg: u64 = 1;
    let mut res_pos: u64 = 1;

    // For g <= N/L
    let g_limit = N / l;
    for g in 1..=g_limit {
        if mobius[g] == 0 { continue; }
        let pw = pow_mod(g as u64, tr((N / g) as u64) % (MOD - 1), MOD);
        if mobius[g] == 1 {
            res_pos = (res_pos as u128 * pw as u128 % MOD as u128) as u64;
        } else {
            res_neg = (res_neg as u128 * pw as u128 % MOD as u128) as u64;
        }
    }

    // For g > N/L, group by exponent q = N/g
    for q in 1..l {
        let mut sub_neg: u64 = 1;
        let mut sub_pos: u64 = 1;
        let g_lo = N / (q + 1) + 1;
        let g_hi = N / q;
        for g in g_lo..=g_hi {
            if mobius[g] == 0 { continue; }
            if mobius[g] == 1 {
                sub_pos = (sub_pos as u128 * g as u128 % MOD as u128) as u64;
            } else {
                sub_neg = (sub_neg as u128 * g as u128 % MOD as u128) as u64;
            }
        }
        let exp_val = tr(q as u64) % (MOD - 1);
        res_pos = (res_pos as u128 * pow_mod(sub_pos, exp_val, MOD) as u128 % MOD as u128) as u64;
        res_neg = (res_neg as u128 * pow_mod(sub_neg, exp_val, MOD) as u128 % MOD as u128) as u64;
    }

    // Multiply by product of factorials
    for g in 1..=N {
        if mobius[g] == 0 { continue; }
        if mobius[g] == 1 {
            res_pos = (res_pos as u128 * prod_fact[N / g] as u128 % MOD as u128) as u64;
        } else {
            res_neg = (res_neg as u128 * prod_fact[N / g] as u128 % MOD as u128) as u64;
        }
    }

    let ans = (res_pos as u128 * mod_inv(res_neg, MOD) as u128 % MOD as u128) as u64;
    println!("{}", ans);
}
