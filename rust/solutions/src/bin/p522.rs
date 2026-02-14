// Project Euler 522 - Hilbert's Hotel
//
// Combinatorial counting with modular arithmetic.
// Compute factorials and inverse factorials mod MOD, then sum terms.

use euler_utils::mod_pow;

const NN: usize = 12_344_321;
const MOD: u64 = 135_707_531;

fn main() {
    let mut fact = vec![0u64; NN + 1];
    let mut inv_fact = vec![0u64; NN + 1];

    fact[0] = 1;
    for i in 1..=NN {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }

    inv_fact[NN] = mod_pow(fact[NN], MOD - 2, MOD);
    for i in (1..=NN).rev() {
        inv_fact[i - 1] = inv_fact[i] * (i as u64) % MOD;
    }

    let n = NN as u64;
    let mut ans = n * (n - 1) % MOD;
    ans = ans * mod_pow(n - 2, n - 1, MOD) % MOD;

    for l in 2..NN as u64 {
        let ncr = fact[NN] * inv_fact[l as usize] % MOD * inv_fact[(NN - l as usize)] % MOD;
        let term = ncr * fact[(l - 1) as usize] % MOD
            * mod_pow(n - l - 1, n - l, MOD) % MOD;
        ans += term;
        if ans >= MOD { ans -= MOD; }
    }

    println!("{}", ans % MOD);
}
