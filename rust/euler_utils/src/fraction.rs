//! Fraction/rational arithmetic re-exports from the `num` crate.
//! Replaces hand-rolled fraction code in 65+ C files.

pub use num::rational::{Ratio, Rational64};
pub use num::BigRational;

/// Create a Rational64 from numerator and denominator.
pub fn frac(numer: i64, denom: i64) -> Rational64 {
    Rational64::new(numer, denom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frac_basic() {
        let a = frac(1, 3);
        let b = frac(1, 6);
        assert_eq!(a + b, frac(1, 2));
    }

    #[test]
    fn test_frac_reduce() {
        let f = frac(4, 6);
        assert_eq!(*f.numer(), 2);
        assert_eq!(*f.denom(), 3);
    }

    #[test]
    fn test_frac_mul() {
        assert_eq!(frac(2, 3) * frac(3, 4), frac(1, 2));
    }
}
