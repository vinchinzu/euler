// Project Euler 325: Stone Game II
// Recursive O(log N) computation using Beatty/lattice-point identities.

use euler_utils::mod_pow;

static mut MOD: u64 = 0;
static mut INV2: u64 = 0;
static mut INV6: u64 = 0;

fn isqrt_u128(n: u128) -> u64 {
    if n <= 1 { return n as u64; }
    let mut x = (n as f64).sqrt() as u128;
    loop {
        let x1 = (x + n / x) / 2;
        if x1 >= x { break; }
        x = x1;
    }
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x as u64
}

fn floor_phi_inv(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let n2 = n as u128 * n as u128;
    let s = isqrt_u128(5 * n2);
    ((s as i64) - n) / 2
}

fn floor_phi(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let n2 = n as u128 * n as u128;
    let s = isqrt_u128(5 * n2);
    ((s as i64) + n) / 2
}

fn sum_k_mod(n: i64) -> u64 {
    unsafe {
        let m = MOD;
        let nm = ((n % m as i64 + m as i64) % m as i64) as u128;
        let np1 = (((n + 1) % m as i64 + m as i64) % m as i64) as u128;
        ((nm * np1 % m as u128) * INV2 as u128 % m as u128) as u64
    }
}

fn sum_k2_mod(n: i64) -> u64 {
    unsafe {
        let m = MOD;
        let nm = ((n % m as i64 + m as i64) % m as i64) as u128;
        let np1 = (((n + 1) % m as i64 + m as i64) % m as i64) as u128;
        let n2p1 = (((2 * nm as u64 % m + 1) % m) as u128);
        ((nm * np1 % m as u128 * n2p1 % m as u128) * INV6 as u128 % m as u128) as u64
    }
}

struct Sums { s01: u64, s02: u64, s11: u64 }

fn compute(n: i64, memo: &mut Vec<(i64, Sums)>) -> Sums {
    if n <= 0 { return Sums { s01: 0, s02: 0, s11: 0 }; }

    for entry in memo.iter() {
        if entry.0 == n { return Sums { s01: entry.1.s01, s02: entry.1.s02, s11: entry.1.s11 }; }
    }

    let m = floor_phi_inv(n);
    if m == 0 {
        memo.push((n, Sums { s01: 0, s02: 0, s11: 0 }));
        return Sums { s01: 0, s02: 0, s11: 0 };
    }

    let sm = compute(m, memo);
    let sm1 = compute(m - 1, memo);

    let modv = unsafe { MOD };
    let inv2 = unsafe { INV2 };
    let inv6 = unsafe { INV6 };

    let p = floor_phi(m);
    let q = n - p;
    let (p, q) = if q < 0 { (n, 0i64) } else { (p, q) };

    let nm = |v: i64| -> u64 { ((v % modv as i64 + modv as i64) % modv as i64) as u64 };

    let v_mod = nm(n);
    let m_mod = nm(m);
    let m1_mod = nm(m - 1);
    let p_mod = nm(p);
    let q_mod = nm(q);

    // S01(n) = n*m - m*(m+1)/2 - S01(m)
    let t1 = (v_mod as u128 * m_mod as u128 % modv as u128) as u64;
    let t2 = (m_mod as u128 * ((m_mod + 1) % modv) as u128 % modv as u128 * inv2 as u128 % modv as u128) as u64;
    let s01 = ((t1 as u128 + modv as u128 * 2 - t2 as u128 - sm.s01 as u128) % modv as u128) as u64;

    // Helpers for S02 and S11
    let sk2m1 = (m1_mod as u128 * ((m1_mod + 1) % modv) as u128 % modv as u128
        * ((2 * m1_mod % modv + 1) % modv) as u128 % modv as u128
        * inv6 as u128 % modv as u128) as u64;
    let skm1 = (m1_mod as u128 * ((m1_mod + 1) % modv) as u128 % modv as u128
        * inv2 as u128 % modv as u128) as u64;

    let s11_phi_m1 = (sk2m1 as u128 + sm1.s11 as u128) % modv as u128;
    let s01_phi_m1 = (skm1 as u128 + sm1.s01 as u128) % modv as u128;

    let s02_a = ((m1_mod as u128 * m1_mod as u128 % modv as u128 * p_mod as u128 % modv as u128
        + modv as u128 * 4
        - 2 * s11_phi_m1 % modv as u128
        + s01_phi_m1) % modv as u128) as u64;

    let s02 = ((s02_a as u128 + m_mod as u128 * m_mod as u128 % modv as u128 * q_mod as u128 % modv as u128) % modv as u128) as u64;

    let s01_a = ((s01 as u128 + modv as u128 - m_mod as u128 * q_mod as u128 % modv as u128) % modv as u128) as u64;
    let s02_phi_m1 = ((sk2m1 as u128 + 2 * sm1.s11 as u128 % modv as u128 + sm1.s02 as u128 + modv as u128) % modv as u128) as u64;

    let s11_a_raw = ((m1_mod as u128 * p_mod as u128 % modv as u128 * p_mod as u128 % modv as u128
        + modv as u128 * 4
        - s02_phi_m1 as u128
        + s01_a as u128) % modv as u128) as u64;
    let s11_a = (s11_a_raw as u128 * inv2 as u128 % modv as u128) as u64;

    let sum_k_n_v = sum_k_mod(n);
    let sum_k_p_v = sum_k_mod(p);
    let sum_tail = ((sum_k_n_v as u128 + modv as u128 - sum_k_p_v as u128) % modv as u128) as u64;
    let s11 = ((s11_a as u128 + m_mod as u128 * sum_tail as u128 % modv as u128) % modv as u128) as u64;

    let result = Sums { s01, s02, s11 };
    memo.push((n, Sums { s01, s02, s11 }));
    result
}

fn main() {
    let n_val: i64 = 10_000_000_000_000_000;

    let mut m: u64 = 1;
    for _ in 0..10 { m *= 7; }
    unsafe {
        MOD = m;
        let phi_m = m / 7 * 6;
        INV2 = mod_pow(2, phi_m - 1, m);
        INV6 = mod_pow(6, phi_m - 1, m);
    }
    let modv = m;
    let inv2 = unsafe { INV2 };

    let mut memo = Vec::new();
    let s = compute(n_val, &mut memo);

    let sum_y = sum_k_mod(n_val);
    let sum_y2 = sum_k2_mod(n_val);

    let part1 = (3u128 * ((sum_y2 as u128 + modv as u128 - sum_y as u128) % modv as u128) % modv as u128 * inv2 as u128 % modv as u128) as u64;
    let part2 = ((s.s02 as u128 + s.s01 as u128) % modv as u128 * inv2 as u128 % modv as u128) as u64;
    let part3 = s.s11;

    let ans = ((part1 as u128 + modv as u128 * 2 - part2 as u128 - part3 as u128) % modv as u128) as u64;

    println!("{}", ans);
}
