// Project Euler Problem 812: Dynamical Polynomials
// Compute S(10_000) mod 998244353

const MOD: i64 = 998244353;
const PRIMITIVE_ROOT: i64 = 3;

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    let mut r = 1i64;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        e >>= 1;
    }
    r
}

fn ntt(a: &mut [i64], invert: bool) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }

    let mut length = 2usize;
    while length <= n {
        let mut wlen = mod_pow(PRIMITIVE_ROOT, ((MOD - 1) / length as i64) as i64, MOD);
        if invert {
            wlen = mod_pow(wlen, MOD - 2, MOD);
        }
        let half = length >> 1;
        for i in (0..n).step_by(length) {
            let mut w = 1i64;
            for j in i..(i + half) {
                let u = a[j];
                let v = a[j + half] * w % MOD;
                a[j] = (u + v) % MOD;
                a[j + half] = (u - v + MOD) % MOD;
                w = w * wlen % MOD;
            }
        }
        length <<= 1;
    }

    if invert {
        let inv_n = mod_pow(n as i64, MOD - 2, MOD);
        for i in 0..n {
            a[i] = a[i] * inv_n % MOD;
        }
    }
}

fn polymul(a: &[i64], b: &[i64]) -> Vec<i64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }

    let n = a.len() + b.len() - 1;

    // Small-size fallback
    if a.len().min(b.len()) <= 32 {
        let mut res = vec![0i64; n];
        for (i, &ai) in a.iter().enumerate() {
            if ai == 0 {
                continue;
            }
            for (j, &bj) in b.iter().enumerate() {
                res[i + j] = (res[i + j] + ai * bj) % MOD;
            }
        }
        return res;
    }

    let mut size = 1usize;
    while size < n {
        size <<= 1;
    }

    let mut fa = vec![0i64; size];
    let mut fb = vec![0i64; size];
    fa[..a.len()].copy_from_slice(a);
    fb[..b.len()].copy_from_slice(b);

    ntt(&mut fa, false);
    ntt(&mut fb, false);
    for i in 0..size {
        fa[i] = fa[i] * fb[i] % MOD;
    }
    ntt(&mut fa, true);

    fa.truncate(n);
    fa
}

fn poly_der(a: &[i64]) -> Vec<i64> {
    if a.len() <= 1 {
        return vec![0];
    }
    (1..a.len()).map(|i| i as i64 * a[i] % MOD).collect()
}

fn poly_int(a: &[i64], invs: &[i64]) -> Vec<i64> {
    let mut res = vec![0i64; a.len() + 1];
    for (i, &x) in a.iter().enumerate() {
        res[i + 1] = x * invs[i + 1] % MOD;
    }
    res
}

fn poly_inv(a: &[i64], n: usize) -> Vec<i64> {
    assert!(!a.is_empty() && a[0] != 0);
    let mut res = vec![mod_pow(a[0], MOD - 2, MOD)];
    let mut m = 1usize;
    while m < n {
        let m2 = (2 * m).min(n);
        let mut t = polymul(&a[..m2.min(a.len())], &res);
        t.truncate(m2);
        t[0] = (2 - t[0] + MOD) % MOD;
        for i in 1..m2 {
            t[i] = (MOD - t[i]) % MOD;
        }
        res = polymul(&res, &t);
        res.truncate(m2);
        m = m2;
    }
    res.truncate(n);
    res
}

fn poly_ln(a: &[i64], n: usize, invs: &[i64]) -> Vec<i64> {
    assert!(!a.is_empty() && a[0] == 1);
    let der = poly_der(a);
    let inv_a = poly_inv(a, n);
    let q = polymul(&der, &inv_a);
    let mut res = poly_int(&q[..(n - 1).max(0)], invs);
    res.truncate(n);
    res
}

fn poly_exp(f: &[i64], n: usize, invs: &[i64]) -> Vec<i64> {
    if !f.is_empty() {
        assert_eq!(f[0], 0);
    }
    let mut g = vec![1i64];
    let mut m = 1usize;
    while m < n {
        let m2 = (2 * m).min(n);
        let mut g_pad = vec![0i64; m2];
        g_pad[..g.len()].copy_from_slice(&g);
        let ln_g = poly_ln(&g_pad, m2, invs);
        let mut diff = vec![0i64; m2];
        for i in 0..m2 {
            let fi = if i < f.len() { f[i] } else { 0 };
            diff[i] = (fi - ln_g[i] + MOD) % MOD;
        }
        diff[0] = (diff[0] + 1) % MOD;
        g = polymul(&g, &diff);
        g.truncate(m2);
        m = m2;
    }
    g.truncate(n);
    g
}

fn special_component(n: usize) -> Vec<i64> {
    let inv2 = (MOD + 1) / 2;

    // V1(x) = product (1 - x^{2^r})^{-1}
    let mut v1 = vec![0i64; n + 1];
    v1[0] = 1;
    let mut p = 2i64;
    while p as usize <= n {
        for d in (p as usize)..=n {
            v1[d] = (v1[d] + v1[d - p as usize]) % MOD;
        }
        p <<= 1;
    }

    // Vminus(x) = product (1 + x^{2^r})^{-1}
    let mut vminus = vec![0i64; n + 1];
    vminus[0] = 1;
    p = 2;
    while p as usize <= n {
        let prev = vminus.clone();
        let mut new = vec![0i64; n + 1];
        for d in 0..=n {
            let mut val = prev[d];
            if d >= p as usize {
                val = (val - new[d - p as usize] + MOD) % MOD;
            }
            new[d] = val;
        }
        vminus = new;
        p <<= 1;
    }

    // P(x) = 1/2 * ((1+x)V1 + (1-x)Vminus)
    let mut pser = vec![0i64; n + 1];
    for d in 0..=n {
        let t1 = (v1[d] + if d > 0 { v1[d - 1] } else { 0 }) % MOD;
        let t2 = (vminus[d] - if d > 0 { vminus[d - 1] } else { 0 } + MOD) % MOD;
        pser[d] = ((t1 + t2) % MOD) * inv2 % MOD;
    }

    // Multiply by 1/(1-x): prefix sums
    let mut a = vec![0i64; n + 1];
    let mut s = 0i64;
    for d in 0..=n {
        s = (s + pser[d]) % MOD;
        a[d] = s;
    }

    // Multiply by 1/(1-x^2): b[d] = a[d] + b[d-2]
    let mut b = vec![0i64; n + 1];
    for d in 0..=n {
        let mut val = a[d];
        if d >= 2 {
            val = (val + b[d - 2]) % MOD;
        }
        b[d] = val;
    }
    b
}

fn sieve_primes(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b)
        .map(|(i, _)| i)
        .collect()
}

fn add_component_multiplicities(c: &mut [i64], n: usize) {
    let limit_phi = 2 * n;
    let primes: Vec<usize> = sieve_primes(limit_phi + 1)
        .into_iter()
        .filter(|&p| p % 2 == 1)
        .collect();

    fn dfs(
        c: &mut [i64],
        primes: &[usize],
        start_idx: usize,
        n_val: usize,
        phi_val: usize,
        n_limit: usize,
        limit_phi: usize,
    ) {
        if n_val > 1 {
            let ph = phi_val;
            let deg0 = ph / 2;
            let mut w = 0usize;
            let mut k = 0usize;
            loop {
                let deg = if k == 0 || k == 1 {
                    deg0
                } else {
                    ph * (1 << (k - 2))
                };
                w += deg;
                if w > n_limit {
                    break;
                }
                c[w] += 1;
                k += 1;
            }
        }

        for i in start_idx..primes.len() {
            let p = primes[i];
            if phi_val * (p - 1) > limit_phi {
                break;
            }

            let n2 = n_val * p;
            let phi2 = phi_val * (p - 1);
            dfs(c, primes, i + 1, n2, phi2, n_limit, limit_phi);

            let mut n_e = n2 * p;
            let mut phi_e = phi2 * p;
            while phi_e <= limit_phi {
                dfs(c, primes, i + 1, n_e, phi_e, n_limit, limit_phi);
                n_e *= p;
                phi_e *= p;
            }
        }
    }

    dfs(c, &primes, 0, 1, 1, n, limit_phi);
}

fn solve(n: usize) -> i64 {
    let mut c = vec![0i64; n + 1];
    add_component_multiplicities(&mut c, n);

    // Build h(x) = log(F)(x)
    let mut g = vec![0i64; n + 1];
    for d in 1..=n {
        let cd = c[d];
        if cd > 0 {
            let add = (d as i64 * (cd % MOD)) % MOD;
            for k in (d..=n).step_by(d) {
                g[k] = (g[k] + add) % MOD;
            }
        }
    }

    let mut invs = vec![0i64; n + 2];
    for i in 1..=n + 1 {
        invs[i] = mod_pow(i as i64, MOD - 2, MOD);
    }

    let mut h = vec![0i64; n + 1];
    for k in 1..=n {
        h[k] = g[k] * invs[k] % MOD;
    }

    let colored = poly_exp(&h, n + 1, &invs);
    let special = special_component(n);

    let total = polymul(&colored, &special);

    // Asserts from problem
    assert_eq!(total[2], 6);
    assert_eq!(total[5], 58);
    assert_eq!(total[20], 122087);

    total[n]
}

fn main() {
    println!("{}", solve(10000));
}
