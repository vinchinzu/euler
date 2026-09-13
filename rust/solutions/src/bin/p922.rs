// Problem 922 - Young's Game A
//
// Uses Fast Walsh-Hadamard Transform (FWT) for XOR convolution combined with
// shifted polynomial exponentiation to count winning positions for Right.

const MOD: u64 = 1_000_000_007;
const XOR_SIZE: usize = 64;

/// Fast Walsh-Hadamard Transform for XOR convolution (in-place).
/// If inverse is true, performs the inverse transform.
fn fwt_xor(arr: &mut [u64], inverse: bool) {
    let n = arr.len();
    let mut step = 1;
    while step < n {
        let jump = step * 2;
        let mut i = 0;
        while i < n {
            // SAFETY: j < i + step < n, and j + step < i + 2*step <= n
            for j in i..i + step {
                unsafe {
                    let x = *arr.get_unchecked(j);
                    let y = *arr.get_unchecked(j + step);
                    *arr.get_unchecked_mut(j) = (x + y) % MOD;
                    *arr.get_unchecked_mut(j + step) = (x + MOD - y) % MOD;
                }
            }
            i += jump;
        }
        step = jump;
    }

    if inverse {
        let inv_n = mod_pow_local(n as u64, MOD - 2, MOD);
        let inv_n_128 = inv_n as u128;
        // SAFETY: iterating over entire array
        for i in 0..n {
            unsafe {
                let v = arr.get_unchecked_mut(i);
                *v = ((*v as u128 * inv_n_128) % MOD as u128) as u64;
            }
        }
    }
}

/// Local modular exponentiation using u64 (MOD < 2^32, so a*b < 2^64 is NOT
/// guaranteed since MOD ~ 10^9, products can be up to ~10^18 which fits u64).
/// Actually 10^9 * 10^9 = 10^18 which fits in u64 (max ~1.8 * 10^19). So we
/// can use u64 directly.
#[inline]
fn mod_pow_local(mut base: u64, mut exp: u64, m: u64) -> u64 {
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

/// Shifted polynomial: (coefficients, offset)
/// coeffs[i] represents the coefficient at degree (i - offset).
type ShiftedPoly = (Vec<u64>, usize);

/// Multiply two shifted polynomials mod MOD.
fn poly_mul(pa: &ShiftedPoly, pb: &ShiftedPoly) -> ShiftedPoly {
    let (a, oa) = pa;
    let (b, ob) = pb;

    let (a, oa, b, ob) = if a.len() >= b.len() {
        (a.as_slice(), *oa, b.as_slice(), *ob)
    } else {
        (b.as_slice(), *ob, a.as_slice(), *oa)
    };

    let res_len = a.len() + b.len() - 1;
    let mut res = vec![0u128; res_len];

    for i in 0..a.len() {
        let ai = a[i];
        if ai != 0 {
            let ai128 = ai as u128;
            for j in 0..b.len() {
                let bj = b[j];
                if bj != 0 {
                    let idx = i + j;
                    // SAFETY: idx = i + j < res_len
                    unsafe {
                        *res.get_unchecked_mut(idx) += ai128 * bj as u128;
                    }
                }
            }
        }
    }

    // Apply modulo once at the end
    let res_final: Vec<u64> = res.iter().map(|&v| (v % MOD as u128) as u64).collect();

    (res_final, oa + ob)
}

/// Raise a shifted polynomial to the given power using binary exponentiation.
fn poly_pow(poly: &ShiftedPoly, mut exp: u64) -> ShiftedPoly {
    let mut result: ShiftedPoly = (vec![1u64], 0);
    let mut base = poly.clone();

    while exp > 0 {
        if exp & 1 == 1 {
            result = poly_mul(&result, &base);
        }
        exp >>= 1;
        if exp > 0 {
            base = poly_mul(&base, &base);
        }
    }

    result
}

fn compute_r(m: u64, w: usize) -> u64 {
    let dmax = w - 2;
    let diff_count = 2 * dmax + 1;

    let mut counts = vec![0u64; diff_count * XOR_SIZE];

    // Count staircases by (d = b - a, g = k - 1)
    for k in 1..w - 1 {
        let limit = w - k;
        if limit < 2 {
            continue;
        }
        let g = k - 1;
        let tmax = limit - 2;
        // SAFETY: g < XOR_SIZE (since k < w-1 and g = k-1 < w-2)
        // and indices are within bounds
        unsafe {
            for t in 0..=tmax {
                let c = ((limit - t) / 2) as u64;
                if c != 0 {
                    let idx_pos = (dmax + t) * XOR_SIZE + g;
                    let val = counts.get_unchecked_mut(idx_pos);
                    *val = (*val + c) % MOD;
                    if t != 0 {
                        let idx_neg = (dmax - t) * XOR_SIZE + g;
                        let val = counts.get_unchecked_mut(idx_neg);
                        *val = (*val + c) % MOD;
                    }
                }
            }
        }
    }

    // FWT over XOR dimension for each diff coefficient
    let mut transformed = vec![0u64; diff_count * XOR_SIZE];
    for d in 0..diff_count {
        let start = d * XOR_SIZE;
        transformed[start..start + XOR_SIZE]
            .copy_from_slice(&counts[start..start + XOR_SIZE]);
        fwt_xor(&mut transformed[start..start + XOR_SIZE], false);
    }

    // Build XOR_SIZE polynomials, one per transformed XOR index t
    let mut polys: Vec<ShiftedPoly> = Vec::with_capacity(XOR_SIZE);
    for t in 0..XOR_SIZE {
        let coeffs: Vec<u64> = (0..diff_count)
            .map(|d| unsafe { *transformed.get_unchecked(d * XOR_SIZE + t) })
            .collect();
        polys.push((coeffs, dmax));
    }

    // Raise each polynomial to the m-th power
    let pow_polys: Vec<ShiftedPoly> = polys
        .iter()
        .map(|p| poly_pow(p, m))
        .collect();

    let final_offset = (m as usize) * dmax;
    let final_len = 2 * final_offset + 1;

    // Align into matrix Qhat[t][degree], stored flat
    let mut qhat = vec![0u64; XOR_SIZE * final_len];
    for t in 0..XOR_SIZE {
        let (ref coeffs, off) = pow_polys[t];
        assert_eq!(off, final_offset);
        // SAFETY: t < XOR_SIZE, coeffs.len() >= final_len from polynomial properties
        unsafe {
            for i in 0..coeffs.len().min(final_len) {
                *qhat.get_unchecked_mut(t * final_len + i) = *coeffs.get_unchecked(i);
            }
        }
    }

    // Inverse FWT at each coefficient position, accumulate winning cases
    let mut ans = 0u64;
    let mut vec_buf = vec![0u64; XOR_SIZE];

    for idx in 0..final_len {
        // Gather column: vec_buf[t] = qhat[t][idx]
        // SAFETY: t < XOR_SIZE, idx < final_len
        unsafe {
            for t in 0..XOR_SIZE {
                *vec_buf.get_unchecked_mut(t) = *qhat.get_unchecked(t * final_len + idx);
            }
        }

        fwt_xor(&mut vec_buf, true);

        let total_diff = idx as i64 - final_offset as i64;
        if total_diff > 0 {
            let s: u64 = vec_buf.iter().fold(0u64, |acc, &v| (acc + v) % MOD);
            ans = (ans + s) % MOD;
        } else if total_diff == 0 {
            let s: u64 = vec_buf[1..].iter().fold(0u64, |acc, &v| (acc + v) % MOD);
            ans = (ans + s) % MOD;
        }
    }

    ans
}

fn main() {
    // Verify examples
    debug_assert_eq!(compute_r(2, 4), 7);
    debug_assert_eq!(compute_r(3, 9), 314104);

    println!("{}", compute_r(8, 64));
}
