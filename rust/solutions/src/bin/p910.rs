// Project Euler 910 — Phi-recursion + CRT solver
// Optimized: streaming jump-table approach (2 levels in memory at a time)
// + u32 tables + rayon for per-level work

use rayon::prelude::*;

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

/// Apply function `func` exactly `steps` times to each element in `values`,
/// using streaming binary lifting (only 2 jump levels in memory at once).
/// This is much more cache-friendly than building the full jump table.
fn iterate_all_streaming(
    func: &[u32],
    steps: u64,
    values: &mut [u32],
    use_par: bool,
    buf_a: &mut [u32],
    buf_b: &mut [u32],
) {
    let n = func.len();
    let bits = if steps == 0 { return } else { bit_len(steps) };

    buf_a[..n].copy_from_slice(func);

    for bit in 0..bits {
        // If this bit is set in steps, apply buf_a to all values
        if steps & (1u64 << bit) != 0 {
            // Sequential is fine here: values[i] is accessed sequentially,
            // buf_a[*v] is random but read-only (shared cache lines)
            for v in values.iter_mut() {
                // SAFETY: *v < modulus < n
                *v = unsafe { *buf_a.get_unchecked(*v as usize) };
            }
        }

        // Build next level: buf_b[x] = buf_a[buf_a[x]]
        if bit + 1 < bits {
            if use_par {
                let a_slice = &buf_a[..n];
                buf_b[..n].par_iter_mut().enumerate().for_each(|(i, out)| {
                    let mid = unsafe { *a_slice.get_unchecked(i) } as usize;
                    *out = unsafe { *a_slice.get_unchecked(mid) };
                });
            } else {
                for i in 0..n {
                    let mid = unsafe { *buf_a.get_unchecked(i) } as usize;
                    unsafe { *buf_b.get_unchecked_mut(i) = *buf_a.get_unchecked(mid) };
                }
            }
            buf_a[..n].copy_from_slice(&buf_b[..n]);
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
    let pow_c = if size <= 1024 {
        (0..size as u64).map(|x| mod_pow(x, C, modulus) as u32).collect()
    } else {
        precompute_pow_table(C, modulus)
    };
    let pow_cp1 = if size <= 1024 {
        (0..size as u64).map(|x| mod_pow(x, C + 1, modulus) as u32).collect()
    } else {
        precompute_pow_table(C + 1, modulus)
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
