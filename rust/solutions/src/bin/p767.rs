// Project Euler 767 - Matrix Counting
// NTT-based modular convolution for counting 16xN matrices.
// B(K, N) where K=10^5, N=10^16, T=16 rows, answer mod 10^9+7.
// Uses 3 NTT primes + CRT since max convolution output ~ 10^23.

const MOD: u64 = 1_000_000_007;

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
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

// Three NTT primes, each with large power-of-2 factor and primitive root 3
const NTT_P1: u64 = 998244353;  // 2^23 * 119 + 1, prim root 3
const NTT_P2: u64 = 985661441;  // 2^22 * 5 * 47 + 1, prim root 3
const NTT_P3: u64 = 754974721;  // 2^24 * 45 + 1, prim root 11

fn ntt(a: &mut [u64], invert: bool, p: u64, g: u64) {
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

    let mut len = 2;
    while len <= n {
        let w = if invert {
            pow_mod(pow_mod(g, (p - 1) / len as u64, p), p - 2, p)
        } else {
            pow_mod(g, (p - 1) / len as u64, p)
        };
        for i in (0..n).step_by(len) {
            let mut wn = 1u64;
            for jj in 0..len / 2 {
                let u = a[i + jj];
                let v = (a[i + jj + len / 2] as u128 * wn as u128 % p as u128) as u64;
                a[i + jj] = (u + v) % p;
                a[i + jj + len / 2] = (u + p - v) % p;
                wn = (wn as u128 * w as u128 % p as u128) as u64;
            }
        }
        len <<= 1;
    }

    if invert {
        let inv_n = pow_mod(n as u64, p - 2, p);
        for x in a.iter_mut() {
            *x = (*x as u128 * inv_n as u128 % p as u128) as u64;
        }
    }
}

fn convolve_ntt(a: &[u64], b: &[u64], n: usize, p: u64, g: u64) -> Vec<u64> {
    let mut m = 1;
    while m < 2 * n {
        m *= 2;
    }

    let mut fa = vec![0u64; m];
    let mut fb = vec![0u64; m];
    for i in 0..n {
        fa[i] = a[i] % p;
        fb[i] = b[i] % p;
    }

    ntt(&mut fa, false, p, g);
    ntt(&mut fb, false, p, g);

    let mut fc = vec![0u64; m];
    for i in 0..m {
        fc[i] = (fa[i] as u128 * fb[i] as u128 % p as u128) as u64;
    }

    ntt(&mut fc, true, p, g);
    fc.truncate(2 * n - 1);
    fc
}

fn convolve_mod(a: &[u64], b: &[u64], n: usize) -> Vec<u64> {
    // Convolve mod MOD using three NTT primes and CRT
    let r1 = convolve_ntt(a, b, n, NTT_P1, 3);
    let r2 = convolve_ntt(a, b, n, NTT_P2, 3);
    let r3 = convolve_ntt(a, b, n, NTT_P3, 11);

    // 3-way CRT: recover x mod (P1*P2*P3), then reduce mod MOD
    // Step 1: combine r1, r2 to get x mod (P1*P2)
    let inv_p1_mod_p2 = pow_mod(NTT_P1 % NTT_P2, NTT_P2 - 2, NTT_P2);
    // Step 2: combine that with r3 to get x mod (P1*P2*P3)
    let p1p2 = NTT_P1 as u128 * NTT_P2 as u128;
    let p1p2_mod_p3 = (p1p2 % NTT_P3 as u128) as u64;
    let inv_p1p2_mod_p3 = pow_mod(p1p2_mod_p3, NTT_P3 - 2, NTT_P3);
    let p1p2_mod_mod = (p1p2 % MOD as u128) as u64;

    let len = r1.len();
    let mut result = vec![0u64; len];
    for i in 0..len {
        // Step 1: x12 = r1 + P1 * ((r2 - r1) * inv_p1 mod P2), x12 in [0, P1*P2)
        let diff12 = ((r2[i] as i128 - (r1[i] % NTT_P2) as i128 + NTT_P2 as i128) % NTT_P2 as i128) as u64;
        let t12 = (diff12 as u128 * inv_p1_mod_p2 as u128 % NTT_P2 as u128) as u64;
        let x12: u128 = r1[i] as u128 + NTT_P1 as u128 * t12 as u128; // x mod P1*P2

        // Step 2: x = x12 + P1*P2 * ((r3 - x12) * inv_p1p2 mod P3)
        let x12_mod_p3 = (x12 % NTT_P3 as u128) as u64;
        let diff3 = ((r3[i] as i128 - x12_mod_p3 as i128 + NTT_P3 as i128) % NTT_P3 as i128) as u64;
        let t3 = (diff3 as u128 * inv_p1p2_mod_p3 as u128 % NTT_P3 as u128) as u64;

        // x mod MOD = (x12 mod MOD + (P1*P2 mod MOD) * t3) mod MOD
        let x12_mod = (x12 % MOD as u128) as u64;
        let contribution = (p1p2_mod_mod as u128 * t3 as u128 % MOD as u128) as u64;
        result[i] = (x12_mod as u128 + contribution as u128) as u64 % MOD;
    }
    result
}

fn main() {
    let n: u64 = 10_000_000_000_000_000; // 10^16
    let k: usize = 100_000;
    let t: u64 = 16;

    // Precompute factorials mod MOD
    let mut fact = vec![1u64; k + 1];
    for i in 1..=k {
        fact[i] = (fact[i - 1] as u128 * i as u128 % MOD as u128) as u64;
    }
    let mut inv_fact = vec![1u64; k + 1];
    inv_fact[k] = pow_mod(fact[k], MOD - 2, MOD);
    for i in (0..k).rev() {
        inv_fact[i] = (inv_fact[i + 1] as u128 * (i + 1) as u128 % MOD as u128) as u64;
    }

    // coeffs[i] = (1/i!)^T mod MOD
    let mut coeffs = vec![0u64; k + 1];
    for i in 0..=k {
        coeffs[i] = pow_mod(inv_fact[i], t, MOD);
    }

    // Convolve coeffs with itself
    let p2 = convolve_mod(&coeffs, &coeffs, k + 1);

    // f[i] = i!^T * p2[i] mod MOD
    let mut f = vec![0u64; k + 1];
    for i in 0..=k {
        f[i] = (pow_mod(fact[i], t, MOD) as u128 * p2[i] as u128 % MOD as u128) as u64;
    }

    let base_val = pow_mod(2, n / k as u64, MOD);
    let term = (base_val + MOD - 2) % MOD;

    let mut ans: u64 = 0;
    let mut term_pow: u64 = 1;
    for i in 0..=k {
        let ncr = (fact[k] as u128 * inv_fact[i] as u128 % MOD as u128
            * inv_fact[k - i] as u128 % MOD as u128) as u64;
        ans = (ans as u128 + ncr as u128 * term_pow as u128 % MOD as u128
            * f[k - i] as u128 % MOD as u128) as u64 % MOD;
        term_pow = (term_pow as u128 * term as u128 % MOD as u128) as u64;
    }

    println!("{}", ans);
}
