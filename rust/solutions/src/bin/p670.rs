// Project Euler 670 - Coloured Tiles
// Matrix exponentiation with 10 states. N=10^16, K=4, T=3, M=1000004321.

const MOD: i64 = 1_000_004_321;
const SZ: usize = 10;
const T: usize = 3;
const K: i64 = 4;

type Mat = [[i64; SZ]; SZ];

fn mat_mult(a: &Mat, b: &Mat) -> Mat {
    let mut result = [[0i64; SZ]; SZ];
    for i in 0..SZ {
        for k in 0..SZ {
            if a[i][k] == 0 { continue; }
            for j in 0..SZ {
                result[i][j] = (result[i][j] + a[i][k] as i128 * b[k][j] as i128 % MOD as i128) as i64;
            }
        }
    }
    result
}

fn mat_pow(mat: &Mat, mut exp: i64) -> Mat {
    let mut result = [[0i64; SZ]; SZ];
    for i in 0..SZ { result[i][i] = 1; }
    let mut base = *mat;
    while exp > 0 {
        if exp & 1 == 1 { result = mat_mult(&result, &base); }
        base = mat_mult(&base, &base);
        exp >>= 1;
    }
    result
}

fn point_idx(i: usize, j: usize) -> usize { 1 + i * T + j }

fn main() {
    let n: i64 = 10_000_000_000_000_000;
    let mut a = [[0i64; SZ]; SZ];
    a[0][0] = K - 1;
    a[0][point_idx(0, 0)] = K - 2;
    for i in 0..T {
        for j in 0..T {
            a[point_idx(i, j)][0] = (K - 1) * (K - 2);
        }
    }
    for i in 1..T {
        for j in 0..T { a[point_idx(i - 1, j)][point_idx(i, 0)] = K - 2; }
    }
    for i in 0..T {
        for j in 1..T { a[point_idx(i, j - 1)][point_idx(0, j)] = K - 2; }
    }
    for i in 1..T {
        for j in 1..T { a[point_idx(i - 1, j - 1)][point_idx(i, j)] = 1; }
    }
    let ae = mat_pow(&a, n - 1);
    let mut ans = 0i64;
    for t in 0..2 {
        ans = (ans + K * ae[t][0]) % MOD;
        for i in 0..T {
            for j in 0..T {
                ans = (ans + K * (K - 1) % MOD * ae[t][point_idx(i, j)]) % MOD;
            }
        }
    }
    println!("{}", ans);
}
