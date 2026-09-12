// Project Euler 638 - Weighted Paths in a Grid
// q-binomial coefficients for k=1..7
//
// M^2 < 2^64, so all mulmods stay in u64 (no i128). Prefix q-factorial is
// loop-carried; only the running product and the snapshot at i=a are kept.

const M: u64 = 1_000_000_007;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    a * b % M
}

fn powmod(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(result, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    result
}

/// Gaussian binomial [2a choose a]_K mod M.
fn qbinom<const K: u64>(a: u64) -> u64 {
    let n = a << 1;
    let mut qfact = 1u64;
    let mut pow_k = 1u64;
    let mut sum = 0u64;

    let mut i = 0u64;
    while i < a {
        i += 1;
        sum += pow_k;
        if sum >= M {
            sum -= M;
        }
        qfact = mul(qfact, sum);
        pow_k = mul(pow_k, K);
    }
    let qfact_a = qfact;
    while i < n {
        i += 1;
        sum += pow_k;
        if sum >= M {
            sum -= M;
        }
        qfact = mul(qfact, sum);
        pow_k = mul(pow_k, K);
    }
    mul(qfact, powmod(mul(qfact_a, qfact_a), M - 2))
}

fn main() {
    let ans = qbinom::<1>(10 + 1)
        + qbinom::<2>(100 + 2)
        + qbinom::<3>(1_000 + 3)
        + qbinom::<4>(10_000 + 4)
        + qbinom::<5>(100_000 + 5)
        + qbinom::<6>(1_000_000 + 6)
        + qbinom::<7>(10_000_000 + 7);
    println!("{}", ans % M);
}
