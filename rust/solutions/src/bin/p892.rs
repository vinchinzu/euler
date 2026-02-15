// Project Euler 892
// Modular inverse sieve with central binomial coefficients.

const MOD: i64 = 1_234_567_891;

fn main() {
    let n = 10_000_000;
    let limit = n / 2 + 2;

    let mut inv = vec![0i64; limit + 1];
    inv[1] = 1;
    for i in 2..=limit {
        inv[i] = (MOD - MOD / i as i64) * inv[(MOD % i as i64) as usize] % MOD;
    }

    let m = n / 2;
    let mut comb = 1i64;
    let mut total_sum = 0i64;
    let inv2 = inv[2];

    for mi in 1..=m {
        comb = comb * 2 % MOD * (2 * mi as i64 - 1) % MOD * inv[mi] % MOD;

        let tm = comb * comb % MOD;

        let term_even = tm * inv2 % MOD;
        total_sum = (total_sum + term_even) % MOD;

        if 2 * mi + 1 <= n {
            let term_odd = tm * 2 % MOD * mi as i64 % MOD * inv[mi + 1] % MOD;
            total_sum = (total_sum + term_odd) % MOD;
        }
    }

    println!("{}", total_sum);
}
