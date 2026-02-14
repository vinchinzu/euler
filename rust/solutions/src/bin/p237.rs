// Project Euler 237: Tours on a 4 x n playing board
// T(n) = 2*T(n-1) + 2*T(n-2) - 2*T(n-3) + T(n-4), matrix exponentiation.

const MOD: u64 = 100_000_000;
const SZ: usize = 4;

type Mat = [[u64; SZ]; SZ];

fn mat_mult(a: &Mat, b: &Mat) -> Mat {
    let mut c = [[0u64; SZ]; SZ];
    for i in 0..SZ {
        for j in 0..SZ {
            let mut s: u64 = 0;
            for k in 0..SZ {
                s = (s + a[i][k] * b[k][j]) % MOD;
            }
            c[i][j] = s;
        }
    }
    c
}

fn mat_pow(mut base: Mat, mut exp: u64) -> Mat {
    let mut result = [[0u64; SZ]; SZ];
    for i in 0..SZ {
        result[i][i] = 1;
    }
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mult(&result, &base);
        }
        base = mat_mult(&base, &base);
        exp >>= 1;
    }
    result
}

fn main() {
    let n: u64 = 1_000_000_000_000;
    let t = [0u64, 1, 1, 4, 8];
    // coeffs: 2, 2, -2, 1
    let mut m: Mat = [[0; SZ]; SZ];
    m[0][0] = 2;
    m[0][1] = 2;
    m[0][2] = MOD - 2; // -2 mod MOD
    m[0][3] = 1;
    for i in 1..SZ {
        m[i][i - 1] = 1;
    }

    let init = [t[4], t[3], t[2], t[1]];
    let mp = mat_pow(m, n - SZ as u64);

    let mut result: u64 = 0;
    for j in 0..SZ {
        result = (result + mp[0][j] * (init[j] % MOD)) % MOD;
    }

    println!("{}", result);
}
