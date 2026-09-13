// Project Euler 916 - Permutation Subsequences
// P(n) = C_n^2 * (1 + (3n/(n+2))^2) mod 10^9+7
// where C_n is the nth Catalan number.

const MOD: u64 = 1_000_000_007;

#[inline(always)]
fn mulm(a: u64, b: u64) -> u64 {
    a * b % MOD
}

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

fn inverse(a: u64, m: u64) -> u64 {
    power(a, m - 2, m)
}

fn main() {
    let n: u64 = 100_000_000; // 10^8

    // Optimized factorial with 8-way unrolling and independent accumulators
    let mut i = 1u64;
    let mut a0 = 1u64;
    let mut a1 = 1u64;
    let mut a2 = 1u64;
    let mut a3 = 1u64;
    let mut a4 = 1u64;
    let mut a5 = 1u64;
    let mut a6 = 1u64;
    let mut a7 = 1u64;
    
    // Compute n! with 8-way unrolling
    while i + 7 <= n {
        a0 = mulm(a0, i);
        a1 = mulm(a1, i + 1);
        a2 = mulm(a2, i + 2);
        a3 = mulm(a3, i + 3);
        a4 = mulm(a4, i + 4);
        a5 = mulm(a5, i + 5);
        a6 = mulm(a6, i + 6);
        a7 = mulm(a7, i + 7);
        i += 8;
    }
    while i <= n {
        a0 = mulm(a0, i);
        i += 1;
    }
    
    let fact_n = mulm(
        mulm(mulm(a0, a1), mulm(a2, a3)),
        mulm(mulm(a4, a5), mulm(a6, a7)),
    );
    
    // Continue from n+1 to 2n for (2n)!
    a0 = fact_n;
    a1 = 1;
    a2 = 1;
    a3 = 1;
    a4 = 1;
    a5 = 1;
    a6 = 1;
    a7 = 1;
    
    while i + 7 <= 2 * n {
        a0 = mulm(a0, i);
        a1 = mulm(a1, i + 1);
        a2 = mulm(a2, i + 2);
        a3 = mulm(a3, i + 3);
        a4 = mulm(a4, i + 4);
        a5 = mulm(a5, i + 5);
        a6 = mulm(a6, i + 6);
        a7 = mulm(a7, i + 7);
        i += 8;
    }
    while i <= 2 * n {
        a0 = mulm(a0, i);
        i += 1;
    }
    
    let fact_2n = mulm(
        mulm(mulm(a0, a1), mulm(a2, a3)),
        mulm(mulm(a4, a5), mulm(a6, a7)),
    );

    // C_n = (2n)! / ((n+1)! * n!) = (2n)! * inv(n+1) * inv(n!)^2
    let inv_fact_n = inverse(fact_n, MOD);
    let inv_n_plus_1 = inverse(n + 1, MOD);

    let mut cn = mulm(fact_2n, inv_n_plus_1);
    cn = mulm(cn, inv_fact_n);
    cn = mulm(cn, inv_fact_n);

    // term2_val = 3n / (n+2) mod p
    let term2_num = 3 * n % MOD;
    let term2_den = inverse(n + 2, MOD);
    let term2_val = mulm(term2_num, term2_den);

    // P(n) = Cn^2 * (1 + term2_val^2)
    let cn_sq = mulm(cn, cn);
    let term2_sq = mulm(term2_val, term2_val);
    let bracket = (1 + term2_sq) % MOD;

    let ans = mulm(cn_sq, bracket);
    println!("{}", ans);
}
