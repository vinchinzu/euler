// Project Euler 956 - D(1000*, 1000) mod 999999001
// Uses DFT-based approach with roots of unity

const MOD: u64 = 999_999_001;

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: u64, m: u64) -> u64 {
    mod_pow(a, m - 2, m)
}

fn sieve_primes(limit: usize) -> Vec<usize> {
    let mut is_p = vec![true; limit + 1];
    is_p[0] = false;
    if limit >= 1 { is_p[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= limit { is_p[j] = false; j += i; }
        }
        i += 1;
    }
    (2..=limit).filter(|&i| is_p[i]).collect()
}

fn is_primitive_root(g: u64, m: u64, factors: &[u64]) -> bool {
    let phi = m - 1;
    factors.iter().all(|&f| mod_pow(g, phi / f, m) != 1)
}

fn main() {
    let n = 1000usize;
    let m = 1000usize;
    let phi_factors: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 37];

    // Find primitive root of MOD
    let mut g = 2u64;
    while !is_primitive_root(g, MOD, &phi_factors) { g += 1; }

    let primes = sieve_primes(n);

    // Compute exponents of each prime in 1000-bigstar
    let mut exponents = Vec::with_capacity(primes.len());
    for &pp in &primes {
        let mut e = 0u64;
        for k in 1..=n {
            let mut vk = 0u64;
            let mut pk = pp;
            while pk <= k {
                vk += k as u64 / pk as u64;
                pk *= pp;
            }
            e += (n - k + 1) as u64 * vk;
        }
        exponents.push(e);
    }

    let phi = MOD - 1;
    let omega = mod_pow(g, phi / m as u64, MOD);
    let inv_m = mod_inv(m as u64, MOD);

    let mut sum_s = 0u64;
    for j in 0..m {
        let y = mod_pow(omega, j as u64, MOD);
        let mut prod = 1u64;
        for (idx, &pp) in primes.iter().enumerate() {
            let e = exponents[idx];
            let r = (pp as u128 * y as u128 % MOD as u128) as u64;
            let geo = if r == 1 {
                (e + 1) % MOD
            } else {
                let exp_mod = (e + 1) % phi;
                let re_plus1 = mod_pow(r, exp_mod, MOD);
                let num = (1 + MOD + MOD - re_plus1) % MOD;
                let den = (1 + MOD + MOD - r) % MOD;
                let den_inv = mod_inv(den, MOD);
                (num as u128 * den_inv as u128 % MOD as u128) as u64
            };
            prod = (prod as u128 * geo as u128 % MOD as u128) as u64;
        }
        sum_s = (sum_s + prod) % MOD;
    }

    let result = (sum_s as u128 * inv_m as u128 % MOD as u128) as u64;
    println!("{}", result);
}
