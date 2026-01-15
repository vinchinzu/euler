"""Project Euler Problem 251: Cardano Triplets.

Find the number of Cardano triplets (a, b, c) that satisfy
³√(a + b√c) + ³√(a - b√c) = 1 and a+b+c ≤ N.
"""

from __future__ import annotations

from math import gcd


def ext_gcd(a: int, b: int) -> tuple[int, int, int]:
    """Extended Euclidean algorithm."""
    if b == 0:
        return (a, 1, 0)
    g, x1, y1 = ext_gcd(b, a % b)
    return (g, y1, x1 - (a // b) * y1)


def lin_comb(a: int, b: int, c: int) -> tuple[int, int]:
    """Find (x, y) such that ax + by = c."""
    g, x1, y1 = ext_gcd(a, b)
    if c % g != 0:
        return (0, 0)
    return (x1 * (c // g), y1 * (c // g))


def mod_inverse(n: int, mod: int) -> int:
    """Compute modular inverse of n mod mod."""
    g, x, _ = ext_gcd(n, mod)
    if g != 1:
        return 0
    return (x % mod + mod) % mod


def mod(n: int, m: int) -> int:
    """Modular arithmetic."""
    return (n % m + m) % m


def sq(n: int) -> int:
    """Square."""
    return n * n


def cb(n: int) -> int:
    """Cube."""
    return n * n * n


def solve() -> int:
    """Solve Problem 251."""
    N = 110_000_000
    ans = 0

    for r in range(1, int((N * 8 / 3) ** 0.5) + 1, 2):
        min_t = (5 * mod_inverse(sq(r), 8)) % 8
        if min_t == 0:
            min_t = 8
        for s in range(1, int((N / min_t - 3 * sq(r) / 8) ** 0.5) + 1):
            if gcd(r, s) == 1:
                sol_x, sol_y = lin_comb(8 * s, -sq(r), 3)
                g = mod(sol_x - 1, sq(r)) + 1
                t = mod(sol_y - 1, 8 * s) + 1
                start = 3 * g * s - 1 + g * r + sq(s) * t
                if start <= N:
                    increment = (3 * s + r) * sq(r) + 8 * cb(s)
                    ans += (N - start) // increment + 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
