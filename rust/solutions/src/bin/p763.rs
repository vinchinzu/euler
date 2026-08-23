#![allow(unsafe_op_in_unsafe_fn)]
// Project Euler 763 - Amoebas in a 3D Grid
// Each n only needs the last n+2 idx values, so u/v are circular of length n+3
// with k stored innermost. AVX2 8-wide 32-bit addmod on the k-stencil.

const M_VAL: usize = 9999;
const MOD: u64 = 1_000_000_000;
const MAX_N: usize = 160;

#[inline(always)]
fn red(x: u64) -> u32 {
    let mut x = x;
    x -= 4 * MOD * u64::from(x >= 4 * MOD);
    x -= 2 * MOD * u64::from(x >= 2 * MOD);
    x -= MOD * u64::from(x >= MOD);
    x as u32
}

fn main() {
    let mut n_tmp = 0usize;
    while (n_tmp + 1) * (n_tmp + 2) / 2 <= M_VAL {
        n_tmp += 1;
    }
    let n_val = n_tmp + 2;

    let mut offset = [0i32; MAX_N];
    let mut lens = [0i32; MAX_N];
    let mut cap = [0usize; MAX_N];
    for n in 0..n_val + 2 {
        offset[n] = ((n + 1) * (n + 2) / 2) as i32;
        let ln = M_VAL as i32 - offset[n] + 1;
        lens[n] = if ln > 0 { ln } else { 0 };
        cap[n] = n + 3;
    }

    let mut u_store: Vec<Vec<u32>> = (0..n_val + 2).map(|_| Vec::new()).collect();
    let mut v_store: Vec<Vec<u32>> = (0..n_val + 2).map(|_| Vec::new()).collect();
    for n in 1..n_val + 2 {
        if lens[n] > 0 {
            let sz = cap[n] * n;
            u_store[n] = vec![0u32; sz];
            v_store[n] = vec![0u32; sz];
        }
    }

    let mut u_ptr = [std::ptr::null_mut::<u32>(); MAX_N];
    let mut v_ptr = [std::ptr::null_mut::<u32>(); MAX_N];
    for n in 1..n_val + 2 {
        u_ptr[n] = u_store[n].as_mut_ptr();
        v_ptr[n] = v_store[n].as_mut_ptr();
    }

    let mut f0 = [0u32; M_VAL + 1];
    let mut a2 = [0u32; M_VAL + 1];
    a2[0] = 1;

    unsafe {
        if is_x86_feature_detected!("avx2") {
            solve_avx2(n_val, &offset, &lens, &cap, &u_ptr, &v_ptr, &mut f0, &mut a2);
        } else {
            solve::<false>(n_val, &offset, &lens, &cap, &u_ptr, &v_ptr, &mut f0, &mut a2);
        }
    }

    println!("{}", a2[9999]);
}

#[target_feature(enable = "avx2")]
unsafe fn solve_avx2(
    n_val: usize,
    offset: &[i32; MAX_N],
    lens: &[i32; MAX_N],
    cap: &[usize; MAX_N],
    u_ptr: &[*mut u32; MAX_N],
    v_ptr: &[*mut u32; MAX_N],
    f0: &mut [u32; M_VAL + 1],
    a2: &mut [u32; M_VAL + 1],
) {
    solve::<true>(n_val, offset, lens, cap, u_ptr, v_ptr, f0, a2);
}

#[inline(always)]
unsafe fn solve<const AVX2: bool>(
    n_val: usize,
    offset: &[i32; MAX_N],
    lens: &[i32; MAX_N],
    cap: &[usize; MAX_N],
    u_ptr: &[*mut u32; MAX_N],
    v_ptr: &[*mut u32; MAX_N],
    f0: &mut [u32; M_VAL + 1],
    a2: &mut [u32; M_VAL + 1],
) {
    let mut slot = [0usize; MAX_N];
    let mut n_active: i32 = 0;
    for m in 0..=M_VAL as i32 {
        while n_active + 1 < n_val as i32 + 1 && offset[(n_active + 1) as usize] <= m {
            n_active += 1;
        }

        for n in 1..=n_active as usize {
            let idx1 = m - n as i32 - 2 - offset[n];
            let idx2 = m - n as i32 - 3 - offset[n + 1];
            let lnp = lens[n + 1];
            let cn = cap[n];
            let sn = slot[n];

            if n == 1 {
                let mut val_u: u64 = 0;
                let mut val_v: u64 = 0;
                if idx1 >= 0 {
                    let s1 = sn + 1;
                    let s1 = if s1 == cn { 0 } else { s1 };
                    let u = *u_ptr[1].add(s1);
                    let v = *v_ptr[1].add(s1);
                    val_u += 2 * u as u64 + v as u64;
                    val_v += 2 * v as u64 + 2 * u as u64;
                }
                if idx2 >= 0 && lnp > 0 {
                    let c2 = cap[2];
                    let s2 = slot[2] + 1;
                    let s2 = if s2 == c2 { 0 } else { s2 };
                    val_u += *v_ptr[2].add(s2 * 2) as u64 + *u_ptr[2].add(s2 * 2 + 1) as u64;
                    val_v += *v_ptr[2].add(s2 * 2 + 1) as u64 + 2 * *u_ptr[2].add(s2 * 2) as u64;
                }
                if m >= 2 {
                    let f = f0[(m - 2) as usize] as u64;
                    val_u += f;
                    val_v += f;
                }
                *u_ptr[1].add(sn) = red(val_u);
                *v_ptr[1].add(sn) = red(val_v);
                let next = sn + 1;
                slot[1] = if next == cn { 0 } else { next };
                continue;
            }

            let un = u_ptr[n];
            let vn = v_ptr[n];
            let dst_u = un.add(sn * n);
            let dst_v = vn.add(sn * n);
            let nm = n - 1;
            let np = n + 1;

            if idx1 < 0 {
                // n-1 already wrote and incremented slot this m.
                let sm = slot[nm];
                let src_u = u_ptr[nm].add(sm * nm);
                let src_v = v_ptr[nm].add(sm * nm);
                std::ptr::copy_nonoverlapping(src_u, dst_u, n - 1);
                std::ptr::copy_nonoverlapping(src_v, dst_v, n - 1);
                *dst_u.add(n - 1) = *dst_u.add(n - 2);
                *dst_v.add(n - 1) = *dst_v.add(n - 2);
                let next = sn + 1;
                slot[n] = if next == cn { 0 } else { next };
                continue;
            }

            let s1 = sn + 1;
            let s1 = if s1 == cn { 0 } else { s1 };
            let src_u = un.add(s1 * n);
            let src_v = vn.add(s1 * n);
            let u_n1 = *src_u;
            let v_n1 = *src_v;

            let sm = slot[nm];
            let m_u = u_ptr[nm].add(sm * nm);
            let m_v = v_ptr[nm].add(sm * nm);

            if idx2 >= 0 && lnp > 0 {
                let cp = cap[np];
                let sp = slot[np] + 1;
                let sp = if sp == cp { 0 } else { sp };
                let p_u = u_ptr[np].add(sp * np);
                let p_v = v_ptr[np].add(sp * np);
                let u_p1 = *p_u;
                let v_p1 = *p_v;
                let add_u = v_p1 as u64 + v_n1 as u64;
                let add_v = u_p1 as u64 + u_n1 as u64;

                if AVX2 {
                    stencil_full_avx2(
                        dst_u, dst_v, src_u, src_v, p_u, p_v, m_u, m_v, n, v_p1, v_n1, u_p1, u_n1,
                    );
                } else {
                    for k in 0..n - 1 {
                        let uval = *src_u.add(k) as u64
                            + *src_u.add(k + 1) as u64
                            + *p_u.add(k + 1) as u64
                            + *m_u.add(k) as u64
                            + add_u;
                        *dst_u.add(k) = red(uval);
                        let vval = *src_v.add(k) as u64
                            + *src_v.add(k + 1) as u64
                            + *p_v.add(k + 1) as u64
                            + *m_v.add(k) as u64
                            + add_v;
                        *dst_v.add(k) = red(vval);
                    }
                    let uval = 2 * *src_u.add(n - 1) as u64
                        + v_n1 as u64
                        + v_p1 as u64
                        + *p_u.add(n) as u64
                        + *m_u.add(n - 2) as u64;
                    *dst_u.add(n - 1) = red(uval);
                    let vval = 2 * *src_v.add(n - 1) as u64
                        + 2 * u_n1 as u64
                        + *p_v.add(n) as u64
                        + 2 * u_p1 as u64
                        + *m_v.add(n - 2) as u64;
                    *dst_v.add(n - 1) = red(vval);
                }
            } else {
                let add_u = v_n1 as u64;
                let add_v = u_n1 as u64;
                if AVX2 {
                    stencil_nop_avx2(dst_u, dst_v, src_u, src_v, m_u, m_v, n, v_n1, u_n1);
                } else {
                    for k in 0..n - 1 {
                        let uval = *src_u.add(k) as u64
                            + *m_u.add(k) as u64
                            + add_u
                            + *src_u.add(k + 1) as u64;
                        *dst_u.add(k) = red(uval);
                        let vval = *src_v.add(k) as u64
                            + *m_v.add(k) as u64
                            + *src_v.add(k + 1) as u64
                            + add_v;
                        *dst_v.add(k) = red(vval);
                    }
                    let uval = 2 * *src_u.add(n - 1) as u64 + v_n1 as u64 + *m_u.add(n - 2) as u64;
                    *dst_u.add(n - 1) = red(uval);
                    let vval = 2 * *src_v.add(n - 1) as u64 + 2 * u_n1 as u64 + *m_v.add(n - 2) as u64;
                    *dst_v.add(n - 1) = red(vval);
                }
            }

            let next = sn + 1;
            slot[n] = if next == cn { 0 } else { next };
        }

        let mut val_f: u64 = 0;
        if m >= 1 {
            val_f += a2[(m - 1) as usize] as u64;
        }
        if m >= 2 {
            val_f += 4 * f0[(m - 2) as usize] as u64;
        }
        let mp = m - 3;
        if mp >= offset[1] && lens[1] > 0 {
            // n=1 already incremented; slot[1] holds idx_cur-3 (delay 3, cap 4).
            let id1 = slot[1];
            val_f += 2 * (*u_ptr[1].add(id1)) as u64 + (*v_ptr[1].add(id1)) as u64;
        }
        f0[m as usize] = red(val_f);

        if m >= 1 {
            let mut val_a: u64 = 3 * a2[(m - 1) as usize] as u64;
            if m >= 2 {
                val_a += 3 * f0[(m - 2) as usize] as u64;
            }
            a2[m as usize] = red(val_a);
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn addmod_m256(a: std::arch::x86_64::__m256i, b: std::arch::x86_64::__m256i) -> std::arch::x86_64::__m256i {
    use std::arch::x86_64::*;
    let s = _mm256_add_epi32(a, b);
    let gt = _mm256_cmpgt_epi32(s, _mm256_set1_epi32(999_999_999));
    _mm256_sub_epi32(s, _mm256_and_si256(gt, _mm256_set1_epi32(1_000_000_000)))
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn stencil_full_avx2(
    dst_u: *mut u32,
    dst_v: *mut u32,
    src_u: *const u32,
    src_v: *const u32,
    p_u: *const u32,
    p_v: *const u32,
    m_u: *const u32,
    m_v: *const u32,
    n: usize,
    v_p1: u32,
    v_n1: u32,
    u_p1: u32,
    u_n1: u32,
) {
    use std::arch::x86_64::*;
    let add_u = addmod_m256(_mm256_set1_epi32(v_p1 as i32), _mm256_set1_epi32(v_n1 as i32));
    let add_v = addmod_m256(_mm256_set1_epi32(u_p1 as i32), _mm256_set1_epi32(u_n1 as i32));
    let mut k = 0usize;
    while k + 8 <= n - 1 {
        let su = _mm256_loadu_si256(src_u.add(k) as *const __m256i);
        let su1 = _mm256_loadu_si256(src_u.add(k + 1) as *const __m256i);
        let pu1 = _mm256_loadu_si256(p_u.add(k + 1) as *const __m256i);
        let mu = _mm256_loadu_si256(m_u.add(k) as *const __m256i);
        let ru = addmod_m256(addmod_m256(addmod_m256(su, su1), pu1), addmod_m256(mu, add_u));
        _mm256_storeu_si256(dst_u.add(k) as *mut __m256i, ru);

        let sv = _mm256_loadu_si256(src_v.add(k) as *const __m256i);
        let sv1 = _mm256_loadu_si256(src_v.add(k + 1) as *const __m256i);
        let pv1 = _mm256_loadu_si256(p_v.add(k + 1) as *const __m256i);
        let mv = _mm256_loadu_si256(m_v.add(k) as *const __m256i);
        let rv = addmod_m256(addmod_m256(addmod_m256(sv, sv1), pv1), addmod_m256(mv, add_v));
        _mm256_storeu_si256(dst_v.add(k) as *mut __m256i, rv);
        k += 8;
    }
    let add_u64 = v_p1 as u64 + v_n1 as u64;
    let add_v64 = u_p1 as u64 + u_n1 as u64;
    while k < n - 1 {
        let uval = *src_u.add(k) as u64
            + *src_u.add(k + 1) as u64
            + *p_u.add(k + 1) as u64
            + *m_u.add(k) as u64
            + add_u64;
        *dst_u.add(k) = red(uval);
        let vval = *src_v.add(k) as u64
            + *src_v.add(k + 1) as u64
            + *p_v.add(k + 1) as u64
            + *m_v.add(k) as u64
            + add_v64;
        *dst_v.add(k) = red(vval);
        k += 1;
    }
    let uval = 2 * *src_u.add(n - 1) as u64
        + v_n1 as u64
        + v_p1 as u64
        + *p_u.add(n) as u64
        + *m_u.add(n - 2) as u64;
    *dst_u.add(n - 1) = red(uval);
    let vval = 2 * *src_v.add(n - 1) as u64
        + 2 * u_n1 as u64
        + *p_v.add(n) as u64
        + 2 * u_p1 as u64
        + *m_v.add(n - 2) as u64;
    *dst_v.add(n - 1) = red(vval);
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn stencil_full_avx2(
    dst_u: *mut u32,
    dst_v: *mut u32,
    src_u: *const u32,
    src_v: *const u32,
    p_u: *const u32,
    p_v: *const u32,
    m_u: *const u32,
    m_v: *const u32,
    n: usize,
    v_p1: u32,
    v_n1: u32,
    u_p1: u32,
    u_n1: u32,
) {
    let add_u = v_p1 as u64 + v_n1 as u64;
    let add_v = u_p1 as u64 + u_n1 as u64;
    for k in 0..n - 1 {
        let uval = *src_u.add(k) as u64
            + *src_u.add(k + 1) as u64
            + *p_u.add(k + 1) as u64
            + *m_u.add(k) as u64
            + add_u;
        *dst_u.add(k) = red(uval);
        let vval = *src_v.add(k) as u64
            + *src_v.add(k + 1) as u64
            + *p_v.add(k + 1) as u64
            + *m_v.add(k) as u64
            + add_v;
        *dst_v.add(k) = red(vval);
    }
    let uval = 2 * *src_u.add(n - 1) as u64 + v_n1 as u64 + v_p1 as u64 + *p_u.add(n) as u64 + *m_u.add(n - 2) as u64;
    *dst_u.add(n - 1) = red(uval);
    let vval = 2 * *src_v.add(n - 1) as u64 + 2 * u_n1 as u64 + *p_v.add(n) as u64 + 2 * u_p1 as u64 + *m_v.add(n - 2) as u64;
    *dst_v.add(n - 1) = red(vval);
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn stencil_nop_avx2(
    dst_u: *mut u32,
    dst_v: *mut u32,
    src_u: *const u32,
    src_v: *const u32,
    m_u: *const u32,
    m_v: *const u32,
    n: usize,
    v_n1: u32,
    u_n1: u32,
) {
    use std::arch::x86_64::*;
    let add_u = _mm256_set1_epi32(v_n1 as i32);
    let add_v = _mm256_set1_epi32(u_n1 as i32);
    let mut k = 0usize;
    while k + 8 <= n - 1 {
        let su = _mm256_loadu_si256(src_u.add(k) as *const __m256i);
        let su1 = _mm256_loadu_si256(src_u.add(k + 1) as *const __m256i);
        let mu = _mm256_loadu_si256(m_u.add(k) as *const __m256i);
        let ru = addmod_m256(addmod_m256(su, su1), addmod_m256(mu, add_u));
        _mm256_storeu_si256(dst_u.add(k) as *mut __m256i, ru);

        let sv = _mm256_loadu_si256(src_v.add(k) as *const __m256i);
        let sv1 = _mm256_loadu_si256(src_v.add(k + 1) as *const __m256i);
        let mv = _mm256_loadu_si256(m_v.add(k) as *const __m256i);
        let rv = addmod_m256(addmod_m256(sv, sv1), addmod_m256(mv, add_v));
        _mm256_storeu_si256(dst_v.add(k) as *mut __m256i, rv);
        k += 8;
    }
    let add_u64 = v_n1 as u64;
    let add_v64 = u_n1 as u64;
    while k < n - 1 {
        let uval = *src_u.add(k) as u64 + *m_u.add(k) as u64 + add_u64 + *src_u.add(k + 1) as u64;
        *dst_u.add(k) = red(uval);
        let vval = *src_v.add(k) as u64 + *m_v.add(k) as u64 + *src_v.add(k + 1) as u64 + add_v64;
        *dst_v.add(k) = red(vval);
        k += 1;
    }
    let uval = 2 * *src_u.add(n - 1) as u64 + v_n1 as u64 + *m_u.add(n - 2) as u64;
    *dst_u.add(n - 1) = red(uval);
    let vval = 2 * *src_v.add(n - 1) as u64 + 2 * u_n1 as u64 + *m_v.add(n - 2) as u64;
    *dst_v.add(n - 1) = red(vval);
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn stencil_nop_avx2(
    dst_u: *mut u32,
    dst_v: *mut u32,
    src_u: *const u32,
    src_v: *const u32,
    m_u: *const u32,
    m_v: *const u32,
    n: usize,
    v_n1: u32,
    u_n1: u32,
) {
    let add_u = v_n1 as u64;
    let add_v = u_n1 as u64;
    for k in 0..n - 1 {
        let uval = *src_u.add(k) as u64 + *m_u.add(k) as u64 + add_u + *src_u.add(k + 1) as u64;
        *dst_u.add(k) = red(uval);
        let vval = *src_v.add(k) as u64 + *m_v.add(k) as u64 + *src_v.add(k + 1) as u64 + add_v;
        *dst_v.add(k) = red(vval);
    }
    let uval = 2 * *src_u.add(n - 1) as u64 + v_n1 as u64 + *m_u.add(n - 2) as u64;
    *dst_u.add(n - 1) = red(uval);
    let vval = 2 * *src_v.add(n - 1) as u64 + 2 * u_n1 as u64 + *m_v.add(n - 2) as u64;
    *dst_v.add(n - 1) = red(vval);
}
