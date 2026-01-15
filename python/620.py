"""Project Euler Problem 620: Gears.

Let a circle of circumference s lie inside a circle of circumference c such that
they are at least 1 unit apart, and four circles of circumferences p,p,q,q can
be placed between them and tangent to both circles. Also add the restriction
that each circle is a gear with one tooth per unit length (so a circle of
circumference n has n teeth), and all gears mesh perfectly. Let g(c,s,p,q) be
the number of configurations that satisfy these restrictions. Find
Σ_{s+p+q≤N} g(s+p+q,s,p,q).

Let A,B,C,D be the centers of the circles with circumferences c,s,p,q
respectively. Let d=AB. Then triangles ABC and ABD both have sides d,s+p,s+q,
and are congruent. Let α=∠ABC and β=∠ABD.

Now we start at the point where circles A and D touch, and walk along the
circumferences of the four circles in a loop until we get back to where we
start. The total distance must be an integer in order for the gears to mesh
perfectly. We compute this value for the two extreme configurations and find
the number of integers within the range.
"""

from __future__ import annotations

import math


def g(s: int, p: int, q: int) -> int:
    """Compute g(s+p+q, s, p, q)."""
    a = s + p
    b = p + q - 2 * math.pi
    c = s + q
    alpha = math.acos((a * a + b * b - c * c) / (2 * a * b))
    beta = math.asin(a * math.sin(alpha) / c)
    return int(((s + q) * beta - (s + p) * alpha) / math.pi + s + p)


def solve() -> int:
    """Solve Problem 620."""
    N = 500
    ans = 0
    for s in range(5, N - 9):
        for p in range(5, N - s):
            for q in range(p + 1, N - s - p + 1):
                ans += g(s, p, q)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
