"""Project Euler Problem 454: Solutions to 1/x + 1/y = 1/n.

Find the number of solutions to 1/x + 1/y = 1/n for positive integers n and
x < y ≤ N.
"""

from __future__ import annotations

from math import isqrt


def pre_mobius(limit: int) -> list[int]:
    """Precompute Möbius function."""
    mu = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]
    return mu


def sq(n: int) -> int:
    """Square."""
    return n * n


def cb(n: int) -> int:
    """Cube."""
    return n * n * n


def solve() -> int:
    """Solve Problem 454."""
    N = 10**12
    L = isqrt(N)
    mobius = pre_mobius(L)
    ans = 0

    for g_val in range(1, L + 1):
        n = N // sq(g_val)
        for y in range(2, isqrt(n) + 1):
            if cb(y) <= n:
                for x in range(1, y):
                    ans += mobius[g_val] * n // y // (x + y)
            else:
                q = max(n // y // (2 * y - 1), 1)
                while True:
                    upper = min(n // y // q, 2 * y - 1)
                    lower = max(n // y // (q + 1), y)
                    ans += mobius[g_val] * (upper - lower) * q
                    if lower == y:
                        break
                    q += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
