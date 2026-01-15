"""Project Euler Problem 283: Integer Sided Triangles.

Find the number of integer-sided triangles whose ratio of area to perimeter is
an integer up to N.

Such triangles must satisfy K = rP, where K is the area, P is the perimeter,
and r≤N. Using Heron's Formula and squaring both sides gives (s-a)(s-b)(s-c)
= 4r²s, where s is the semiperimeter. Note that s cannot be a half-integer,
because then the left-hand side would have a denominator of 8. Therefore, we
let integers x = s - a, y = s - b, and z = s - c, which gives xyz = 4r²(x+y+z)
=> (xy - 4r²)(xz - 4r²) = 4r²(4r² + x²).

Without loss of generality we let x≤y≤z, so the above inequality gives
4r²(4r² + x²) ≥ (x² - 4r²)² => x < 2√3 r. This means we can iterate over all
r≤N and x≤(2√3)N, and compute all divisors of the right-hand side. For each
divisor, we solve for y and z. If they are both integral, and they pass our
original assumption x≤y≤z, then we add the perimeter a + b + c = 2(x + y + z).

As optimizations, we compute the prime factors of the two right-hand side
factors 4r² and (4r² + x²), and build up a list of all divisors only up to
the square root of the product (which also ensures that y≤z). We use an array
with a end pointer as a further performance optimization.
"""

from __future__ import annotations

from math import isqrt, sqrt
from typing import List, Set

from sympy import factorint


def isq(n: int) -> int:
    """Square of n."""
    return n * n


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def fsq(n: int) -> int:
    """Floor square root."""
    return isqrt(n)


def build_spf(limit: int) -> List[int]:
    """Build smallest prime factor array."""
    spf = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def solve() -> int:
    """Solve Problem 283."""
    N = 1000
    L = int(2 * sqrt(3) * N)

    # Precompute smallest prime factor array
    max_val = 4 * N * N + L * L
    spf = build_spf(max_val)

    divisors: List[int] = [0] * 10000

    ans = 0

    for r in range(1, N + 1):
        factors_4r2: Set[int] = set(factorint(4 * isq(r)).keys())

        for x in range(1, L + 1):
            product = k = 4 * sq(r) * (4 * sq(r) + sq(x))
            divisors_size = 1
            divisors[0] = 1

            # Add divisors from factors of 4r²
            for d in factors_4r2:
                e = 0
                temp_k = k
                while temp_k % d == 0:
                    temp_k //= d
                    e += 1
                old_size = divisors_size
                for i in range(old_size - 1, -1, -1):
                    mult = d
                    for j in range(e):
                        if fsq(divisors[i] * mult) <= product:
                            divisors[divisors_size] = divisors[i] * mult
                            divisors_size += 1
                        mult *= d

            # Add divisors from factors of (4r² + x²) using SPF
            temp_k = k
            while temp_k > 1:
                d = spf[temp_k] if temp_k < len(spf) else temp_k
                e = 0
                while temp_k % d == 0:
                    temp_k //= d
                    e += 1
                old_size = divisors_size
                for i in range(old_size - 1, -1, -1):
                    mult = d
                    for j in range(e):
                        if fsq(divisors[i] * mult) <= product:
                            divisors[divisors_size] = divisors[i] * mult
                            divisors_size += 1
                        mult *= d

            # Check each divisor
            for i in range(divisors_size):
                xy = divisors[i] + 4 * sq(r)
                xz = product // divisors[i] + 4 * sq(r)
                if xy % x == 0 and xz % x == 0 and x * x <= xy:
                    ans += 2 * (x + xy // x + xz // x)

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
