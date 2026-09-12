// Project Euler 910 — Phi-recursion + CRT solver
// Optimized: streaming jump-table approach (2 levels in memory at a time)
// + u32 tables + AVX2 gather compose/apply (swap buffers, no per-level memcpy)

use std::arch::x86_64::*;

const MOD: u64 = 1_000_000_000;
const M1: u64 = 512; // 2^9
const M2: u64 = 1_953_125; // 5^9

const A: usize = 12;
const B: u64 = 345_678;
const C: u64 = 9_012_345;
const D: u64 = 678;
const E: u64 = 90;

fn bit_len(n: u64) -> usize {
    64 - n.leading_zeros() as usize
}

#[inline(always)]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    a * b % m
}

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    base %= m;
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    result
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = egcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

fn mod_inverse(a: u64, m: u64) -> u64 {
    let (g, x, _) = egcd(a as i128, m as i128);
    assert_eq!(g, 1);
    ((x.rem_euclid(m as i128)) as u64) % m
}

fn crt(x1: u64, m1: u64, x2: u64, m2: u64) -> u64 {
    let q = mod_inverse(m1, m2);
    let delta = (x2 + m2 - (x1 % m2)) % m2;
    let k = mul_mod(delta, q, m2);
    (x1 as u128 + m1 as u128 * k as u128) as u64 % MOD
}

#[target_feature(enable = "avx2")]
unsafe fn apply_gather_avx2(values: *mut u32, table: *const u32, n: usize) {
    unsafe {
        let base = table as *const i32;
        let mut i = 0;
        while i + 16 <= n {
            let idx0 = _mm256_loadu_si256(values.add(i) as *const __m256i);
            let idx1 = _mm256_loadu_si256(values.add(i + 8) as *const __m256i);
            let g0 = _mm256_i32gather_epi32(base, idx0, 4);
            let g1 = _mm256_i32gather_epi32(base, idx1, 4);
            _mm256_storeu_si256(values.add(i) as *mut __m256i, g0);
            _mm256_storeu_si256(values.add(i + 8) as *mut __m256i, g1);
            i += 16;
        }
        while i < n {
            *values.add(i) = *table.add(*values.add(i) as usize);
            i += 1;
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn compose_gather_avx2(src: *const u32, dst: *mut u32, n: usize) {
    unsafe {
        let base = src as *const i32;
        let mut i = 0;
        while i + 16 <= n {
            let idx0 = _mm256_loadu_si256(src.add(i) as *const __m256i);
            let idx1 = _mm256_loadu_si256(src.add(i + 8) as *const __m256i);
            let g0 = _mm256_i32gather_epi32(base, idx0, 4);
            let g1 = _mm256_i32gather_epi32(base, idx1, 4);
            _mm256_storeu_si256(dst.add(i) as *mut __m256i, g0);
            _mm256_storeu_si256(dst.add(i + 8) as *mut __m256i, g1);
            i += 16;
        }
        while i < n {
            *dst.add(i) = *src.add(*src.add(i) as usize);
            i += 1;
        }
    }
}

/// Apply function `func` exactly `steps` times to each element in `values`,
/// using streaming binary lifting (only 2 jump levels in memory at once).
/// This is much more cache-friendly than building the full jump table.
fn iterate_all_streaming(
    func: &[u32],
    steps: u64,
    values: &mut [u32],
    _use_par: bool,
    buf_a: &mut [u32],
    buf_b: &mut [u32],
) {
    let n = func.len();
    let bits = if steps == 0 { return } else { bit_len(steps) };

    unsafe {
        std::ptr::copy_nonoverlapping(func.as_ptr(), buf_a.as_mut_ptr(), n);
        let mut src = buf_a.as_mut_ptr();
        let mut dst = buf_b.as_mut_ptr();
        let vptr = values.as_mut_ptr();
        let vn = values.len();

        for bit in 0..bits {
            if steps & (1u64 << bit) != 0 {
                apply_gather_avx2(vptr, src, vn);
            }
            if bit + 1 < bits {
                compose_gather_avx2(src, dst, n);
                std::mem::swap(&mut src, &mut dst);
            }
        }
    }
}

/// Precompute x^exp mod m for all x in [0, m) using a multiplicative sieve.
fn precompute_pow_table(exp: u64, m: u64) -> Vec<u32> {
    let size = m as usize;
    let mut spf = vec![0u32; size];
    for i in 2..size {
        if spf[i] == 0 {
            spf[i] = i as u32;
            let mut j = i * i;
            while j < size {
                if spf[j] == 0 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }

    let mut table = vec![0u32; size];
    if size > 1 {
        table[1] = 1;
    }

    for x in 2..size {
        let p = spf[x] as usize;
        if x == p {
            table[x] = mod_pow(x as u64, exp, m) as u32;
        } else {
            let mut pa = p;
            let mut rest = x / p;
            while rest % p == 0 {
                rest /= p;
                pa *= p;
            }
            if rest == 1 {
                table[x] = mod_pow(x as u64, exp, m) as u32;
            } else {
                table[x] = mul_mod(table[pa] as u64, table[rest] as u64, m) as u32;
            }
        }
    }

    table
}

fn phi_mod_table(modulus: u64) -> u64 {
    let size = modulus as usize;
    let use_par = size > 100_000;

    // Precompute x^C and x^(C+1) mod modulus for all x
    let (pow_c, pow_cp1) = if size <= 1024 {
        (
            (0..size as u64).map(|x| mod_pow(x, C, modulus) as u32).collect::<Vec<_>>(),
            (0..size as u64).map(|x| mod_pow(x, C + 1, modulus) as u32).collect::<Vec<_>>(),
        )
    } else {
        rayon::join(
            || precompute_pow_table(C, modulus),
            || precompute_pow_table(C + 1, modulus),
        )
    };

    // g_c(x) = x^C * (x+1) mod m
    // g_{c+1}(x) = x^(C+1) * (x+1) mod m
    let mut gc = vec![0u32; size];
    let mut gcp1 = vec![0u32; size];
    for x in 0..size {
        let xp1 = if x as u64 + 1 >= modulus { 0u64 } else { x as u64 + 1 };
        gc[x] = mul_mod(pow_c[x] as u64, xp1, modulus) as u32;
        gcp1[x] = mul_mod(pow_cp1[x] as u64, xp1, modulus) as u32;
    }
    drop(pow_c);
    drop(pow_cp1);

    let mut buf_a = vec![0u32; size];
    let mut buf_b = vec![0u32; size];

    let mut prev = gcp1;
    iterate_all_streaming(&gc, B + 1, &mut prev, use_par, &mut buf_a, &mut buf_b);
    drop(gc);

    let mut curr = vec![0u32; size];

    for _level in 1..A {
        // Compute starting values
        for x in 0..size {
            curr[x] = (((x as u64) * prev[x] as u64) % modulus) as u32;
        }

        iterate_all_streaming(&prev, B, &mut curr, use_par, &mut buf_a, &mut buf_b);
        std::mem::swap(&mut prev, &mut curr);
    }

    // Last level A: only evaluate at D % modulus!
    let start_x = (D % modulus) as usize;
    let mut val = (((start_x as u64) * prev[start_x] as u64) % modulus) as usize;
    for _ in 0..B {
        val = prev[val] as usize;
    }
    val as u64
}

fn main() {
    debug_assert_eq!(M1 * M2, MOD);

    let v1 = phi_mod_table(M1);
    let v2 = phi_mod_table(M2);
    let ans = (crt(v1, M1, v2, M2) + E) % MOD;
    println!("{}", ans);
}
