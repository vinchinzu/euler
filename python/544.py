"""Project Euler Problem 544: Chromatic Polynomial.

Let f(r,c,n) be the number of ways to color each cell of a r x c grid graph
one of n different colors, so that no two orthogonally adjacent squares are
the same color. Find sum_{k=1}^N f(R,C,k).

We use dynamic programming to compute the number of ways to color the
remaining squares after coloring the first r squares in reading order, where
the state is r and the previous R squares.

The final values must satisfy a polynomial (the chromatic polynomial). So the
sum must also be a polynomial, and we can extrapolate to get the sum up to N.
"""

from __future__ import annotations

import sys
from typing import Dict, List, Tuple

sys.setrecursionlimit(20000)


def lagrange_extrapolation(values: List[int], x: int, mod: int) -> int:
    """Lagrange interpolation at x given values at 1, 2, ..., len(values).

    Uses O(n) precomputation for efficient evaluation.
    """
    n = len(values)
    # Compute prefix and suffix products of (x - i)
    prefix = [1] * (n + 1)
    for i in range(n):
        prefix[i + 1] = prefix[i] * ((x - (i + 1)) % mod) % mod

    suffix = [1] * (n + 1)
    for i in range(n - 1, -1, -1):
        suffix[i] = suffix[i + 1] * ((x - (i + 1)) % mod) % mod

    # Precompute factorials and inverse factorials
    fact = [1] * (n + 1)
    for i in range(1, n + 1):
        fact[i] = fact[i - 1] * i % mod
    inv_fact = [1] * (n + 1)
    inv_fact[n] = pow(fact[n], mod - 2, mod)
    for i in range(n - 1, -1, -1):
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % mod

    result = 0
    for i in range(n):
        num = prefix[i] * suffix[i + 1] % mod
        denom = inv_fact[i] * inv_fact[n - 1 - i] % mod
        if (n - 1 - i) % 2 == 1:
            denom = (-denom) % mod
        result = (result + values[i] * num % mod * denom) % mod
    return result


def solve() -> int:
    """Solve Problem 544."""
    N = 1112131415
    R = 9
    C = 10
    M = 10**9 + 7

    cache: Dict[Tuple[int, Tuple[int, ...]], List[int]] = {}
    F_SIZE = 2 * R * C + 5

    def F(r: int, colors: List[int]) -> List[int]:
        """Compute F for given state.

        colors: list of colors used so far (may be longer than R, will be trimmed)
        Returns: array where F[n] = number of ways with n colors
        """
        # Normalize: keep only last R colors, relabel starting from 1
        lastColors = []
        mapping = [0] * (2 * R + 1)  # mapping[old_color] = new_color
        currMaxColor = 0
        start = max(len(colors) - R, 0)
        for i in range(start, len(colors)):
            color = colors[i]
            if mapping[color] == 0:
                currMaxColor += 1
                mapping[color] = currMaxColor
            lastColors.append(mapping[color])

        maxColor = currMaxColor
        key = (r, tuple(lastColors))

        if key in cache:
            return cache[key]

        F_arr = [0] * F_SIZE

        if r == R * C:
            F_arr = [1] * F_SIZE
            cache[key] = F_arr
            return F_arr

        for color in range(1, maxColor + 2):
            # Check constraints
            # Can't match previous cell (horizontal neighbor)
            if r % R != 0 and len(lastColors) > 0 and color == lastColors[-1]:
                continue
            # Can't match cell R positions back (vertical neighbor)
            if len(lastColors) >= R and color == lastColors[len(lastColors) - R]:
                continue

            # Add this color and recurse
            newColors = lastColors + [color]
            nextF = F(r + 1, newColors)

            for n_idx in range(F_SIZE):
                # If using a new color (color == maxColor + 1), we have n - color + 1 choices
                choices = (n_idx - color + 1) if color == maxColor + 1 else 1
                F_arr[n_idx] = (F_arr[n_idx] + choices * nextF[n_idx]) % M

        cache[key] = F_arr
        return F_arr

    F_result = F(0, [])

    # Compute cumulative sum values at points 1, 2, ..., R*C+2
    # The chromatic polynomial has degree R*C, so the sum has degree R*C+1
    # We need R*C+2 sample points for exact interpolation
    n_points = R * C + 2
    sum_values = []
    for k in range(1, n_points + 1):
        S = 0
        for i in range(k + 1):
            S = (S + F_result[i]) % M
        sum_values.append(S)

    # Extrapolate to N
    ans = lagrange_extrapolation(sum_values, N, M)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
