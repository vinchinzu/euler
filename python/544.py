"""Project Euler Problem 544: Chromatic Polynomial.

Let f(r,c,n) be the number of ways to color each cell of a r x c grid graph
one of n different colors, so that no two orthogonally adjacent squares are
the same color. Find Σ_{k=1}^N f(R,C,k).

We use dynamic programming to compute the number of ways to color the
remaining squares after coloring the first r squares in reading order, where
the state is r and the previous R squares.

The final values must satisfy a polynomial (the chromatic polynomial). So the
sum must also be a polynomial, and we can extrapolate to get the sum up to N.
"""

from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from typing import Callable, Dict, List, Tuple


@dataclass(frozen=True)
class Key:
    """Cache key."""

    r: int
    last_colors: Tuple[int, ...]


def lagrange_extrapolation(
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


def solve() -> int:
    """Solve Problem 544."""
    N = 1112131415
    R = 9
    C = 10
    M = 10**9 + 7

    cache: Dict[Key, List[int]] = {}

    def F(r: int, colors: List[int]) -> List[int]:
        """Compute F for given state."""
        # Normalize colors
        last_colors = []
        mapping = [0] * (2 * R)
        curr_max_color = 0
        start_idx = max(len(colors) - R, 0)
        for i in range(start_idx, len(colors)):
            color = colors[i]
            if mapping[color] == 0:
                curr_max_color += 1
                mapping[color] = curr_max_color
            last_colors.append(mapping[color])
        
        max_color = curr_max_color
        key = Key(r, tuple(last_colors))
        
        if key in cache:
            return cache[key]
        
        F_arr = [0] * (2 * R * C + 5)
        
        if r == R * C:
            F_arr = [1] * len(F_arr)
            cache[key] = F_arr
            return F_arr
        
        for color in range(1, max_color + 2):
            # Check constraints
            if r % R != 0 and len(last_colors) > 0 and color == last_colors[-1]:
                continue
            if len(last_colors) >= R and color == last_colors[len(last_colors) - R]:
                continue
            
            new_colors = colors + [color]
            next_F = F(r + 1, new_colors)
            
            choices = (n - color + 1) if color == max_color + 1 else 1
            for n in range(len(F_arr)):
                if n < len(next_F):
                    F_arr[n] = (F_arr[n] + choices * next_F[n]) % M
        
        cache[key] = F_arr
        return F_arr

    F_result = F(0, [])
    
    def sum_func(n: int) -> int:
        """Sum F up to n."""
        total = 0
        for i in range(min(n + 1, len(F_result))):
            total = (total + F_result[i]) % M
        return total
    
    extrap = lagrange_extrapolation(sum_func, R * C, M)
    ans = extrap(N)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
