// Project Euler 447: Retractions C
// F(N) = sum_g g*mu(g)*S(N/g^2) - N*(N+1)/2,  S(n) = sum_{k<=n} sigma(k)
// S(n) = sum_{i=1}^{isqrt(n)} (i*floor(n/i) + tri(floor(n/i))) - s*tri(s)

use rayon::prelude::*;

const BIG_N: u64 = 100_000_000_000_000; // 10^14
const MOD: u64 = 1_000_000_007;
const INV2: u64 = (MOD + 1) / 2;

#[inline(always)]
fn isqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut x = (n as f64).sqrt() as u64;
    if x == 0 {
        return 0;
    }
    let y = (x + n / x) >> 1;
    if y < x {
        x = y;
    }
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// n_mod = n % MOD, 0 <= n_mod < MOD. Returns n*(n+1)/2 % MOD.
#[inline(always)]
fn tri_from_mod(n_mod: u64) -> u64 {
    let prod = if n_mod & 1 == 0 {
        (n_mod >> 1) * (n_mod + 1)
    } else {
        n_mod * ((n_mod + 1) >> 1)
    };
    prod % MOD
}

#[inline(always)]
fn tri_mod(n: u64) -> u64 {
    let a = n % MOD;
    let b = (n + 1) % MOD;
    a * b % MOD * INV2 % MOD
}

#[inline(always)]
fn mul_coeff(coeff: i64, s: u64) -> u64 {
    let mut c = coeff % MOD as i64;
    if c < 0 {
        c += MOD as i64;
    }
    (c as u64) * s % MOD
}

/// S(n) = sum_{k=1}^n sigma(k)  (n > table bound; uses hyperbola, u64 mulmod)
fn sum_sigma_large(n: u64) -> u64 {
    let s = isqrt(n);
    let mut acc = 0u64;
    let mut i = 1u64;

    while i + 3 <= s {
        let q0 = n / i;
        let q1 = n / (i + 1);
        let q2 = n / (i + 2);
        let q3 = n / (i + 3);
        let r0 = q0 % MOD;
        let r1 = q1 % MOD;
        let r2 = q2 % MOD;
        let r3 = q3 % MOD;
        acc = acc
            .wrapping_add(i * r0)
            .wrapping_add(tri_from_mod(r0))
            .wrapping_add((i + 1) * r1)
            .wrapping_add(tri_from_mod(r1))
            .wrapping_add((i + 2) * r2)
            .wrapping_add(tri_from_mod(r2))
            .wrapping_add((i + 3) * r3)
            .wrapping_add(tri_from_mod(r3));
        i += 4;
        if i & 63 == 1 {
            acc %= MOD;
        }
    }
    while i <= s {
        let r = (n / i) % MOD;
        acc = acc.wrapping_add(i * r).wrapping_add(tri_from_mod(r));
        i += 1;
    }
    acc %= MOD;
    let sub = (s % MOD) * tri_from_mod(s % MOD) % MOD;
    if acc >= sub { acc - sub } else { acc + MOD - sub }
}

fn main() {
    let l = isqrt(BIG_N) as usize;

    let mut mu = vec![0i8; l + 1];
    let mut sig = vec![0u32; l + 1];
    let mut lp = vec![0u32; l + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(700_000);
    mu[1] = 1;
    sig[1] = 1;
    for i in 2..=l {
        if lp[i] == 0 {
            lp[i] = i as u32;
            primes.push(i as u32);
            mu[i] = -1;
            sig[i] = (i + 1) as u32;
        }
        let si = sig[i] as u64;
        for &p in &primes {
            let v = i * (p as usize);
            if v > l {
                break;
            }
            lp[v] = p;
            if i as u32 % p == 0 {
                mu[v] = 0;
                let sip = sig[i / (p as usize)] as u64;
                sig[v] = (si + p as u64 * (si - sip)) as u32;
                break;
            }
            mu[v] = -mu[i];
            sig[v] = (si * (p as u64 + 1)) as u32;
        }
    }
    drop(lp);
    drop(primes);

    // sp[n] = S(n) % MOD for n <= l
    let mut acc_s = 0u32;
    let mod_u32 = MOD as u32;
    for i in 1..=l {
        acc_s += sig[i];
        if acc_s >= mod_u32 {
            acc_s -= mod_u32;
        }
        sig[i] = acc_s;
    }
    let sp = sig;

    let mut pm = vec![0i64; l + 1];
    for g in 1..=l {
        pm[g] = pm[g - 1] + (g as i64) * (mu[g] as i64);
    }
    drop(mu);

    let l_u = l as u64;
    let mut large: Vec<(i64, u64)> = Vec::with_capacity(4096);
    let mut g = 1usize;
    while g <= l {
        let q = BIG_N / (g as u64 * g as u64);
        if q <= l_u {
            break;
        }
        let mut g_hi = isqrt(BIG_N / q) as usize;
        if g_hi > l {
            g_hi = l;
        }
        if g_hi < g {
            g_hi = g;
        }
        large.push((pm[g_hi] - pm[g - 1], q));
        g = g_hi + 1;
    }

    let ans_large: u64 = large
        .par_iter()
        .map(|&(coeff, q)| mul_coeff(coeff, sum_sigma_large(q)))
        .sum();

    let mut ans_small = 0u64;
    while g <= l {
        let q = BIG_N / (g as u64 * g as u64);
        if q == 0 {
            break;
        }
        let mut g_hi = isqrt(BIG_N / q) as usize;
        if g_hi > l {
            g_hi = l;
        }
        if g_hi < g {
            g_hi = g;
        }
        // SAFETY: q <= l after the large/small split; g, g_hi in 1..=l
        let s = unsafe { *sp.get_unchecked(q as usize) } as u64;
        ans_small += mul_coeff(pm[g_hi] - pm[g - 1], s);
        g = g_hi + 1;
    }

    let mut ans = (ans_large + ans_small) % MOD;
    let t = tri_mod(BIG_N);
    ans = if ans >= t { ans - t } else { ans + MOD - t };
    println!("{ans}");
}
