"""Project Euler Problem 132.

Sum the first 40 prime factors of the repunit R(10^9).
"""

from sympy import primerange

EXPONENT = 1_000_000_000
TARGET_COUNT = 40


def pow_mod(base: int, exponent: int, modulus: int) -> int:
    """Modular exponentiation."""
    result = 1 % modulus
    base %= modulus
    while exponent > 0:
        if exponent % 2 == 1:
            result = (result * base) % modulus
        base = (base * base) % modulus
        exponent >>= 1
    return result


def divides_repunit(prime_val: int, exponent: int) -> bool:
    """Check if prime divides repunit."""
    if prime_val == 2 or prime_val == 5:
        return False
    return pow_mod(10, exponent, 9 * prime_val) == 1


def main() -> int:
    """Main function."""
    factors = []
    for p in primerange(2, 10**9):
        if p == 2 or p == 5:
            continue
        if divides_repunit(p, EXPONENT):
            factors.append(p)
            if len(factors) == TARGET_COUNT:
                break
    return sum(factors)


if __name__ == "__main__":
    print(main())
