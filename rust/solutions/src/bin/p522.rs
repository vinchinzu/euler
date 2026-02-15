// Project Euler 522 - Hilbert's Hotel
//
// Combinatorial counting with modular arithmetic.
// Compute factorials and inverse factorials mod MOD, then sum terms.

const NN: u64 = 12_344_321;
const MOD: u64 = 135_707_531;

#[inline(always)]
fn mod_pow_local(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

fn main() {
    let nn = NN as usize;
    let mut fact = vec![0u64; nn + 1];
    let mut inv_fact = vec![0u64; nn + 1];

    fact[0] = 1;
    for i in 1..=nn {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }

    inv_fact[nn] = mod_pow_local(fact[nn], MOD - 2);
    for i in (1..=nn).rev() {
        inv_fact[i - 1] = inv_fact[i] * (i as u64) % MOD;
    }

    let mut ans = NN * (NN - 1) % MOD;
    ans = ans * mod_pow_local(NN - 2, NN - 1) % MOD;

    for l in 2..NN {
        let lu = l as usize;
        let ncr = fact[nn] * inv_fact[lu] % MOD * inv_fact[nn - lu] % MOD;
        let term = ncr * fact[lu - 1] % MOD
            * mod_pow_local(NN - l - 1, NN - l) % MOD;
        ans += term;
        if ans >= MOD { ans -= MOD; }
    }

    println!("{}", ans % MOD);
}
