"""Project Euler Problem 139: Pythagorean Tiling.

We are looking for Pythagorean triangles (a,b,c) such that their perimeter
a+b+c is less than 100,000,000.
Additionally, these triangles must satisfy the condition that c is divisible
by |a-b|. This relates to tiling a square hole.
"""

import math
from math import gcd

PERIMETER_LIMIT = 100_000_000


def main() -> int:
    """Main function."""
    total_tiling_triangles = 0

    # Determine m_limit for the loop:
    # P0 = 2m(m+k). Smallest k is 1. So P0_min approx 2m(m+1) approx 2m^2.
    # 2m^2 < perimeter_limit
    # m^2 < perimeter_limit / 2
    # m < sqrt(perimeter_limit / 2)
    m_limit = int(math.sqrt(PERIMETER_LIMIT / 2))

    for m in range(2, m_limit + 1):
        # k runs from 1 up to m-1
        for k in range(1, m):
            # Condition for primitive triples:
            # 1. m and k must have opposite parity. (m-k) must be odd.
            if (m - k) % 2 == 0:  # same parity
                continue

            # 2. m and k must be coprime.
            if gcd(m, k) != 1:
                continue

            # Generate primitive triple
            a0 = m * m - k * k
            b0 = 2 * m * k
            c0 = m * m + k * k
            p0 = a0 + b0 + c0

            # Check if primitive satisfies condition
            diff = abs(a0 - b0)
            if diff > 0 and c0 % diff == 0:
                # Count all multiples
                d = 1
                while d * p0 < PERIMETER_LIMIT:
                    total_tiling_triangles += 1
                    d += 1

    return total_tiling_triangles


if __name__ == "__main__":
    print(main())
