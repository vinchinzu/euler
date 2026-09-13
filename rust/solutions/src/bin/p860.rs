// Project Euler 860 - Fair arrangements of stacks
// Multinomial approach with scaled values

const MOD: u64 = 989898989;

#[inline(always)]
fn power(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut res = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    res
}

#[inline(always)]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    a * b % m
}

fn main() {
    let n = 9898;

    let mut fact = vec![0u64; n + 1];
    let mut invfact = vec![0u64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = mul_mod(fact[i - 1], i as u64, MOD);
    }
    invfact[n] = power(fact[n], MOD - 2, MOD);
    for i in (0..n).rev() {
        invfact[i] = mul_mod(invfact[i + 1], (i + 1) as u64, MOD);
    }

    let mut total = 0u64;

    let mut j = 0;
    while j <= n / 5 {
        let nj = n + 3 * j;
        if nj % 2 != 0 { j += 2; continue; }
        let s = nj / 2;
        let low_c = 4 * j;
        if s < low_c { j += 2; continue; }

        let mut sum_contrib = 0u64;
        for c in low_c..=s {
            let a = s - c;
            let b = a + j;
            let d = c - 4 * j;
            if a > n || b > n || d > n { continue; }

            let mut term = fact[n];
            term = mul_mod(term, invfact[a], MOD);
            term = mul_mod(term, invfact[b], MOD);
            term = mul_mod(term, invfact[c], MOD);
            term = mul_mod(term, invfact[d], MOD);
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
