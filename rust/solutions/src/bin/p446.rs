// Project Euler 446 - Retractions B
// F(N) = sum_{n=1}^N R(n^4 + 4), using Sophie Germain identity and sieve.

const N: usize = 10_000_000;
const MOD: i64 = 1_000_000_007;

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
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

fn tonelli_shanks(a: i64, p: i64) -> i64 {
    let a = ((a % p) + p) % p;
    if a == 0 { return 0; }

    let test = mod_pow(a, (p - 1) / 2, p);
    if test != 1 { return -1; }

    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 { q /= 2; s += 1; }

    let mut z = 2i64;
    loop {
        if mod_pow(z, (p - 1) / 2, p) == p - 1 { break; }
        z += 1;
    }

    let mut m = s;
    let mut c = mod_pow(z, q, p);
    let mut t = mod_pow(a, q, p);
    let mut r = mod_pow(a, (q + 1) / 2, p);

    loop {
        if t == 1 { return r; }
        let mut i = 1;
        let mut temp = (t as i128 * t as i128 % p as i128) as i64;
        while temp != 1 {
            temp = (temp as i128 * temp as i128 % p as i128) as i64;
            i += 1;
        }
        let mut b = c;
        for _ in 0..(m - i - 1) {
            b = (b as i128 * b as i128 % p as i128) as i64;
        }
        m = i;
        c = (b as i128 * b as i128 % p as i128) as i64;
        t = (t as i128 * c as i128 % p as i128) as i64;
        r = (r as i128 * b as i128 % p as i128) as i64;
    }
}

fn main() {
    // Sieve primes up to N+1
    let mut is_prime = vec![true; N + 2];
    is_prime[0] = false;
    is_prime[1] = false;
    {
        let mut i = 2;
        while (i as u64) * (i as u64) <= (N + 1) as u64 {
            if is_prime[i] {
                let mut j = i * i;
                while j <= N + 1 { is_prime[j] = false; j += i; }
            }
            i += 1;
        }
    }

    // factors[k] = k^2 + 1
    let mut factors: Vec<i64> = (0..=N + 1).map(|k| k as i64 * k as i64 + 1).collect();
    let mut res: Vec<i64> = vec![1; N + 3];

    // Remove factor of 2
    for k in (1..=N + 1).step_by(2) {
        factors[k] /= 2;
    }

    // res for even indices: 5
    for i in (2..=N).step_by(2) {
        res[i] = 5;
    }

    // Process primes ≡ 1 mod 4
    for p in 5..=N + 1 {
        if !is_prime[p] || p % 4 != 1 { continue; }

        let sqrt_neg1 = tonelli_shanks(p as i64 - 1, p as i64);
        if sqrt_neg1 < 0 { continue; }

        let mut roots = [sqrt_neg1 % p as i64, (p as i64 - sqrt_neg1) % p as i64];
        roots.sort();
        let nroots = if roots[0] == roots[1] { 1 } else { 2 };

        for ri in 0..nroots {
            let start = roots[ri] as usize;
            let mut k = start;
            while k <= N + 1 {
                let mut pw = 1i64;
                while factors[k] % p as i64 == 0 {
                    factors[k] /= p as i64;
                    pw *= p as i64;
                }
                let term = (1 + pw) % MOD;
                if k >= 1 {
                    res[k - 1] = res[k - 1] * term % MOD;
                }
                if k + 1 <= N + 2 {
                    res[k + 1] = res[k + 1] * term % MOD;
                }
                k += p;
            }
        }
    }

    // Remaining large prime factors
    for k in 0..=N + 1 {
        if factors[k] > 1 {
            let term = (1 + factors[k]) % MOD;
            if k >= 1 {
                res[k - 1] = res[k - 1] * term % MOD;
            }
            if k + 1 <= N + 2 {
                res[k + 1] = res[k + 1] * term % MOD;
            }
        }
    }

    let mut ans: i64 = 0;
    for i in 1..=N {
        let n2 = (i as i64 % MOD) * (i as i64 % MOD) % MOD;
        let n4 = n2 * n2 % MOD;
        let n4p4 = (n4 + 4) % MOD;
        ans = (ans + res[i] - n4p4 + MOD) % MOD;
    }

    println!("{}", ans);
}
