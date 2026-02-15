// Project Euler 364: Comfortable Distance
use euler_utils::mod_pow;

const MOD: u64 = 100_000_007;

fn main() {
    let n = 1_000_000usize;
    let n_max = n + 2;

    let mut fact = vec![0u64; n_max + 1];
    let mut inv_fact = vec![0u64; n_max + 1];
    fact[0] = 1;
    for i in 1..=n_max {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }
    inv_fact[n_max] = mod_pow(fact[n_max], MOD - 2, MOD);
    for i in (0..n_max).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) as u64 % MOD;
    }

    let c_mod = |n: usize, k: usize| -> u64 {
        if k > n { return 0; }
        fact[n] * inv_fact[k] % MOD * inv_fact[n - k] % MOD
    };

    let mut r = 0u64;
    for v in 0..3 {
        let rem = n as i64 - 1 - v;
        if rem < 0 { continue; }
        let rem = rem as usize;

        let m_start = match rem % 3 {
            0 => 0,
            1 => 2,
            _ => 1,
        };

        let mut m = m_start;
        while 2 * m <= rem {
            let k = (rem - 2 * m) / 3;
            let mk = m + k;

            let mut term = fact[mk + 1];
            term = term * c_mod(mk, m) % MOD;
            term = term * mod_pow(2, k as u64, MOD) % MOD;
            term = term * fact[k + v as usize] % MOD;
            term = term * fact[mk] % MOD;
            if v == 1 { term = term * 2 % MOD; }

            r = (r + term) % MOD;
            m += 3;
        }
    }

    println!("{}", r);
}
