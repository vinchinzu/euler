// Project Euler 754 - Product of Gauss Factorials
// Compute product of g(i) for i=1..N using Mobius function and factorial grouping.
//
// Optimizations:
// - Pure u64 arithmetic (MOD < 2^30, so product < 2^60 < u64::MAX)
// - Linear sieve for Mobius (O(N), no separate is_prime array)
// - Eliminate fact[] array (compute prod_fact in single pass)
// - Quotient grouping for third loop: O(sqrt(N)) instead of O(N)
// - Merged O(N) pass for prod_fact and prefix sums
// - unsafe get_unchecked in hot loops

fn main() {
    const MOD: u64 = 1_000_000_007;
    const N: usize = 100_000_000;

    #[inline(always)]
    fn mul_mod(a: u64, b: u64) -> u64 {
        a * b % MOD
    }

    fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
        let mut result = 1u64;
        base %= MOD;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            exp >>= 1;
            base = base * base % MOD;
        }
        result
    }

    fn mod_inv(a: u64) -> u64 {
        pow_mod(a, MOD - 2)
    }

    #[inline(always)]
    fn tr(n: u64) -> u64 {
        n.wrapping_mul(n + 1) / 2
    }

    // Linear sieve for Mobius function
    let mut mobius = vec![0i8; N + 1];
    mobius[1] = 1;
    let mut primes: Vec<usize> = Vec::with_capacity(6_000_000);
    let mut smallest_prime = vec![0u32; N + 1];

    for i in 2..=N {
        if smallest_prime[i] == 0 {
            smallest_prime[i] = i as u32;
            primes.push(i);
            mobius[i] = -1;
        }
        for &p in &primes {
            let ip = i * p;
            if ip > N { break; }
            smallest_prime[ip] = p as u32;
            if i % p == 0 {
                break;
            } else {
                mobius[ip] = -mobius[i];
            }
        }
    }
    drop(smallest_prime);
    drop(primes);

    // Merged O(N) pass: compute prefix product of factorials and prefix sums of mobius
    let mut prod_fact = vec![0u64; N + 1];
    let mut mu_pos_prefix = vec![0u32; N + 1];
    let mut mu_neg_prefix = vec![0u32; N + 1];
    {
        let mut running_fact = 1u64;
        prod_fact[0] = 1;
        for i in 1..=N {
            running_fact = running_fact * (i as u64) % MOD;
            unsafe {
                *prod_fact.get_unchecked_mut(i) = *prod_fact.get_unchecked(i - 1) * running_fact % MOD;
                let m = *mobius.get_unchecked(i);
                *mu_pos_prefix.get_unchecked_mut(i) = *mu_pos_prefix.get_unchecked(i - 1) + (m == 1) as u32;
                *mu_neg_prefix.get_unchecked_mut(i) = *mu_neg_prefix.get_unchecked(i - 1) + (m == -1) as u32;
            }
        }
    }

    let l = (N as f64).sqrt() as usize;

    let mut res_neg: u64 = 1;
    let mut res_pos: u64 = 1;

    // Phase 1: For g <= N/L, each g gets its own pow_mod call
    let g_limit = N / l;
    for g in 1..=g_limit {
        unsafe {
            let m = *mobius.get_unchecked(g);
            if m == 0 { continue; }
            let pw = pow_mod(g as u64, tr((N / g) as u64) % (MOD - 1));
            if m == 1 {
                res_pos = res_pos * pw % MOD;
            } else {
                res_neg = res_neg * pw % MOD;
            }
        }
    }

    // Phase 2: For g > N/L, group by exponent q = N/g
    for q in 1..l {
        let mut sub_neg: u64 = 1;
        let mut sub_pos: u64 = 1;
        let g_lo = N / (q + 1) + 1;
        let g_hi = N / q;
        for g in g_lo..=g_hi {
            unsafe {
                let m = *mobius.get_unchecked(g);
                if m == 0 { continue; }
                if m == 1 {
                    sub_pos = sub_pos * (g as u64) % MOD;
                } else {
                    sub_neg = sub_neg * (g as u64) % MOD;
                }
            }
        }
        let exp_val = tr(q as u64) % (MOD - 1);
        res_pos = mul_mod(res_pos, pow_mod(sub_pos, exp_val));
        res_neg = mul_mod(res_neg, pow_mod(sub_neg, exp_val));
    }

    // Phase 3: Multiply by product of factorials using quotient grouping
    {
        let mut g = 1usize;
        while g <= N {
            let q = N / g;
            let g_hi = N / q;
            let cnt_pos;
            let cnt_neg;
            unsafe {
                cnt_pos = *mu_pos_prefix.get_unchecked(g_hi) - *mu_pos_prefix.get_unchecked(g - 1);
                cnt_neg = *mu_neg_prefix.get_unchecked(g_hi) - *mu_neg_prefix.get_unchecked(g - 1);
            }
            let pf_q = unsafe { *prod_fact.get_unchecked(q) };
            if cnt_pos > 0 {
                res_pos = mul_mod(res_pos, pow_mod(pf_q, cnt_pos as u64));
            }
            if cnt_neg > 0 {
                res_neg = mul_mod(res_neg, pow_mod(pf_q, cnt_neg as u64));
            }
            g = g_hi + 1;
        }
    }

    let ans = mul_mod(res_pos, mod_inv(res_neg));
    println!("{}", ans);
}
