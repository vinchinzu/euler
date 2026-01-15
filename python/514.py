"""Project Euler Problem 514: Geoboard Shapes.

Each lattice point (x, y) with 0 ≤ x,y ≤ N has a pin placed on it with
probability P. Find the expected area of the convex hull of all pins.
"""

from __future__ import annotations

from typing import List


def sq(n: int) -> float:
    """Square."""
    return n * n


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> str:
    """Solve Problem 514."""
    N = 100
    P = 1.0 / (N + 1)
    Q = 1 - P

    # Precompute powers of Q
    max_power = sq(N + 1)
    pow_q: List[float] = [Q**i for i in range(max_power + 1)]

    # Compute E(w, h) - expected area under convex hull
    E: List[List[float]] = [[0.0] * (N + 1) for _ in range(N + 1)]

    for w in range(1, N + 1):
        for h in range(1, N + 1):
            # Sum over all possible first pinned points
            for s in range(min(w, h - 1) + 1):
                for x in range(s):
                    if s - 1 >= 0:
                        prob = P * pow_q[tr(s - 1) + x]
                        area = sq(s) / 2 + E[x][h - s] + E[w - s][s - x]
                        E[w][h] += prob * area

            # Handle case where w or h is the limiting dimension
            min_dim = min(w, h - 1)
            if min_dim >= 0:
                prob = pow_q[tr(min_dim)]
                if w < h:
                    area = sq(w) / 2 + E[w][h - w]
                else:
                    area = sq(h) / 2 + E[w - h][h]
                E[w][h] += prob * area

    def f(w: int, h: int) -> float:
        """Probability function."""
        return pow_q[sq(N + 1) - (w + 1) * (h + 1)]

    ans = 0.0
    for w in range(1, N + 1):
        for h in range(1, N + 1):
            num_regions = (N - w + 1) * (N - h + 1)
            prob = (
                f(w, h)
                - 2 * f(w, h - 1)
                - 2 * f(w - 1, h)
                + f(w, h - 2)
                + 4 * f(w - 1, h - 1)
                + f(w - 2, h)
                - 2 * f(w - 1, h - 2)
                - 2 * f(w - 2, h - 1)
                + f(w - 2, h - 2)
            )
            ans += num_regions * prob * w * h

            for x in range(1, w + 1):
                for y in range(1, h + 1):
                    if x == w and y == h:
                        mult = 1.0
                    elif x == w:
                        mult = 1.0 - pow_q[w]
                    elif y == h:
                        mult = 1.0 - pow_q[h + 1]
                    else:
                        mult = P + Q * (1 - pow_q[w - 1]) * (1 - pow_q[h])
                    ans -= (
                        4
                        * num_regions
                        * mult
                        * sq(P)
                        * pow_q[x + h - y]
                        * f(w, h)
                        * E[x][y]
                    )

    return f"{ans:.5f}"


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
