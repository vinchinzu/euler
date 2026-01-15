"""Project Euler Problem 760: Sum over Bitwise Operators.

Find G(N) = Σ_{n=0}^N Σ_{k=0}^n g(k,n-k), where g(m,n) is the sum of
the bitwise XOR, OR, and AND of m and n.

We split the sum into (1) k and n-k are both even, (2) both odd, and
(3) different parity. For (1), the least significant bit is always zero,
so g(k, n-k) = 2g(k/2, (n-k)/2). For (2), g(k, n-k) = 2g(k/2, (n-k)/2)
+ 2, because the OR and AND generate the least significant bit. In G(N),
there are tr(⌊n/2⌋) of these 2s. For (3), we have the same recurrence,
this time because of XOR and OR, and this time there are tr(⌈n/2⌉) 2s.

This gives the recurrence:
G(n) = 2G(⌊n/2⌋) + 2G(⌊n/2⌋-1) + 2tr(⌊n/2⌋)
     + 2(2G(⌈n/2⌉-1) + 2tr(⌈n/2⌉)).
"""

from __future__ import annotations

from functools import lru_cache


def tr(n: int, mod: int) -> int:
    """Triangular number modulo mod."""
    return (n * (n + 1) // 2) % mod


def solve() -> int:
    """Solve Problem 760."""
    N = 10**18
    M = 10**9 + 7

    @lru_cache(maxsize=None)
    def G(n: int) -> int:
        """Compute G(n) modulo M."""
        if n <= 0:
            return 0

        return (
            2 * G(n // 2)
            + 2 * G(n // 2 - 1)
            + 2 * tr(n // 2, M)
            + 2 * (2 * G((n - 1) // 2) + 2 * tr((n + 1) // 2, M))
        ) % M

    return G(N)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
