// Project Euler 857 - Beautiful Graphs
// G(n) via sliding window recurrence with A = {0,1,2,6,18,12}
//
// Note: This implementation is already near-optimal. Attempted optimizations
// (u128 deferred reduction, unsafe get_unchecked, copy_within, pre-computed values)
// showed no improvement or regressions. The tight loop with modular arithmetic
// is compiler-friendly and bottlenecked by ~100M unavoidable mod operations.
// See p857_optimization_report.md for details.

const MOD: u64 = 1_000_000_007;
const A_VALS: [u64; 6] = [0, 1, 2, 6, 18, 12];

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result: u64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let n: u64 = 10_000_000;

    let mut inv_fact = [0u64; 6];
    inv_fact[0] = 1;
    let mut fact: u64 = 1;
    for i in 1..=5 {
        fact = fact * i as u64 % MOD;
        inv_fact[i] = mod_pow(fact, MOD - 2, MOD);
    }

    let mut a_invf = [0u64; 6];
    for k in 1..=5 {
        a_invf[k] = A_VALS[k] * inv_fact[k] % MOD;
    }

    // prev[k-1] = G(i-k) while computing G(i); start with G(0)=1
    let mut prev = [0u64; 5];
    prev[0] = 1;

    let a1 = a_invf[1];
    let a2 = a_invf[2];
    let a3 = a_invf[3];
    let a4 = a_invf[4];
    let a5 = a_invf[5];

    // i < 5: product includes a 0 factor for k > i
    for i in 1..5u64 {
        let mut val: u64 = 0;
        let mut n_prod: u64 = 1;
        for k in 1..=i as usize {
            n_prod = n_prod * (i - k as u64 + 1) % MOD;
            val += n_prod * a_invf[k] % MOD * prev[k - 1];
        }
        val %= MOD;
        prev[4] = prev[3];
        prev[3] = prev[2];
        prev[2] = prev[1];
        prev[1] = prev[0];
        prev[0] = val;
    }

    for i in 5..=n {
        let mut n_prod = i;
        let mut val = n_prod * a1 % MOD * prev[0];
        n_prod = n_prod * (i - 1) % MOD;
        val += n_prod * a2 % MOD * prev[1];
        n_prod = n_prod * (i - 2) % MOD;
        val += n_prod * a3 % MOD * prev[2];
        n_prod = n_prod * (i - 3) % MOD;
        val += n_prod * a4 % MOD * prev[3];
        n_prod = n_prod * (i - 4) % MOD;
        val += n_prod * a5 % MOD * prev[4];
        val %= MOD;
        prev[4] = prev[3];
        prev[3] = prev[2];
        prev[2] = prev[1];
        prev[1] = prev[0];
        prev[0] = val;
    }

    println!("{}", prev[0]);
}
