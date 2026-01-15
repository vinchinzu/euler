"""Project Euler Problem 401: Sum of squares of divisors.

Let sigma2(n) be the sum of the squares of the divisors of n. Find
sum_{k=1}^N sigma2(k).

We can write sum_{k=1}^N sigma2(k) = sum_{d=1}^N ⌊N/d⌋ d², because each
divisor d appears ⌊N/d⌋ times. We can compute the first ⌊√N⌋ terms directly,
but we compute the rest of the terms in ranges with the same ⌊N/d⌋. For each
distinct t=⌊N/d⌋, we sum the squares from d = ⌊N/(t+1)⌋ + 1 to ⌊N/t⌋.
"""

from __future__ import annotations

from math import isqrt


def sq(n: int, mod: int | None = None) -> int:
    """Return n squared, optionally modulo mod."""
    result = n * n
    return result % mod if mod else result


def sum_powers(n: int, exp: int, mod: int | None = None) -> int:
    """Return sum_{k=1}^n k^exp, optionally modulo mod."""
    if exp == 1:
        result = n * (n + 1) // 2
    elif exp == 2:
        result = n * (n + 1) * (2 * n + 1) // 6
    elif exp == 3:
        result = (n * (n + 1) // 2) ** 2
    else:
        result = sum(k**exp for k in range(1, n + 1))
    return result % mod if mod else result


def solve() -> int:
    """Solve Problem 401."""
    N = 10**15
    L = isqrt(N)
    M = 10**9

    ans = 0
    for d in range(1, N // L):
        ans = (ans + (N // d % M) * sq(d, M)) % M
    for t in range(1, L + 1):
        ans = (
            ans
            + t
            * (
                sum_powers(N // t, 2, M) - sum_powers(N // (t + 1), 2, M)
            )
        ) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
