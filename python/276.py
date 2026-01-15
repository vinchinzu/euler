"""Project Euler Problem 276: Primitive Triangles.

Find the number of primitive integer triangles with perimeter at most N.
"""

from __future__ import annotations

from math import isqrt


def sieve_mobius(limit: int) -> list[int]:
    """Generate Mobius function values."""
    mobius = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                mobius[j] *= -1
            for j in range(i * i, limit + 1, i * i):
                mobius[j] = 0

    return mobius


def solve() -> int:
    """Solve Problem 276."""
    N = 10**7

    mobius = sieve_mobius(N)

    num_triangles = [0] * (N + 1)
    for k in range(N + 1):
        if k % 2 == 0:
            num_triangles[k] = (k * k + 24) // 48
        else:
            num_triangles[k] = ((k + 3) ** 2 + 24) // 48

    sum_triangles = [0] * (N + 1)
    for k in range(1, N + 1):
        sum_triangles[k] = sum_triangles[k - 1] + num_triangles[k]

    ans = 0
    for d in range(1, N + 1):
        ans += mobius[d] * sum_triangles[N // d]

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
