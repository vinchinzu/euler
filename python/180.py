"""Project Euler Problem 180: Rational zeros of a function of three variables."""

from typing import List, Set
from fractions import Fraction
import math

MAX_DEN = 35


def reduced_fractions(limit: int) -> List[Fraction]:
    """Generate all reduced fractions with denominator <= limit."""
    fracs: List[Fraction] = []
    for den in range(1, limit + 1):
        for num in range(1, den):
            if math.gcd(num, den) == 1:
                fracs.append(Fraction(num, den))
    return sorted(fracs)


def integer_sqrt(n: int) -> int:
    """Compute integer square root."""
    x = int(math.sqrt(n))
    while x * x < n:
        x += 1
    while x * x > n:
        x -= 1
    return x


def valid_fraction(r: Fraction, fraction_set: Set[Fraction]) -> bool:
    """Check whether rational satisfies bounds."""
    return r > 0 and r < 1 and r.denominator <= MAX_DEN and r in fraction_set


def sqrt_rational(r: Fraction) -> Fraction | None:
    """Return the square root of a rational when it exists."""
    if r < 0:
        return None
    num = r.numerator
    den = r.denominator
    sqrt_num = integer_sqrt(num)
    if sqrt_num * sqrt_num != num:
        return None
    sqrt_den = integer_sqrt(den)
    if sqrt_den * sqrt_den != den:
        return None
    return Fraction(sqrt_num, sqrt_den)


def main() -> int:
    """Main function."""
    FRACTIONS = reduced_fractions(MAX_DEN)
    FRACTION_SET = set(FRACTIONS)
    FRACTION_COUNT = len(FRACTIONS)

    # Collect distinct sums s(x, y, z)
    sums: Set[Fraction] = set()

    # Case 1: x + y = z
    for i in range(FRACTION_COUNT):
        for j in range(i, FRACTION_COUNT):
            x = FRACTIONS[i]
            y = FRACTIONS[j]
            z = x + y
            if not valid_fraction(z, FRACTION_SET):
                continue
            sums.add(x + y + z)

    # Case 2: x^2 + y^2 = z^2
    squares = [f * f for f in FRACTIONS]
    sum_map: dict[Fraction, List[tuple[int, int]]] = {}
    for i in range(FRACTION_COUNT):
        sx = squares[i]
        for j in range(i, FRACTION_COUNT):
            key = sx + squares[j]
            if key not in sum_map:
                sum_map[key] = []
            sum_map[key].append((i, j))

    for idx, z in enumerate(FRACTIONS):
        target = squares[idx]
        if target in sum_map:
            for i, j in sum_map[target]:
                x = FRACTIONS[i]
                y = FRACTIONS[j]
                sums.add(x + y + z)

    # Case 3: harmonic mean 1/x + 1/y = 1/z
    for i in range(FRACTION_COUNT):
        for j in range(i, FRACTION_COUNT):
            x = FRACTIONS[i]
            y = FRACTIONS[j]
            denom = x + y
            if denom == 0:
                continue
            z = (x * y) / denom
            if not valid_fraction(z, FRACTION_SET):
                continue
            sums.add(x + y + z)

    # Case 4: reciprocal squares 1/x^2 + 1/y^2 = 1/z^2
    for i in range(FRACTION_COUNT):
        sx = FRACTIONS[i] * FRACTIONS[i]
        for j in range(i, FRACTION_COUNT):
            y = FRACTIONS[j]
            denom = sx + y * y
            if denom == 0:
                continue
            z_squared = (sx * y * y) / denom
            z = sqrt_rational(z_squared)
            if z is None:
                continue
            if not valid_fraction(z, FRACTION_SET):
                continue
            sums.add(FRACTIONS[i] + y + z)

    # Sum all distinct values, reduce, and output u + v
    total = sum(sums)
    result = total.numerator + total.denominator
    return result


if __name__ == "__main__":
    print(main())
