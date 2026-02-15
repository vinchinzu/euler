// Project Euler 857 - Beautiful Graphs
// G(n) via sliding window recurrence with A = {0,1,2,6,18,12}

const MOD: i64 = 1_000_000_007;
const A_VALS: [i64; 6] = [0, 1, 2, 6, 18, 12];

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
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
    let n: i64 = 10_000_000;

    // Precompute inverse factorials for k=1..5
    let mut inv_fact = [0i64; 6];
    inv_fact[0] = 1;
    let mut fact: i64 = 1;
    for i in 1..=5 {
        fact = fact * i as i64 % MOD;
        inv_fact[i] = mod_pow(fact, MOD - 2, MOD);
    }

    let mut a_invf = [0i64; 6];
    for k in 1..=5 {
        a_invf[k] = A_VALS[k] % MOD * inv_fact[k] % MOD;
    }

    let mut g_hist = [0i64; 5];
    g_hist[4] = 1; // G(0) = 1
    let mut head = 0usize;

    for i in 1..=n {
        let mut val: i64 = 0;
        let mut n_prod: i64 = 1;

        for k in 1..=5 {
            let prev_g = g_hist[(head + 5 - k) % 5];
            n_prod = n_prod % MOD * ((i - k as i64 + 1).rem_euclid(MOD)) % MOD;
            let term = n_prod % MOD * a_invf[k] % MOD * (prev_g % MOD) % MOD;
            val = (val + term) % MOD;
        }

        g_hist[head] = val;
        head = (head + 1) % 5;
    }

    let ans = g_hist[(head + 4) % 5];
    println!("{}", ans);
}
