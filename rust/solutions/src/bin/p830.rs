// Project Euler 830 - Binomial Coefficients mod p^3
// S(n) = sum_{k=0}^{n} C(n,k) * k^n, find S(10^18) mod 83^3 * 89^3 * 97^3.
// Uses Stirling number expansion and CRT with p = 83, 89, 97.

fn mod_pow(mut base: i128, mut exp: i128, m: i128) -> i128 {
    let mut result: i128 = 1;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x1, y1) = ext_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn mod_inv_ext(a: i64, m: i64) -> i64 {
    let (_, x, _) = ext_gcd(a, m);
    ((x % m) + m) % m
}

fn build_bin_table(max_m: usize, modulus: i64) -> Vec<Vec<i64>> {
    let mut table = vec![vec![0i64; max_m + 1]; max_m + 1];
    for i in 0..=max_m {
        table[i][0] = 1;
        for j in 1..=i.min(max_m) {
            table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % modulus;
        }
    }
    table
}

/// Compute C(n, m) mod p^3 for large n and small m, properly handling
/// the case where m! has factors of p by extracting p-adic valuations.
fn compute_binom_mod(n: i64, m: usize, p: i64, modulus: i64) -> i64 {
    if m == 0 { return 1 % modulus; }
    let md = modulus as i128;

    // Compute numerator: n * (n-1) * ... * (n-m+1)
    // Extract powers of p from each factor
    let mut num_val = 1i128;
    let mut num_vp: i32 = 0;
    for i in 0..m {
        let mut x = n - i as i64;
        let mut v = 0i32;
        while x % p == 0 {
            v += 1;
            x /= p;
        }
        num_vp += v;
        num_val = num_val * (((x % modulus) + modulus) as i128) % md;
    }

    // Compute denominator: m!
    // Extract powers of p from each factor
    let mut den_val = 1i128;
    let mut den_vp: i32 = 0;
    for i in 1..=m as i64 {
        let mut x = i;
        let mut v = 0i32;
        while x % p == 0 {
            v += 1;
            x /= p;
        }
        den_vp += v;
        den_val = den_val * (x as i128) % md;
    }

    let vp = num_vp - den_vp;
    if vp >= 3 { return 0; }
    if vp < 0 { return 0; } // shouldn't happen for binomials

    let inv = mod_inv_ext(den_val as i64, modulus);
    let mut result = num_val * inv as i128 % md;

    // Multiply by p^vp
    for _ in 0..vp {
        result = result * p as i128 % md;
    }

    result as i64
}

fn compute_s_mod_p3(p: i32, n: i64) -> i64 {
    let p64 = p as i64;
    let modulus = p64 * p64 * p64;
    let phi = p64 * p64 * (p64 - 1);
    let e = mod_pow(10, 18, phi as i128) as i64;

    let max_m = 3 * p as usize - 1;

    let bin_table = build_bin_table(max_m, modulus);
    let md = modulus as i128;

    let mut total: i64 = 0;
    for m in 0..=max_m {
        let mut sum_se: i128 = 0;
        for j in 0..=m {
            let sign: i128 = if (m - j) % 2 == 0 { 1 } else { -1 };
            let je: i128 = if j == 0 { 0 } else { mod_pow(j as i128, e as i128, md) };
            let term = bin_table[m][j] as i128 * je % md * sign;
            sum_se = ((sum_se + term % md) + md) % md;
        }
        let bnm = compute_binom_mod(n, m, p64, modulus) as i128;
        let tw = mod_pow(2, (n - m as i64) as i128, md);
        let term = sum_se * bnm % md * tw % md;
        total = ((total as i128 + term) % md) as i64;
    }

    total
}

fn crt3(vals: &[i64; 3], mods: &[i64; 3]) -> i64 {
    let big_m: i128 = mods[0] as i128 * mods[1] as i128 * mods[2] as i128;
    let mut x: i128 = 0;
    for i in 0..3 {
        let mi = big_m / mods[i] as i128;
        let mi_mod = (mi % mods[i] as i128) as i64;
        let (_, xi, _) = ext_gcd(mi_mod, mods[i]);
        let xi = ((xi as i128 % mods[i] as i128) + mods[i] as i128) % mods[i] as i128;
        x = (x + vals[i] as i128 * mi % big_m * xi) % big_m;
    }
    ((x % big_m + big_m) % big_m) as i64
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000;
    let ps = [83i32, 89, 97];
    let mut ss = [0i64; 3];
    let mut ms = [0i64; 3];

    for i in 0..3 {
        ss[i] = compute_s_mod_p3(ps[i], n);
        ms[i] = (ps[i] as i64) * (ps[i] as i64) * (ps[i] as i64);
    }

    let answer = crt3(&ss, &ms);
    println!("{}", answer);
}
