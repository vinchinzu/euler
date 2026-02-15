const M: i64 = 1_000_000_007;

fn mat_mult(a: &[[i64; 2]; 2], b: &[[i64; 2]; 2], modulus: i64) -> [[i64; 2]; 2] {
    let mut r = [[0i64; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                r[i][j] = (r[i][j] + (a[i][k] as i128 * b[k][j] as i128 % modulus as i128) as i64) % modulus;
            }
        }
    }
    r
}

fn mat_pow(mut m: [[i64; 2]; 2], mut exp: i64, modulus: i64) -> [[i64; 2]; 2] {
    let mut result = [[1i64, 0], [0, 1]];
    while exp > 0 {
        if exp & 1 != 0 { result = mat_mult(&result, &m, modulus); }
        m = mat_mult(&m, &m, modulus);
        exp >>= 1;
    }
    result
}

fn fibonacci(n: i64, modulus: i64) -> i64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let fib_mat = [[1i64, 1], [1, 0]];
    let r = mat_pow(fib_mat, n - 1, modulus);
    r[0][0]
}

fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % modulus) + modulus) % modulus;
    while exp > 0 {
        if exp & 1 != 0 { result = (result as i128 * base as i128 % modulus as i128) as i64; }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let mut n: i64 = 1;
    for _ in 0..14 { n *= 11; }

    let f_n1 = fibonacci(n - 1, M - 1);
    let f_n2 = fibonacci(n - 2, M - 1);

    let a = (M - (pow_mod(-3, 2 * f_n1 - 1, M) + M - pow_mod(2, 4 * f_n2 + 2 * f_n1 - 2, M)) % M) % M;
    let a = (a + M) % M;

    let b = ((M - pow_mod(2, 2 * f_n2 + f_n1 - 2, M)) as i128 * pow_mod(-3, f_n1 - 1, M) as i128 % M as i128) as i64;
    let b = ((b % M) + M) % M;

    let c = (pow_mod(-3, 2 * f_n1 + 1, M) + pow_mod(2, 4 * f_n2 + 2 * f_n1 + 2, M)) % M;

    let d = (pow_mod(2, 2 * f_n2 + f_n1, M) as i128 * pow_mod(-3, f_n1, M) as i128 % M as i128) as i64;

    let ans = ((a + b + c + d) % M + M) % M;
    println!("{}", ans);
}
