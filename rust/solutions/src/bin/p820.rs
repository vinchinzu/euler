// Project Euler 820 - Nth digit of reciprocal
// d_n(1/k) via 10^{N-1} mod k

fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 { return 0; }
    let mut result: i64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    let n: i64 = 10_000_000;

    let mut pows = vec![0i64; (n + 1) as usize];

    for k in (1..=n).rev() {
        if 2 * k <= n {
            pows[k as usize] = pows[(2 * k) as usize] % k;
        } else {
            pows[k as usize] = pow_mod(10, n - 1, k);
        }
    }

    let mut ans: i64 = 0;
    for k in 1..=n {
        ans += pows[k as usize] * 10 / k;
    }

    println!("{}", ans);
}
