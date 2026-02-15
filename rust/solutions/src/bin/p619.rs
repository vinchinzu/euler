// Project Euler 619 - Perfect subsets
// Count subsets of {A..B} with perfect square product
// Answer = 2^(B-A+1-rank) - 1 mod M

const A_VAL: i64 = 1_000_000;
const B_VAL: i64 = 1_234_567;
const M_VAL: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let mut is_prime = vec![true; (B_VAL + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2i64;
    while i * i <= B_VAL { if is_prime[i as usize] { let mut j = i*i; while j <= B_VAL { is_prime[j as usize] = false; j += i; } } i += 1; }

    let mut rank = 0i64;
    for p in 2..=B_VAL {
        if is_prime[p as usize] {
            if (A_VAL - 1) / p != B_VAL / p {
                rank += 1;
            }
        }
    }

    let ans = (pow_mod(2, B_VAL - A_VAL + 1 - rank, M_VAL) - 1 + M_VAL) % M_VAL;
    println!("{}", ans);
}
