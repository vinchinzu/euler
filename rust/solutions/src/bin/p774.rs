// Project Euler 774 - Conjunctive Sequences
// Tensor-Train / MPS with left-sweep Gaussian elimination compression.
//
// Optimizations:
// - i32 storage (MOD < 2^30) → 2× denser cache lines in GE/hadamard
// - Extended-Euclid modinv (no Fermat powmod)
// - Pre-allocated scratch buffers; capacity-preserving mat reuse
// - unsafe get_unchecked + raw ptr GE inner loops
// - Avoid cloning ones each step: scale first core into a reusable buffer

use std::arch::x86_64::*;

const MOD: i32 = 998244353;
const BARRETT_M: u128 = 18479187002;
const R_MOD: u64 = 301989884;
const INV: u32 = 998244351;

#[target_feature(enable = "avx2")]
unsafe fn axpy_sub_avx2(row: *mut i32, prow: *const i32, f_scaled: u64, len: usize) {
    unsafe {
        let vf_scaled = _mm256_set1_epi64x(f_scaled as i64);
        let vinv = _mm256_set1_epi64x(INV as i64);
        let vmod = _mm256_set1_epi64x(MOD as i64);
        let vmod32 = _mm256_set1_epi32(MOD);
        let vmod_m1 = _mm256_set1_epi32(MOD - 1);

        let mut j = 0;
        while j + 16 <= len {
            let cur0 = _mm256_loadu_si256(row.add(j) as *const __m256i);
            let piv0 = _mm256_loadu_si256(prow.add(j) as *const __m256i);
            let cur1 = _mm256_loadu_si256(row.add(j + 8) as *const __m256i);
            let piv1 = _mm256_loadu_si256(prow.add(j + 8) as *const __m256i);

            let prod_even0 = _mm256_mul_epu32(piv0, vf_scaled);
            let prod_even1 = _mm256_mul_epu32(piv1, vf_scaled);

            let piv_odd0 = _mm256_srli_epi64(piv0, 32);
            let piv_odd1 = _mm256_srli_epi64(piv1, 32);

            let prod_odd0 = _mm256_mul_epu32(piv_odd0, vf_scaled);
            let prod_odd1 = _mm256_mul_epu32(piv_odd1, vf_scaled);

            let m_even0 = _mm256_mul_epu32(prod_even0, vinv);
            let m_even1 = _mm256_mul_epu32(prod_even1, vinv);

            let m_odd0 = _mm256_mul_epu32(prod_odd0, vinv);
            let m_odd1 = _mm256_mul_epu32(prod_odd1, vinv);

            let mp_even0 = _mm256_mul_epu32(m_even0, vmod);
            let mp_even1 = _mm256_mul_epu32(m_even1, vmod);

            let mp_odd0 = _mm256_mul_epu32(m_odd0, vmod);
            let mp_odd1 = _mm256_mul_epu32(m_odd1, vmod);

            let sum_even0 = _mm256_add_epi64(prod_even0, mp_even0);
            let sum_even1 = _mm256_add_epi64(prod_even1, mp_even1);

            let sum_odd0 = _mm256_add_epi64(prod_odd0, mp_odd0);
            let sum_odd1 = _mm256_add_epi64(prod_odd1, mp_odd1);

            let res_even0 = _mm256_srli_epi64(sum_even0, 32);
            let res_even1 = _mm256_srli_epi64(sum_even1, 32);

            let res_odd0 = _mm256_srli_epi64(sum_odd0, 32);
            let res_odd1 = _mm256_srli_epi64(sum_odd1, 32);

            let res_odd_shifted0 = _mm256_slli_epi64(res_odd0, 32);
            let res_odd_shifted1 = _mm256_slli_epi64(res_odd1, 32);

            let t0 = _mm256_or_si256(res_even0, res_odd_shifted0);
            let t1 = _mm256_or_si256(res_even1, res_odd_shifted1);

            let sub_t0 = _mm256_sub_epi32(t0, vmod32);
            let sub_t1 = _mm256_sub_epi32(t1, vmod32);

            let cmp_t0 = _mm256_cmpgt_epi32(t0, vmod_m1);
            let cmp_t1 = _mm256_cmpgt_epi32(t1, vmod_m1);

            let t_norm0 = _mm256_blendv_epi8(t0, sub_t0, cmp_t0);
            let t_norm1 = _mm256_blendv_epi8(t1, sub_t1, cmp_t1);

            let sub_cur0 = _mm256_sub_epi32(cur0, t_norm0);
            let sub_cur1 = _mm256_sub_epi32(cur1, t_norm1);

            let add_cur0 = _mm256_add_epi32(sub_cur0, vmod32);
            let add_cur1 = _mm256_add_epi32(sub_cur1, vmod32);

            let cmp_cur0 = _mm256_cmpgt_epi32(t_norm0, cur0);
            let cmp_cur1 = _mm256_cmpgt_epi32(t_norm1, cur1);

            let final_val0 = _mm256_blendv_epi8(sub_cur0, add_cur0, cmp_cur0);
            let final_val1 = _mm256_blendv_epi8(sub_cur1, add_cur1, cmp_cur1);

            _mm256_storeu_si256(row.add(j) as *mut __m256i, final_val0);
            _mm256_storeu_si256(row.add(j + 8) as *mut __m256i, final_val1);

            j += 16;
        }

        while j + 8 <= len {
            let cur = _mm256_loadu_si256(row.add(j) as *const __m256i);
            let piv = _mm256_loadu_si256(prow.add(j) as *const __m256i);

            let prod_even = _mm256_mul_epu32(piv, vf_scaled);
            let m_even = _mm256_mul_epu32(prod_even, vinv);
            let mp_even = _mm256_mul_epu32(m_even, vmod);
            let sum_even = _mm256_add_epi64(prod_even, mp_even);
            let res_even = _mm256_srli_epi64(sum_even, 32);

            let piv_odd = _mm256_srli_epi64(piv, 32);
            let prod_odd = _mm256_mul_epu32(piv_odd, vf_scaled);
            let m_odd = _mm256_mul_epu32(prod_odd, vinv);
            let mp_odd = _mm256_mul_epu32(m_odd, vmod);
            let sum_odd = _mm256_add_epi64(prod_odd, mp_odd);
            let res_odd = _mm256_srli_epi64(sum_odd, 32);
            let res_odd_shifted = _mm256_slli_epi64(res_odd, 32);

            let t = _mm256_or_si256(res_even, res_odd_shifted);

            let sub_t = _mm256_sub_epi32(t, vmod32);
            let cmp_t = _mm256_cmpgt_epi32(t, vmod_m1);
            let t_norm = _mm256_blendv_epi8(t, sub_t, cmp_t);

            let sub_cur = _mm256_sub_epi32(cur, t_norm);
            let add_cur = _mm256_add_epi32(sub_cur, vmod32);
            let cmp_cur = _mm256_cmpgt_epi32(t_norm, cur);
            let final_val = _mm256_blendv_epi8(sub_cur, add_cur, cmp_cur);

            _mm256_storeu_si256(row.add(j) as *mut __m256i, final_val);
            j += 8;
        }

        while j < len {
            let p = row.add(j);
            let piv_val = *prow.add(j) as u64;
            let prod_mod = redc(f_scaled * piv_val);
            let cur = *p as u32;
            let res = if cur >= prod_mod {
                cur - prod_mod
            } else {
                cur + MOD as u32 - prod_mod
            };
            *p = res as i32;
            j += 1;
        }
    }
}

#[allow(dead_code)]
#[target_feature(enable = "avx2")]
unsafe fn hadamard_b01_avx2(c_ptr: *mut i32, a_ptr: *const i32, len: usize, case: u8) {
    unsafe {
        let mut ra = 0;
        while ra + 8 <= len {
            let s = _mm256_loadu_si256(a_ptr.add(ra) as *const __m256i);
            let s_low = _mm256_castsi256_si128(s);
            let s_high = _mm256_extracti128_si256(s, 1);

            let v0 = _mm256_cvtepu32_epi64(s_low);
            let v1 = _mm256_cvtepu32_epi64(s_high);

            let (out0, out1) = match case {
                0 => (v0, v1),
                1 => (_mm256_slli_epi64(v0, 32), _mm256_slli_epi64(v1, 32)),
                _ => {
                    let v0_odd = _mm256_slli_epi64(v0, 32);
                    let v1_odd = _mm256_slli_epi64(v1, 32);
                    (_mm256_or_si256(v0, v0_odd), _mm256_or_si256(v1, v1_odd))
                }
            };

            _mm256_storeu_si256(c_ptr.add(2 * ra) as *mut __m256i, out0);
            _mm256_storeu_si256(c_ptr.add(2 * ra + 8) as *mut __m256i, out1);
            ra += 8;
        }

        while ra < len {
            let v = *a_ptr.add(ra);
            match case {
                0 => *c_ptr.add(2 * ra) = v,
                1 => *c_ptr.add(2 * ra + 1) = v,
                _ => {
                    *c_ptr.add(2 * ra) = v;
                    *c_ptr.add(2 * ra + 1) = v;
                }
            }
            ra += 1;
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn axpy_add_avx2(dst: *mut i32, src: *const i32, f_scaled: u64, len: usize) {
    unsafe {
        let vf_scaled = _mm256_set1_epi64x(f_scaled as i64);
        let vinv = _mm256_set1_epi64x(INV as i64);
        let vmod = _mm256_set1_epi64x(MOD as i64);
        let vmod32 = _mm256_set1_epi32(MOD);
        let vmod_m1 = _mm256_set1_epi32(MOD - 1);

        let mut j = 0;
        while j + 8 <= len {
            let cur = _mm256_loadu_si256(dst.add(j) as *const __m256i);
            let s = _mm256_loadu_si256(src.add(j) as *const __m256i);

            let prod_even = _mm256_mul_epu32(s, vf_scaled);
            let m_even = _mm256_mul_epu32(prod_even, vinv);
            let mp_even = _mm256_mul_epu32(m_even, vmod);
            let sum_even = _mm256_add_epi64(prod_even, mp_even);
            let res_even = _mm256_srli_epi64(sum_even, 32);

            let s_odd = _mm256_srli_epi64(s, 32);
            let prod_odd = _mm256_mul_epu32(s_odd, vf_scaled);
            let m_odd = _mm256_mul_epu32(prod_odd, vinv);
            let mp_odd = _mm256_mul_epu32(m_odd, vmod);
            let sum_odd = _mm256_add_epi64(prod_odd, mp_odd);
            let res_odd = _mm256_srli_epi64(sum_odd, 32);
            let res_odd_shifted = _mm256_slli_epi64(res_odd, 32);

            let t = _mm256_or_si256(res_even, res_odd_shifted);

            let sub_t = _mm256_sub_epi32(t, vmod32);
            let cmp_t = _mm256_cmpgt_epi32(t, vmod_m1);
            let t_norm = _mm256_blendv_epi8(t, sub_t, cmp_t);

            let sum_cur = _mm256_add_epi32(cur, t_norm);
            let sub_cur = _mm256_sub_epi32(sum_cur, vmod32);
            let cmp_cur = _mm256_cmpgt_epi32(sum_cur, vmod_m1);
            let final_val = _mm256_blendv_epi8(sum_cur, sub_cur, cmp_cur);

            _mm256_storeu_si256(dst.add(j) as *mut __m256i, final_val);
            j += 8;
        }

        while j < len {
            let p = dst.add(j);
            let s_val = *src.add(j) as u64;
            let prod_mod = redc(f_scaled * s_val);
            let cur = *p as u32;
            let sum = cur + prod_mod;
            *p = if sum >= MOD as u32 {
                (sum - MOD as u32) as i32
            } else {
                sum as i32
            };
            j += 1;
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn scale_row_avx2(row: *mut i32, inv_scaled: u64, len: usize) {
    unsafe {
        let vf_scaled = _mm256_set1_epi64x(inv_scaled as i64);
        let vinv = _mm256_set1_epi64x(INV as i64);
        let vmod = _mm256_set1_epi64x(MOD as i64);
        let vmod32 = _mm256_set1_epi32(MOD);
        let vmod_m1 = _mm256_set1_epi32(MOD - 1);

        let mut j = 0;
        while j + 8 <= len {
            let s = _mm256_loadu_si256(row.add(j) as *const __m256i);

            let prod_even = _mm256_mul_epu32(s, vf_scaled);
            let m_even = _mm256_mul_epu32(prod_even, vinv);
            let mp_even = _mm256_mul_epu32(m_even, vmod);
            let sum_even = _mm256_add_epi64(prod_even, mp_even);
            let res_even = _mm256_srli_epi64(sum_even, 32);

            let s_odd = _mm256_srli_epi64(s, 32);
            let prod_odd = _mm256_mul_epu32(s_odd, vf_scaled);
            let m_odd = _mm256_mul_epu32(prod_odd, vinv);
            let mp_odd = _mm256_mul_epu32(m_odd, vmod);
            let sum_odd = _mm256_add_epi64(prod_odd, mp_odd);
            let res_odd = _mm256_srli_epi64(sum_odd, 32);
            let res_odd_shifted = _mm256_slli_epi64(res_odd, 32);

            let t = _mm256_or_si256(res_even, res_odd_shifted);

            let sub_t = _mm256_sub_epi32(t, vmod32);
            let cmp_t = _mm256_cmpgt_epi32(t, vmod_m1);
            let t_norm = _mm256_blendv_epi8(t, sub_t, cmp_t);

            _mm256_storeu_si256(row.add(j) as *mut __m256i, t_norm);
            j += 8;
        }

        while j < len {
            let p = row.add(j);
            let val = *p as u64;
            *p = redc(inv_scaled * val) as i32;
            j += 1;
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn hadamard_strided_avx2(c_ptr: *mut i32, a_ptr: *const i32, bv_scaled: u64, len: usize, rb: usize) {
    unsafe {
        let vf_scaled = _mm256_set1_epi64x(bv_scaled as i64);
        let vinv = _mm256_set1_epi64x(INV as i64);
        let vmod = _mm256_set1_epi64x(MOD as i64);
        let vmod32 = _mm256_set1_epi32(MOD);
        let vmod_m1 = _mm256_set1_epi32(MOD - 1);

        let mut ra = 0;
        while ra + 8 <= len {
            let s = _mm256_loadu_si256(a_ptr.add(ra) as *const __m256i);

            let prod_even = _mm256_mul_epu32(s, vf_scaled);
            let m_even = _mm256_mul_epu32(prod_even, vinv);
            let mp_even = _mm256_mul_epu32(m_even, vmod);
            let sum_even = _mm256_add_epi64(prod_even, mp_even);
            let res_even = _mm256_srli_epi64(sum_even, 32);

            let s_odd = _mm256_srli_epi64(s, 32);
            let prod_odd = _mm256_mul_epu32(s_odd, vf_scaled);
            let m_odd = _mm256_mul_epu32(prod_odd, vinv);
            let mp_odd = _mm256_mul_epu32(m_odd, vmod);
            let sum_odd = _mm256_add_epi64(prod_odd, mp_odd);
            let res_odd = _mm256_srli_epi64(sum_odd, 32);
            let res_odd_shifted = _mm256_slli_epi64(res_odd, 32);

            let t = _mm256_or_si256(res_even, res_odd_shifted);

            let sub_t = _mm256_sub_epi32(t, vmod32);
            let cmp_t = _mm256_cmpgt_epi32(t, vmod_m1);
            let t_norm = _mm256_blendv_epi8(t, sub_t, cmp_t);

            let t_low = _mm256_castsi256_si128(t_norm);
            let t_high = _mm256_extracti128_si256(t_norm, 1);

            let mut exp0 = _mm256_cvtepu32_epi64(t_low);
            let mut exp1 = _mm256_cvtepu32_epi64(t_high);

            if rb == 1 {
                exp0 = _mm256_slli_epi64(exp0, 32);
                exp1 = _mm256_slli_epi64(exp1, 32);
            }

            let dst0 = c_ptr.add(2 * ra);
            let cur0 = _mm256_loadu_si256(dst0 as *const __m256i);
            let sum0 = _mm256_add_epi32(cur0, exp0);
            let sub0 = _mm256_sub_epi32(sum0, vmod32);
            let cmp0 = _mm256_cmpgt_epi32(sum0, vmod_m1);
            let res0 = _mm256_blendv_epi8(sum0, sub0, cmp0);
            _mm256_storeu_si256(dst0 as *mut __m256i, res0);

            let dst1 = c_ptr.add(2 * ra + 8);
            let cur1 = _mm256_loadu_si256(dst1 as *const __m256i);
            let sum1 = _mm256_add_epi32(cur1, exp1);
            let sub1 = _mm256_sub_epi32(sum1, vmod32);
            let cmp1 = _mm256_cmpgt_epi32(sum1, vmod_m1);
            let res1 = _mm256_blendv_epi8(sum1, sub1, cmp1);
            _mm256_storeu_si256(dst1 as *mut __m256i, res1);

            ra += 8;
        }

        while ra < len {
            let av = *a_ptr.add(ra);
            if av != 0 {
                let term = redc(bv_scaled * av as u64);
                let p = c_ptr.add(ra * 2 + rb);
                let sum = *p as u32 + term;
                *p = if sum >= MOD as u32 {
                    (sum - MOD as u32) as i32
                } else {
                    sum as i32
                };
            }
            ra += 1;
        }
    }
}

#[inline(always)]
fn redc(t: u64) -> u32 {
    let m = (t as u32).wrapping_mul(INV);
    let res = ((t + m as u64 * (MOD as u64)) >> 32) as u32;
    if res >= MOD as u32 {
        res - MOD as u32
    } else {
        res
    }
}

#[inline(always)]
fn barrett_reduce(z: u64) -> u32 {
    let q = ((z as u128 * BARRETT_M) >> 64) as u64;
    let mut r = (z - q * (MOD as u64)) as u32;
    if r >= MOD as u32 {
        r -= MOD as u32;
    }
    r
}

#[inline(always)]
fn modd(x: i32) -> i32 {
    let r = x % MOD;
    r + (((r >> 31) as u32 & MOD as u32) as i32)
}

#[inline(always)]
fn mulmod(a: i32, b: i32) -> i32 {
    barrett_reduce(a as u32 as u64 * b as u32 as u64) as i32
}

/// Extended Euclidean modular inverse.
#[inline(always)]
fn modinv(a: i32) -> i32 {
    let mut a = a % MOD;
    if a < 0 {
        a += MOD;
    }
    let mut t = 0i32;
    let mut newt = 1i32;
    let mut r = MOD;
    let mut newr = a;
    while newr != 0 {
        let q = r / newr;
        let tmp = newt;
        newt = t - q * newt;
        t = tmp;
        let tmp = newr;
        newr = r - q * newr;
        r = tmp;
    }
    if t < 0 {
        t + MOD
    } else {
        t
    }
}

struct Core {
    r_l: usize,
    r_r: usize,
    data: Vec<i32>,
}

impl Core {
    fn new(r_l: usize, r_r: usize) -> Self {
        Core {
            r_l,
            r_r,
            data: vec![0; r_l * 2 * r_r],
        }
    }
    fn with_capacity_like(r_l: usize, r_r: usize, cap_hint: usize) -> Self {
        let need = r_l * 2 * r_r;
        let mut data = Vec::with_capacity(need.max(cap_hint));
        data.resize(need, 0);
        Core { r_l, r_r, data }
    }
    #[inline(always)]
    fn get(&self, l: usize, bit: usize, r: usize) -> i32 {
        // SAFETY: callers ensure l < r_l, bit < 2, r < r_r
        unsafe { *self.data.get_unchecked(l * 2 * self.r_r + bit * self.r_r + r) }
    }
    #[inline(always)]
    fn set(&mut self, l: usize, bit: usize, r: usize, val: i32) {
        let i = l * 2 * self.r_r + bit * self.r_r + r;
        // SAFETY: callers ensure indices in range
        unsafe {
            *self.data.get_unchecked_mut(i) = val;
        }
    }
}

impl Clone for Core {
    fn clone(&self) -> Self {
        Core {
            r_l: self.r_l,
            r_r: self.r_r,
            data: self.data.clone(),
        }
    }
}

struct TT {
    m: usize,
    cores: Vec<Core>,
}

impl Clone for TT {
    fn clone(&self) -> Self {
        TT {
            m: self.m,
            cores: self.cores.clone(),
        }
    }
}

thread_local! {
    static SCRATCH: std::cell::RefCell<Scratch> = std::cell::RefCell::new(Scratch::new());
}

struct Scratch {
    mat: Vec<i32>,
    pivots: Vec<usize>,
    is_pivot: Vec<bool>,
    sum_vec: Vec<i32>,
    sum_new: Vec<i32>,
    t_gauss: std::time::Duration,
    t_update: std::time::Duration,
}

impl Scratch {
    fn new() -> Self {
        Scratch {
            mat: Vec::with_capacity(1 << 16),
            pivots: Vec::with_capacity(256),
            is_pivot: Vec::with_capacity(256),
            sum_vec: Vec::with_capacity(256),
            sum_new: Vec::with_capacity(256),
            t_gauss: std::time::Duration::ZERO,
            t_update: std::time::Duration::ZERO,
        }
    }
}

impl TT {
    fn all_ones(m: usize) -> Self {
        let cores = (0..m)
            .map(|_| {
                let mut c = Core::new(1, 1);
                c.set(0, 0, 0, 1);
                c.set(0, 1, 0, 1);
                c
            })
            .collect();
        TT { m, cores }
    }

    fn indicator_leq(b: i64, m: usize) -> Self {
        let bits: Vec<usize> = (0..m)
            .map(|i| ((b >> (m - 1 - i)) & 1) as usize)
            .collect();
        let mut cores = Vec::with_capacity(m);
        if m == 1 {
            let mut c = Core::new(1, 1);
            c.set(0, 0, 0, 1);
            c.set(0, 1, 0, if bits[0] >= 1 { 1 } else { 0 });
            cores.push(c);
            return TT { m, cores };
        }
        for idx in 0..m {
            let bb = bits[idx];
            let t: [[[i32; 2]; 2]; 2] = if bb == 0 {
                [[[1, 0], [0, 1]], [[1, 0], [0, 0]]]
            } else {
                [[[1, 0], [1, 0]], [[1, 0], [0, 1]]]
            };
            if idx == 0 {
                let mut c = Core::new(1, 2);
                for xbit in 0..2 {
                    c.set(0, xbit, 0, t[xbit][1][0]);
                    c.set(0, xbit, 1, t[xbit][1][1]);
                }
                cores.push(c);
            } else if idx == m - 1 {
                let mut c = Core::new(2, 1);
                for prev in 0..2 {
                    for xbit in 0..2 {
                        c.set(prev, xbit, 0, modd(t[xbit][prev][0] + t[xbit][prev][1]));
                    }
                }
                cores.push(c);
            } else {
                let mut c = Core::new(2, 2);
                for prev in 0..2 {
                    for xbit in 0..2 {
                        for next in 0..2 {
                            c.set(prev, xbit, next, t[xbit][prev][next]);
                        }
                    }
                }
                cores.push(c);
            }
        }
        TT { m, cores }
    }

    fn scalar_mul(&self, c: i32) -> Self {
        let c = modd(c);
        let mut tt = self.clone();
        for v in tt.cores[0].data.iter_mut() {
            *v = mulmod(*v, c);
        }
        tt
    }

    fn add(&self, other: &TT, coef_b: i32) -> Self {
        let coef_b = modd(coef_b);
        let m = self.m;
        let cores = (0..m)
            .map(|i| {
                let a = &self.cores[i];
                let b = &other.cores[i];
                if i == 0 {
                    let mut c = Core::new(1, a.r_r + b.r_r);
                    for bit in 0..2 {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                a.data.as_ptr().wrapping_add(bit * a.r_r),
                                c.data.as_mut_ptr().wrapping_add(bit * c.r_r),
                                a.r_r,
                            );
                        }
                        for r in 0..b.r_r {
                            c.set(0, bit, a.r_r + r, mulmod(b.get(0, bit, r), coef_b));
                        }
                    }
                    c
                } else if i == m - 1 {
                    let mut c = Core::new(a.r_l + b.r_l, 1);
                    for l in 0..a.r_l {
                        for bit in 0..2 {
                            c.set(l, bit, 0, a.get(l, bit, 0));
                        }
                    }
                    for l in 0..b.r_l {
                        for bit in 0..2 {
                            c.set(a.r_l + l, bit, 0, b.get(l, bit, 0));
                        }
                    }
                    c
                } else {
                    let mut c = Core::new(a.r_l + b.r_l, a.r_r + b.r_r);
                    for l in 0..a.r_l {
                        for bit in 0..2 {
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    a.data.as_ptr().wrapping_add(l * 2 * a.r_r + bit * a.r_r),
                                    c.data.as_mut_ptr().wrapping_add(l * 2 * c.r_r + bit * c.r_r),
                                    a.r_r,
                                );
                            }
                        }
                    }
                    for l in 0..b.r_l {
                        for bit in 0..2 {
                            unsafe {
                                std::ptr::copy_nonoverlapping(
                                    b.data.as_ptr().wrapping_add(l * 2 * b.r_r + bit * b.r_r),
                                    c.data.as_mut_ptr().wrapping_add((a.r_l + l) * 2 * c.r_r + bit * c.r_r + a.r_r),
                                    b.r_r,
                                );
                            }
                        }
                    }
                    c
                }
            })
            .collect();
        TT { m, cores }
    }

    fn hadamard(&self, other: &TT) -> Self {
        let m = self.m;
        let cores = (0..m)
            .map(|i| {
                let a = &self.cores[i];
                let b = &other.cores[i];
                let r_l = a.r_l * b.r_l;
                let r_r = a.r_r * b.r_r;
                let mut c = Core::new(r_l, r_r);
                let b_r_l = b.r_l;
                let b_r_r = b.r_r;

                if b_r_r == 1 {
                    for lb in 0..b_r_l {
                        for bit in 0..2 {
                            let bv = b.get(lb, bit, 0);
                            if bv == 0 {
                                continue;
                            }
                            if bv == 1 {
                                for la in 0..a.r_l {
                                    let l = la * b_r_l + lb;
                                    let c_ptr = c.data.as_mut_ptr().wrapping_add(l * 2 * r_r + bit * r_r);
                                    let a_ptr = a.data.as_ptr().wrapping_add(la * 2 * a.r_r + bit * a.r_r);
                                    unsafe {
                                        std::ptr::copy_nonoverlapping(a_ptr, c_ptr, a.r_r);
                                    }
                                }
                            } else {
                                let bv_scaled = mulmod(bv, R_MOD as i32) as u64;
                                for la in 0..a.r_l {
                                    let l = la * b_r_l + lb;
                                    let c_ptr = c.data.as_mut_ptr().wrapping_add(l * 2 * r_r + bit * r_r);
                                    let a_ptr = a.data.as_ptr().wrapping_add(la * 2 * a.r_r + bit * a.r_r);
                                    unsafe {
                                        axpy_add_avx2(c_ptr, a_ptr, bv_scaled, a.r_r);
                                    }
                                }
                            }
                        }
                    }
                } else if b_r_r == 2 {
                    for lb in 0..b_r_l {
                        for bit in 0..2 {
                            let b0 = b.get(lb, bit, 0);
                            let b1 = b.get(lb, bit, 1);
                            if b0 == 0 && b1 == 0 {
                                continue;
                            }
                            if b0 == 1 && b1 == 0 {
                                for la in 0..a.r_l {
                                    let l = la * b_r_l + lb;
                                    let c_ptr = c.data.as_mut_ptr().wrapping_add(l * 2 * r_r + bit * r_r);
                                    let a_ptr = a.data.as_ptr().wrapping_add(la * 2 * a.r_r + bit * a.r_r);
                                    unsafe {
                                        for ra in 0..a.r_r {
                                            let v = *a_ptr.add(ra);
                                            if v != 0 {
                                                *c_ptr.add(2 * ra) = v;
                                            }
                                        }
                                    }
                                }
                            } else if b0 == 0 && b1 == 1 {
                                for la in 0..a.r_l {
                                    let l = la * b_r_l + lb;
                                    let c_ptr = c.data.as_mut_ptr().wrapping_add(l * 2 * r_r + bit * r_r);
                                    let a_ptr = a.data.as_ptr().wrapping_add(la * 2 * a.r_r + bit * a.r_r);
                                    unsafe {
                                        for ra in 0..a.r_r {
                                            let v = *a_ptr.add(ra);
                                            if v != 0 {
                                                *c_ptr.add(2 * ra + 1) = v;
                                            }
                                        }
                                    }
                                }
                            } else if b0 == 1 && b1 == 1 {
                                for la in 0..a.r_l {
                                    let l = la * b_r_l + lb;
                                    let c_ptr = c.data.as_mut_ptr().wrapping_add(l * 2 * r_r + bit * r_r);
                                    let a_ptr = a.data.as_ptr().wrapping_add(la * 2 * a.r_r + bit * a.r_r);
                                    unsafe {
                                        for ra in 0..a.r_r {
                                            let v = *a_ptr.add(ra);
                                            if v != 0 {
                                                *c_ptr.add(2 * ra) = v;
                                                *c_ptr.add(2 * ra + 1) = v;
                                            }
                                        }
                                    }
                                }
                            } else {
                                // General fallback for arbitrary values
                                if b0 != 0 {
                                    let bv_scaled = mulmod(b0, R_MOD as i32) as u64;
                                    for la in 0..a.r_l {
                                        let l = la * b_r_l + lb;
                                        let c_ptr = c.data.as_mut_ptr().wrapping_add(l * 2 * r_r + bit * r_r);
                                        let a_ptr = a.data.as_ptr().wrapping_add(la * 2 * a.r_r + bit * a.r_r);
                                        unsafe {
                                            hadamard_strided_avx2(c_ptr, a_ptr, bv_scaled, a.r_r, 0);
                                        }
                                    }
                                }
                                if b1 != 0 {
                                    let bv_scaled = mulmod(b1, R_MOD as i32) as u64;
                                    for la in 0..a.r_l {
                                        let l = la * b_r_l + lb;
                                        let c_ptr = c.data.as_mut_ptr().wrapping_add(l * 2 * r_r + bit * r_r);
                                        let a_ptr = a.data.as_ptr().wrapping_add(la * 2 * a.r_r + bit * a.r_r);
                                        unsafe {
                                            hadamard_strided_avx2(c_ptr, a_ptr, bv_scaled, a.r_r, 1);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c
            })
            .collect();
        TT { m, cores }
    }

    fn apply_disjoint(&self) -> Self {
        let m = self.m;
        let cores = (0..m)
            .map(|i| {
                let s = &self.cores[i];
                let mut c = Core::with_capacity_like(s.r_l, s.r_r, s.data.len());
                let r_r = s.r_r;
                for l in 0..s.r_l {
                    let a0_ptr = s.data.as_ptr().wrapping_add(l * 2 * r_r);
                    let a1_ptr = a0_ptr.wrapping_add(r_r);
                    let c0_ptr = c.data.as_mut_ptr().wrapping_add(l * 2 * r_r);
                    let c1_ptr = c0_ptr.wrapping_add(r_r);
                    unsafe {
                        std::ptr::copy_nonoverlapping(a0_ptr, c1_ptr, r_r);
                        for r in 0..r_r {
                            let sum = *a0_ptr.add(r) as u32 + *a1_ptr.add(r) as u32;
                            *c0_ptr.add(r) = if sum >= MOD as u32 {
                                (sum - MOD as u32) as i32
                            } else {
                                sum as i32
                            };
                        }
                    }
                }
                c
            })
            .collect();
        TT { m, cores }
    }

    fn sum_all(&self) -> i32 {
        SCRATCH.with(|sc| {
            let mut sc = sc.borrow_mut();
            sc.sum_vec.clear();
            sc.sum_vec.push(1i32);
            for i in 0..self.m {
                let c = &self.cores[i];
                sc.sum_new.clear();
                sc.sum_new.resize(c.r_r, 0);
                let dst = sc.sum_new.as_mut_ptr();
                for l in 0..c.r_l {
                    let vl = sc.sum_vec[l];
                    if vl == 0 {
                        continue;
                    }
                    let vl_scaled = mulmod(vl, R_MOD as i32) as u64;
                    let p0 = c.data.as_ptr().wrapping_add(l * 2 * c.r_r);
                    let p1 = p0.wrapping_add(c.r_r);
                    unsafe {
                        axpy_add_avx2(dst, p0, vl_scaled, c.r_r);
                        axpy_add_avx2(dst, p1, vl_scaled, c.r_r);
                    }
                }
                let tmp = std::mem::take(&mut sc.sum_new);
                sc.sum_new = std::mem::replace(&mut sc.sum_vec, tmp);
            }
            modd(sc.sum_vec[0])
        })
    }

    fn gauss_elim(mat: &mut [i32], nrows: usize, ncols: usize, pivots: &mut Vec<usize>) {
        pivots.clear();
        let mut row_ptr = 0;
        for c in 0..ncols {
            if row_ptr >= nrows {
                break;
            }
            let mut piv = usize::MAX;
            unsafe {
                for rr in row_ptr..nrows {
                    if *mat.get_unchecked(rr * ncols + c) != 0 {
                        piv = rr;
                        break;
                    }
                }
            }
            if piv == usize::MAX {
                continue;
            }
            if piv != row_ptr {
                unsafe {
                    let pa = mat.as_mut_ptr().add(row_ptr * ncols);
                    let pb = mat.as_mut_ptr().add(piv * ncols);
                    for j in 0..ncols {
                        let tmp = *pa.add(j);
                        *pa.add(j) = *pb.add(j);
                        *pb.add(j) = tmp;
                    }
                }
            }
            let pivot_base = row_ptr * ncols;
            let p_pivot = unsafe { *mat.get_unchecked(pivot_base + c) };
            if p_pivot != 1 {
                let inv = modinv(p_pivot);
                let inv_scaled = mulmod(inv, R_MOD as i32) as u64;
                unsafe {
                    let prow = mat.as_mut_ptr().add(pivot_base);
                    *prow.add(c) = 1;
                    let rem = ncols - (c + 1);
                    if rem > 0 {
                        scale_row_avx2(prow.add(c + 1), inv_scaled, rem);
                    }
                }
            }
            unsafe {
                let prow = mat.as_ptr().add(pivot_base);
                for rr in 0..nrows {
                    if rr == row_ptr {
                        continue;
                    }
                    let rr_base = rr * ncols;
                    let f = *mat.get_unchecked(rr_base + c) as u32;
                    if f == 0 {
                        continue;
                    }
                    let f_scaled = mulmod(f as i32, R_MOD as i32) as u64;
                    let row = mat.as_mut_ptr().add(rr_base);
                    *row.add(c) = 0;
                    let rem = ncols - (c + 1);
                    if rem > 0 {
                        axpy_sub_avx2(row.add(c + 1), prow.add(c + 1), f_scaled, rem);
                    }
                }
            }
            pivots.push(c);
            row_ptr += 1;
        }
    }

    fn reduce_left(&mut self) {
        let m = self.m;
        SCRATCH.with(|sc| {
            let mut sc = sc.borrow_mut();
            for i in 0..m - 1 {
                let r_l = self.cores[i].r_l;
                let r_r = self.cores[i].r_r;
                if r_r <= 1 {
                    continue;
                }
                let nrows = 2 * r_l;
                let need = nrows * r_r;
                if sc.mat.len() < need {
                    sc.mat.resize(need, 0);
                }
                unsafe {
                    let mptr = sc.mat.as_mut_ptr();
                    let cptr = self.cores[i].data.as_ptr();
                    let rr = r_r;
                    for l in 0..r_l {
                        let src0 = cptr.add(l * 2 * rr);
                        let src1 = cptr.add(l * 2 * rr + rr);
                        let dst0 = mptr.add((2 * l) * rr);
                        let dst1 = mptr.add((2 * l + 1) * rr);
                        core::ptr::copy_nonoverlapping(src0, dst0, rr);
                        core::ptr::copy_nonoverlapping(src1, dst1, rr);
                    }
                }
                let mut pivots = std::mem::take(&mut sc.pivots);
                let mat_slice = &mut sc.mat[..need];
                let t_g0 = std::time::Instant::now();
                Self::gauss_elim(mat_slice, nrows, r_r, &mut pivots);
                let t_g1 = std::time::Instant::now();
                sc.t_gauss += t_g1 - t_g0;
                let rank = pivots.len();
                if rank == 0 || rank == r_r {
                    sc.pivots = pivots;
                    continue;
                }

                let mut new_core = Core::new(r_l, rank);
                for l in 0..r_l {
                    for (k, &p) in pivots.iter().enumerate() {
                        new_core.set(l, 0, k, self.cores[i].get(l, 0, p));
                        new_core.set(l, 1, k, self.cores[i].get(l, 1, p));
                    }
                }

                let r_next = self.cores[i + 1].r_r;
                let mut new_nxt = Core::new(rank, r_next);
                sc.is_pivot.clear();
                sc.is_pivot.resize(r_r, false);
                for &p in &pivots {
                    sc.is_pivot[p] = true;
                }

                for (k, &p) in pivots.iter().enumerate() {
                    for bit in 0..2 {
                        for t in 0..r_next {
                            new_nxt.set(k, bit, t, self.cores[i + 1].get(p, bit, t));
                        }
                    }
                }

                let t_u0 = std::time::Instant::now();
                let len2 = 2 * r_next;
                for j in 0..r_r {
                    if sc.is_pivot[j] {
                        continue;
                    }
                    for k in 0..rank {
                        let coeff = unsafe { *sc.mat.get_unchecked(k * r_r + j) };
                        if coeff == 0 {
                            continue;
                        }
                        let c_scaled = mulmod(coeff, R_MOD as i32) as u64;
                        let dst_base = k * len2;
                        let src_base = j * len2;
                        unsafe {
                            let dst = new_nxt.data.as_mut_ptr().add(dst_base);
                            let src = self.cores[i + 1].data.as_ptr().add(src_base);
                            axpy_add_avx2(dst, src, c_scaled, len2);
                        }
                    }
                }
                let t_u1 = std::time::Instant::now();
                sc.t_update += t_u1 - t_u0;
                sc.pivots = pivots;

                self.cores[i] = new_core;
                self.cores[i + 1] = new_nxt;
            }
        });
    }
}

fn solve(n: usize, b: i64) -> i32 {
    let m = if b == 0 {
        1
    } else {
        (64 - b.leading_zeros()) as usize
    };

    let mut mask = TT::indicator_leq(b, m);
    mask.reduce_left();

    let mut dp = TT::indicator_leq(b, m);
    dp.reduce_left();

    let ones = TT::all_ones(m);

    for _ in 0..n - 1 {
        let total = dp.sum_all();
        let j = ones.scalar_mul(total);
        let bv = dp.apply_disjoint();
        let nxt = j.add(&bv, MOD - 1);
        let mut masked = nxt.hadamard(&mask);
        masked.reduce_left();
        dp = masked;
    }

    dp.sum_all()
}

fn main() {
    println!("{}", solve(123, 123456789));
}
