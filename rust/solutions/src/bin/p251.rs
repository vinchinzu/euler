// Project Euler 251: Cardano Triplets
// Count Cardano triplets (a,b,c) with a+b+c <= N=110000000.

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (mut g, mut x, mut y) = (m, 0i64, 1i64);
    let mut aa = a;
    while aa != 0 {
        let q = g / aa;
        let t = g - q * aa;
        g = aa;
        aa = t;
        let t = x - q * y;
        x = y;
        y = t;
    }
    ((x % m) + m) % m
}

fn main() {
    let n: i64 = 110_000_000;
    let mut ans: i64 = 0;

    let mut max_r = ((8.0 * n as f64 / 3.0).sqrt()) as i64;
    if max_r % 2 == 0 {
        max_r -= 1;
    }

    let mut r = 1i64;
    while r <= max_r {
        let r2 = r * r;

        let min_t = (5 * r2) % 8;
        let min_t = if min_t == 0 { 8 } else { min_t };

        let s_limit_sq = n as f64 / min_t as f64 - 3.0 * r2 as f64 / 8.0;
        if s_limit_sq < 1.0 {
            r += 2;
            continue;
        }
        let max_s = s_limit_sq.sqrt() as i64;

        for s in 1..=max_s {
            if gcd(r, s) != 1 {
                continue;
            }

            let a8s = 8 * s;
            let g_inv = mod_inverse(a8s, r2);
            let mut g = (g_inv * 3) % r2;
            if g == 0 {
                g = r2;
            }

            let t_val = (a8s * g - 3) / r2;

            let s2 = s * s;
            let start = 3 * g * s - 1 + g * r + s2 * t_val;
            if start <= n {
                let increment = (3 * s + r) * r2 + 8 * s2 * s;
                ans += (n - start) / increment + 1;
            }
        }

        r += 2;
    }

    println!("{}", ans);
}
