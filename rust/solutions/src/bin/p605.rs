// Project Euler 605 - Pairwise Coin-Tossing Game
// P(K wins) = ((K-1)(2^N-1)+N) * 2^(N-K) / (2^N-1)^2
// Answer = numerator * denominator mod M

const M: u64 = 100_000_000;

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn main() {
    let n: u64 = 100_000_007;
    let k: u64 = 10_007;

    let two_n = pow_mod(2, n, M);
    let two_n_m1 = (two_n + M - 1) % M;
    let mut num = ((k - 1) % M as u64) as u128 * two_n_m1 as u128 % M as u128;
    num = (num + n % M as u64 as u128) % M as u128;
    num = num * pow_mod(2, n - k, M) as u128 % M as u128;
    let den = two_n_m1 as u128 * two_n_m1 as u128 % M as u128;
    let ans = num * den % M as u128;

    println!("{}", ans);
}
