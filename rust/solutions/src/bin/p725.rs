// Project Euler 725 - Digit Sum Numbers
//
// S(N) computed using repunit, binomial coefficient, and modular arithmetic.
// N=2020, B=10, M=10^16.

const BIG_M: u64 = 10_000_000_000_000_000; // 10^16

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn main() {
    let n = 2020;

    // term1 = repunit(N) mod M = repunit(16) since N > 16
    let mut term1: u64 = 0;
    let mut pw: u64 = 1;
    for i in 0..n.min(16) {
        term1 = (term1 + pw) % BIG_M;
        pw = mulmod(pw, 10, BIG_M);
    }

    // term2 = N - 1 = 2019
    let term2: u64 = n as u64 - 1;

    // term3 = 2 * C(2028, 8) - C(10, 2)
    // C(2028, 8) computed exactly using u128
    let mut comb: u128 = 1;
    for i in 0..8u128 {
        comb = comb * (2028 - i) / (i + 1);
    }
    let comb_mod = (comb % BIG_M as u128) as u64;
    let c10_2: u64 = 45;
    let term3 = (mulmod(2, comb_mod, BIG_M) + BIG_M - c10_2) % BIG_M;

    let ans = mulmod(mulmod(term1, term2, BIG_M), term3, BIG_M);
    println!("{}", ans);
}
