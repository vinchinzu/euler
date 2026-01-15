"""Project Euler Problem 498: Remainder of polynomial division.

Find the coefficient of x^D in the remainder when x^N is divided by (x-1)^K.
"""

from __future__ import annotations


def nCr(n: int, k: int, mod: int) -> int:
    """Binomial coefficient modulo mod."""
    if k < 0 or k > n:
        return 0
    result = 1
    for i in range(min(k, n - k)):
        result = (result * (n - i)) % mod
        result = (result * pow(i + 1, mod - 2, mod)) % mod
    return result


def solve() -> int:
    """Solve Problem 498."""
    N = 10**13
    K = 10**12
    D = 10**4
    M = 999_999_937

    ans = nCr(N, D, M) * nCr(N - D - 1, K - 1 - D, M) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
