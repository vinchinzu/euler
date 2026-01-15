"""Project Euler Problem 682: 5-Smooth Pairs.

Find the number of pairs (p, q) of Hamming numbers (5-smooth numbers) such
that p and q have the same number of prime factors counted with multiplicity,
and the sum of the prime factors of a and b together, counted with multiplicity,
is N.

Let the number of factors of 2, 3, 5 of p be p2, p3, p5 respectively, and
define q2, q3, q5 similarly for q. The number of pairs corresponds to the
number of lattice points in a region defined by planar restrictions, i.e. a
polyhedron. Therefore, the solution obeys a linear recurrence, and we can
extrapolate the final result from small values.
"""

from __future__ import annotations

from typing import Callable


def extrapolation(
    f: Callable[[int], int], n_points: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function using Lagrange interpolation."""
    values = []
    for i in range(1, n_points + 1):
        values.append(f(i) % mod)

    def interpolate(x: int) -> int:
        """Interpolate at point x."""
        result = 0
        for i in range(n_points):
            term = values[i]
            for j in range(n_points):
                if i != j:
                    denom = (i + 1 - (j + 1)) % mod
                    if denom == 0:
                        continue
                    inv = pow(denom, mod - 2, mod)
                    term = (term * (x - (j + 1)) * inv) % mod
            result = (result + term) % mod
        return result

    return interpolate


def nCr(n: int, r: int) -> int:
    """Binomial coefficient."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = result * (n - i) // (i + 1)
    return result


def solve() -> int:
    """Solve Problem 682."""
    N = 10**7
    M = 10**9 + 7

    def f(n: int) -> int:
        """Count pairs for given n."""
        count = 0
        for p2 in range(n // 2 + 1):
            for p3 in range((n - 2 * p2) // 3 + 1):
                for p5 in range((n - 2 * p2 - 3 * p3) // 5 + 1):
                    remaining = n - 2 * p2 - 3 * p3 - 5 * p5
                    for q2 in range(remaining // 2 + 1):
                        for q3 in range((remaining - 2 * q2) // 3 + 1):
                            q5 = p2 + p3 + p5 - q2 - q3
                            if (
                                2 * p2 + 3 * p3 + 5 * p5
                                + 2 * q2 + 3 * q3 + 5 * q5
                                == n
                            ):
                                count += 1
        return count

    extrap_func = extrapolation(f, 5, M)
    ans = extrap_func(N) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
