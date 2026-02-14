// Project Euler 435: Polynomials of Fibonacci numbers
// F_n(x) = sum_{i=0}^n f_i * x^i. Find sum_{x=0}^{100} F_n(x) mod 15!
// where n = 10^15. Uses 3x3 matrix exponentiation + CRT.

type Mat = [[i64; 3]; 3];

fn mat_mult(a: &Mat, b: &Mat, m: i64) -> Mat {
    let mut r = [[0i64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut s: i128 = 0;
            for k in 0..3 {
                s += a[i][k] as i128 * b[k][j] as i128;
            }
            r[i][j] = (s % m as i128) as i64;
        }
    }
    r
}

fn mat_pow(mut a: Mat, mut n: i64, m: i64) -> Mat {
    let mut result: Mat = [[0; 3]; 3];
    result[0][0] = 1; result[1][1] = 1; result[2][2] = 1; // identity
    while n > 0 {
        if n & 1 == 1 { result = mat_mult(&result, &a, m); }
        a = mat_mult(&a, &a, m);
        n >>= 1;
    }
    result
}

fn f_n(x: i64, n_val: i64, m: i64) -> i64 {
    if x == 0 { return 0; }
    let xm = x % m;
    let x2m = ((xm as i128 * xm as i128) % m as i128) as i64;
    let a: Mat = [
        [1, 0, 0],
        [0, 0, 1],
        [xm, x2m, xm],
    ];
    let an = mat_pow(a, n_val, m);
    let result = (an[1][0] as i128 + xm as i128 * an[1][2] as i128) % m as i128;
    ((result + m as i128) % m as i128) as i64
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x1, y1) = extended_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn crt(remainders: &[i64], moduli: &[i64]) -> i64 {
    let n = remainders.len();
    let mut m_total: i64 = 1;
    for i in 0..n { m_total *= moduli[i]; }

    let mut result: i128 = 0;
    for i in 0..n {
        let mi = m_total / moduli[i];
        let (_, inv, _) = extended_gcd(mi % moduli[i], moduli[i]);
        let inv = ((inv % moduli[i]) + moduli[i]) % moduli[i];
        result = (result + (remainders[i] as i128 * mi as i128 % m_total as i128) * inv as i128) % m_total as i128;
    }
    ((result % m_total as i128 + m_total as i128) % m_total as i128) as i64
}

fn main() {
    let n_val: i64 = 1_000_000_000_000_000; // 10^15
    let k = 100;
    let m: i64 = 1_307_674_368_000; // 15!

    // 15! = 2^11 * 3^6 * 5^3 * 7^2 * 11 * 13
    let prime_powers: [i64; 6] = [2048, 729, 125, 49, 11, 13];

    let mut ans: i64 = 0;
    for x in 0..=k {
        let remainders: Vec<i64> = prime_powers.iter()
            .map(|&pp| f_n(x, n_val, pp))
            .collect();
        let fx = crt(&remainders, &prime_powers.to_vec());
        ans = (ans + fx) % m;
    }

    println!("{}", ans);
}
