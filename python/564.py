"""Project Euler Problem 564: Expected Maximal Polygon Area.

If a segment of length 2n-3 is divided into n integer-length segments such that each
division is equally likely, and the segments are arranged into an n-sided polygon with
maximal area, then let E(n) be the expected maximal area. Find Σ_{n=3}^N E(n).

We can recurse over all possible unordered sets of n segments. For each unordered set,
the maximal area is achieved when the polygon is cyclic, so we use binary search to
compute the diameter of the circumcircle. (Note that if using the largest side as the
diameter still results in all chords spanning over 2π, then the diameter is even larger,
and the center of the circle is outside the polygon. In that case, we have to compute the
angle, and later on the area, slightly differently.) With the diameter, we can then
compute the area and multiply by the number of ordered sets. By balls and bins, there are
nCr(2n-4,n-1) total divisions, so we divide the sum of all areas by that value to get E(n).
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from typing import List


N = 50


@dataclass(frozen=True)
class Side:
    """Side of polygon."""

    val: int
    count: int


def feq(a: float, b: float, eps: float = 1e-10) -> bool:
    """Check if two floats are approximately equal."""
    return abs(a - b) < eps


def ffactorial(n: int) -> float:
    """Float factorial."""
    result = 1.0
    for i in range(2, n + 1):
        result *= i
    return result


def fnCr(n: int, r: int) -> float:
    """Float binomial coefficient."""
    if r < 0 or r > n:
        return 0.0
    if r == 0 or r == n:
        return 1.0
    result = 1.0
    for i in range(min(r, n - r)):
        result = result * (n - i) / (i + 1)
    return result


def fsq(x: float) -> float:
    """Square of float."""
    return x * x


def last(lst: List[Side]) -> Side:
    """Get last element of list."""
    return lst[-1]


def remove_last(lst: List[Side]) -> None:
    """Remove last element of list."""
    lst.pop()


def center_outside_polygon(sides: List[Side]) -> bool:
    """Check if center is outside polygon."""
    if last(sides).count > 1:
        return False
    angle = 0.0
    max_side = last(sides).val
    for side in sides:
        angle += side.count * math.asin(1.0 * side.val / max_side)
    return angle < math.pi


ans = 0.0


def helper(n: int, remaining_sides: int, remaining_perim: int, sides: List[Side]) -> None:
    """Recursive helper to enumerate all possible side combinations."""
    global ans
    if remaining_sides == 0:
        if remaining_perim == 0:
            center_outside = center_outside_polygon(sides)
            low = float(last(sides).val)
            high = 2.0 * n
            prev = 0.0
            while not feq(low, high):
                mid = (low + high) / 2.0
                angle = 0.0
                for side in sides:
                    angle += prev = side.count * math.asin(side.val / mid)
                if center_outside:
                    angle = math.pi + 2 * prev - angle
                if angle > math.pi:
                    low = mid
                else:
                    high = mid
            area = 0.0
            for side in sides:
                area += prev = (
                    side.count
                    * side.val
                    * math.sqrt(fsq(low) - fsq(side.val))
                    / 4
                )
            if center_outside:
                area -= 2 * prev
            area *= ffactorial(n)
            for side in sides:
                area /= ffactorial(side.count)
            ans += area / fnCr(2 * n - 4, n - 1)
        return

    start_val = 1 if not sides else last(sides).val + 1
    for val in range(start_val, remaining_perim // remaining_sides + 1):
        for count in range(1, remaining_sides + 1):
            if val * count <= remaining_perim:
                sides.append(Side(val, count))
                helper(n, remaining_sides - count, remaining_perim - val * count, sides)
                remove_last(sides)


def solve() -> float:
    """Solve Problem 564."""
    global ans
    ans = 0.0
    for n in range(3, N + 1):
        helper(n, n, 2 * n - 3, [])
    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.6f}")


if __name__ == "__main__":
    main()
