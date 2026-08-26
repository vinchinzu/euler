// Problem 976: XO Game
// Ported from python/976.py. Specialized for n = k = 10^7 (e != 0, c == 0).

use rayon::prelude::*;

const MOD: u64 = 1_234_567_891;

#[inline(always)]
const fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

#[inline(always)]
const fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}

#[inline(always)]
const fn half(a: u64) -> u64 {
    if a & 1 == 0 {
        a >> 1
    } else {
        (a + MOD) >> 1
    }
}

fn build_inverses(n: usize) -> Vec<u32> {
    let mut inv = Vec::with_capacity(n + 1);
    unsafe {
        inv.set_len(n + 1);
        let p = inv.as_mut_ptr();
        *p = 0;
        if n >= 1 {
            *p.add(1) = 1;
        }
        const MOD32: u32 = MOD as u32;
        let mut i = 2usize;
        // SAFETY: for i >= 2, remainder MOD % i is in 1..i, so inv[r] is already filled.
        while i + 7 <= n {
            let i0 = i as u32;
            let i1 = i0 + 1;
            let i2 = i0 + 2;
            let i3 = i0 + 3;
            let i4 = i0 + 4;
            let i5 = i0 + 5;
            let i6 = i0 + 6;
            let i7 = i0 + 7;
            let q0 = MOD32 / i0;
            let r0 = MOD32 % i0;
            let q1 = MOD32 / i1;
            let r1 = MOD32 % i1;
            let q2 = MOD32 / i2;
            let r2 = MOD32 % i2;
            let q3 = MOD32 / i3;
            let r3 = MOD32 % i3;
            let q4 = MOD32 / i4;
            let r4 = MOD32 % i4;
            let q5 = MOD32 / i5;
            let r5 = MOD32 % i5;
            let q6 = MOD32 / i6;
            let r6 = MOD32 % i6;
            let q7 = MOD32 / i7;
            let r7 = MOD32 % i7;
            *p.add(i) = (MOD - q0 as u64 * *p.add(r0 as usize) as u64 % MOD) as u32;
            *p.add(i + 1) = (MOD - q1 as u64 * *p.add(r1 as usize) as u64 % MOD) as u32;
            *p.add(i + 2) = (MOD - q2 as u64 * *p.add(r2 as usize) as u64 % MOD) as u32;
            *p.add(i + 3) = (MOD - q3 as u64 * *p.add(r3 as usize) as u64 % MOD) as u32;
            *p.add(i + 4) = (MOD - q4 as u64 * *p.add(r4 as usize) as u64 % MOD) as u32;
            *p.add(i + 5) = (MOD - q5 as u64 * *p.add(r5 as usize) as u64 % MOD) as u32;
            *p.add(i + 6) = (MOD - q6 as u64 * *p.add(r6 as usize) as u64 % MOD) as u32;
            *p.add(i + 7) = (MOD - q7 as u64 * *p.add(r7 as usize) as u64 % MOD) as u32;
            i += 8;
        }
        while i <= n {
            let iu = i as u32;
            let q = MOD32 / iu;
            let r = MOD32 % iu;
            *p.add(i) = (MOD - q as u64 * *p.add(r as usize) as u64 % MOD) as u32;
            i += 1;
        }
    }
    inv
}

fn rising_binom(e: usize, t: usize, inv: &[u32]) -> u64 {
    // C(e + t - 1, t) = prod_{j=0}^{t-1} (e + j) * inv[j + 1]
    if t == 0 {
        return 1;
    }
    let threads = rayon::current_num_threads().max(1);
    if t < 4096 || threads == 1 {
        let mut p = 1u64;
        unsafe {
            for j in 0..t {
                p = mul_mod(p, (e + j) as u64);
                p = mul_mod(p, *inv.get_unchecked(j + 1) as u64);
            }
        }
        return p;
    }
    let chunk = (t + threads - 1) / threads;
    (0..threads)
        .into_par_iter()
        .map(|c| {
            let start = c * chunk;
            let end = (start + chunk).min(t);
            if start >= end {
                return 1u64;
            }
            let mut p = 1u64;
            unsafe {
                for j in start..end {
                    p = mul_mod(p, (e + j) as u64);
                    p = mul_mod(p, *inv.get_unchecked(j + 1) as u64);
                }
            }
            p
        })
        .reduce(|| 1u64, mul_mod)
}

fn solve() -> u64 {
    const N: usize = 10_000_000;
    const K: usize = 10_000_000;

    let e = N / 2;
    let a_cnt = (N + 3) / 4;
    let ab = e;

    let inv = build_inverses(e + K + 2);
    let inv_ptr = inv.as_ptr();

    let (mut total_even, mut e0) = rayon::join(
        || rising_binom(e, K, &inv),
        || rising_binom(e, K / 2, &inv),
    );

    unsafe {
        // s = 0: h = 1, q = 1, coeff = 1, h_odd_a = 0
        let mut h = 1u64;
        let mut q = 1u64;
        let mut sum_even = 1u64;
        let mut sum_odd = 0u64;
        let mut sum_odd_a = 0u64;

        // m = K (even): t0 = 0, t1 = (total_even - e0) * 1
        let mut ans = sub_mod(total_even, e0);

        // SAFETY: e + K - 1 < inv.len()
        total_even = mul_mod(mul_mod(total_even, K as u64), *inv_ptr.add(e + K - 1) as u64);
        e0 = mul_mod(
            mul_mod(e0, (K / 2) as u64),
            *inv_ptr.add(e + K / 2 - 1) as u64,
        );

        let mut m = K - 1;
        let mut s = 1usize;
        while s < K {
            // odd s
            h = mul_mod(mul_mod(h, (ab + s - 1) as u64), *inv_ptr.add(s) as u64);
            sum_odd += h;
            if sum_odd >= MOD {
                sum_odd -= MOD;
            }
            sum_odd_a += half(h);
            if sum_odd_a >= MOD {
                sum_odd_a -= MOD;
            }
            ans += mul_mod(total_even, sum_odd);
            total_even = mul_mod(mul_mod(total_even, m as u64), *inv_ptr.add(e + m - 1) as u64);
            m -= 1;
            s += 1;

            // even s
            h = mul_mod(mul_mod(h, (ab + s - 1) as u64), *inv_ptr.add(s) as u64);
            let r = s >> 1;
            q = mul_mod(mul_mod(q, (a_cnt + r - 1) as u64), *inv_ptr.add(r) as u64);
            let h_odd_a = half(sub_mod(h, q));
            sum_even += h;
            if sum_even >= MOD {
                sum_even -= MOD;
            }
            sum_odd_a += h_odd_a;
            if sum_odd_a >= MOD {
                sum_odd_a -= MOD;
            }
            ans += mul_mod(e0, sum_odd_a);
            ans += mul_mod(sub_mod(total_even, e0), sum_even);
            if m > 0 {
                total_even =
                    mul_mod(mul_mod(total_even, m as u64), *inv_ptr.add(e + m - 1) as u64);
            }
            if m >= 2 {
                let qcur = m >> 1;
                e0 = mul_mod(mul_mod(e0, qcur as u64), *inv_ptr.add(e + qcur - 1) as u64);
            }
            m -= 1;
            s += 1;
        }

        ans % MOD
    }
}

fn main() {
    println!("{}", solve());
}
