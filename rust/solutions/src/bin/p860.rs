// Project Euler 860 - Fair arrangements of stacks
// Multinomial approach with scaled values

const MOD: i64 = 989898989;

fn power(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut res: i64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    res
}

fn main() {
    let n = 9898usize;

    let mut fact = vec![0i64; n + 1];
    let mut invfact = vec![0i64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }
    invfact[n] = power(fact[n], MOD - 2, MOD);
    for i in (0..n).rev() {
        invfact[i] = invfact[i + 1] * (i + 1) as i64 % MOD;
    }

    let mut total: i64 = 0;

    let mut j = 0i32;
    while j as usize <= n / 5 {
        let nj = n as i32 + 3 * j;
        if nj % 2 != 0 { j += 2; continue; }
        let s = nj / 2;
        let low_c = 4 * j;
        if s < low_c { j += 2; continue; }

        let mut sum_contrib: i64 = 0;
        for c in low_c..=s {
            let a = s - c;
            let b = a + j;
            let d = c - 4 * j;
            if a < 0 || b < 0 || d < 0 { continue; }
            if (a + b + c + d) as usize != n { continue; }

            let mut term = fact[n];
            term = (term as i128 * invfact[a as usize] as i128 % MOD as i128) as i64;
            term = (term as i128 * invfact[b as usize] as i128 % MOD as i128) as i64;
            term = (term as i128 * invfact[c as usize] as i128 % MOD as i128) as i64;
            term = (term as i128 * invfact[d as usize] as i128 % MOD as i128) as i64;
            sum_contrib = (sum_contrib + term) % MOD;
        }

        if j == 0 {
            total = (total + sum_contrib) % MOD;
        } else {
            total = (total + 2 * sum_contrib) % MOD;
        }
        j += 2;
    }

    println!("{}", total);
}
