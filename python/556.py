"""Project Euler Problem 556: Squarefree Gaussian integers.

Find the number of proper squarefree Gaussian integers a+bi with a²+b²≤N.

Let f(n) be the number of proper squarefree Gaussian integers with a²+b²≤|n|.
Then Σ_z f(n/z²) over all Gaussian integers z is exactly the total number of
proper Gaussian integers with a²+b²≤|n|. The latter can be computed directly
as Σ_a ⌊√(n-a²)⌋+1, so this gives a recursive way to compute f(n).
"""

from __future__ import annotations

from math import isqrt
from typing import Dict


def solve() -> int:
    """Solve Problem 556."""
    N = 10**14

    cache: Dict[int, int] = {}

    def f(n: int) -> int:
        """Compute f(n) recursively with memoization."""
        if n == 0:
            return 0
        if n in cache:
            return cache[n]

        result = 0
        # Count all proper Gaussian integers with a²+b²≤n
        for a in range(1, isqrt(n) + 1):
            if a * a <= n:
                result += isqrt(n - a * a) + 1

        # Subtract counts for multiples of squares
        # For each Gaussian integer z = a + bi where |z|² = a² + b²
        # We subtract f(n / |z|²) for all z with |z|² > 1
        max_a = isqrt(isqrt(n))
        for a in range(1, max_a + 1):
            a_sq = a * a
            if a_sq * a_sq > n:
                break
            max_b = isqrt(n - a_sq * a_sq)
            for b in range(max_b + 1):
                b_sq = b * b
                z_norm_sq = a_sq + b_sq
                if z_norm_sq == 0:
                    continue
                if z_norm_sq == 1:
                    continue  # Skip z = 1 (already handled)
                if z_norm_sq * z_norm_sq > n:
                    break
                # z = a + bi, |z|² = a² + b²
                # We need to subtract f(n / |z|²)
                result -= f(n // (z_norm_sq * z_norm_sq))

        cache[n] = result
        return result

    return f(N)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
