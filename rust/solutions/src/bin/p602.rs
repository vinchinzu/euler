// Project Euler 602 - Product of Head Counts
// Inclusion-exclusion with modular arithmetic

const MOD: u64 = 1_000_000_007;
const NN: u64 = 10_000_000;
const K: usize = 4_000_000;

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
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
    let mut inv = vec![0u64; K + 1];
    inv[1] = 1;
    for i in 2..=K {
        inv[i] = (MOD - (MOD / i as u64) % MOD * inv[(MOD % i as u64) as usize] % MOD) % MOD;
    }

    let mut ncr = vec![0u64; K + 1];
    ncr[0] = 1;
    for i in 1..=K {
        let top = ((NN + 2 - i as u64) % MOD + MOD) % MOD;
        ncr[i] = (ncr[i - 1] as u128 * top as u128 % MOD as u128) as u64;
        ncr[i] = (ncr[i] as u128 * inv[i] as u128 % MOD as u128) as u64;
    }

    let mut pows = vec![0u64; K + 1];
    for t in 1..=K {
        pows[t] = pow_mod(t as u64, NN, MOD);
    }

    let mut ans = 0u64;
    for t in 0..=K {
        let sign = if (K - t) & 1 == 1 { MOD - 1 } else { 1 };
        let coeff = (ncr[K - t] as u128 * sign as u128 % MOD as u128) as u64;
        let coeff = (coeff as u128 * pows[t] as u128 % MOD as u128) as u64;
        ans = (ans + coeff) % MOD;
    }

    println!("{}", ans);
}
