// Project Euler 658 - Incomplete Words II
// Sum over 1<=k<=K of I(k), N=10^12, K=10^7, M=10^9+7.

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;

#[inline(always)]
fn power_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            r = (r * base) % MOD;
        }
        base = (base * base) % MOD;
        exp >>= 1;
    }
    r
}

fn main() {
    let n: u64 = 1_000_000_000_000;
    let k = 10_000_000usize;
    let mut spf = vec![0u32; k + 1];
    for i in (2..=k).step_by(2) {
        spf[i] = 2;
    }
    for i in (3..=k).step_by(2) {
        if spf[i] == 0 {
            spf[i] = i as u32;
            if (i as u64) * (i as u64) <= k as u64 {
                let step = 2 * i;
                let mut j = i * i;
                while j <= k {
                    if spf[j] == 0 {
                        spf[j] = i as u32;
                    }
                    j += step;
                }
            }
        }
    }

    // Collect primes, power in parallel (pure u64 arithmetic)
    let primes: Vec<u32> = (2..=k as u32).filter(|&i| spf[i as usize] == i).collect();
    let prime_pows: Vec<u32> = primes
        .par_iter()
        .map(|&p| power_mod(p as u64, n + 1) as u32)
        .collect();

    let mut pows = vec![0u32; k + 1];
    pows[1] = 1;
    for (idx, &p) in primes.iter().enumerate() {
        pows[p as usize] = prime_pows[idx];
    }
    for i in 2..=k {
        let p = unsafe { *spf.get_unchecked(i) } as usize;
        if p != i {
            unsafe {
                let a = *pows.get_unchecked(p) as u64;
                let b = *pows.get_unchecked(i / p) as u64;
                *pows.get_unchecked_mut(i) = ((a * b) % MOD) as u32;
            }
        }
    }

    let mut invs = vec![0u32; k + 1];
    invs[1] = 1;
    for i in 2..=k {
        let q = MOD / i as u64;
        let rem = (MOD % i as u64) as usize;
        let inv_rem = unsafe { *invs.get_unchecked(rem) } as u64;
        let prod = (q * inv_rem) % MOD;
        unsafe {
            *invs.get_unchecked_mut(i) = if prod == 0 { 0 } else { (MOD - prod) as u32 };
        }
    }

    let inv2 = power_mod(2, MOD - 2);
    let mut ans = 0u64;
    let mut ncr = 1u64;
    let mut inner_sum = (k % 2) as u64;

    for t in 0..k {
        let num_words = if t == 0 {
            1u64
        } else if t == 1 {
            (n + 1) % MOD
        } else {
            let pt = unsafe { *pows.get_unchecked(t) };
            let diff = if pt == 0 { MOD - 1 } else { pt as u64 - 1 };
            let it = unsafe { *invs.get_unchecked(t - 1) } as u64;
            (diff * it) % MOD
        };

        ans = (ans + num_words * inner_sum) % MOD;

        if t < k - 1 {
            let kt = (k - t) as u64;
            let it1 = unsafe { *invs.get_unchecked(t + 1) } as u64;
            let new_ncr = ((ncr * kt) % MOD * it1) % MOD;
            let parity_val = if kt % 2 == 0 { 1u64 } else { MOD - 1 };
            let sum_ncr = if ncr + new_ncr >= MOD { ncr + new_ncr - MOD } else { ncr + new_ncr };
            let term = (parity_val * sum_ncr) % MOD;
            let s = inner_sum + 1 + term;
            let s_red = s % MOD;
            inner_sum = (s_red * inv2) % MOD;
            ncr = new_ncr;
        }
    }

    println!("{}", ans % MOD);
}
