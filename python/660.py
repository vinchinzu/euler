"""Project Euler Problem 660: Pandigital Triangles.

Find the number of triangles with a 120° angle and integer sides that are
pandigital in some base A ≤ n ≤ B.

We iterate over all 120° triangles using the parameterization from p143, and
check if the sides are pandigital. For each base, the maximum side length is
(base)^(base/3) + (base)^(base/3 - 1). In order for three sides to be
pandigital, their sum must be congruent to 0+1+2+...+(base-1) in mod (base-1).
"""

from __future__ import annotations

from dataclasses import dataclass
from math import gcd

from sympy import isqrt


@dataclass(frozen=True)
class Triangle:
    """Triangle with sides a, b, c."""

    a: int
    b: int
    c: int


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def ceil_div(a: int, b: int) -> int:
    """Ceiling division."""
    return (a + b - 1) // b


def longest_side(m: int, n: int, k: int) -> int:
    """Compute longest side of 120° triangle."""
    return k * (sq(m) - m * n + sq(n))


def imod(n: int, mod: int) -> int:
    """Integer modulo (non-negative result)."""
    return ((n % mod) + mod) % mod


def is_pandigital_triangle(a: int, b: int, c: int, base: int) -> bool:
    """Check if triangle sides are pandigital in given base."""
    used = [False] * base
    count = 0
    for side in [a, b, c]:
        temp = side
        while temp > 0:
            d = imod(temp, base)
            if used[d]:
                return False
            used[d] = True
            temp //= base
            count += 1
    return count == base


def solve() -> int:
    """Solve Problem 660."""
    A = 9
    B = 18

    triangles: set[Triangle] = set()
    for base in range(A, B + 1):
        limit = pow(base, ceil_div(base, 3)) + pow(
            base, ceil_div(base, 3) - 1
        )
        expected_perimeter_mod = tr(base - 1) % (base - 1)

        n = 1
        while longest_side(n, n, 1) <= limit:
            m = n + 1
            while m < 2 * n and longest_side(m, n, 1) <= limit:
                if (m + n) % 3 != 0 and gcd(m, n) == 1:
                    k = 1
                    while longest_side(m, n, k) <= limit:
                        if (
                            k * m * (m + n) % (base - 1)
                            != expected_perimeter_mod
                        ):
                            k += 1
                            continue

                        a = k * (sq(m) - sq(n))
                        b = k * m * (2 * n - m)
                        c = k * (sq(m) - m * n + sq(n))

                        if is_pandigital_triangle(a, b, c, base):
                            triangles.add(Triangle(a, b, c))
                        k += 1
                m += 1
            n += 1

    ans = 0
    for triangle in triangles:
        ans += triangle.c
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
