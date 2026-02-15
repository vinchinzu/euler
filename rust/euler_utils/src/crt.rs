use super::modular::extended_gcd;

/// Chinese Remainder Theorem: find x such that x ≡ r_i (mod m_i) for all i.
/// Returns Some((x, lcm)) where x is the smallest non-negative solution and lcm is the combined modulus.
/// Returns None if the system is inconsistent.
pub fn crt(remainders: &[i64], moduli: &[i64]) -> Option<(i64, i64)> {
    if remainders.is_empty() || remainders.len() != moduli.len() {
        return None;
    }

    let mut r = remainders[0];
    let mut m = moduli[0];

    for i in 1..remainders.len() {
        let r2 = remainders[i];
        let m2 = moduli[i];
        let (g, p, _) = extended_gcd(m, m2);

        if (r2 - r) % g != 0 {
            return None; // Inconsistent
        }

        let lcm = m / g * m2;
        let diff = ((r2 - r) / g % (m2 / g) * p % (m2 / g) + m2 / g) % (m2 / g);
        r = (r + m * diff) % lcm;
        if r < 0 { r += lcm; }
        m = lcm;
    }

    Some((r, m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt_basic() {
        // x ≡ 2 (mod 3), x ≡ 3 (mod 5) → x ≡ 8 (mod 15)
        let (x, m) = crt(&[2, 3], &[3, 5]).unwrap();
        assert_eq!(m, 15);
        assert_eq!(x % 3, 2);
        assert_eq!(x % 5, 3);
    }

    #[test]
    fn test_crt_three() {
        // x ≡ 1 (mod 2), x ≡ 2 (mod 3), x ≡ 3 (mod 5) → x ≡ 23 (mod 30)
        let (x, m) = crt(&[1, 2, 3], &[2, 3, 5]).unwrap();
        assert_eq!(m, 30);
        assert_eq!(x % 2, 1);
        assert_eq!(x % 3, 2);
        assert_eq!(x % 5, 3);
    }

    #[test]
    fn test_crt_inconsistent() {
        // x ≡ 0 (mod 2), x ≡ 1 (mod 2) → no solution
        assert!(crt(&[0, 1], &[2, 2]).is_none());
    }
}
