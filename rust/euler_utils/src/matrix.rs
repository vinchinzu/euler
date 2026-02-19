/// Fixed-size square matrix for modular matrix exponentiation.
///
/// Stack-allocated, suitable for dimensions up to ~100.
/// Uses u128 accumulation with deferred modular reduction.
///
/// # Example
/// ```
/// use euler_utils::ModMatrix;
///
/// // Fibonacci via matrix exponentiation: [[1,1],[1,0]]^n
/// let mut fib = ModMatrix::<2>::identity(1_000_000_007);
/// fib.data = [[1, 1], [1, 0]];
/// let result = fib.pow(10);
/// assert_eq!(result.data[0][1], 55); // F(10) = 55
/// ```
#[derive(Clone, Debug)]
pub struct ModMatrix<const N: usize> {
    pub data: [[u64; N]; N],
    pub modulus: u64,
}

impl<const N: usize> ModMatrix<N> {
    /// Zero matrix with given modulus.
    #[inline]
    pub fn zero(modulus: u64) -> Self {
        ModMatrix {
            data: [[0u64; N]; N],
            modulus,
        }
    }

    /// Identity matrix with given modulus.
    pub fn identity(modulus: u64) -> Self {
        let mut m = Self::zero(modulus);
        for i in 0..N {
            m.data[i][i] = 1;
        }
        m
    }

    /// Create from a 2D array, reducing each element mod modulus.
    pub fn from_data(data: [[u64; N]; N], modulus: u64) -> Self {
        let mut m = ModMatrix { data, modulus };
        for i in 0..N {
            for j in 0..N {
                m.data[i][j] %= modulus;
            }
        }
        m
    }

    /// Matrix multiplication mod self.modulus.
    ///
    /// Uses u128 accumulation with batched modular reduction.
    /// For modulus < 2^32, reduces every N iterations (near-optimal).
    /// For larger modulus, reduces every 4 iterations to prevent u128 overflow.
    pub fn mul(&self, other: &Self) -> Self {
        let m = self.modulus;
        let mut result = Self::zero(m);

        // Each product: up to (m-1)^2. Sum of k products: up to k*(m-1)^2.
        // u128 max ≈ 3.4×10^38. Safe if k*(m-1)^2 < 2^128.
        // m < 2^32 → (m-1)^2 < 2^64 → can accumulate 2^64 terms (unlimited).
        // m < 2^63 → (m-1)^2 < 2^126 → can accumulate ~4 terms.
        let batch = if m <= (1u64 << 32) { N } else { 4.min(N) };

        for i in 0..N {
            let mut acc = [0u128; N];
            for k in 0..N {
                let aik = self.data[i][k] as u128;
                for j in 0..N {
                    acc[j] += aik * other.data[k][j] as u128;
                }
                if (k + 1) % batch == 0 {
                    for j in 0..N {
                        acc[j] %= m as u128;
                    }
                }
            }
            for j in 0..N {
                result.data[i][j] = (acc[j] % m as u128) as u64;
            }
        }
        result
    }

    /// Matrix exponentiation: self^exp mod modulus.
    /// Uses binary exponentiation: O(N^3 log(exp)).
    pub fn pow(&self, mut exp: u64) -> Self {
        let mut result = Self::identity(self.modulus);
        let mut base = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exp >>= 1;
        }
        result
    }

    /// Multiply a column vector by this matrix: result = self * vec.
    /// Returns the resulting column vector.
    pub fn mul_vec(&self, vec: &[u64; N]) -> [u64; N] {
        let m = self.modulus;
        let mut result = [0u64; N];
        for i in 0..N {
            let mut acc = 0u128;
            for j in 0..N {
                acc += self.data[i][j] as u128 * vec[j] as u128;
            }
            result[i] = (acc % m as u128) as u64;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let id = ModMatrix::<3>::identity(1000);
        assert_eq!(id.data, [[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    }

    #[test]
    fn test_fibonacci() {
        // F(n) via [[1,1],[1,0]]^n, result at [0][1]
        let m = ModMatrix::<2>::from_data([[1, 1], [1, 0]], 1_000_000_007);
        assert_eq!(m.pow(10).data[0][1], 55);
        assert_eq!(m.pow(20).data[0][1], 6765);
        assert_eq!(m.pow(50).data[0][1], 12586269025 % 1_000_000_007);
    }

    #[test]
    fn test_mul_identity() {
        let m = ModMatrix::<3>::from_data([[1, 2, 3], [4, 5, 6], [7, 8, 9]], 101);
        let id = ModMatrix::<3>::identity(101);
        let result = m.mul(&id);
        assert_eq!(result.data, m.data);
    }

    #[test]
    fn test_pow_zero() {
        let m = ModMatrix::<2>::from_data([[3, 1], [0, 2]], 1000);
        let result = m.pow(0);
        assert_eq!(result.data, [[1, 0], [0, 1]]);
    }

    #[test]
    fn test_mul_vec() {
        let m = ModMatrix::<2>::from_data([[1, 1], [1, 0]], 1_000_000_007);
        let v = [1u64, 0];
        let result = m.mul_vec(&v);
        assert_eq!(result, [1, 1]); // [1,1;1,0] * [1,0]^T = [1,1]^T
    }

    #[test]
    fn test_large_modulus() {
        // Test with modulus > 2^32 to exercise the batch=4 path
        let m = ModMatrix::<2>::from_data(
            [[1_000_000_000_000, 1], [1, 0]],
            1_000_000_000_000_000_003,
        );
        let result = m.pow(2);
        // Manual: [[a,1],[1,0]]^2 = [[a^2+1, a],[a, 1]]
        let a = 1_000_000_000_000u128;
        let md = 1_000_000_000_000_000_003u128;
        assert_eq!(result.data[0][0], ((a * a + 1) % md) as u64);
        assert_eq!(result.data[0][1], a as u64);
    }
}
