"""Project Euler Problem 716: Grid Graphs.

Let a grid graph G be a grid of HxW nodes such that each vertical line is up
or down, and each horizontal line is left or right, and let S(G) be the number
of strongly connected components in G. Find the sum of S(G) over all possible
grid graphs of HxW nodes.

For a given G, each corner may have a rectangular subgrid of vertices that are
not strongly connected to anything else, if the horizontal and vertical lines
are either both pointed away or towards the corner. So the number of strongly
connected components is the number of such isolated vertices, plus one strongly
connected "big" component if the isolated vertices don't take up the entire grid.

We can compute the answer for the given H,W with standard extrapolation.
"""

from __future__ import annotations

from itertools import product
from typing import Callable


def extrapolation(
    f: Callable[[int], int], order: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function using Lagrange interpolation."""
    # Generate order+1 points
    n_points = order + 1
    x_vals = list(range(1, n_points + 1))
    y_vals = [f(x) % mod for x in x_vals]

    def interpolate(x: int) -> int:
        """Interpolate at point x."""
        result = 0
        for i in range(n_points):
            term = y_vals[i]
            for j in range(n_points):
                if i != j:
                    denom = (x_vals[i] - x_vals[j]) % mod
                    if denom == 0:
                        continue
                    inv = pow(denom, mod - 2, mod)
                    term = (term * (x - x_vals[j]) * inv) % mod
            result = (result + term) % mod
        return result % mod

    return interpolate


def C(h: int, w: int) -> int:
    """Compute C(h, w) for grid graph strongly connected components."""
    M = 10**9 + 7
    total = 0

    # Generate all vertical line directions (h bits)
    for vert in product([0, 1], repeat=h):
        # Generate all horizontal line directions (w bits)
        for horiz in product([0, 1], repeat=w):
            # Find corner positions
            try:
                x1 = horiz.index(1 - vert[0])
            except ValueError:
                x1 = -1
            try:
                y1 = vert.index(1 - horiz[0])
            except ValueError:
                y1 = -1
            try:
                x2 = len(horiz) - 1 - list(reversed(horiz)).index(vert[0])
            except ValueError:
                x2 = -1
            try:
                y2 = vert.index(horiz[-1])
            except ValueError:
                y2 = -1
            try:
                x3 = horiz.index(vert[-1])
            except ValueError:
                x3 = -1
            try:
                y3 = len(vert) - 1 - list(reversed(vert)).index(horiz[0])
            except ValueError:
                y3 = -1
            try:
                x4 = len(horiz) - 1 - list(reversed(horiz)).index(1 - vert[-1])
            except ValueError:
                x4 = -1
            try:
                y4 = len(vert) - 1 - list(reversed(vert)).index(1 - horiz[-1])
            except ValueError:
                y4 = -1

            if x1 == -1 or x2 == -1 or y1 == -1 or y3 == -1:
                total = (total + w * h) % M
            else:
                area = (
                    x1 * y1
                    + (w - 1 - max(x1 - 1, x2)) * y2
                    + x3 * (h - 1 - max(y1 - 1, y3))
                    + (w - 1 - max(x3 - 1, x4)) * (h - 1 - max(y2 - 1, y4))
                )
                if area < w * h:
                    area += 1
                total = (total + area) % M

    return total


def solve() -> int:
    """Solve Problem 716."""
    H = 10000
    W = 20000
    M = 10**9 + 7

    # Nested extrapolation: extrapolate over w, then over h
    def inner_extrap(w: int) -> int:
        """Extrapolate over w."""
        def f_w(h_val: int) -> int:
            return C(h_val + 1, w + 1) % M

        extrap_w = extrapolation(f_w, 1, M)
        return extrap_w(W - 1)

    def f_h(h_val: int) -> int:
        return inner_extrap(h_val + 1) % M

    extrap_h = extrapolation(f_h, 1, M)
    ans = extrap_h(H - 1)

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
