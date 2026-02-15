// Project Euler 458: Permutations of Project
const K: usize = 7;
const MOD: i64 = 1_000_000_000;

type Matrix = [[i64; K]; K];

fn mat_mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = [[0i64; K]; K];
    for i in 0..K {
        for k in 0..K {
            if a[i][k] == 0 { continue; }
            for j in 0..K {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    c
}

fn mat_pow(mut base: Matrix, mut exp: i64) -> Matrix {
    let mut result = [[0i64; K]; K];
    for i in 0..K { result[i][i] = 1; }
    while exp > 0 {
        if exp & 1 != 0 { result = mat_mul(&result, &base); }
        base = mat_mul(&base, &base);
        exp >>= 1;
    }
    result
}

fn main() {
    let big_n: i64 = 1_000_000_000_000;
    let mut a = [[0i64; K]; K];
    for n in 1..K {
        a[n][n - 1] = (K - n + 1) as i64;
        for i in n..K {
            a[n][i] = 1;
        }
    }

    let an = mat_pow(a, big_n);
    let mut ans: i64 = 0;
    for i in 0..K {
        ans = (ans + an[i][0]) % MOD;
    }
    println!("{ans}");
}
