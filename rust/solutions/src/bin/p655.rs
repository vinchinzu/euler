// Project Euler 655 - Divisible Palindromes
// Count palindromes up to 32 digits divisible by K=10^7+19.

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn num_palindromes(num_digits: usize, k: i64) -> i64 {
    let ku = k as usize;
    let mut dp = vec![0i64; ku];
    let mut new_dp = vec![0i64; ku];
    dp[0] = 1;
    let half = (num_digits + 1) / 2;
    for i in 0..half {
        let mult = if 2 * i + 1 == num_digits {
            pow_mod(10, i as i64, k)
        } else {
            (pow_mod(10, i as i64, k) + pow_mod(10, (num_digits - 1 - i) as i64, k)) % k
        };
        new_dp.iter_mut().for_each(|v| *v = 0);
        for d in 0..10i64 {
            let shift = (mult * d % k) as usize;
            for j in 0..ku {
                if dp[j] != 0 {
                    let nj = (j + shift) % ku;
                    new_dp[nj] += dp[j];
                }
            }
        }
        std::mem::swap(&mut dp, &mut new_dp);
    }
    dp[0] - 1
}

fn main() {
    let k: i64 = 10_000_019;
    let nn = 32;
    let ans = num_palindromes(nn - 1, k) + num_palindromes(nn, k);
    println!("{}", ans);
}
