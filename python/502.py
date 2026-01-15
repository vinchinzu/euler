"""Project Euler Problem 502: Counting Castles.

For given pairs (w, h), find the number of castles (configurations of stacked
horizontal blocks) with width w, height h, no horizontally adjacent blocks,
a single block on the bottom row, and an even number of blocks.

Let f(p, x, y) be the number of castles with width x, rightmost column of
height y, and the number of blocks has parity p. If the second rightmost column
is taller than the rightmost column, then the castle of width x-1 has the same
number of blocks, and therefore the same parity. Otherwise, the parity changes
if the heights of the rightmost two columns have different parity.

Let dp(p, x, y) be the sum of all f(p, x, y') where y' ≤ y has the same parity
as y; this gives a more efficient algorithm.

To compute the answer for large w or h, note that the total satisfies a
generating function and therefore we can extrapolate the value from a linear
recurrence.
"""

from __future__ import annotations

from typing import Callable


def lagrange_extrapolation(
    f: Callable[[int], int], n_points: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function using Lagrange interpolation."""
    # Generate n_points values
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


def num_castles(w: int, h: int, mod: int) -> int:
    """Count castles with width w and height h."""
    if h <= 0:
        return 0
    # dp[p][x][y] = sum of f(p, x, y') for y' <= y with same parity
    dp = [[[0] * (h + 1) for _ in range(w + 1)] for _ in range(2)]
    
    # Base case: width 0, even parity, even heights
    for y in range(0, h + 1, 2):
        dp[0][0][y] = 1
    
    for x in range(1, w + 1):
        for y in range(1, h + 1):
            for p in range(2):
                # Add all from previous width
                dp[p][x][y] += dp[p][x - 1][h]
                dp[p][x][y] += dp[p][x - 1][h - 1]
                # Subtract those <= y-1 with different parity
                dp[p][x][y] -= dp[p][x - 1][y - 1]
                # Add those <= y-1 with opposite parity
                dp[p][x][y] += dp[1 - p][x - 1][y - 1]
                # Cumulative sum: add previous y-2 if exists
                if y >= 2:
                    dp[p][x][y] += dp[p][x][y - 2]
                dp[p][x][y] %= mod
    
    return (dp[0][w][h] + dp[0][w][h - 1]) % mod


def num_castles_big(W: int, H: int, L: int, mod: int) -> int:
    """Count castles for large W or H using extrapolation."""
    if W <= L:
        def f_h(h: int) -> int:
            return num_castles(W, h, mod)
        extrap = lagrange_extrapolation(f_h, L, mod)
        return extrap(H)
    if H <= L:
        def f_w(w: int) -> int:
            return num_castles(w, H, mod)
        extrap = lagrange_extrapolation(f_w, L, mod)
        return extrap(W)
    return num_castles(W, H, mod)


def solve() -> int:
    """Solve Problem 502."""
    inputs = [
        (10**12, 100),
        (10000, 10000),
        (100, 10**12),
    ]
    L = 100
    M = 10**9 + 7
    
    ans = 0
    for w, h in inputs:
        ans = (ans + num_castles_big(w, h, L, M) - 
               num_castles_big(w, h - 1, L, M)) % M
    
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
