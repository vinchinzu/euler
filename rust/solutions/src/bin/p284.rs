// Project Euler 284: Steady Squares in base 14
//
// Hensel lift of the two non-trivial idempotents (start 8 and 7) using
// x <- x^2 (3 - 2x) while doubling precision. Limbs are base 14^16 (fits
// in u64). Schoolbook mul skips zero outer limbs; carry is always applied.

const N: usize = 10000;
const DIGITS_PER_LIMB: usize = 16;
const BASE: u128 = 14u128.pow(DIGITS_PER_LIMB as u32);
const BASE_U64: u64 = BASE as u64;

#[inline(always)]
fn divmod_base(t: u128) -> (u64, u128) {
    let q = t / BASE;
    ((t - q * BASE) as u64, q)
}

/// Schoolbook product truncated to `out_len` limbs (mod BASE^out_len).
fn mul_trunc(a: &[u64], b: &[u64], out_len: usize) -> Vec<u64> {
    let mut tmp = vec![0u64; out_len];
    let na = a.len().min(out_len);
    let nb = b.len().min(out_len);
    for i in 0..na {
        // SAFETY: i < na <= a.len()
        let ai = unsafe { *a.get_unchecked(i) };
        if ai == 0 {
            continue;
        }
        let ai = ai as u128;
        let jmax = (out_len - i).min(nb);
        let mut carry = 0u128;
        for j in 0..jmax {
            // SAFETY: j < nb <= b.len(); i+j < i+jmax <= out_len
            let t = unsafe { *tmp.get_unchecked(i + j) } as u128
                + ai * unsafe { *b.get_unchecked(j) } as u128
                + carry;
            let (r, q) = divmod_base(t);
            unsafe {
                *tmp.get_unchecked_mut(i + j) = r;
            }
            carry = q;
        }
        let mut k = i + jmax;
        while carry != 0 && k < out_len {
            let t = unsafe { *tmp.get_unchecked(k) } as u128 + carry;
            let (r, q) = divmod_base(t);
            unsafe {
                *tmp.get_unchecked_mut(k) = r;
            }
            carry = q;
            k += 1;
        }
    }
    tmp
}

/// 2x - 3 with x interpreted as `n` little-endian limbs.
fn two_x_minus_3(x: &[u64], n: usize) -> Vec<u64> {
    let mut f = Vec::with_capacity(n + 1);
    let mut carry = 0u128;
    for i in 0..n {
        let t = unsafe { *x.get_unchecked(i) } as u128 * 2 + carry;
        let (r, q) = divmod_base(t);
        f.push(r);
        carry = q;
    }
    if carry != 0 {
        f.push(carry as u64);
    }
    let mut borrow = 3u64;
    for slot in &mut f {
        if *slot >= borrow {
            *slot -= borrow;
            break;
        }
        *slot = *slot + BASE_U64 - borrow;
        borrow = 1;
    }
    f
}

fn neg_mod_inplace(a: &mut [u64]) {
    let n = a.len();
    let mut i = 0;
    while i < n && unsafe { *a.get_unchecked(i) } == 0 {
        i += 1;
    }
    if i == n {
        return;
    }
    unsafe {
        *a.get_unchecked_mut(i) = BASE_U64 - *a.get_unchecked(i);
    }
    i += 1;
    while i < n {
        unsafe {
            *a.get_unchecked_mut(i) = BASE_U64 - 1 - *a.get_unchecked(i);
        }
        i += 1;
    }
}

/// One Newton step: x^2 (3-2x) = -(x^2 (2x-3)) mod BASE^new_n.
/// `x` is correct for `old_n` limbs (higher digits treated as 0).
fn hensel_step(x: &[u64], old_n: usize, new_n: usize) -> Vec<u64> {
    let x_lo = &x[..old_n];
    let x2 = mul_trunc(x_lo, x_lo, new_n);
    let f = two_x_minus_3(x_lo, old_n);
    let mut prod = mul_trunc(&x2, &f, new_n);
    neg_mod_inplace(&mut prod);
    prod
}

fn lift(start: u64) -> Vec<u64> {
    let mut val = start as u128;
    let mut prec = 1u32;
    while prec < DIGITS_PER_LIMB as u32 {
        let np = prec * 2;
        let m = 14u128.pow(np);
        val = (val * val % m) * ((3 + m - (2 * val) % m) % m) % m;
        prec = np;
    }

    let mut limbs = vec![val as u64];
    let mut prec_digits = DIGITS_PER_LIMB;
    while prec_digits < N {
        let new_digits = (prec_digits * 2).min(N);
        let old_n = limbs.len();
        let new_n = new_digits.div_ceil(DIGITS_PER_LIMB);
        limbs = hensel_step(&limbs, old_n, new_n);
        prec_digits = new_digits;
    }
    limbs
}

fn digit_sum_contrib(limbs: &[u64]) -> i64 {
    let mut digits = vec![0u8; N];
    let mut idx = 0;
    for &limb in limbs {
        let mut v = limb;
        for _ in 0..DIGITS_PER_LIMB {
            digits[idx] = (v % 14) as u8;
            v /= 14;
            idx += 1;
            if idx == N {
                break;
            }
        }
    }
    let mut suffix = 0i64;
    let mut total = 0i64;
    for i in (0..N).rev() {
        if digits[i] != 0 {
            suffix += 1;
        }
        total += digits[i] as i64 * suffix;
    }
    total
}

fn to_base14(mut v: i64) -> String {
    if v == 0 {
        return "0".to_string();
    }
    const HEX: &[u8] = b"0123456789abcd";
    let mut buf = Vec::new();
    while v > 0 {
        buf.push(HEX[(v % 14) as usize]);
        v /= 14;
    }
    buf.reverse();
    buf.into_iter().map(|c| c as char).collect()
}

fn main() {
    let (a, b) = rayon::join(|| lift(8), || lift(7));
    let result = 1 + digit_sum_contrib(&a) + digit_sum_contrib(&b);
    println!("{}", to_base14(result));
}
