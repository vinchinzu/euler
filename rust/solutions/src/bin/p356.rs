// Project Euler 356: Largest Roots of Cubic Polynomials
use euler_utils::mod_mul;

const MOD: u64 = 100_000_000;
const K: u64 = 987_654_321;

type Mat = [[u64; 3]; 3];

fn mat_mul(a: &Mat, b: &Mat) -> Mat {
    let mut c = [[0u64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut s = 0u128;
            for k in 0..3 {
                s += a[i][k] as u128 * b[k][j] as u128;
            }
            c[i][j] = (s % MOD as u128) as u64;
        }
    }
    c
}

fn mat_pow(mut m: Mat, mut p: u64) -> Mat {
    let mut result: Mat = [[0; 3]; 3];
    for i in 0..3 { result[i][i] = 1; }

    while p > 0 {
        if p & 1 == 1 { result = mat_mul(&result, &m); }
        m = mat_mul(&m, &m);
        p >>= 1;
    }
    result
}

fn mod_pow_ll(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 { result = mod_mul(result, base, MOD); }
        base = mod_mul(base, base, MOD);
        exp >>= 1;
    }
    result
}

fn compute_sk(i: u32) -> u64 {
    let p = mod_pow_ll(2, i as u64);
    let neg_i = ((-(i as i64) % MOD as i64) + MOD as i64) as u64;

    let s0 = 3 % MOD;
    let s1 = p;
    let s2 = mod_mul(p, p, MOD);

    if K == 0 { return s0; }
    if K == 1 { return s1; }
    if K == 2 { return s2; }

    let m: Mat = [
        [p,  0, neg_i],
        [1,  0, 0],
        [0,  1, 0],
    ];

    let mpow = mat_pow(m, K - 2);

    let sk = ((mpow[0][0] as u128 * s2 as u128
             + mpow[0][1] as u128 * s1 as u128
             + mpow[0][2] as u128 * s0 as u128) % MOD as u128) as u64;
    sk
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
