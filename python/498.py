"""Project Euler Problem 498: Remainder of polynomial division.

Find the coefficient of x^D in the remainder when x^N is divided by (x-1)^K.
"""

from __future__ import annotations


def ncr_small(n: int, k: int, mod: int) -> int:
    """nCk modulo prime mod with small k."""
    if k < 0 or k > n:
        return 0
    k = min(k, n - k)
    if k == 0:
        return 1
    num = 1
    den = 1
    for i in range(1, k + 1):
        num = (num * (n - k + i)) % mod
        den = (den * i) % mod
    return num * pow(den, mod - 2, mod) % mod


def ncr_lucas(n: int, k: int, mod: int) -> int:
    """Lucas theorem for prime mod."""
    if k < 0 or k > n:
        return 0
    result = 1
    while n > 0 or k > 0:
        ni = n % mod
        ki = k % mod
        if ki > ni:
            return 0
        result = (result * ncr_small(ni, ki, mod)) % mod
        n //= mod
        k //= mod
    return result


def solve() -> int:
    """Solve Problem 498."""
    N = 10**13
    K = 10**12
    D = 10**4
    M = 999_999_937
    n1 = N - D - 1
    k1 = K - 1 - D
    coeff = ncr_lucas(N, D, M)
    coeff = (coeff * ncr_lucas(n1, k1, M)) % M
    return coeff


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
