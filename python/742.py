"""Project Euler Problem 742: Minimum Area of a Convex Grid Polygon.

Find the minimum area of a convex polygon with N lattice point vertices and
with both horizontal and vertical symmetry.

First we assume the polygon is centered at the origin and look at only the
first quadrant with N/4 vertices. We can sort all possible sides in order of
angle. For every (n, k), we find the best ways to arrange k of the first n
sides, which we call "branches". Some of these branches aren't comparable; one
might have a larger x displacement but a smaller y displacement, but ones
with a larger x displacement and larger area can be removed. We keep the
remaining optimal branches ordered by x displacement. We start with a branch
of zero sides, starting at x=1/2, and maintain the area of the region between
the branch and the y-axis.

For a (n, k), we can either use the nth side (which means we add that x,y to
all possible branches of (n-1, k-1)), or we don't (which means we just take
the branches of (n-1, k)). If we take the nth side, we need to add the area
of the trapezoid (from this side to the y-axis) added to the total region,
which is (side.y)(2 branch.x + side.x + 1). Each of these are ordered by x
displacement, so we can merge them into a new list ordered by x displacement,
removing ones that have both strictly larger x displacement and area than some
other branch.

For efficiency, we only search for branches until we get to the side with
displacement (1,1). We can then try all pairs of branches up to (1,1), and
compute the area that the two branches form when joined by the side (1,1) in
the first quadrant. This area is 4(branch1.area + branch2.area) +
4(branch1.x * branch2.x) + 6(branch1.x + branch2.x) + 7.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import atan2, gcd
from typing import List, Tuple


@dataclass
class Point:
    """2D integer point."""

    x: int
    y: int


@dataclass
class Branch:
    """Branch representing a path of sides."""

    x: int
    y: int
    twice_area: int

    def __lt__(self, other: Branch) -> bool:
        """Compare branches by x, then by area."""
        if self.x != other.x:
            return self.x < other.x
        return self.twice_area < other.twice_area


def solve() -> int:
    """Solve Problem 742."""
    N = 1000
    L = 40

    # Generate all primitive sides
    sides: List[Point] = []
    for x in range(1, L + 1):
        for y in range(1, x):
            if gcd(x, y) == 1:
                sides.append(Point(x, y))

    # Sort by angle
    sides.sort(key=lambda p: atan2(p.y, p.x))

    # Find side (1,1) index
    side_1_1_idx = -1
    for i, side in enumerate(sides):
        if side.x == 1 and side.y == 1:
            side_1_1_idx = i
            break

    if side_1_1_idx == -1:
        return 0

    # Process sides up to (1,1)
    all_branches: dict[int, List[Branch]] = {}
    all_branches[0] = [Branch(0, 0, 0)]

    for side_idx, side in enumerate(sides[: side_1_1_idx + 1]):
        new_all_branches: dict[int, List[Branch]] = {}
        new_all_branches[0] = [Branch(0, 0, 0)]

        for num_sides in range(1, N // 4):
            prev_branches = all_branches.get(num_sides, [])
            curr_branches: List[Branch] = []

            # Add branches using this side
            for branch in all_branches.get(num_sides - 1, []):
                new_x = branch.x + side.x
                new_y = branch.y + side.y
                new_area = branch.twice_area + side.y * (
                    2 * branch.x + side.x + 1
                )
                curr_branches.append(Branch(new_x, new_y, new_area))

            # Merge and filter
            merged = sorted(prev_branches + curr_branches)
            new_branches: List[Branch] = []
            for branch in merged:
                if (
                    not new_branches
                    or branch.twice_area < new_branches[-1].twice_area
                ):
                    new_branches.append(branch)

            new_all_branches[num_sides] = new_branches

        all_branches = new_all_branches

    # Find minimum area
    ans = float("inf")
    for num_sides1 in range(1, N // 8):
        num_sides2 = N // 4 - 2 - num_sides1
        if num_sides2 < 0:
            continue
        branches1 = all_branches.get(num_sides1, [])
        branches2 = all_branches.get(num_sides2, [])
        for branch1 in branches1:
            for branch2 in branches2:
                area = (
                    2 * branch1.twice_area
                    + 2 * branch2.twice_area
                    + 4 * branch1.x * branch2.x
                    + 6 * branch1.x
                    + 6 * branch2.x
                    + 7
                )
                ans = min(ans, area)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
