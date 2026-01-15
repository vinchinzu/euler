"""Project Euler Problem 785: Binary Quadratic Diophantine.

Find the sum of x+y+z for all 1≤x≤y≤z≤N such that
15(x²+y²+z²)=34(xy+yz+xz) and GCD(x,y,z)=1.

Using techniques for solving homogeneous quadratic Diophantine equations, the
solutions can be parameterized as x=3(m+n)(n-m), y=3m(17m+2n),
z=(14m+5n)(4m+n), where x≢y (mod 19) and GCD(m,n)=1. Their GCD is 9 if
m+n≡0 (mod 3) and 1 otherwise. So we can iterate over all m,n with m+n≢0 such
that z≤N, and also iterate over all m,n with m+n≡0 such that z≤9N.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def gcds(limit: int) -> List[List[int]]:
    """Precompute GCD table."""
    result = [[0] * limit for _ in range(limit)]
    for i in range(limit):
        for j in range(limit):
            a, b = i, j
            while b:
                a, b = b, a % b
            result[i][j] = a
    return result


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def solve() -> int:
    """Solve Problem 785."""
    N = 10**9
    sqrt_N = isqrt(9 * N // 95)
    gcd_table = gcds(sqrt_N + 1)

    ans = 0

    # Case 1: m+n ≢ 0 (mod 3)
    m_max = int((N / 95) ** 0.5)
    for m in range(1, m_max + 1):
        if 95 * sq(m) > N:
            break
        n = m + 1
        while True:
            z = (14 * m + 5 * n) * (4 * m + n)
            if z > N:
                break
            if m % 19 != n % 19 and gcd_table[m][n % m] == 1 and (m + n) % 3 != 0:
                ans += 8 * (13 * sq(m) + 5 * m * n + sq(n))
            n += 1

    # Case 2: m+n ≡ 0 (mod 3)
    m_max = int((9 * N / 95) ** 0.5)
    for m in range(1, m_max + 1):
        if 95 * sq(m) > 9 * N:
            break
        n_start = m + (m % 3)
        n = n_start
        while True:
            z = (14 * m + 5 * n) * (4 * m + n)
            if z > 9 * N:
                break
            if m % 19 != n % 19 and gcd_table[m][n % m] == 1:
                ans += 8 * (13 * sq(m) + 5 * m * n + sq(n)) // 9
            n += 3

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
