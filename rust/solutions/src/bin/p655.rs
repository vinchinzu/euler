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

fn add_shifted(dp: &[i64], new_dp: &mut [i64], shift: usize) {
    let ku = dp.len();
    if shift == 0 {
        for j in 0..ku {
            new_dp[j] += dp[j];
        }
        return;
    }
    let n1 = ku - shift;
    for j in 0..n1 {
        new_dp[j + shift] += dp[j];
    }
    for j in n1..ku {
        new_dp[j + shift - ku] += dp[j];
    }
}

fn num_palindromes(num_digits: usize, k: i64) -> i64 {
    let ku = k as usize;
    let mut dp = vec![0i64; ku];
    let mut new_dp = vec![0i64; ku];
    dp[0] = 1;
    let half = (num_digits + 1) / 2;
    let inv10 = pow_mod(10, k - 2, k); // k is prime
    let mut lo = 1i64; // 10^i
    let mut hi = pow_mod(10, (num_digits - 1) as i64, k); // 10^{n-1-i}
    for i in 0..half {
        let mult = if 2 * i + 1 == num_digits {
            lo
        } else {
            (lo + hi) % k
        };
        new_dp.fill(0);
        for d in 0..10i64 {
            let shift = (mult * d % k) as usize;
            add_shifted(&dp, &mut new_dp, shift);
        }
        std::mem::swap(&mut dp, &mut new_dp);
        lo = lo * 10 % k;
        hi = hi * inv10 % k;
    }
    dp[0] - 1
}

fn main() {
    let k: i64 = 10_000_019;
    let nn = 32;
    let ans = num_palindromes(nn - 1, k) + num_palindromes(nn, k);
    println!("{}", ans);
}
