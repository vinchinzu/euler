// Project Euler 835 - Supernatural Triangles
// Two cases: consecutive leg-hypotenuse and consecutive legs (Pell)

const M: i64 = 1234567891;

fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

fn mat_mul(a: &[[i64; 3]; 3], b: &[[i64; 3]; 3]) -> [[i64; 3]; 3] {
    let mut c = [[0i64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut s: i128 = 0;
            for k in 0..3 {
                s += a[i][k] as i128 * b[k][j] as i128;
            }
            c[i][j] = (s % M as i128 + M as i128) as i64 % M;
        }
    }
    c
}

fn mat_pow(base: &[[i64; 3]; 3], mut exp: i64) -> [[i64; 3]; 3] {
    let mut result = [[0i64; 3]; 3];
    for i in 0..3 { result[i][i] = 1; }
    let mut b = *base;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mul(&result, &b);
        }
        b = mat_mul(&b, &b);
        exp >>= 1;
    }
    result
}

fn main() {
    let n: i64 = 10_000_000_000;
    let b: i64 = 10;

    let inv2 = pow_mod(2, M - 2, M);
    let inv6 = pow_mod(6, M - 2, M);

    // First case: limit1 = 10^(N/2)/2 - 1
    let limit1 = (pow_mod(b, n / 2, M) as i128 * inv2 as i128 % M as i128 - 1 + M as i128) as i64 % M;

    let nn = limit1;
    let sum_t = (nn as i128 * ((nn + 1) % M) as i128 % M as i128 * inv2 as i128 % M as i128) as i64;
    let sum_t2 = (nn as i128 * ((nn + 1) % M) as i128 % M as i128
        * ((2 * nn + 1) % M) as i128 % M as i128
        * inv6 as i128 % M as i128) as i64;

    let mut ans = (4i128 * sum_t2 as i128 % M as i128
        + 6i128 * sum_t as i128 % M as i128
        + 2i128 * nn as i128 % M as i128) as i64 % M;

    // Second case: Pell equation
    let sqrt2: f64 = 2.0_f64.sqrt();
    let log_base = (3.0 + 2.0 * sqrt2).ln();
    let limit2 = ((n as f64 * 10.0_f64.ln() + (2.0 * sqrt2).ln()) / log_base) as i64;

    let s_limit2 = if limit2 == 0 {
        0
    } else if limit2 == 1 {
        2
    } else if limit2 == 2 {
        14
    } else {
        let rec_mat: [[i64; 3]; 3] = [
            [((7i64 % M) + M) % M, ((-7i64 % M) + M) % M, 1],
            [1, 0, 0],
            [0, 1, 0],
        ];
        let result = mat_pow(&rec_mat, limit2 - 2);
        (result[0][0] as i128 * 14 % M as i128
            + result[0][1] as i128 * 2 % M as i128) as i64 % M
    };

    ans = ((ans as i128 + s_limit2 as i128 - 14 + M as i128) % M as i128) as i64;
    println!("{}", ans);
}
