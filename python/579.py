"""Project Euler Problem 579: Lattice points in cubes.

Let f(C) be the number of lattice points on or inside a cube C. Find Σ f(C)
for all cubes with lattice point vertices with coordinates from 0 to n inclusive.

See https://arxiv.org/pdf/1111.1150.pdf. Equation (3) gives a parameterization
of the primitive cubes, with integer parameters a,b,c,d, not all zero. To
ensure all cubes are counted exactly once, we need a≤b≤c, and either c≤d, or
one of (a=0, a=b, b=d). For each cube, we check that it is actually primitive
(there is no common divisor of all coordinates).

Given a primitive cube of side length l, we find (1) its bounds and (2) the
number of distinct rotations or reflections of the cube. Then for each scale
factor t of this primitive cube, f(C) can be computed by evaluating the Ehrhart
polynomial is (lt)³+lDt²+Dt+1 (from the paper), where D is the sum of the GCDs
of the coordinates of each basis vector of the cube.

We then multiply the number of points by the number of valid translations of
this scaled cube (which can be computed from its bounds) and by the number of
symmetries (its rotations or reflections). Summing this for all scale factors
over all primitive cubes gives the answer.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List, Tuple


def solve() -> int:
    """Solve Problem 579."""
    N = 5000
    M = 10**9

    ans = 0

    for a in range(isqrt(N) + 1):
        for b in range(a, isqrt(N - a * a) + 1):
            for c in range(b, isqrt(N - a * a - b * b) + 1):
                for d in range(b, isqrt(N - a * a - b * b - c * c) + 1):
                    l_sq = a * a + b * b + c * c + d * d
                    if l_sq > N:
                        break
                    if a + b + c + d == 0 or (d < c and (a == 0 or a == b or b == d)):
                        continue

                    # Compute axes
                    axes: List[List[int]] = [
                        [
                            a * a + b * b - c * c - d * d,
                            2 * (b * c + d * a),
                            2 * (b * d - c * a),
                        ],
                        [
                            2 * (b * c - d * a),
                            a * a - b * b + c * c - d * d,
                            2 * (c * d + b * a),
                        ],
                        [
                            2 * (b * d + c * a),
                            2 * (c * d - b * a),
                            a * a - b * b - c * c + d * d,
                        ],
                    ]

                    # Compute GCDs
                    gcds = [
                        abs(gcd(gcd(axes[i][0], axes[i][1]), axes[i][2]))
                        for i in range(3)
                    ]
                    if gcd(gcd(gcds[0], gcds[1]), gcds[2]) > 1:
                        continue

                    # Compute bounds
                    mins = [3 * N] * 3
                    maxs = [-3 * N] * 3
                    for subset in range(8):
                        v = [0, 0, 0]
                        for i in range(3):
                            if subset & (1 << i):
                                for dim in range(3):
                                    v[dim] += axes[i][dim]
                        for dim in range(3):
                            mins[dim] = min(v[dim], mins[dim])
                            maxs[dim] = max(v[dim], maxs[dim])

                    # Compute symmetries
                    num_symmetries = 24
                    if a == 0 and (b == c or c == d):
                        num_symmetries //= 2
                    if b == c and (a == b or c == d):
                        num_symmetries //= 3
                    if b == 0:
                        num_symmetries //= 4

                    D = gcds[0] + gcds[1] + gcds[2]
                    l = isqrt(l_sq)
                    for t in range(1, N + 1):
                        num_points = (l * t) ** 3 + l * D * t * t + D * t + 1
                        num_cubes = 1
                        for i in range(3):
                            size = (maxs[i] - mins[i]) * t
                            num_cubes *= max(N - size + 1, 0)
                        if num_cubes == 0:
                            break
                        ans = (
                            ans
                            + (num_points % M)
                            * (num_cubes % M)
                            % M
                            * num_symmetries
                        ) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
