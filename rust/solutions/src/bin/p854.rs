// Project Euler 854 - Pisano Periods 2
// Product of Pisano period related values modulo 1234567891

use std::collections::HashMap;

const MOD: i64 = 1_234_567_891;
const LIMIT: usize = 1_000_000;
const SPF_LIMIT: usize = 2_000_100;

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base = base.rem_euclid(m);
    while exp > 0 {
        if exp & 1 == 1 { r = r * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn mod_inv(a: i64, m: i64) -> i64 { mod_pow(a, m - 2, m) }

#[derive(Clone, Copy)]
struct Mat2 { a: i64, b: i64, c: i64, d: i64 }

fn mat_mul(a: Mat2, b: Mat2, m: i64) -> Mat2 {
    Mat2 {
        a: (a.a * b.a + a.b * b.c) % m,
        b: (a.a * b.b + a.b * b.d) % m,
        c: (a.c * b.a + a.d * b.c) % m,
        d: (a.c * b.b + a.d * b.d) % m,
    }
}

fn mat_pow(mut a: Mat2, mut p: i64, m: i64) -> Mat2 {
    let mut res = Mat2 { a: 1, b: 0, c: 0, d: 1 };
    while p > 0 {
        if p & 1 == 1 { res = mat_mul(res, a, m); }
        a = mat_mul(a, a, m);
        p >>= 1;
    }
    res
}

fn fib_n_mod_m(n: i64, m: i64) -> i64 {
    if n == 0 { return 0; }
    if n == 1 { return 1 % m; }
    if m == 1 { return 0; }
    let mm = Mat2 { a: 1, b: 1, c: 1, d: 0 };
    let r = mat_pow(mm, n - 1, m);
    r.a
}

fn get_divisors(d: i64, spf: &[i32]) -> Vec<i32> {
    let mut factors: Vec<(i64, i32)> = Vec::new();
    let mut temp = d;
    while temp > 1 && (temp as usize) <= SPF_LIMIT {
        let p = spf[temp as usize] as i64;
        let mut cnt = 0;
        while temp % p == 0 { temp /= p; cnt += 1; }
        factors.push((p, cnt));
    }
    if temp > 1 {
        factors.push((temp, 1));
    }

    let mut divs = vec![1i64];
    for &(p, e) in &factors {
        let prev_count = divs.len();
        let mut pk = 1i64;
        for _ in 0..e {
            pk *= p;
            for k in 0..prev_count {
                let d = divs[k] * pk;
                if d <= 2_000_000_000 {
                    divs.push(d);
                }
            }
        }
    }
    divs.sort();
    divs.iter().map(|&x| x as i32).collect()
}

fn get_z_rank(p: i32, spf: &[i32]) -> i32 {
    if p == 2 { return 3; }
    if p == 5 { return 5; }
    let d: i64 = if p % 5 == 1 || p % 5 == 4 {
        p as i64 - 1
    } else {
        2 * (p as i64 + 1)
    };
    let divs = get_divisors(d, spf);
    for &dv in &divs {
        if fib_n_mod_m(dv as i64, p as i64) == 0 {
            return dv;
        }
    }
    d as i32
}

fn get_ratio(m: i32) -> i32 {
    if m == 3 { return 1; }
    if m % 2 != 0 { return 4; }
    if m % 4 == 2 { return 1; }
    if m % 4 == 0 { return 2; }
    0
}

fn get_v2(mut n: i32) -> i32 {
    let mut c = 0;
    while n > 0 && n % 2 == 0 { c += 1; n /= 2; }
    c
}

fn get_vp(mut n: i32, p: i32) -> i32 {
    let mut c = 0;
    while n > 0 && n % p == 0 { c += 1; n /= p; }
    c
}

fn get_v2_f(m: i32) -> i32 {
    if m % 3 != 0 { return 0; }
    if m % 6 != 0 { return 1; }
    get_v2(m) + 2
}

fn get_vp_f(m: i32, z: i32, p: i32) -> i32 {
    1 + get_vp(m / z, p)
}

fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn main() {
    // Build SPF
    let mut spf = vec![0i32; SPF_LIMIT + 1];
    for i in 0..=SPF_LIMIT { spf[i] = i as i32; }
    for i in 2..=SPF_LIMIT {
        if spf[i] == i as i32 {
            let mut j = i * i;
            while j <= SPF_LIMIT {
                if spf[j] == j as i32 { spf[j] = i as i32; }
                j += i;
            }
        }
    }

    let small_primes: Vec<i32> = (2..=LIMIT).filter(|&i| spf[i] == i as i32).map(|i| i as i32).collect();

    // Compute z_vals
    let mut z_vals = vec![0i32; LIMIT + 1];
    for &p in &small_primes {
        z_vals[p as usize] = get_z_rank(p, &spf);
    }

    // Updates by period
    let mut ubp: HashMap<i32, i64> = HashMap::new();

    for &p in &small_primes {
        if p == 2 {
            let mut period = 3i32;
            while period <= LIMIT as i32 {
                let e = ubp.entry(period).or_insert(1);
                *e = *e * 2 % MOD;
                period *= 2;
            }
        } else {
            let z = z_vals[p as usize];
            let mut curr_z = z as i64;
            loop {
                let ratio = get_ratio(curr_z as i32);
                let period = curr_z * ratio as i64;
                if period > LIMIT as i64 { break; }
                let e = ubp.entry(period as i32).or_insert(1);
                *e = *e * p as i64 % MOD;
                curr_z *= p as i64;
                if curr_z > LIMIT as i64 && period * p as i64 > LIMIT as i64 { break; }
            }
        }
    }

    // Compute G_m (Fibonacci values)
    let mut g = vec![0i64; LIMIT + 1];
    {
        let (mut a, mut b) = (0i64, 1i64);
        g[1] = 1;
        for i in 2..=LIMIT {
            let c = (a + b) % MOD;
            a = b; b = c;
            g[i] = b;
        }
    }

    // Sieve G_m: remove factors at multiples
    for i in 1..=LIMIT {
        if g[i] <= 1 { continue; }
        let inv_g = mod_inv(g[i], MOD);
        let mut j = 2 * i;
        while j <= LIMIT { g[j] = g[j] * inv_g % MOD; j += i; }
    }

    // Remove small prime contributions
    for &p in &small_primes {
        let z = z_vals[p as usize];
        let max_k = LIMIT as i32 / z;
        if max_k == 0 { continue; }

        let mut counts_buf = vec![0i32; max_k as usize + 1];
        for k in 1..=max_k as usize {
            counts_buf[k] = if p == 2 {
                get_v2_f((k as i32) * z)
            } else {
                get_vp_f((k as i32) * z, z, p)
            };
        }

        // Mobius-like sieve
        for k in 1..=max_k as usize {
            if counts_buf[k] == 0 { continue; }
            let c = counts_buf[k];
            let mut m = 2 * k;
            while m <= max_k as usize { counts_buf[m] -= c; m += k; }
        }

        let inv_p = mod_inv(p as i64, MOD);
        for k in 1..=max_k as usize {
            let c = counts_buf[k];
            if c > 0 {
                let m = k as i32 * z;
                if g[m as usize] > 0 {
                    let term = mod_pow(inv_p, c as i64, MOD);
                    g[m as usize] = g[m as usize] * term % MOD;
                }
            }
        }
    }

    // Add large G_m updates
    for m in 1..=LIMIT {
        if g[m] > 1 {
            let ratio = get_ratio(m as i32);
            let period = m as i64 * ratio as i64;
            if period <= LIMIT as i64 {
                let e = ubp.entry(period as i32).or_insert(1);
                *e = *e * g[m] % MOD;
            }
        }
    }

    // Execute updates
    let mut used_periods: Vec<i32> = ubp.keys().cloned().collect();
    used_periods.sort();

    let mut m_arr = vec![1i64; LIMIT + 1];
    let mut l_arr = vec![1i64; LIMIT + 1];

    for &w in &used_periods {
        let val = *ubp.get(&w).unwrap_or(&1);
        if val == 1 { continue; }
        let mut p = w as usize;
        while p <= LIMIT {
            m_arr[p] = m_arr[p] * val % MOD;
            let x = l_arr[p];
            let wl = w as i64;
            if x % wl != 0 {
                if wl % x == 0 {
                    l_arr[p] = wl;
                } else {
                    l_arr[p] = x / gcd_ll(x, wl) * wl;
                }
            }
            p += w as usize;
        }
    }

    let mut total_prod = 1i64;
    for p in 1..=LIMIT {
        if l_arr[p] == p as i64 {
            total_prod = total_prod * m_arr[p] % MOD;
        }
    }

    println!("{}", total_prod);
}
