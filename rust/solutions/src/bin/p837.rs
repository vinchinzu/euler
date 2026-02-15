// Project Euler 837 - Amidakuji

const A: i64 = 123456789;
const B: i64 = 987654321;
const MOD: i64 = 1234567891;

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i128;
    let mut b = (base % m) as i128;
    let m128 = m as i128;
    while exp > 0 {
        if exp & 1 == 1 { result = result * b % m128; }
        b = b * b % m128;
        exp >>= 1;
    }
    result as i64
}

fn mod_inv(a: i64, m: i64) -> i64 { mod_pow(a, m - 2, m) }

fn main() {
    // Precompute modular inverses 1..A
    let mut mod_invs = vec![0i64; A as usize + 1];
    mod_invs[1] = 1;
    for i in 2..=A as usize {
        mod_invs[i] = (MOD - (MOD / i as i64) * mod_invs[(MOD % i as i64) as usize] % MOD) % MOD;
    }

    // Compute factorial((A-1)/2) and factorial((B-1)/2)
    let mut fact_a = 1i64;
    for i in 2..=(A - 1) / 2 {
        fact_a = (fact_a as i128 * i as i128 % MOD as i128) as i64;
    }
    let mut fact_b = 1i64;
    for i in 2..=(B - 1) / 2 {
        fact_b = (fact_b as i128 * i as i128 % MOD as i128) as i64;
    }

    let mut term1 = mod_inv((fact_a as i128 * fact_b as i128 % MOD as i128) as i64, MOD);
    let mut term2 = 0i64;
    let mut ans = 0i64;

    let mut t = 3i64;
    while t <= A {
        term1 = (term1 as i128 * mod_invs[(t - 1) as usize] as i128 % MOD as i128) as i64;
        term1 = (term1 as i128 * mod_invs[t as usize] as i128 % MOD as i128) as i64;
        term1 = (term1 as i128 * (((A - t + 2) / 2) % MOD) as i128 % MOD as i128) as i64;
        term1 = (term1 as i128 * (((B - t + 2) / 2) % MOD) as i128 % MOD as i128) as i64;

        term2 = (4 * term2 + 2) % MOD;

        ans = (ans + (term1 as i128 * term2 as i128 % MOD as i128) as i64) % MOD;

        t += 2;
    }

    // Multiply by ((A+B)/2)!
    let mut fact_total = 1i64;
    for i in 2..=(A + B) / 2 {
        fact_total = (fact_total as i128 * (i % MOD) as i128 % MOD as i128) as i64;
    }

    ans = (ans as i128 * fact_total as i128 % MOD as i128) as i64;
    println!("{}", ans);
}
