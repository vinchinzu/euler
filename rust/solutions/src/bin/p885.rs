// Project Euler 885
// Digit sequences with multinomial coefficients mod 1123455689.

const N: usize = 18;
const B: usize = 10;
const M: i64 = 1_123_455_689;

#[inline(always)]
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

    let fact_n = fact[N];
    let mut counts = [0u8; B];
    let mut ans = 0i128;
    let m128 = M as i128;

    #[inline(always)]
    fn helper(
        index: usize, min_d: usize, n: i64, fact_n: i64,
        counts: &mut [u8; B], ans: &mut i128,
        inv_fact: &[i64], m128: i128,
    ) {
        if index == N {
            let mut result = fact_n;
            for i in 0..B {
                let cnt = unsafe { *counts.get_unchecked(i) };
                if cnt > 0 {
                    result = result * unsafe { *inv_fact.get_unchecked(cnt as usize) } % M;
                }
            }
            *ans += (n as i128) * (result as i128);
            if *ans >= m128 << 10 { *ans %= m128; }
            return;
        }
        for d in min_d..B {
            unsafe { *counts.get_unchecked_mut(d) += 1; }
            helper(index + 1, d, n * 10 + d as i64, fact_n, counts, ans, inv_fact, m128);
            unsafe { *counts.get_unchecked_mut(d) -= 1; }
        }
    }

    helper(0, 0, 0, fact_n, &mut counts, &mut ans, &inv_fact, m128);
    ans %= m128;
    println!("{}", ans);
}
