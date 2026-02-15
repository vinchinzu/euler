// Project Euler 817 - Digits in Squares
// M(p, d) = smallest m with m^2 in base p containing digit d
// Sum M(P, P-d) for d=1..N where P=10^9+7, N=10^5

const P: i64 = 1_000_000_007;

fn mod_pow_128(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    let mut result: i128 = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn is_square_mod(n: i64, p: i64) -> bool {
    let n = n.rem_euclid(p);
    if n == 0 { return true; }
    mod_pow_128(n as i128, (p as i128 - 1) / 2, p as i128) == 1
}

fn sqrt_mod(n: i64, p: i64) -> i64 {
    let n = n.rem_euclid(p);
    if n == 0 { return 0; }

    // p-1 = 10^9+6 = 2*500000003, so S=1, Q=500000003
    let q = (p - 1) / 2;
    let s = 1;

    // Find quadratic non-residue
    let mut z = 2i64;
    while is_square_mod(z, p) { z += 1; }

    let mut m_val = s;
    let mut c = mod_pow_128(z as i128, q as i128, p as i128);
    let mut t = mod_pow_128(n as i128, q as i128, p as i128);
    let mut r = mod_pow_128(n as i128, ((q + 1) / 2) as i128, p as i128);

    loop {
        if t == 1 { return r as i64; }

        let mut i = 0;
        let mut temp = t;
        while temp != 1 {
            temp = temp * temp % p as i128;
            i += 1;
        }

        let mut b = c;
        for _ in 0..(m_val - i - 1) {
            b = b * b % p as i128;
        }

        m_val = i;
        c = b * b % p as i128;
        t = t * c % p as i128;
        r = r * b % p as i128;
    }
}

fn isqrt128(n: i128) -> i64 {
    if n <= 1 { return n as i64; }
    let mut x = (n as f64).sqrt() as i64;
    while (x as i128) * (x as i128) > n { x -= 1; }
    while ((x + 1) as i128) * ((x + 1) as i128) <= n { x += 1; }
    x
}

fn m_func(p: i64, d: i64) -> i64 {
    if is_square_mod(d, p) {
        let sq = sqrt_mod(d, p);
        return sq.min(p - sq);
    }

    // Check "tens" digit
    let mut h: i128 = 0;
    loop {
        let low = h + d as i128 * p as i128;
        let high = h + (d as i128 + 1) * p as i128 - 1;
        let sq_high = isqrt128(high);
        if (sq_high as i128) * (sq_high as i128) >= low {
            return sq_high;
        }
        h += p as i128 * p as i128;
    }
}

fn main() {
    let nn = 100_000;
    let mut ans: i64 = 0;
    for d in 1..=nn {
        ans += m_func(P, P - d);
    }
    println!("{}", ans);
}
