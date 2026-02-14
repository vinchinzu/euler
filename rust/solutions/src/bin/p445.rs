// Project Euler 445 - Retractions A

use euler_utils::mod_pow;

const NN: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;

fn power_mod(base: u64, exp: u64) -> u64 {
    mod_pow(base, exp, MOD)
}

fn mod_inv(a: u64) -> u64 {
    power_mod(a, MOD - 2)
}

fn main() {
    // Sieve smallest prime factor
    let mut spf = vec![0u32; NN + 1];
    for i in 0..=NN {
        spf[i] = i as u32;
    }
    for i in 2..=NN {
        if (i as u64) * (i as u64) > NN as u64 {
            break;
        }
        if spf[i] == i as u32 {
            let mut j = i * i;
            while j <= NN {
                if spf[j] == j as u32 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }

    let mut exps = vec![0i32; NN + 1];
    let mut res_prod_pe: u64 = 1;
    let mut res_prod_1pe: u64 = 1;
    let mut ans: u64 = 0;

    for k in 1..=NN / 2 {
        // Multiply by (N + 1 - k)
        let mut num = (NN + 1 - k) as u32;
        while num > 1 {
            let p = spf[num as usize];
            let mut e = 0i32;
            while num % p == 0 {
                num /= p;
                e += 1;
            }

            let old_pow = power_mod(p as u64, exps[p as usize] as u64);
            exps[p as usize] += e;
            let new_pow = power_mod(p as u64, exps[p as usize] as u64);

            res_prod_pe = res_prod_pe % MOD * mod_inv(old_pow) % MOD * new_pow % MOD;
            let old_term = if old_pow == 1 { 1 } else { (1 + old_pow) % MOD };
            let new_term = (1 + new_pow) % MOD;
            res_prod_1pe = res_prod_1pe % MOD * mod_inv(old_term) % MOD * new_term % MOD;
        }

        // Divide by k
        let mut den = k as u32;
        while den > 1 {
            let p = spf[den as usize];
            let mut e = 0i32;
            while den % p == 0 {
                den /= p;
                e += 1;
            }

            let old_pow = power_mod(p as u64, exps[p as usize] as u64);
            exps[p as usize] -= e;
            let new_pow = power_mod(p as u64, exps[p as usize] as u64);

            res_prod_pe = res_prod_pe % MOD * mod_inv(old_pow) % MOD * new_pow % MOD;
            let old_term = (1 + old_pow) % MOD;
            let new_term = if new_pow == 1 { 1 } else { (1 + new_pow) % MOD };
            res_prod_1pe = res_prod_1pe % MOD * mod_inv(old_term) % MOD * new_term % MOD;
        }

        let r_val = (res_prod_1pe + MOD - res_prod_pe) % MOD;

        if k == NN / 2 {
            ans = (ans + r_val) % MOD;
        } else {
            ans = (ans + 2 * r_val) % MOD;
        }
    }

    println!("{}", ans);
}
