// Project Euler 747 - Triangular Pizza
//
// Counting triangular configurations on a grid.

const MOD: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        exp >>= 1;
        base = (base as i128 * base as i128 % m as i128) as i64;
    }
    result
}

fn ncr(n: i64, r: i64, m: i64) -> i64 {
    if r < 0 || r > n { return 0; }
    let r = r.min(n - r);
    let mut result: i64 = 1;
    for i in 0..r {
        result = (result as i128 * ((n - i) % m) as i128 % m as i128) as i64;
        result = (result as i128 * pow_mod(i + 1, m - 2, m) as i128 % m as i128) as i64;
    }
    result
}

fn tr(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn isqrt_ll(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn main() {
    let n: i64 = 100_000_000; // 10^8
    let m = MOD;

    let mut ans = ncr(n, 3, m);
    ans = (ans + 6 * (tr(n - 2) % m)) % m;

    let sqrt_2n = isqrt_ll(2 * n);
    for a in 1..=sqrt_2n {
        let min_n = (2 * a + 1) * (2 * a + 1);
        if min_n <= n {
            ans = (ans + 6 * ((n - min_n) % m) % m + 3) % m;
        }

        let mut b = a + 1;
        loop {
            let prod = 4 * (a + 1) * (b + 1) * a * b;
            let sq_root = isqrt_ll(prod);
            let min_n2 = (a + 1) * (b + 1) + a * b + sq_root;
            if min_n2 > n { break; }
            ans = (ans + 12 * ((n - min_n2) % m)) % m;
            if sq_root * sq_root == prod {
                ans = (ans + 6) % m;
            }
            b += 1;
        }
    }

    println!("{}", ((ans % m) + m) % m);
}
