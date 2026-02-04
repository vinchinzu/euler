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

sys.setrecursionlimit(10000)


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

    def F(r: int, last_colors: List[int], max_color: int) -> List[int]:
        """Compute F for given state."""
        key = (r, tuple(last_colors))

        if key in cache:
            return cache[key]

        F_arr = [0] * F_SIZE

        if r == R * C:
            F_arr = [1] * F_SIZE
            cache[key] = F_arr
            return F_arr

        for color in range(1, max_color + 2):
            # Check constraints
            if r % R != 0 and len(last_colors) > 0 and color == last_colors[-1]:
                continue
            if len(last_colors) >= R and color == last_colors[len(last_colors) - R]:
                continue

            new_last = list(last_colors)
            new_last.append(color)

            # Normalize: mapping colors starting from 1
            new_max = max_color
            if color == max_color + 1:
                new_max = max_color + 1

            # For the next call, we only need the last R colors
            trimmed = new_last[max(0, len(new_last) - R):]
            # Re-normalize
            mapping = {}
            curr = 0
            norm_colors = []
            for c in trimmed:
                if c not in mapping:
                    curr += 1
                    mapping[c] = curr
                norm_colors.append(mapping[c])

            next_F = F(r + 1, norm_colors, curr)

            for n_idx in range(F_SIZE):
                choices = (n_idx - color + 1) if color == max_color + 1 else 1
                F_arr[n_idx] = (F_arr[n_idx] + choices * next_F[n_idx]) % M

        cache[key] = F_arr
        return F_arr

    F_result = F(0, [], 0)

    # Compute cumulative sum values at points 1, 2, ..., R*C
    n_points = R * C
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
