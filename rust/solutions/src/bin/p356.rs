// Project Euler 356: Largest Roots of Cubic Polynomials
use euler_utils::{mod_mul, mod_pow, ModMatrix};

const MOD: u64 = 100_000_000;
const K: u64 = 987_654_321;

fn compute_sk(i: u32) -> u64 {
    let p = mod_pow(2, i as u64, MOD);
    let neg_i = ((-(i as i64) % MOD as i64) + MOD as i64) as u64;

    let s0 = 3 % MOD;
    let s1 = p;
    let s2 = mod_mul(p, p, MOD);

    if K == 0 { return s0; }
    if K == 1 { return s1; }
    if K == 2 { return s2; }

    let m = ModMatrix::<3>::from_data([
        [p,  0, neg_i],
        [1,  0, 0],
        [0,  1, 0],
    ], MOD);

    let mpow = m.pow(K - 2);
    let v = [s2, s1, s0];
    let result = mpow.mul_vec(&v);
    result[0]
}

fn main() {
    let mut total = 0u64;
    for i in 1..=30u32 {
        let sk = compute_sk(i);
        let floor_ak = (sk + MOD - 1) % MOD;
        total = (total + floor_ak) % MOD;
    }
    println!("{}", total);
}
