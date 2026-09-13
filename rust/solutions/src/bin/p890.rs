// Problem 890 - Binary Partitions
//
// p(n) = number of ways to write n as unordered sum of powers of 2.
// Compute p(7^777) mod 10^9+7.
//
// Carry-DP convolution. Even/odd split for contiguous schoolbook;
// Karatsuba above a size cutoff. Serial in the bit loop.

const MOD: u64 = 1_000_000_007;
const KARATSUBA_CUTOFF: usize = 64;

/// Big integer as little-endian limbs of u64 (base 2^64).
struct BigUint {
    limbs: Vec<u64>,
}

impl BigUint {
    fn from_u64(v: u64) -> Self {
        BigUint {
            limbs: if v == 0 { vec![0] } else { vec![v] },
        }
    }

    fn mul_assign_u64(&mut self, x: u64) {
        let mut carry = 0u128;
        for limb in self.limbs.iter_mut() {
            let prod = *limb as u128 * x as u128 + carry;
            *limb = prod as u64;
            carry = prod >> 64;
        }
        if carry > 0 {
            self.limbs.push(carry as u64);
        }
    }

    fn shr1(&mut self) {
        let mut carry = 0u64;
        for limb in self.limbs.iter_mut().rev() {
            let new_carry = *limb & 1;
            *limb = (*limb >> 1) | (carry << 63);
            carry = new_carry;
        }
        while self.limbs.len() > 1 && *self.limbs.last().unwrap() == 0 {
            self.limbs.pop();
        }
    }

    fn bit(&self, k: usize) -> u64 {
        let limb_idx = k / 64;
        let bit_idx = k % 64;
        if limb_idx >= self.limbs.len() {
            0
        } else {
            (self.limbs[limb_idx] >> bit_idx) & 1
        }
    }

    fn bit_length(&self) -> usize {
        if self.limbs.is_empty() {
            return 0;
        }
        let top = *self.limbs.last().unwrap();
        if top == 0 {
            return 0;
        }
        (self.limbs.len() - 1) * 64 + (64 - top.leading_zeros() as usize)
    }
}

#[inline]
fn pascal_next(row: &mut Vec<u64>) {
    row.push(1);
    let n = row.len() - 1;
    for j in (1..n).rev() {
        let s = row[j] + row[j - 1];
        row[j] = if s >= MOD { s - MOD } else { s };
    }
}

#[inline]
fn add_mod(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline]
fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}

fn schoolbook(a: &[u64], b: &[u64]) -> Vec<u64> {
    let la = a.len();
    let lb = b.len();
    if la == 0 || lb == 0 {
        return Vec::new();
    }
    let mut conv = vec![0u128; la + lb - 1];
    unsafe {
        let ap = a.as_ptr();
        let bp = b.as_ptr();
        let cp = conv.as_mut_ptr();
        for j in 0..la {
            let aj = *ap.add(j);
            if aj == 0 {
                continue;
            }
            let mut dst = cp.add(j);
            let mut k = 0usize;
            while k + 4 <= lb {
                *dst += aj.wrapping_mul(*bp.add(k)) as u128;
                *dst.add(1) += aj.wrapping_mul(*bp.add(k + 1)) as u128;
                *dst.add(2) += aj.wrapping_mul(*bp.add(k + 2)) as u128;
                *dst.add(3) += aj.wrapping_mul(*bp.add(k + 3)) as u128;
                dst = dst.add(4);
                k += 4;
            }
            while k < lb {
                *dst += aj.wrapping_mul(*bp.add(k)) as u128;
                dst = dst.add(1);
                k += 1;
            }
        }
    }
    let m128 = MOD as u128;
    conv.iter().map(|&x| (x % m128) as u64).collect()
}

fn add_poly(a: &[u64], b: &[u64]) -> Vec<u64> {
    let n = a.len().max(b.len());
    let mut out = vec![0u64; n];
    for i in 0..a.len() {
        out[i] = a[i];
    }
    for i in 0..b.len() {
        out[i] = add_mod(out[i], b[i]);
    }
    out
}

fn karatsuba(a: &[u64], b: &[u64]) -> Vec<u64> {
    let la = a.len();
    let lb = b.len();
    if la == 0 || lb == 0 {
        return Vec::new();
    }
    if la.min(lb) <= KARATSUBA_CUTOFF {
        return schoolbook(a, b);
    }
    let m = (la.max(lb) + 1) / 2;
    let a0 = if la > m { &a[..m] } else { a };
    let a1 = if la > m { &a[m..] } else { &[] as &[u64] };
    let b0 = if lb > m { &b[..m] } else { b };
    let b1 = if lb > m { &b[m..] } else { &[] as &[u64] };
    let z0 = karatsuba(a0, b0);
    let z2 = karatsuba(a1, b1);
    let sa = add_poly(a0, a1);
    let sb = add_poly(b0, b1);
    let mut z1 = karatsuba(&sa, &sb);
    let n1 = z1.len().max(z0.len()).max(z2.len());
    z1.resize(n1, 0);
    for i in 0..z0.len() {
        z1[i] = sub_mod(z1[i], z0[i]);
    }
    for i in 0..z2.len() {
        z1[i] = sub_mod(z1[i], z2[i]);
    }
    let out_len = la + lb - 1;
    let mut out = vec![0u64; out_len];
    for i in 0..z0.len() {
        if i < out_len {
            out[i] = add_mod(out[i], z0[i]);
        }
    }
    for i in 0..z1.len() {
        let t = i + m;
        if t < out_len {
            out[t] = add_mod(out[t], z1[i]);
        }
    }
    for i in 0..z2.len() {
        let t = i + 2 * m;
        if t < out_len {
            out[t] = add_mod(out[t], z2[i]);
        }
    }
    out
}

fn split_eo(a: &[u64]) -> (Vec<u64>, Vec<u64>) {
    let mut e = Vec::with_capacity((a.len() + 1) / 2);
    let mut o = Vec::with_capacity(a.len() / 2);
    for (i, &x) in a.iter().enumerate() {
        if i % 2 == 0 {
            e.push(x);
        } else {
            o.push(x);
        }
    }
    (e, o)
}

fn add_shifted(dst: &mut Vec<u64>, src: &[u64], sh: usize) {
    let need = src.len() + sh;
    if dst.len() < need {
        dst.resize(need, 0);
    }
    for i in 0..src.len() {
        dst[i + sh] = add_mod(dst[i + sh], src[i]);
    }
}

fn convolve_and_decimate(a: &[u64], b: &[u64], bit: usize, out: &mut Vec<u64>) {
    let la = a.len();
    let lb = b.len();
    if la == 0 || lb == 0 {
        out.clear();
        return;
    }
    let out_full = la + lb - 1;
    let new_len = if out_full > bit {
        (out_full - bit + 1) / 2
    } else {
        0
    };
    if new_len == 0 {
        out.clear();
        return;
    }

    // Small: original stride-2 schoolbook (no allocs besides `out`).
    if la.min(lb) <= KARATSUBA_CUTOFF {
        let mut conv = vec![0u128; new_len];
        unsafe {
            let ap = a.as_ptr();
            let bp = b.as_ptr();
            let cp = conv.as_mut_ptr();
            for j in 0..la {
                let aj = *ap.add(j);
                let k0 = (bit ^ j) & 1;
                if k0 >= lb {
                    continue;
                }
                let mut dst = cp.add((j + k0 - bit) >> 1);
                let mut k = k0;
                while k + 6 < lb {
                    *dst += aj.wrapping_mul(*bp.add(k)) as u128;
                    *dst.add(1) += aj.wrapping_mul(*bp.add(k + 2)) as u128;
                    *dst.add(2) += aj.wrapping_mul(*bp.add(k + 4)) as u128;
                    *dst.add(3) += aj.wrapping_mul(*bp.add(k + 6)) as u128;
                    dst = dst.add(4);
                    k += 8;
                }
                while k < lb {
                    *dst += aj.wrapping_mul(*bp.add(k)) as u128;
                    dst = dst.add(1);
                    k += 2;
                }
            }
        }
        out.clear();
        let m128 = MOD as u128;
        out.extend(conv.iter().map(|&x| (x % m128) as u64));
        return;
    }

    let (ae, ao) = split_eo(a);
    let (be, bo) = split_eo(b);
    let mut acc = if bit == 0 {
        let p0 = karatsuba(&ae, &be);
        let p1 = karatsuba(&ao, &bo);
        let mut acc = p0;
        add_shifted(&mut acc, &p1, 1);
        acc
    } else {
        let p0 = karatsuba(&ae, &bo);
        let p1 = karatsuba(&ao, &be);
        let mut acc = p0;
        add_shifted(&mut acc, &p1, 0);
        acc
    };
    if acc.len() > new_len {
        acc.truncate(new_len);
    }
    *out = acc;
}

fn main() {
    let mut n = BigUint::from_u64(7);
    for _ in 1..777 {
        n.mul_assign_u64(7);
    }

    n.shr1();
    let m = n;

    let num_bits = m.bit_length();
    let mut bits = vec![0u8; num_bits];
    for k in 0..num_bits {
        bits[k] = m.bit(k) as u8;
    }

    let mut row = vec![1u64, 2, 1];
    let mut dp: Vec<u64> = vec![1];
    let mut next: Vec<u64> = Vec::with_capacity(4096);

    for k in 0..num_bits {
        if k > 0 {
            pascal_next(&mut row);
        }
        convolve_and_decimate(&dp, &row, bits[k] as usize, &mut next);
        std::mem::swap(&mut dp, &mut next);
    }

    let answer = if dp.is_empty() { 0 } else { dp[0] % MOD };
    println!("{}", answer);
}
