// Project Euler 416: A frog's trip
use euler_utils::mod_inv;
use rayon::prelude::*;

const K: usize = 10;
const FK: usize = 2 * K;
// (FK+1)*(FK+2) = 462 states; pad to a multiple of 8 for the AVX2 kernel.
const ACC_CAP: usize = 464;

fn multinomial(n: i32, a: i32, b: i32, c: i32) -> i64 {
    if a < 0 || b < 0 || c < 0 || a + b + c != n {
        return 0;
    }
    let mut r = 1i64;
    for i in 0..a {
        r = r * (n - i) as i64 / (i + 1) as i64;
    }
    for i in 0..b {
        r = r * (n - a - i) as i64 / (i + 1) as i64;
    }
    r
}

// n * (m2-1)^2 ≈ 1.76e15 < 2^63, so a full row of i64 products cannot overflow.
// i128 deferred reduction is unnecessary (and slower) for both CRT moduli.

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn saxpy_avx2(acc: *mut i64, b: *const i32, aik: i32, n: usize) {
    use std::arch::x86_64::*;
    // SAFETY: caller guarantees acc/b have n elements and AVX2 is available.
    unsafe {
        let av = _mm256_set1_epi64x(aik as i64);
        let mut j = 0usize;
        while j + 8 <= n {
            let b0 = _mm256_cvtepi32_epi64(_mm_loadu_si128(b.add(j) as *const __m128i));
            let b1 = _mm256_cvtepi32_epi64(_mm_loadu_si128(b.add(j + 4) as *const __m128i));
            let a0 = _mm256_loadu_si256(acc.add(j) as *const __m256i);
            let a1 = _mm256_loadu_si256(acc.add(j + 4) as *const __m256i);
            // Values are in 0..m2 < 2^21; vpmuludq on the low 32 bits is exact.
            _mm256_storeu_si256(acc.add(j) as *mut __m256i, _mm256_add_epi64(a0, _mm256_mul_epu32(av, b0)));
            _mm256_storeu_si256(
                acc.add(j + 4) as *mut __m256i,
                _mm256_add_epi64(a1, _mm256_mul_epu32(av, b1)),
            );
            j += 8;
        }
        while j < n {
            *acc.add(j) += aik as i64 * *b.add(j) as i64;
            j += 1;
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn mul_row_avx2(a: &[i32], b: &[i32], row: &mut [i32], i: usize, n: usize, modulus: i32) {
    let mut acc = [0i64; ACC_CAP];
    // SAFETY: i < n, a/b length n*n, row length n.
    unsafe {
        let ap = a.as_ptr().add(i * n);
        let bp = b.as_ptr();
        let accp = acc.as_mut_ptr();
        for k in 0..n {
            let aik = *ap.add(k);
            if aik == 0 {
                continue;
            }
            saxpy_avx2(accp, bp.add(k * n), aik, n);
        }
    }
    if modulus == 512 {
        for j in 0..n {
            row[j] = (acc[j] & 511) as i32;
        }
    } else {
        let m = modulus as i64;
        for j in 0..n {
            row[j] = (acc[j] % m) as i32;
        }
    }
}

fn mul_row_scalar(a: &[i32], b: &[i32], row: &mut [i32], i: usize, n: usize, modulus: i32) {
    let mut acc = [0i64; ACC_CAP];
    let a_off = i * n;
    for k in 0..n {
        // SAFETY: i,k,j < n and a,b have length n*n.
        let aik = unsafe { *a.get_unchecked(a_off + k) };
        if aik == 0 {
            continue;
        }
        let b_off = k * n;
        unsafe {
            let mut j = 0usize;
            while j + 4 <= n {
                *acc.get_unchecked_mut(j) += aik as i64 * *b.get_unchecked(b_off + j) as i64;
                *acc.get_unchecked_mut(j + 1) += aik as i64 * *b.get_unchecked(b_off + j + 1) as i64;
                *acc.get_unchecked_mut(j + 2) += aik as i64 * *b.get_unchecked(b_off + j + 2) as i64;
                *acc.get_unchecked_mut(j + 3) += aik as i64 * *b.get_unchecked(b_off + j + 3) as i64;
                j += 4;
            }
            while j < n {
                *acc.get_unchecked_mut(j) += aik as i64 * *b.get_unchecked(b_off + j) as i64;
                j += 1;
            }
        }
    }
    if modulus == 512 {
        for j in 0..n {
            row[j] = (acc[j] & 511) as i32;
        }
    } else {
        let m = modulus as i64;
        for j in 0..n {
            row[j] = (acc[j] % m) as i32;
        }
    }
}

fn mat_mul(a: &[i32], b: &[i32], res: &mut [i32], n: usize, modulus: i32) {
    debug_assert_eq!(a.len(), n * n);
    debug_assert_eq!(b.len(), n * n);
    debug_assert_eq!(res.len(), n * n);
    debug_assert!(n <= ACC_CAP);

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            res.par_chunks_mut(n).enumerate().for_each(|(i, row)| {
                // SAFETY: AVX2 detected; row is the i-th n-element slice of res.
                unsafe { mul_row_avx2(a, b, row, i, n, modulus) }
            });
            return;
        }
    }

    res.par_chunks_mut(n).enumerate().for_each(|(i, row)| {
        mul_row_scalar(a, b, row, i, n, modulus);
    });
}

fn mat_pow(base: &[i32], n: usize, mut exp: i64, modulus: i32) -> Vec<i32> {
    let mut result = vec![0i32; n * n];
    for i in 0..n {
        result[i * n + i] = 1;
    }
    let mut b = base.to_vec();
    let mut tmp = vec![0i32; n * n];
    while exp > 0 {
        if exp & 1 == 1 {
            mat_mul(&result, &b, &mut tmp, n, modulus);
            std::mem::swap(&mut result, &mut tmp);
        }
        mat_mul(&b, &b, &mut tmp, n, modulus);
        std::mem::swap(&mut b, &mut tmp);
        exp >>= 1;
    }
    result
}

fn main() {
    let n_val: i64 = 1_000_000_000_000;
    let m1: i32 = 512; // 2^9
    let m2: i32 = 1_953_125; // 5^9

    let mut states: Vec<(i32, i32, i32)> = Vec::new(); // (a, b, u)
    let mut state_idx = vec![vec![vec![-1i32; 2]; FK + 1]; FK + 1];

    for a in 0..=FK as i32 {
        for b in 0..=(FK as i32 - a) {
            for u in 0..2i32 {
                state_idx[a as usize][b as usize][u as usize] = states.len() as i32;
                states.push((a, b, u));
            }
        }
    }
    let n_states = states.len();
    debug_assert!(n_states <= ACC_CAP);

    let mut a_mat = vec![0i32; n_states * n_states];
    for i in 0..n_states {
        let (a, b, u) = states[i];
        let c = FK as i32 - a - b;
        let new_u = u + if a == 0 { 1 } else { 0 };
        if new_u > 1 {
            continue;
        }

        for j1 in 0..=a {
            for j2 in 0..=(a - j1) {
                let j3 = a - j1 - j2;
                let new_a = b + j1;
                let new_b = c + j2;
                if new_a + new_b + j3 != FK as i32 {
                    continue;
                }
                if new_a > FK as i32 || new_b > FK as i32 {
                    continue;
                }

                let j = state_idx[new_a as usize][new_b as usize][new_u as usize];
                if j < 0 {
                    continue;
                }

                let coeff = multinomial(a, j1, j2, j3);
                a_mat[j as usize * n_states + i] += coeff as i32;
            }
        }
    }

    let start = state_idx[FK][0][0] as usize;
    let end0 = state_idx[FK][0][0] as usize;
    let end1 = state_idx[FK][0][1] as usize;

    let (r1, r2) = rayon::join(
        || {
            let mut a1 = a_mat.clone();
            for v in &mut a1 {
                *v %= m1;
            }
            let r1_mat = mat_pow(&a1, n_states, n_val - 1, m1);
            (r1_mat[end0 * n_states + start] as i64 + r1_mat[end1 * n_states + start] as i64) % m1 as i64
        },
        || {
            let mut a2 = a_mat.clone();
            for v in &mut a2 {
                *v %= m2;
            }
            let r2_mat = mat_pow(&a2, n_states, n_val - 1, m2);
            (r2_mat[end0 * n_states + start] as i64 + r2_mat[end1 * n_states + start] as i64) % m2 as i64
        },
    );

    let big_m = m1 as i64 * m2 as i64;
    let inv1 = mod_inv(m2 as u64, m1 as u64).unwrap() as i64;
    let inv2 = mod_inv(m1 as u64, m2 as u64).unwrap() as i64;
    let result = (r1 * m2 as i64 % big_m * inv1 % big_m + r2 * m1 as i64 % big_m * inv2 % big_m) % big_m;

    println!("{}", result);
}
