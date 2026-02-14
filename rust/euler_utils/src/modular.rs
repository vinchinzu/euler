use std::ops::{Add, Sub, Mul};

/// Modular exponentiation: base^exp mod modulus.
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exp >>= 1;
        base = mod_mul(base, base, modulus);
    }
    result
}

/// Modular multiplication using u128 to avoid overflow.
#[inline]
pub fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// Modular inverse using extended Euclidean algorithm.
pub fn mod_inv(a: u64, m: u64) -> Option<u64> {
    let (mut old_r, mut r) = (a as i64, m as i64);
    let (mut old_s, mut s) = (1i64, 0i64);

    while r != 0 {
        let q = old_r / r;
        let tmp = r;
        r = old_r - q * r;
        old_r = tmp;
        let tmp = s;
        s = old_s - q * s;
        old_s = tmp;
    }

    if old_r != 1 {
        return None; // No inverse exists
    }

    Some(((old_s % m as i64 + m as i64) % m as i64) as u64)
}

/// A modular integer with compile-time modulus via const generic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModInt<const M: u64> {
    pub val: u64,
}

impl<const M: u64> ModInt<M> {
    pub fn new(v: u64) -> Self {
        ModInt { val: v % M }
    }

    pub fn from_i64(v: i64) -> Self {
        ModInt {
            val: ((v % M as i64 + M as i64) % M as i64) as u64,
        }
    }

    pub fn pow(self, exp: u64) -> Self {
        ModInt {
            val: mod_pow(self.val, exp, M),
        }
    }

    pub fn inv(self) -> Option<Self> {
        mod_inv(self.val, M).map(|v| ModInt { val: v })
    }
}

impl<const M: u64> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut v = self.val + rhs.val;
        if v >= M {
            v -= M;
        }
        ModInt { val: v }
    }
}

impl<const M: u64> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let v = if self.val >= rhs.val {
            self.val - rhs.val
        } else {
            M - rhs.val + self.val
        };
        ModInt { val: v }
    }
}

impl<const M: u64> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        ModInt {
            val: mod_mul(self.val, rhs.val, M),
        }
    }
}

impl<const M: u64> std::fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 7, 13), 2187 % 13);
    }

    #[test]
    fn test_mod_inv() {
        assert_eq!(mod_inv(3, 7), Some(5)); // 3*5 = 15 ≡ 1 (mod 7)
        assert_eq!(mod_inv(2, 4), None); // gcd(2,4) != 1
    }

    #[test]
    fn test_modint_ops() {
        type M = ModInt<1000000007>;
        let a = M::new(500000004);
        let b = M::new(500000004);
        let c = a + b;
        assert_eq!(c.val, 1); // 500000004 + 500000004 = 1000000008 ≡ 1
    }

    #[test]
    fn test_modint_pow() {
        type M = ModInt<1000000007>;
        let a = M::new(2);
        assert_eq!(a.pow(10).val, 1024);
    }
}
