"""Project Euler Problem 141.

Sum all progressive perfect squares below 10^12.
"""

import math
from typing import Set

LIMIT = 1_000_000_000_000


def is_square(value: int) -> bool:
    """Check if value is a perfect square."""
    root = int(math.sqrt(value))
    return root * root == value


def main() -> int:
    """Main function."""
    progressive_squares: Set[int] = set()
    max_q = int(math.sqrt(math.sqrt(LIMIT)))  # q^4 < LIMIT => q < LIMIT^(1/4)

    for q in range(1, max_q + 1):
        max_p = int(((LIMIT / q) ** (1.0 / 3)))
        if max_p <= q:
            continue

        for p in range(q + 1, max_p + 1):
            from math import gcd
            if gcd(p, q) != 1:
                continue

            coeff = p ** 3 * q
            linear = q ** 2

            discriminant = linear * linear + 4 * coeff * (LIMIT - 1)
            max_a = int((-linear + math.sqrt(discriminant)) / (2 * coeff))
            if max_a < 1:
                continue

            for a in range(1, max_a + 1):
                n = coeff * a * a + linear * a
                if n >= LIMIT:
                    break
                if is_square(n):
                    progressive_squares.add(n)

    return sum(progressive_squares)


if __name__ == "__main__":
    print(main())
