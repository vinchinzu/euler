"""Project Euler Problem 647: Linear Transformations of Polygonal Numbers.

Find the sum, over all odd k, of all A + B for positive integer pairs (A, B)
such that A(X_n) + B is always an k-gonal number.

The definition of a k-gonal number is given by Y_n = (1/2) n ( n(k-2) + 4 - k).
Solving yields B = (k-4)²(A-1) / 8(k-2). We iterate over all odd perfect
squares A, and find all values of d = k-2 that divide (√A) - 1.
"""

from __future__ import annotations

from math import isqrt

from sympy import divisors, primerange


def solve() -> int:
    """Solve Problem 647."""
    N = 10**12
    L = isqrt(N)

    primes = list(primerange(2, L + 1))
    ans = 0

    for sqrt_a in range(1, L + 1, 2):  # Odd sqrt_a
        A = sqrt_a * sqrt_a
        d_max = (sqrt_a - 1) // 2
        all_divs = divisors(d_max)

        for d in all_divs:
            if d % 2 == 1:  # d must be odd
                k = d + 2
                B = ((A - 1) // (8 * d)) * ((d - 2) ** 2)
                if 1 <= B <= N:
                    ans += A + B

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
