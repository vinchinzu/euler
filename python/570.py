"""Project Euler Problem 570: Snowflakes.

Start with an equilateral triangle and repeatedly overlay an upside-down triangle
for each of the smallest equilateral triangles. Let A(n) be the number of
triangles that are one layer thick, and B(n) be the number of triangles that
are two layers thick. Find Σ_{n=3}^N GCD(A(n), B(n)).

The number of each type of triangle clearly obeys a linear recurrence, so we
can work that A(n) = 3*4^{n-1} - 2*3^{n-1} and B(n) = (18n-138)*4^{n-2} -
(4n+26)*3^{n-1}. Then taking linear combinations gives GCD(2*4^{n-2} -
3^{n-2}, 7n-3). This can be evaluated quickly (mod 7n-3).
"""

from __future__ import annotations

from math import gcd


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Compute base^exp mod mod."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp //= 2
    return result


def solve() -> int:
    """Solve Problem 570."""
    N = 10**7

    ans = 0
    for n in range(3, N + 1):
        mod = 7 * n + 3
        term = (2 * pow_mod(4, n - 2, mod) - pow_mod(3, n - 2, mod)) % mod
        ans += 6 * abs(gcd(term, mod))

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
