"""Project Euler Problem 496: Incenter and circumcircle.

Given a triangle ABC with incenter I, the other intersection of AI and the
circumcircle of ABC is D. Find the sum of BC for all integer triangles where
AC = DI and BC ≤ N.
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


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 496."""
    N = 10**9
    mobius = pre_mobius(isqrt(N))
    ans = 0

    for g in range(1, isqrt(N) + 1):
        n = N // sq(g)
        L = int((n / 2) ** (1 / 3)) + 1
        res = 0

        for x in range(1, L):
            for y in range(x + 1, min(2 * x, n // x + 1)):
                if x * y <= n:
                    res += tr(n // x // y) * x * y

        for x in range(L, isqrt(n) + 1):
            max_y = min(2 * x - 1, n // x)
            q = n // x // max_y
            while sq(x) * q <= n:
                upper = min(max_y, n // x // q)
                lower = max(x, n // x // (q + 1))
                res += tr(q) * x * (tr(upper) - tr(lower))
                q += 1

        ans += mobius[g] * sq(g) * res

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
