"""Project Euler Problem 727: Triangle of Circles.

If r_a, r_b, and r_c are radii of three mutually externally tangent circles,
let D be the center of the circle through their tangent points, and let E
be the center of the small circle externally tangent to all three. Find the
expected value of DE for all 1 ≤ r_a < r_b < r_c ≤ N with
GCD(r_a, r_b, r_c) = 1.

Draw triangle ABC whose vertices are the centers of the three large circles,
and whose side lengths are r_a + r_b, r_a + r_c, and r_b + r_c. Then we can
see that D is the in-center of ABC, and E is the equal detour point. We use
barycentric coordinates to compute the distance.
"""

from __future__ import annotations

from math import gcd, sqrt


def fsq(x: float) -> float:
    """Square of x."""
    return x * x


def de(ra: int, rb: int, rc: int) -> float:
    """Compute distance DE."""
    a = float(rb + rc)
    b = float(ra + rc)
    c = float(ra + rb)
    s = (a + b + c) / 2.0
    k = sqrt(s * (s - a) * (s - b) * (s - c))

    da = a
    db = b
    dc = c
    d_norm = da + db + dc

    ea = a + k / (s - a)
    eb = b + k / (s - b)
    ec = c + k / (s - c)
    e_norm = ea + eb + ec

    x = da / d_norm - ea / e_norm
    y = db / d_norm - eb / e_norm
    z = dc / d_norm - ec / e_norm

    return sqrt(-(fsq(a) * y * z + fsq(b) * x * z + fsq(c) * x * y))


def solve() -> float:
    """Solve Problem 727."""
    n = 100
    count = 0
    total = 0.0

    for ra in range(1, n + 1):
        for rb in range(ra + 1, n + 1):
            for rc in range(rb + 1, n + 1):
                if gcd(gcd(ra, rb), rc) == 1:
                    count += 1
                    total += de(ra, rb, rc)

    return total / count if count > 0 else 0.0


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return int(result * 1e8)


if __name__ == "__main__":
    main()
