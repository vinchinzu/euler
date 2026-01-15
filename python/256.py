"""Project Euler Problem 256: Tatami-Free Rooms.

Find the smallest s such that there are exactly N Tatami-free rooms with
area s, where a room is Tatami-free if it cannot be tiled with 1x2 Tatamis
such that no four Tatamis meet at a single corner.
"""

from __future__ import annotations

from math import isqrt


def ceil_div(a: int, b: int) -> int:
    """Ceiling division."""
    return (a + b - 1) // b


def solve() -> int:
    """Solve Problem 256."""
    N = 200
    L = 10**8

    # Precompute number of divisors for all numbers up to L
    num_divisors = [0] * (L + 1)
    for i in range(1, L + 1):
        for j in range(i, L + 1, i):
            num_divisors[j] += 1

    # Find smallest room
    for s in range(1, L + 1):
        if num_divisors[s] >= 2 * N:
            num_tatami_free = 0
            for d in range(1, isqrt(s) + 1):
                if s % d == 0:
                    a, b = d, s // d
                    if a <= b and is_tatami_free(a, b):
                        num_tatami_free += 1
            if num_tatami_free == N:
                return s

    return 0


def is_tatami_free(a: int, b: int) -> bool:
    """Check if a*b room is Tatami-free."""
    return a > 2 and ceil_div(b - 1, a + 1) > (b + 1) // (a - 1)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
