// Project Euler 624 - Two heads are better than one
// Matrix exponentiation for D_n recurrence, then modular fraction

const MOD: i64 = 1_000_000_009;

fn md(a: i64) -> i64 { ((a % MOD) + MOD) % MOD }

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn modinv(a: i64, m: i64) -> i64 { powmod(a, m - 2, m) }

type Mat = [[i64; 2]; 2];

fn matmul(a: &Mat, b: &Mat) -> Mat {
    let mut c = [[0i64; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            let mut s = 0i128;
            for k in 0..2 { s += a[i][k] as i128 * b[k][j] as i128; }
            c[i][j] = md((s % MOD as i128) as i64);
        }
    }
    c
}

fn matpow(a: &Mat, mut exp: i64) -> Mat {
    let mut result: Mat = [[1, 0], [0, 1]];
    let mut base = *a;
    while exp > 0 {
        if exp & 1 == 1 { result = matmul(&result, &base); }
        base = matmul(&base, &base);
        exp >>= 1;
    }
    result
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;

    let a_mat: Mat = [[md(-2), 4], [1, 0]];
    let an1 = matpow(&a_mat, n - 1);
    let d1 = an1[0][0]; // D_{N-1}
    let d2 = an1[1][0]; // D_{N-2}

    let parity_n = if n % 2 == 0 { 1i64 } else { MOD - 1 };

    let a = md((parity_n as i128 * md(4 * d2 % MOD - 1 + MOD) as i128 % MOD as i128) as i64);
    let inner = md(2 * d1 % MOD - 8 * d2 % MOD + 1 + 3 * MOD);
    let b_val = md(powmod(4, n, MOD) + (parity_n as i128 * inner as i128 % MOD as i128) as i64);

    let ans = md((a as i128 * modinv(b_val, MOD) as i128 % MOD as i128) as i64);
    println!("{}", ans);
}
