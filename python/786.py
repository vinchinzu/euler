"""Project Euler Problem 786: Billiard Ball Bounces.

A billiard ball starts on vertex A of a quadrilateral table ABCD with A=120º,
B=90º, C=60º, and D=90º, with AB=AD. Find the number of ways it can bounce
along the edges up to N times, never hitting any corners, and arrive back at A.

Using the standard reflection trick, we can tessellate the plane with
reflections of the table, forming a lattice consisting of a triangular grid and
its dual hexagonal grid.

The problem then becomes: find the number of Eisenstein integers T such that
the path from A to T crosses up to N lines in both the triangular and hexagonal
grid, and do not intersect any of the vertices in the hexagonal grid (which
are reflections of A), any of the centers of the hexagonal (which are
reflections of C), or the midpoints of any of the edges (which are reflections
of B and D).

It turns out that to avoid intersecting vertices or centers of hexagons, we need
T=a+bω to satisfy (a,b) = 1, and to avoid intersecting midpoints of edges, we
need a+b≡0 (mod 3).

The answer is 4S+2 where S is computed using Inclusion Exclusion over GCD
conditions.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def pre_mobius(limit: int) -> List[int]:
    """Precompute Möbius function."""
    mu = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]
    return mu


def num_lattice_points(t: int, is_div_by_3: bool) -> int:
    """Count lattice points under line 5x+dy = t, where d=3 if is_div_by_3 else 9."""
    d = 3 if is_div_by_3 else 9
    count = 0
    for x in range(1, t // 5 + 1):
        y_max = (t - 5 * x) // d
        if y_max >= 0:
            count += y_max
    return count


def solve() -> int:
    """Solve Problem 786."""
    N = 10**9
    L = (3 * N + 5) // 2
    sqrt_L = isqrt(L)

    mobius = pre_mobius(L)

    ans = 0

    # Direct computation for small g
    for g in range(1, L // sqrt_L + 1):
        ans += mobius[g] * num_lattice_points(L // g, g % 3 == 0)

    # For larger ranges, use Mertens function (simplified)
    # In practice, would use QuotientValues for efficiency
    for t in range(5, sqrt_L):
        mertens_t = sum(mobius[i] for i in range(1, t + 1))
        mertens_t_plus_1 = sum(mobius[i] for i in range(1, t + 2))
        sum_mult3_t = sum(mobius[i] for i in range(1, t + 1) if i % 3 == 0)
        sum_mult3_t_plus_1 = sum(mobius[i] for i in range(1, t + 2) if i % 3 == 0)

        a = mertens_t - mertens_t_plus_1
        b = sum_mult3_t - sum_mult3_t_plus_1

        ans += b * num_lattice_points(t, True) + (a - b) * num_lattice_points(t, False)

    ans *= 4
    ans += 2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
