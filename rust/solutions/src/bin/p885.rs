// Project Euler 885
// Digit sequences with multinomial coefficients mod 1123455689.

const N: usize = 18;
const B: usize = 10;
const M: i64 = 1_123_455_689;

fn power(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = base.rem_euclid(m);
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let mut fact = [0i64; N + 2];
    let mut inv_fact = [0i64; N + 2];
    fact[0] = 1;
    for i in 1..=N { fact[i] = fact[i - 1] * i as i64 % M; }
    inv_fact[N] = power(fact[N], M - 2, M);
    for i in (0..N).rev() { inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % M; }

    let mut counts = [0i32; B];
    let mut ans = 0i64;

    fn gn_cr(counts: &[i32; B], fact: &[i64], inv_fact: &[i64], m: i64) -> i64 {
        let mut total = 0;
        for i in 0..B { total += counts[i]; }
        let mut result = fact[total as usize];
        for i in 0..B { result = result * inv_fact[counts[i] as usize] % m; }
        result
    }

    fn helper(
        index: usize, min_d: usize, n: i64,
        counts: &mut [i32; B], ans: &mut i64,
        fact: &[i64], inv_fact: &[i64],
    ) {
        if index == N {
            let cr = gn_cr(counts, fact, inv_fact, M);
            *ans = (*ans + (n % M) * (cr % M)) % M;
            return;
        }
        for d in min_d..B {
            counts[d] += 1;
            helper(index + 1, d, n * B as i64 + d as i64, counts, ans, fact, inv_fact);
            counts[d] -= 1;
        }
    }

    helper(0, 0, 0, &mut counts, &mut ans, &fact, &inv_fact);
    ans %= M;
    println!("{}", ans);
}
