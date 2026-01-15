"""Project Euler Problem 804: Counting Binary Quadratic Representations.

Find Σ_{n=1}^N g(n), where g(n) is the number of ways to write
n = x²+xy+41y² for integers x,y.

This is just the number of ways to choose (x,y) such that
x²+xy+41y²≤N, where (x,y)≠(0,0). We can compute this by iterating over
all valid y, and using the quadratic formula to find for each y the
minimum and maximum x.
"""

from __future__ import annotations

from math import ceil, floor, isqrt, sqrt


def solve() -> int:
    """Solve Problem 804."""
    N = 10**16
    K = 41

    max_y = isqrt(4 * N // (4 * K - 1))
    ans = 0

    for y in range(-max_y, max_y + 1):
        discriminant = 4 * N - (4 * K - 1) * (y * y)
        if discriminant < 0:
            continue
        sqrt_disc = sqrt(discriminant)
        x_max = floor((-y + sqrt_disc) / 2)
        x_min = ceil((-y - sqrt_disc) / 2)
        ans += max(0, x_max - x_min + 1)

    # Subtract 1 for (0,0)
    return ans - 1


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
