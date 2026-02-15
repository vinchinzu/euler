// Project Euler 791 - Average and Variance
// S(n) = sum of (a+b+c+d) over ordered quadruples where average = 2*variance.
// O(sqrt(N)) algorithm with modular arithmetic.

const MOD: i64 = 433_494_437;

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn isqrt(n: i64) -> i64 {
    if n < 0 { return -1; }
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn main() {
    let n: i64 = 100_000_000;
    let inv3 = powmod(3, MOD - 2, MOD);
    let inv6 = powmod(6, MOD - 2, MOD);

    let modf = |x: i64| -> i64 { ((x % MOD) + MOD) % MOD };

    let closed_sum = |g: i64, h_val: i64| -> i64 {
        if h_val < 0 { return 0; }
        let gm = g % MOD;
        let hm = h_val % MOD;
        let t1 = 2 * gm % MOD * gm % MOD * ((hm + 1) % MOD) % MOD * ((hm + 1) % MOD) % MOD;
        let t2 = hm * ((hm + 1) % MOD) % MOD * ((2 * hm + 1) % MOD) % MOD
            * ((2 * hm + 3) % MOD) % MOD * inv3 % MOD;
        (t1 + t2) % MOD
    };

    let sum_sq_to = |nn: i64| -> i64 {
        if nn < 0 { return 0; }
        let nm = nn % MOD;
        nm * ((nm + 1) % MOD) % MOD * ((2 * nm + 1) % MOD) % MOD * inv6 % MOD
    };

    let sum_sq_range = |a: i64, b: i64| -> i64 {
        if a > b { return 0; }
        if a >= 0 {
            modf(sum_sq_to(b) - sum_sq_to(a - 1))
        } else if b < 0 {
            modf(sum_sq_to(-a) - sum_sq_to(-b - 1))
        } else {
            (sum_sq_to(b) + sum_sq_to(-a)) % MOD
        }
    };

    let mut ans: i64 = 0;

    let mut g_max = isqrt(2 * n);
    while g_max * (g_max + 1) > 2 * n { g_max -= 1; }

    for g in 0..=g_max {
        let g2 = g * g;
        let t_full = 2 * n - g2 - g;
        if t_full < 0 { break; }

        let mut h_full = (-1 + isqrt(1 + 2 * t_full)) / 2;
        if h_full > g { h_full = g; }

        ans = (ans + closed_sum(g, h_full)) % MOD;

        let t_any = t_full;
        let mut h_any = (-1 + isqrt(1 + 4 * t_any)) / 2;
        if h_any > g { h_any = g; }

        for h in (h_full + 1)..=h_any {
            let h2 = h * h;
            let t_val = 2 * n - g2 - h2 - g - h;
            if t_val < 0 { break; }

            let mut r_hi = (-1 + isqrt(1 + 4 * t_val)) / 2;
            if r_hi > h { r_hi = h; }

            let r_neg_max = (1 + isqrt(1 + 4 * t_val)) / 2;
            let mut r_lo = -h;
            if r_lo < -r_neg_max { r_lo = -r_neg_max; }

            if r_lo > r_hi { continue; }

            let cnt = r_hi - r_lo + 1;
            let sr = sum_sq_range(r_lo, r_hi);
            let gh2 = (g2 + h2) % MOD;
            let contrib = (2 * sr % MOD + 2 * (cnt % MOD) % MOD * gh2 % MOD) % MOD;
            ans = (ans + contrib) % MOD;
        }
    }

    ans = (ans - 12 + MOD) % MOD;

    println!("{}", ans);
}
