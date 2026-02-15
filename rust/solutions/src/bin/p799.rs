// Project Euler 799 - Pentagonal Number Representations
// Two-pass sieve with Gaussian integer factorization to count representations.

fn mod_pow_i128(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    if base < 0 { base += m; }
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn sqrt_mod_func(n: i64, p: i64) -> i64 {
    let n = ((n % p) + p) % p;
    if n == 0 { return 0; }
    if mod_pow_i128(n, (p - 1) / 2, p) != 1 { return -1; }
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 { q /= 2; s += 1; }
    if s == 1 { return mod_pow_i128(n, (p + 1) / 4, p); }
    let mut z = 2i64;
    while mod_pow_i128(z, (p - 1) / 2, p) != p - 1 { z += 1; }
    let mut mm = s;
    let mut c = mod_pow_i128(z, q, p);
    let mut t = mod_pow_i128(n, q, p);
    let mut r = mod_pow_i128(n, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 1u32;
        let mut temp = (t as i128 * t as i128 % p as i128) as i64;
        while temp != 1 { temp = (temp as i128 * temp as i128 % p as i128) as i64; i += 1; }
        let b = mod_pow_i128(c, 1i64 << (mm - i - 1), p);
        mm = i;
        c = (b as i128 * b as i128 % p as i128) as i64;
        t = (t as i128 * c as i128 % p as i128) as i64;
        r = (r as i128 * b as i128 % p as i128) as i64;
    }
    r
}

fn decompose_prime(p: i64) -> (i64, i64) {
    let r = {
        let rv = sqrt_mod_func(p - 1, p);
        if rv > p / 2 { p - rv } else { rv }
    };
    let (mut x, mut y) = (p, r);
    while y * y >= p {
        let t = x % y;
        x = y;
        y = t;
    }
    let rem = p - y * y;
    let s = (rem as f64).sqrt() as i64;
    for ds in [-1i64, 0, 1] {
        let ss = s + ds;
        if ss >= 0 && ss * ss == rem {
            let a = y.min(ss);
            let b = y.max(ss);
            return (a, b);
        }
    }
    // Fallback brute force
    for a in 1.. {
        if a * a >= p { break; }
        let bsq = p - a * a;
        let b = (bsq as f64).sqrt() as i64;
        for db in [0i64, 1] {
            if (b + db) * (b + db) == bsq { return (a, b + db); }
        }
    }
    unreachable!()
}

const CLIMIT: usize = 28_000_000;
const PLIMIT: usize = 500_000;
const THRESHOLD: u16 = 1000;

fn main() {
    // Compute primes ≡ 1 (mod 4)
    let mut is_prime = vec![true; PLIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=PLIMIT {
        if is_prime[i] {
            let mut j = i * i;
            while j <= PLIMIT { is_prime[j] = false; j += i; }
        }
    }
    let primes1: Vec<i64> = (5..=PLIMIT).filter(|&i| is_prime[i] && i % 4 == 1).map(|i| i as i64).collect();

    // Compute roots and Gaussian decompositions
    let mut root1v = vec![0i64; primes1.len()];
    let mut root2v = vec![0i64; primes1.len()];
    let mut gauss_a = vec![0i64; primes1.len()];
    let mut gauss_b = vec![0i64; primes1.len()];

    for (idx, &p) in primes1.iter().enumerate() {
        let sq = sqrt_mod_func(p - 1, p);
        if sq == -1 { root1v[idx] = -1; root2v[idx] = -1; continue; }
        let inv6 = mod_pow_i128(6, p - 2, p);
        root1v[idx] = ((1 + sq) % p * inv6 % p) as i64;
        root2v[idx] = (((1 - sq + p) % p) * inv6 % p) as i64;
        let (a, b) = decompose_prime(p);
        gauss_a[idx] = a;
        gauss_b[idx] = b;
    }

    // Pass 1: sieve R values
    let mut r_arr = vec![1u16; CLIMIT];
    for (idx, &p) in primes1.iter().enumerate() {
        if root1v[idx] == -1 { continue; }
        for r in &[root1v[idx], root2v[idx]] {
            let mut c = *r;
            while c < CLIMIT as i64 {
                if c >= 2 {
                    let mut m = 18 * c * c - 6 * c + 1;
                    let mut e = 0u32;
                    while m % p == 0 { m /= p; e += 1; }
                    let new_r = r_arr[c as usize] as u64 * (2 * e as u64 + 1);
                    r_arr[c as usize] = if new_r > 65535 { 65535 } else { new_r as u16 };
                }
                c += p;
            }
            if root2v[idx] == root1v[idx] { break; }
        }
    }

    // Collect candidates
    struct CandInfo {
        c: i64,
        factors: Vec<(usize, i32)>, // (prime_index, exponent)
        remaining: i64,
    }

    let mut cands: Vec<CandInfo> = Vec::new();
    for c in 2..CLIMIT as i64 {
        if r_arr[c as usize] >= THRESHOLD {
            cands.push(CandInfo {
                c,
                factors: Vec::new(),
                remaining: 18 * c * c - 6 * c + 1,
            });
        }
    }

    // Pass 2: build factorizations
    for (idx, &p) in primes1.iter().enumerate() {
        if root1v[idx] == -1 { continue; }
        for r in &[root1v[idx], root2v[idx]] {
            let mut c = *r;
            while c < CLIMIT as i64 {
                if c >= 2 && r_arr[c as usize] >= THRESHOLD {
                    if let Ok(ci) = cands.binary_search_by_key(&c, |x| x.c) {
                        let info = &mut cands[ci];
                        if info.remaining % p == 0 {
                            let mut e = 0;
                            while info.remaining % p == 0 { info.remaining /= p; e += 1; }
                            info.factors.push((idx, e));
                        }
                    }
                }
                c += p;
            }
            if root2v[idx] == root1v[idx] { break; }
        }
    }

    // Check candidates
    let target = 100;
    for info in &cands {
        let mut pfactors: Vec<(i64, i64, i32)> = Vec::new(); // (a, b, exp)
        for &(pidx, e) in &info.factors {
            pfactors.push((gauss_a[pidx], gauss_b[pidx], e));
        }
        if info.remaining > 1 {
            let (a, b) = decompose_prime(info.remaining);
            pfactors.push((a, b, 1));
        }

        let mut pair_count = 0;

        fn enumerate(
            pfactors: &[(i64, i64, i32)],
            idx: usize,
            re: i128,
            im: i128,
            pair_count: &mut i32,
        ) {
            if idx == pfactors.len() {
                let coords = [
                    (re, im), (-im, re), (-re, -im), (im, -re),
                ];
                for (x, y) in coords {
                    if x > 0 && y > 0 && x <= y {
                        let xm = ((x % 6) + 6) % 6;
                        let ym = ((y % 6) + 6) % 6;
                        if xm == 5 && ym == 5 { *pair_count += 1; }
                    }
                }
                return;
            }
            let (a, b, e) = pfactors[idx];
            for j in 0..=e {
                let mut fre: i128 = 1;
                let mut fim: i128 = 0;
                // Multiply by (a+bi)^j
                for _ in 0..j {
                    let nr = fre * a as i128 - fim * b as i128;
                    let ni = fre * b as i128 + fim * a as i128;
                    fre = nr; fim = ni;
                }
                // Multiply by (a-bi)^(e-j)
                for _ in 0..(e - j) {
                    let nr = fre * a as i128 - fim * (-b as i128);
                    let ni = fre * (-b as i128) + fim * a as i128;
                    fre = nr; fim = ni;
                }
                let nre = re * fre - im * fim;
                let nim = re * fim + im * fre;
                enumerate(pfactors, idx + 1, nre, nim, pair_count);
            }
        }

        enumerate(&pfactors, 0, 1, 1, &mut pair_count);

        if pair_count > target {
            let c = info.c;
            println!("{}", c * (3 * c - 1) / 2);
            return;
        }
    }

    println!("Not found");
}
