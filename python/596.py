"""Project Euler Problem 596: Number of integer quadruples.

Find the number of integer quadruples x,y,z,t such that x² + y² + z² + t² ≤ N.

By Jacobi's Four Square Theorem, the number of ways to represent n as the
sum of four squares is f(n) = 8σ(n) if n is not divisible by 4, and
8σ(n) - 32σ(n/4) otherwise. The number of quadruples is 1 (the origin)
plus the sum of f(n) from 1 to N², which is 1 + 8σ₂(N²) - 32σ₂(N²/4).
"""

from __future__ import annotations

from math import isqrt


def sigma2(n: int, mod: int) -> int:
    """Compute sum of divisors squared modulo mod."""
    sigma2_val = 0
    for k in range(1, isqrt(n) + 1):
        sigma2_val = (sigma2_val + (n // k) * k) % mod

    for t in range(1, isqrt(n)):
        low = n // (t + 1) + 1
        high = n // t
        count = high - low + 1
        sigma2_val = (sigma2_val + (low + high) * count // 2 * t) % mod

    return sigma2_val % mod


def solve() -> int:
    """Solve Problem 596."""
    N = 10**8
    M = 10**9 + 7

    ans = (1 + 8 * sigma2(N * N, M) - 32 * sigma2(N * N // 4, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
