// Project Euler 969 - Kangaroo Hops
// Sum_{n=1}^{10^18} S(n) mod 10^9+7
// Uses Stirling numbers and primorial factorization

const MOD: u64 = 1_000_000_007;

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn main() {
    let big_m: u64 = 1_000_000_000_000_000_000; // 10^18
    let maxj = 60usize;
    let primes = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59];

    // Stirling numbers of the second kind
    let mut stir = vec![vec![0u64; maxj + 1]; maxj + 1];
    stir[0][0] = 1;
    for j in 1..=maxj {
        for i in 1..=j {
            stir[j][i] = ((i as u128 * stir[j - 1][i] as u128 + stir[j - 1][i - 1] as u128) % MOD as u128) as u64;
        }
    }

    let mut fact = vec![0u64; maxj + 1];
    fact[0] = 1;
    for i in 1..=maxj {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }

    let mut total = 0u64;
    for j in 0..=maxj {
        // Compute primorial P = product of primes <= j
        let mut p_val: u64 = 1;
        let mut overflow = false;
        for &pr in &primes {
            if pr > j as u64 { break; }
            if p_val > (big_m - j as u64 + 1) / pr { overflow = true; break; }
            p_val *= pr;
        }
        if overflow { continue; }
        if p_val > big_m - j as u64 + 1 { continue; }

        let k = (big_m - j as u64) / p_val;
        if k == 0 { continue; }

        let s = if j == 0 {
            k % MOD
        } else {
            let mut s_val = 0u64;
            for i in 1..=j {
                let mut falling = 1u64;
                for t in 0..=i as u64 {
                    let term = ((k + 1 - t) % MOD + MOD) % MOD;
                    falling = (falling as u128 * term as u128 % MOD as u128) as u64;
                }
                let mut term = (stir[j][i] as u128 * falling as u128 % MOD as u128) as u64;
                let inv = mod_pow((i + 1) as u64, MOD - 2, MOD);
                term = (term as u128 * inv as u128 % MOD as u128) as u64;
                s_val = (s_val + term) % MOD;
            }
            s_val
        };

        let signs = if j % 2 == 0 { 1u64 } else { MOD - 1 };
        let p_mod = p_val % MOD;
        let pj = mod_pow(p_mod, j as u64, MOD);
        let inv_fact = mod_pow(fact[j], MOD - 2, MOD);

        let mut contrib = (signs as u128 * pj as u128 % MOD as u128) as u64;
        contrib = (contrib as u128 * s as u128 % MOD as u128) as u64;
        contrib = (contrib as u128 * inv_fact as u128 % MOD as u128) as u64;
        total = (total + contrib) % MOD;
    }

    println!("{}", total);
}
