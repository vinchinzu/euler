// Project Euler 747 - Triangular Pizza
//
// Chunked (a, b-range) work units so small-a inner loops split across threads.
// Inner loop uses u64::isqrt and defers modular reduction.

use rayon::prelude::*;

const MOD: i64 = 1_000_000_007;

#[inline]
fn mul(a: i64, b: i64) -> i64 {
    ((a as u64 * b as u64) % MOD as u64) as i64
}

fn pow_mod(mut base: i64, mut exp: i64) -> i64 {
    let mut result: i64 = 1;
    base = ((base % MOD) + MOD) % MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(result, base);
        }
        exp >>= 1;
        base = mul(base, base);
    }
    result
}

fn ncr(n: i64, r: i64) -> i64 {
    if r < 0 || r > n {
        return 0;
    }
    let r = r.min(n - r);
    let mut result: i64 = 1;
    for i in 0..r {
        result = mul(result, (n - i) % MOD);
        result = mul(result, pow_mod(i + 1, MOD - 2));
    }
    result
}

fn tr(n: i64) -> i64 {
    n * (n + 1) / 2
}

#[inline(always)]
fn contrib_ab(a: u64, b0: u64, b1: u64, n: u64) -> i64 {
    let mut local = 0i64;
    let a1 = a + 1;
    let c = 4 * a * a1; // 4*a*(a+1)
    let two_a_1 = 2 * a + 1;
    let mut b = b0;
    while b < b1 {
        let prod = c.wrapping_mul(b.wrapping_mul(b + 1));
        let sq_root = prod.isqrt();
        let min_n2 = two_a_1 * b + a1 + sq_root;
        if min_n2 > n {
            break;
        }
        local += 12 * (n - min_n2) as i64;
        if sq_root * sq_root == prod {
            local += 6;
        }
        b += 1;
    }
    local % MOD
}

fn main() {
    let n: u64 = 100_000_000;

    let mut ans = ncr(n as i64, 3);
    ans = (ans + 6 * (tr(n as i64 - 2) % MOD)) % MOD;

    let sqrt_2n = (2 * n).isqrt();

    // Diagonal a=b contribution (the min_n = (2a+1)^2 term).
    let extra_diag: i64 = (1..=sqrt_2n)
        .into_par_iter()
        .map(|a| {
            let min_n = (2 * a + 1) * (2 * a + 1);
            if min_n <= n {
                (6 * ((n - min_n) % MOD as u64) as i64 + 3) % MOD
            } else {
                0
            }
        })
        .sum();
    ans = (ans + extra_diag) % MOD;

    // Work units: split each a's b-loop into chunks so a=1 (b up to ~n/4) is
    // shared across threads instead of sitting in one range-split half.
    const B_CHUNK: u64 = 8192;
    let mut units: Vec<(u64, u64, u64)> = Vec::new();
    for a in 1..=sqrt_2n {
        // min_n2 ≈ 4ab for large b, so b_max ≈ n/(4a). Add slack for the isqrt error.
        let b_hi_est = (n / (4 * a) + a + 64).min(n);
        if b_hi_est <= a {
            continue;
        }
        let mut b0 = a + 1;
        while b0 <= b_hi_est {
            let b1 = (b0 + B_CHUNK).min(b_hi_est + 1);
            units.push((a, b0, b1));
            b0 = b1;
        }
    }

    let extra: i64 = units
        .into_par_iter()
        .map(|(a, b0, b1)| contrib_ab(a, b0, b1, n))
        .sum();

    ans = (ans + extra) % MOD;
    println!("{}", ((ans % MOD) + MOD) % MOD);
}
