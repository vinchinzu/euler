"""Project Euler Problem 176: Right-angled triangles that share a cathetus."""

from typing import List
import math

TARGET = 47_547
VALUE = 2 * TARGET + 1
ODD_PRIMES = [p for p in [3, 5, 7, 11, 13, 17, 19] if p > 2]


def prime_division(value: int) -> List[tuple[int, int]]:
    """Compute prime factorization."""
    factors: List[tuple[int, int]] = []
    d = 2
    while d * d <= value:
        if value % d == 0:
            count = 0
            while value % d == 0:
                value //= d
                count += 1
            factors.append((d, count))
        d += 1
    if value > 1:
        factors.append((value, 1))
    return factors


def divisors_of(value: int) -> List[int]:
    """Generate all divisors of value."""
    factors = prime_division(value)
    divisors = [1]
    for prime, exp in factors:
        current = []
        for e in range(1, exp + 1):
            factor = prime ** e
            for d in divisors:
                current.append(d * factor)
        divisors.extend(current)
    return sorted(set(divisors))


def minimal_m_for(f: int) -> int:
    """Compute the minimal odd component m associated with a given F."""
    factors = prime_division(f)
    exponents: List[int] = []
    for prime, multiplicity in factors:
        for _ in range(multiplicity):
            exponents.append((prime - 1) // 2)
    exponents.sort(reverse=True)
    m = 1
    for idx, exp in enumerate(exponents):
        m *= ODD_PRIMES[idx] ** exp
    return m


def count_triples(n: int) -> int:
    """Count triples for verification."""
    value = n
    a = 0
    while (value & 1) == 0:
        value >>= 1
        a += 1
    odd_part = value
    factors = prime_division(odd_part)
    factor_product = 1
    for _, exp in factors:
        factor_product *= 2 * exp + 1
    if a <= 1:
        return (factor_product - 1) // 2
    else:
        return ((2 * a - 1) * factor_product - 1) // 2


def main() -> int:
    """Main function."""
    best_n: int | None = None

    # Handle a = 0 (odd n) and a = 1 (exactly one factor of 2)
    for a in [0, 1]:
        f = VALUE
        m = minimal_m_for(f)
        n = (1 << a) * m
        if best_n is None or n < best_n:
            best_n = n

    for d in divisors_of(VALUE):
        if d < 3:
            continue
        if (d + 1) % 2 != 0:
            continue
        a = (d + 1) // 2
        if a < 2:
            continue
        f = VALUE // d
        m = minimal_m_for(f)
        n = (1 << a) * m
        if best_n is None or n < best_n:
            best_n = n

    if best_n is None:
        raise ValueError("No solution found")
    return best_n


if __name__ == "__main__":
    result = main()
    print(result)
