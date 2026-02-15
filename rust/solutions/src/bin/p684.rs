// Project Euler 684 - Inverse Digit Sum
// s(n) = smallest number with digit sum n, S(k) = sum s(1..k).
// Answer = sum S(fib(i)) for i=2..90, mod 10^9+7.

const MOD: i64 = 1_000_000_007;

fn power_mod(mut base: i64, mut exp: i64) -> i64 {
    let mut r = 1i64;
    base = ((base % MOD) + MOD) % MOD;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % MOD as i128) as i64; }
        base = (base as i128 * base as i128 % MOD as i128) as i64;
        exp >>= 1;
    }
    r
}

fn main() {
    let big_n = 90;

    let mut fib = vec![0i64; 91];
    fib[1] = 1; fib[2] = 1;
    for i in 3..=big_n {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    let mut ans = 0i64;
    for i in 2..=big_n {
        let n = fib[i];
        let r = n % 9;
        let q = n / 9;
        let coeff = (6 + r + r * (r + 1) / 2) % MOD;
        let pw = power_mod(10, q);
        let term = ((coeff as i128 * pw as i128 % MOD as i128) as i64
            - (6 + n % MOD) % MOD + MOD) % MOD;
        ans = (ans + term) % MOD;
    }

    println!("{}", ans);
}
