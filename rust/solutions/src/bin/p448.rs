// Project Euler 448: Average least common multiple
use std::collections::HashMap;

const N: i64 = 99999999019;
const MOD: i64 = 999999017;

fn mod_inv(mut a: i64, m: i64) -> i64 {
    let (mut g, mut x, mut y) = (m, 0i64, 1i64);
    while a != 0 {
        let q = g / a;
        let t = g - q * a; g = a; a = t;
        let t = x - q * y; x = y; y = t;
    }
    ((x % m) + m) % m
}

fn main() {
    let inv6 = mod_inv(6, MOD);
    let l = (N as f64).sqrt() as usize + 10;

    // Sieve phi
    let mut phi = vec![0i32; l + 1];
    for i in 0..=l { phi[i] = i as i32; }
    for i in 2..=l {
        if phi[i] == i as i32 {
            for j in (i..=l).step_by(i) {
                phi[j] -= phi[j] / i as i32;
            }
        }
    }

    let mut k_phi_sum_small = vec![0i64; l + 1];
    for k in 1..=l {
        k_phi_sum_small[k] = (k_phi_sum_small[k - 1] + k as i64 * phi[k] as i64) % MOD;
    }

    let mut cache: HashMap<i64, i64> = HashMap::new();

    let sum_sq = |m: i64| -> i64 {
        let mm = m % MOD;
        ((mm as i128 * (mm + 1) as i128 % MOD as i128 * (2 * mm + 1) as i128 % MOD as i128 * inv6 as i128) % MOD as i128) as i64
    };

    fn k_phi_sum(n: i64, l: usize, k_phi_sum_small: &[i64], cache: &mut HashMap<i64, i64>,
                 sum_sq: &dyn Fn(i64) -> i64) -> i64 {
        if n <= 0 { return 0; }
        if n <= l as i64 { return k_phi_sum_small[n as usize]; }
        if let Some(&v) = cache.get(&n) { return v; }

        let mut result = sum_sq(n);
        let sqrt_n = {
            let mut v = (n as f64).sqrt() as i64;
            while (v + 1) * (v + 1) <= n { v += 1; }
            while v * v > n { v -= 1; }
            v
        };

        for d in 2..=sqrt_n {
            result = (result - k_phi_sum(n / d, l, k_phi_sum_small, cache, sum_sq) * (d % MOD)) % MOD;
        }

        for q in 1..=sqrt_n {
            if n / q > sqrt_n {
                let d_lo = n / (q + 1) + 1;
                let d_hi = n / q;
                let sum_d = ((d_hi as i128 * (d_hi + 1) as i128 / 2 - d_lo as i128 * (d_lo - 1) as i128 / 2) % MOD as i128 + MOD as i128) as i64 % MOD;
                result = (result - k_phi_sum(q, l, k_phi_sum_small, cache, sum_sq) * sum_d) % MOD;
            }
        }

        if result < 0 { result += MOD; }
        cache.insert(n, result);
        result
    }

    let threshold = (N / l as i64) as usize;
    let mut ans: i64 = 0;

    for k in 1..=threshold {
        let term = (N / k as i64) % MOD * (k as i64 % MOD) % MOD * (phi[k] as i64) % MOD;
        ans = (ans + term) % MOD;
    }

    for q in 1..l {
        let t1 = k_phi_sum(N / q as i64, l, &k_phi_sum_small, &mut cache, &sum_sq);
        let t2 = k_phi_sum(N / (q as i64 + 1), l, &k_phi_sum_small, &mut cache, &sum_sq);
        let diff = (t1 - t2 + MOD) % MOD;
        ans = (ans + diff * (q as i64 % MOD)) % MOD;
    }

    ans = (ans + N % MOD) % MOD;
    let inv2 = mod_inv(2, MOD);
    ans = ans * inv2 % MOD;

    println!("{ans}");
}
