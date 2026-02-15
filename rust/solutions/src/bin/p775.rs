// Project Euler 775 - Paper Wrapping
// Compute sum_{n=1}^N g(n) via rectangular prism face tracking.

fn main() {
    const N: i64 = 10_000_000_000_000_000; // 10^16
    const M: i64 = 1_000_000_007;

    fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
        let mut result = 1i64;
        base = ((base % m) + m) % m;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result as i128 * base as i128 % m as i128) as i64;
            }
            base = (base as i128 * base as i128 % m as i128) as i64;
            exp >>= 1;
        }
        result
    }

    fn tr(n: i64, m: i64) -> i64 {
        let a = n % m;
        let b = (n + 1) % m;
        (a as i128 * b as i128 % m as i128 * pow_mod(2, m - 2, m) as i128 % m as i128) as i64
    }

    fn ncr(n: i64, k: i32, m: i64) -> i64 {
        if k < 0 || k as i64 > n { return 0; }
        if k == 0 { return 1; }
        let mut result = 1i64;
        for i in 0..k {
            result = (result as i128 * ((n - i as i64) % m + m) as i128 % m as i128) as i64;
            result = (result as i128 * pow_mod(i as i64 + 1, m - 2, m) as i128 % m as i128) as i64;
        }
        result
    }

    fn sum_sq(d: i64, m: i64) -> i64 {
        if d <= 0 { return 0; }
        let a = d % m;
        let b = (d + 1) % m;
        let c = (2 * d + 1) % m;
        let inv6 = pow_mod(6, m - 2, m);
        (a as i128 * b as i128 % m as i128 * c as i128 % m as i128 * inv6 as i128 % m as i128) as i64
    }

    fn isqrt64(n: i64) -> i64 {
        if n <= 0 { return 0; }
        let mut s = (n as f64).sqrt() as i64;
        while s * s > n { s -= 1; }
        while (s + 1) * (s + 1) <= n { s += 1; }
        s
    }

    let mut sides = [1i64, 1, 1];
    let mut index: i64 = 1;
    let n_mod = N % M;

    let tr_n = tr(N, M);
    let mut ans = (6 * ((tr_n - n_mod % M + M) % M)) % M;

    loop {
        let side1 = sides[1];
        let side2 = sides[2];
        let d1_lim = isqrt64(N - index - 1);
        let d1 = d1_lim.min(side2 - 1);
        let d2_lim = (isqrt64(4 * (N - index)) - 1) / 2;
        let d2 = d2_lim.min(side1 - 1);

        let ni_mod = ((N - index) % M + M) % M;

        ans = (ans - 4 * ni_mod % M + M) % M;

        {
            let d1_mod = d1 % M;
            let term = ((ni_mod as i128 * d1_mod as i128 % M as i128) as i64 - sum_sq(d1, M) + M) % M;
            ans = (ans - 2 * term % M + M) % M;
        }

        {
            let d2_mod = d2 % M;
            let c3 = ncr(d2 + 2, 3, M);
            let term = ((ni_mod as i128 * d2_mod as i128 % M as i128) as i64 - 2 * c3 % M + M) % M;
            ans = (ans - 2 * term % M + M) % M;
        }

        index += side1 * side2;
        if index >= N { break; }

        let front = sides[0] + 1;
        sides[0] = sides[1];
        sides[1] = sides[2];
        sides[2] = front;
    }

    ans %= M;
    println!("{}", ans);
}
