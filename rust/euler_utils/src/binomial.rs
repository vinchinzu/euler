use super::modular::mod_mul;

/// Precomputed binomial coefficients modulo a prime.
pub struct BinomialMod {
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
    modulus: u64,
}

impl BinomialMod {
    /// Create a new BinomialMod with precomputed factorials up to max_n.
    pub fn new(max_n: usize, modulus: u64) -> Self {
        let mut fact = vec![1u64; max_n + 1];
        for i in 1..=max_n {
            fact[i] = fact[i - 1] * (i as u64) % modulus;
        }
        let mut inv_fact = vec![1u64; max_n + 1];
        inv_fact[max_n] = super::mod_pow(fact[max_n], modulus - 2, modulus);
        for i in (1..=max_n).rev() {
            inv_fact[i - 1] = inv_fact[i] * (i as u64) % modulus;
        }
        BinomialMod { fact, inv_fact, modulus }
    }

    /// Compute C(n, r) mod p. Returns 0 if r > n.
    #[inline]
    pub fn choose(&self, n: usize, r: usize) -> u64 {
        if r > n { return 0; }
        mod_mul(mod_mul(self.fact[n], self.inv_fact[r], self.modulus), self.inv_fact[n - r], self.modulus)
    }

    /// Get n! mod p.
    #[inline]
    pub fn factorial(&self, n: usize) -> u64 {
        self.fact[n]
    }

    /// Get (n!)^{-1} mod p.
    #[inline]
    pub fn inv_factorial(&self, n: usize) -> u64 {
        self.inv_fact[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose() {
        let b = BinomialMod::new(100, 1_000_000_007);
        assert_eq!(b.choose(5, 2), 10);
        assert_eq!(b.choose(10, 3), 120);
        assert_eq!(b.choose(5, 0), 1);
        assert_eq!(b.choose(5, 5), 1);
        assert_eq!(b.choose(3, 5), 0);
    }

    #[test]
    fn test_factorial() {
        let b = BinomialMod::new(10, 1_000_000_007);
        assert_eq!(b.factorial(5), 120);
        assert_eq!(b.factorial(0), 1);
    }
}
