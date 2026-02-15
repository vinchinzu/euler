// Project Euler 427: n-sequences
// Sum of L(S) over all sequences of length N with values 1..N.

use euler_utils::mod_pow;

const N: usize = 7_500_000;
const MOD: u64 = 1_000_000_009;

fn main() {
    let mut fact = vec![1u64; N + 1];
    for i in 1..=N { fact[i] = fact[i - 1] * i as u64 % MOD; }

    let mut inv_fact = vec![1u64; N + 1];
    inv_fact[N] = mod_pow(fact[N], MOD - 2, MOD);
    for i in (0..N).rev() { inv_fact[i] = inv_fact[i + 1] * (i + 1) as u64 % MOD; }

    let mut pow_n = vec![1u64; N + 2];
    for i in 1..=N + 1 { pow_n[i] = pow_n[i - 1] * N as u64 % MOD; }

    let mut pow_nm1 = vec![1u64; N + 2];
    for i in 1..=N + 1 { pow_nm1[i] = pow_nm1[i - 1] * (N as u64 - 1) % MOD; }

    let ncr = |n: usize, r: usize| -> u64 {
        if r > n { return 0; }
        fact[n] % MOD * inv_fact[r] % MOD * inv_fact[n - r] % MOD
    };

    let mut ans: u64 = 0;
    let mut prev_f: u64 = 0;

    for k in 1..=N {
        let mut fk: u64 = 0;
        let mut i = 0usize;
        loop {
            if i * (k + 1) > N { break; }
            let a_val = N as i64 - (i * k) as i64 - 1;
            if a_val < 0 { break; }
            let a = a_val as usize;

            let mut term: u64 = 0;
            {
                let exp1 = if a + 1 >= i { a - i + 1 } else { 0 };
                let t = ncr(a, i) * pow_nm1[i] % MOD * pow_n[exp1] % MOD;
                term = (term + t) % MOD;
            }
            if i >= 1 {
                let exp2 = if a + 2 >= i { a - i + 2 } else { 0 };
                let t = ncr(a, i - 1) * pow_nm1[i - 1] % MOD * pow_n[exp2] % MOD;
                term = (term + t) % MOD;
            }

            if i % 2 == 0 {
                fk = (fk + term) % MOD;
            } else {
                fk = (fk + MOD - term) % MOD;
            }
            i += 1;
        }

        let delta = (fk + MOD - prev_f) % MOD;
        ans = (ans + delta % MOD * (k as u64)) % MOD;
        prev_f = fk;
    }

    println!("{ans}");
}
