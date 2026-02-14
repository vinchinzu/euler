fn mulmod(a: i64, b: i64, m: i64) -> i64 {
    ((a as i128 * b as i128) % m as i128) as i64
}

fn main() {
    let p: i64 = 61;
    let q: i32 = 10_000_000;
    let e: i32 = 10;
    let bbs_mod: i64 = 50515093;

    let mut m: i64 = 1;
    for _ in 0..e { m *= p; }

    let mut pows = vec![0i64; (e + 1) as usize];
    pows[0] = 1;
    for i in 1..=(e as usize) { pows[i] = pows[i - 1] * p; }

    // inv(P-1, M) using extended GCD
    let inv_pm1 = {
        let (mut a0, mut b0) = (p - 1, m);
        let (mut x0, mut x1) = (1i64, 0i64);
        while b0 > 0 {
            let qq = a0 / b0;
            let tmp = b0; b0 = a0 - qq * b0; a0 = tmp;
            let tmp = x1; x1 = x0 - qq * x1; x0 = tmp;
        }
        ((x0 % m) + m) % m
    };

    let coeff_full = mulmod(m - 1, inv_pm1, m);

    let mut coeff_small = vec![0i64; e as usize];
    coeff_small[0] = 0;
    for k in 1..(e as usize) {
        coeff_small[k] = mulmod(pows[k] - 1, inv_pm1, m);
    }

    let mut ans: i64 = 0;
    let mut s: i64 = 290797;

    // Skip T[0]
    s = (s * s) % bbs_mod;

    // k=1 to E-1
    for k in 1..(e as usize) {
        let tk = s % p;
        s = (s * s) % bbs_mod;
        ans = (ans + mulmod(tk, coeff_small[k], m)) % m;
    }

    // k=E to Q
    let mut chunk: i64 = 0;
    for k in (e as i32)..=q {
        let tk = s % p;
        s = (s * s) % bbs_mod;
        chunk += tk;
        if k % 1_000_000 == 0 || k == q {
            ans = (ans + mulmod(coeff_full, chunk % m, m)) % m;
            chunk = 0;
        }
    }

    println!("{}", ans);
}
