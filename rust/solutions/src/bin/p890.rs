// Problem 890 - Binary Partitions
//
// p(n) = number of ways to write n as unordered sum of powers of 2.
// Compute p(7^777) mod 10^9+7.
//
// Algorithm:
//   p(2m) = p(2m+1) = S(m) where S(m) = sum_{i=0..m} p(i).
//   S(m) = [x^m] A(x) where A(x) = prod_{k>=0} (1 + x^{2^k})^{k+2}.
//   Extract [x^m] via carry-DP on binary digits of m, convolving with
//   binomial rows C(k+2, j) at each bit position k.

const MOD: u64 = 1_000_000_007;

/// Big integer as little-endian limbs of u64 (base 2^64).
/// Only positive values needed.
struct BigUint {
    limbs: Vec<u64>,
}

impl BigUint {
    fn from_u64(v: u64) -> Self {
        BigUint {
            limbs: if v == 0 { vec![0] } else { vec![v] },
        }
    }

    /// Multiply by a small u64 in place.
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

    /// Right shift by 1 bit (divide by 2).
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

    /// Get bit at position k (0-indexed from LSB).
    fn bit(&self, k: usize) -> u64 {
        let limb_idx = k / 64;
        let bit_idx = k % 64;
        if limb_idx >= self.limbs.len() {
            0
        } else {
            (self.limbs[limb_idx] >> bit_idx) & 1
        }
    }

    /// Number of bits (position of highest set bit + 1).
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

/// Advance Pascal row from C(n-1, *) to C(n, *) mod MOD.
#[inline]
fn pascal_next(row: &mut Vec<u64>) {
    row.push(1);
    let n = row.len() - 1;
    for j in (1..n).rev() {
        let s = row[j] + row[j - 1];
        row[j] = if s >= MOD { s - MOD } else { s };
    }
}

/// Direct-to-decimated convolution: only (j+k) ≡ bit (mod 2) terms,
/// written at i = (j+k-bit)/2. Products fit in u64 (both < MOD, MOD^2 < 2^64).
fn convolve_and_decimate(
    a: &[u64],
    b: &[u64],
    bit: usize,
    conv: &mut Vec<u128>,
    out: &mut Vec<u64>,
) {
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

    conv.clear();
    conv.resize(new_len, 0);

    // SAFETY: j < la, k < lb; i = (j+k-bit)/2 < new_len by construction
    // (max j+k-bit = la+lb-2-bit, (la+lb-2-bit)/2 = new_len-1).
    unsafe {
        let ap = a.as_ptr();
        let bp = b.as_ptr();
        let cp = conv.as_mut_ptr();
        for j in 0..la {
            let aj = *ap.add(j);
            if aj == 0 {
                continue;
            }
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
    out.reserve(new_len);
    for &x in conv.iter() {
        out.push((x % m128) as u64);
    }
}

fn main() {
    // Compute n = 7^777
    let mut n = BigUint::from_u64(7);
    for _ in 1..777 {
        n.mul_assign_u64(7);
    }

    // m = n / 2  (n is odd, so m = (n-1)/2, and p(n) = p(2m+1) = S(m))
    n.shr1();
    let m = n;

    let num_bits = m.bit_length();
    let mut bits = vec![0u8; num_bits];
    for k in 0..num_bits {
        bits[k] = m.bit(k) as u8;
    }

    // Incremental binomial row via Pascal: start at C(2,*) for k = 0.
    let mut row = vec![1u64, 2, 1];
    let mut dp: Vec<u64> = vec![1];
    let mut conv: Vec<u128> = Vec::with_capacity(4096);
    let mut next: Vec<u64> = Vec::with_capacity(4096);

    for k in 0..num_bits {
        if k > 0 {
            pascal_next(&mut row);
        }
        convolve_and_decimate(&dp, &row, bits[k] as usize, &mut conv, &mut next);
        std::mem::swap(&mut dp, &mut next);
    }

    let answer = if dp.is_empty() { 0 } else { dp[0] % MOD };
    println!("{}", answer);
}
