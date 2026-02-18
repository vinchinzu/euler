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

/// Convolve two arrays mod MOD, then decimate: return elements at positions
/// bit, bit+2, bit+4, ...
fn convolve_and_decimate(a: &[u64], b: &[u64], bit: u64) -> Vec<u64> {
    let la = a.len();
    let lb = b.len();
    let out_len = la + lb - 1;

    // Direct convolution into decimated output.
    // We need c[bit + 2*i] for i = 0, 1, ...
    let bit = bit as usize;
    let new_len = if out_len > bit { (out_len - bit + 1) / 2 } else { 0 };

    let mut res = vec![0u64; new_len];

    // For each target index t = bit + 2*i, c[t] = sum_{j} a[j] * b[t-j]
    // where 0 <= j < la and 0 <= t-j < lb.
    // Equivalently, j ranges from max(0, t-lb+1) to min(la-1, t).
    //
    // Strategy: iterate over a and b, and for each pair (j, k) where j+k = t,
    // accumulate into res[(t - bit) / 2] if (t - bit) is even and t >= bit.
    //
    // More efficient: iterate over all pairs, check if their sum has correct parity.
    // Or: for each j in a, for each target i where bit+2*i - j is a valid b index.

    // Actually, let's just compute the full convolution first if sizes are moderate,
    // then decimate. The arrays shouldn't exceed ~2200 elements.

    // Use i128 accumulation for the convolution to avoid overflow.
    // Max value of c[t] before mod: la * (MOD-1)^2 ~ 2200 * 10^18 ~ 2.2*10^21
    // This fits in u128 (max ~3.4*10^38).
    let mut conv = vec![0u128; out_len];

    for (j, &aj) in a.iter().enumerate() {
        if aj == 0 {
            continue;
        }
        let aj128 = aj as u128;
        for (k, &bk) in b.iter().enumerate() {
            // SAFETY: j+k < out_len by construction
            conv[j + k] += aj128 * bk as u128;
        }
    }

    for i in 0..new_len {
        let idx = bit + 2 * i;
        res[i] = (conv[idx] % MOD as u128) as u64;
    }

    res
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

    let num_bits = m.bit_length(); // L: highest relevant k is L-1

    // Precompute factorials mod MOD up to max_m = L+2
    let max_m = num_bits + 2;
    let mut fact = vec![1u64; max_m + 1];
    for i in 1..=max_m {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }
    let mut inv_fact = vec![1u64; max_m + 1];
    // inv_fact[max_m] = fact[max_m]^(MOD-2) mod MOD
    inv_fact[max_m] = {
        let mut base = fact[max_m] as u128;
        let mut exp = MOD - 2;
        let m128 = MOD as u128;
        let mut result = 1u128;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % m128;
            }
            base = base * base % m128;
            exp >>= 1;
        }
        result as u64
    };
    for i in (1..=max_m).rev() {
        inv_fact[i - 1] = inv_fact[i] * (i as u64) % MOD;
    }

    // DP on binary digits of m
    let mut dp: Vec<u64> = vec![1]; // dp[carry] at current bit position

    for k in 0..num_bits {
        let bit_k = m.bit(k);
        let top = k + 2; // exponent: (1 + x^{2^k})^{k+2}

        // Binomial row: C(top, j) for j = 0..=top
        let mut row = vec![0u64; top + 1];
        let ft = fact[top] as u128;
        let m128 = MOD as u128;
        for j in 0..=top {
            row[j] = (ft * inv_fact[j] as u128 % m128 * inv_fact[top - j] as u128 % m128) as u64;
        }

        dp = convolve_and_decimate(&dp, &row, bit_k);
    }

    // After processing all bits, the answer is dp[0] (carry must be 0).
    let answer = if dp.is_empty() { 0 } else { dp[0] % MOD };
    println!("{}", answer);
}
