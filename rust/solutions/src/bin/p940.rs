// Project Euler 940 - Two-Dimensional Recurrence
// A(0,0)=0, A(0,1)=1
// A(m+1,n) = A(m,n+1) + A(m,n)
// A(m+1,n+1) = 2*A(m+1,n) + A(m,n)
// S(k) = sum_{i=2}^{k} sum_{j=2}^{k} A(f_i, f_j) mod 1123581313
// Uses matrix exponentiation for the n-advancement.

const MOD: i64 = 1123581313;

type Mat3 = [[i64; 3]; 3];
type Mat2 = [[i64; 2]; 2];

fn mat3_mult(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut c = [[0i64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut s: i128 = 0;
            for k in 0..3 {
                s += a[i][k] as i128 * b[k][j] as i128;
            }
            c[i][j] = (s % MOD as i128) as i64;
        }
    }
    c
}

fn mat3_pow(m: &Mat3, bits: &[i64]) -> Mat3 {
    let mut res: Mat3 = [[0; 3]; 3];
    for i in 0..3 {
        res[i][i] = 1;
    }
    // bits is LSB first, process from MSB to LSB
    for i in (0..bits.len()).rev() {
        res = mat3_mult(&res, &res);
        if bits[i] != 0 {
            res = mat3_mult(&res, m);
        }
    }
    res
}

fn mat2_mult(a: &Mat2, b: &Mat2) -> Mat2 {
    let mut c = [[0i64; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            let mut s: i128 = 0;
            for k in 0..2 {
                s += a[i][k] as i128 * b[k][j] as i128;
            }
            c[i][j] = (s % MOD as i128) as i64;
        }
    }
    c
}

fn mat2_pow(m: &Mat2, bits: &[i64]) -> Mat2 {
    let mut res: Mat2 = [[0; 2]; 2];
    for i in 0..2 {
        res[i][i] = 1;
    }
    for i in (0..bits.len()).rev() {
        res = mat2_mult(&res, &res);
        if bits[i] != 0 {
            res = mat2_mult(&res, m);
        }
    }
    res
}

fn to_binary(mut n: i64) -> Vec<i64> {
    if n == 0 {
        return vec![0];
    }
    let mut bits = Vec::new();
    while n > 0 {
        bits.push(n & 1);
        n >>= 1;
    }
    bits
}

fn main() {
    let k = 50;

    // Compute Fibonacci numbers
    let mut fib = vec![0i64; k + 1];
    fib[0] = 0;
    fib[1] = 1;
    for i in 2..=k {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    // a_mat for A(m,0) recurrence: a_m = 3*a_{m-1} + a_{m-2}
    let a_mat: Mat2 = [[3, 1], [1, 0]];

    // M for advancing n: state = [A(m,n), A(m,n+1), A(m+1,n)]
    // M = [[0, 1, 0], [1, -1, 2], [1, 0, 2]]
    let m_mat: Mat3 = [
        [0, 1, 0],
        [1, MOD - 1, 2],
        [1, 0, 2],
    ];

    // Precompute initials for each i=2..k
    let mut initials = vec![[0i64; 3]; k + 1];

    for ii in 2..=k {
        let mm = fib[ii];

        if mm <= 1 {
            if mm == 0 {
                initials[ii] = [0, 1, 1]; // A(0,0)=0, A(0,1)=1, A(1,0)=1
            } else {
                initials[ii] = [1, 2, 3]; // A(1,0)=1, A(1,1)=2, A(2,0)=3
            }
        } else {
            // Compute [a_mm, a_{mm-1}] = a_mat^(mm-1) * [1, 0]
            let bits = to_binary(mm - 1);
            let pow_a = mat2_pow(&a_mat, &bits);
            let a_m = (pow_a[0][0] * 1) % MOD;
            let a_mm1 = (pow_a[1][0] * 1) % MOD;
            let a_mp1 = (3 * a_m + a_mm1) % MOD;
            let b_m = (2 * a_m + a_mm1) % MOD;
            initials[ii] = [a_m, b_m, a_mp1];
        }
    }

    let mut total: i64 = 0;
    for i in 2..=k {
        for j in 2..=k {
            let n_val = fib[j];
            let res;
            if n_val == 0 {
                res = initials[i][0];
            } else if n_val == 1 {
                res = initials[i][1];
            } else {
                let bits = to_binary(n_val);
                let pow_m = mat3_pow(&m_mat, &bits);
                // Vn = pow_m * V0
                let mut r0: i128 = 0;
                for c in 0..3 {
                    r0 += pow_m[0][c] as i128 * initials[i][c] as i128;
                }
                res = (r0 % MOD as i128) as i64;
            }
            total = (total + res) % MOD;
        }
    }

    println!("{}", total);
}
