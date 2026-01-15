"""Project Euler Problem 508: Integers in Base i-1.

Let f(a+b*i) be the number of 1s in the unique base i-1 representation of
a+b*i. Find the sum of f(a+b*i) for all |a|≤N, |b|≤N.

Let d_k = (i-1)^k. The set of all numbers with up to k digits in its i-1
representation has a bounding rectangle b_k, which can be iteratively
computed by taking the union of b_{k-1} and b_{k-1} + d_k.

Let B(r, k, extra) be the number of 1s for all numbers with at most k bits
in the bounding rectangle r, if each number has the specified extra number
of 1s. Then B(r, k, extra) is the sum of B(r, k-1, extra), the numbers with
at most k-1 bits, plus B(r - d_k, k-1, extra+1), the numbers with k bits
shifted over to align with the numbers with at most k-1 bits (but with an
extra bit). The base case is the number of extra bits if the only number in
the rectangle is 0, or zero if the query rectangle doesn't intersect with
b_k at all.

To compute this quickly, we snap the bounding rectangle r to the largest
bounding rectangle b_k. This exponentially reduces the number of distinct
bounding rectangles and gives an efficient algorithm.
"""

from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from typing import Dict, List, Tuple


@dataclass(frozen=True)
class LPoint:
    """A point representing a complex number (x, y) = x + yi."""

    x: int
    y: int

    def complex_multiply(self, other: "LPoint") -> "LPoint":
        """Complex multiplication: (x1 + y1i) * (x2 + y2i)."""
        return LPoint(
            self.x * other.x - self.y * other.y,
            self.x * other.y + self.y * other.x,
        )

    def negate(self) -> "LPoint":
        """Negate: -(x + yi) = -x - yi."""
        return LPoint(-self.x, -self.y)


@dataclass(frozen=True)
class Rectangle:
    """A rectangle defined by min/max x and y coordinates."""

    min_x: int
    max_x: int
    min_y: int
    max_y: int

    def translate(self, d: LPoint) -> "Rectangle":
        """Translate rectangle by point d."""
        return Rectangle(
            self.min_x + d.x,
            self.max_x + d.x,
            self.min_y + d.y,
            self.max_y + d.y,
        )

    def union(self, other: "Rectangle") -> "Rectangle":
        """Union of two rectangles."""
        return Rectangle(
            min(self.min_x, other.min_x),
            max(self.max_x, other.max_x),
            min(self.min_y, other.min_y),
            max(self.max_y, other.max_y),
        )

    def intersection(self, other: "Rectangle") -> "Rectangle":
        """Intersection of two rectangles."""
        return Rectangle(
            max(self.min_x, other.min_x),
            min(self.max_x, other.max_x),
            max(self.min_y, other.min_y),
            min(self.max_y, other.max_y),
        )


@dataclass(frozen=True)
class Key:
    """Cache key for memoization."""

    r: Rectangle
    level: int
    count: int


def solve() -> int:
    """Solve Problem 508."""
    N = 10**15
    M = 10**9 + 7
    L = 128

    # Compute d_k = (i-1)^k
    ds: List[LPoint] = [LPoint(1, 0)]
    for k in range(1, L):
        ds.append(ds[-1].complex_multiply(LPoint(-1, 1)))

    # Compute bounding rectangles b_k
    bounds: List[Rectangle] = [Rectangle(0, 0, 0, 0)]
    for d in ds:
        bounds.append(bounds[-1].union(bounds[-1].translate(d)))

    # Memoization cache
    cache: Dict[Key, int] = {}

    def B(r: Rectangle, k: int, extra: int) -> int:
        """Count 1s in base i-1 representation."""
        key = Key(r, k, extra)
        if key in cache:
            return cache[key]

        if r.min_x > r.max_x or r.min_y > r.max_y:
            result = 0
        elif k == -1:
            result = extra
        else:
            bound = bounds[k]
            result = (
                B(r.intersection(bound), k - 1, extra)
                + B(
                    r.translate(ds[k].negate()).intersection(bound),
                    k - 1,
                    extra + 1,
                )
            ) % M

        cache[key] = result
        return result

    ans = B(Rectangle(-N, N, -N, N), L - 1, 0)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
