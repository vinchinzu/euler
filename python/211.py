"""Project Euler Problem 211: Divisor Square Sum.

Find the sum of all positive integers k<N such that the sum of the squares
of all divisors of k is a perfect square.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def build_spf(limit: int) -> List[int]:
    """Build smallest prime factor array up to limit."""
    spf = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def sum_divisor_powers(limit: int, exp: int) -> List[int]:
    """Compute sum of divisor powers for each number up to limit."""
    spf = build_spf(limit)
    result = [0] * (limit + 1)
    result[1] = 1

    for i in range(2, limit + 1):
        n = i
        mult = 1
        while n % spf[i] == 0:
            n //= spf[i]
            mult = mult * (spf[i] ** exp) + 1
        result[i] = result[n] * mult

    return result


def is_sq(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def solve() -> int:
    """Solve Problem 211."""
    N = 64_000_000

    sum_divisor_squares = sum_divisor_powers(N, 2)
    ans = 0

    for k in range(1, N):
        if is_sq(sum_divisor_squares[k]):
            ans += k

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
