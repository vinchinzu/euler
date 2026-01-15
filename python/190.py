"""Project Euler Problem 190: Maximising a weighted product."""

from fractions import Fraction

MIN_M = 2
MAX_M = 15


def product_pm_rational(m: int) -> Fraction:
    """Calculate P_m exactly as a Rational number."""
    denom = m + 1
    r = Fraction(1, 1)
    for k in range(1, m + 1):
        r *= Fraction(2 * k, denom) ** k
    return r


def main() -> int:
    """Main function."""
    total = 0
    for m in range(MIN_M, MAX_M + 1):
        r = product_pm_rational(m)
        total += r.numerator // r.denominator
    return total


if __name__ == "__main__":
    print(main())
