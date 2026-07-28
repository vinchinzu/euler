// Project Euler 448: Average least common multiple
// Optimizations: FxHashMap, no dyn Fn, u64 modular arithmetic (MOD < 2^30).

use fxhash::FxHashMap;

const N: i64 = 99999999019;
const MOD: u64 = 999999017;

fn mod_inv(mut a: i64, m: i64) -> i64 {
    let (mut g, mut x, mut y) = (m, 0i64, 1i64);
    while a != 0 {
        let q = g / a;
        let t = g - q * a;
        g = a;
        a = t;
        let t = x - q * y;
        x = y;
        y = t;
    }
    ((x % m) + m) % m
}

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    (a * b) % MOD
}

#[inline(always)]
fn sum_sq(m: i64, inv6: u64) -> u64 {
    let mm = (m % MOD as i64) as u64;
    // mm*(mm+1)*(2mm+1)/6 mod MOD; all intermediates fit in u64 since MOD^2 < 2^64 / 4
    let t1 = mul_mod(mm, mm + 1);
    let t2 = mul_mod(t1, (2 * mm + 1) % MOD);
    mul_mod(t2, inv6)
}

fn k_phi_sum(
    n: i64,
    l: usize,
    k_phi_sum_small: &[u64],
    cache: &mut FxHashMap<i64, u64>,
    inv6: u64,
) -> u64 {
    if n <= 0 {
        return 0;
    }
    if n <= l as i64 {
        return k_phi_sum_small[n as usize];
    }
    if let Some(&v) = cache.get(&n) {
        return v;
    }

    let mut result = sum_sq(n, inv6) as i64;
    let sqrt_n = {
        let mut v = (n as f64).sqrt() as i64;
        while (v + 1) * (v + 1) <= n {
            v += 1;
        }
        while v * v > n {
            v -= 1;
        }
        v
    };

    for d in 2..=sqrt_n {
        let sub = mul_mod(
            k_phi_sum(n / d, l, k_phi_sum_small, cache, inv6),
            (d as u64) % MOD,
        );
        result = (result - sub as i64) % MOD as i64;
    }

    for q in 1..=sqrt_n {
        if n / q > sqrt_n {
            let d_lo = n / (q + 1) + 1;
            let d_hi = n / q;
            // sum of d from d_lo to d_hi = hi*(hi+1)/2 - (lo-1)*lo/2
            let sum_hi = (d_hi as u128) * ((d_hi + 1) as u128) / 2;
            let sum_lo = (d_lo as u128) * ((d_lo - 1) as u128) / 2;
            let sum_d = ((sum_hi - sum_lo) % MOD as u128) as u64;
            let sub = mul_mod(
                k_phi_sum(q, l, k_phi_sum_small, cache, inv6),
                sum_d,
            );
            result = (result - sub as i64) % MOD as i64;
        }
    }

    if result < 0 {
        result += MOD as i64;
    }
    let result = result as u64;
    cache.insert(n, result);
    result
}

fn main() {
    let inv6 = mod_inv(6, MOD as i64) as u64;
    let l = (N as f64).sqrt() as usize + 10;

    // Sieve phi
    let mut phi = vec![0i32; l + 1];
    for i in 0..=l {
        phi[i] = i as i32;
    }
    for i in 2..=l {
        if phi[i] == i as i32 {
            for j in (i..=l).step_by(i) {
                phi[j] -= phi[j] / i as i32;
            }
        }
    }

    let mut k_phi_sum_small = vec![0u64; l + 1];
    for k in 1..=l {
        let term = ((k as u64) % MOD) * ((phi[k] as u64) % MOD) % MOD;
        k_phi_sum_small[k] = (k_phi_sum_small[k - 1] + term) % MOD;
    }

    let mut cache: FxHashMap<i64, u64> = FxHashMap::default();
    cache.reserve(1 << 20);

    let threshold = (N / l as i64) as usize;
    let mut ans: u64 = 0;

    for k in 1..=threshold {
        let term = mul_mod(
            mul_mod((N / k as i64) as u64 % MOD, (k as u64) % MOD),
            phi[k] as u64,
        );
        ans = (ans + term) % MOD;
    }

    for q in 1..l {
        let t1 = k_phi_sum(N / q as i64, l, &k_phi_sum_small, &mut cache, inv6);
        let t2 = k_phi_sum(N / (q as i64 + 1), l, &k_phi_sum_small, &mut cache, inv6);
        let diff = (t1 + MOD - t2) % MOD;
        ans = (ans + mul_mod(diff, (q as u64) % MOD)) % MOD;
    }

    ans = (ans + (N as u64) % MOD) % MOD;
    let inv2 = mod_inv(2, MOD as i64) as u64;
    ans = mul_mod(ans, inv2);

    println!("{ans}");
}
