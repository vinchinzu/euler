"""Project Euler Problem 547: Distance of Random Points.

A hollow square lamina of size n is an n x n square from which a rectangle at lattice points
is removed. Find the sum of the expected distance of two randomly selected points on a hollow
square lamina, over all hollow square lamina of size N.

First we compute e(dx, dy), the expected distance between a random point in a unit square
from a random point in another unit square that is (dx, dy) away. Note that the probability
distribution of the difference of x-coordinates is a triangle represented by P(x) ~ 1 -
|x-dx| for dx-1 ≤ x ≤ dx+1, and the distribution for y-coordinates is similar. So

e(dx, dy) = ∫_{dx-1}^{dx+1} ∫_{dy-1}^{dy+1} (1 - |x-dx|)(1 - |y-dy|) √(x²+y²) dx dy.

Consider the "top right" region where x≥dx and y≥dy (so we can remove the absolute values):

top right = ∫_{dx}^{dx+1} ∫_{dy}^{dy+1} (1 - (x-dx))(1 - (y-dy)) √(x²+y²) dx dy
          = ∫∫ (1+dx - x)(1+dy - y) √(x²+y²) dx dy
          = (1+dx)(1+dy) ∫∫ √(x²+y²) dx dy
                 - (1+dy) ∫∫ x√(x²+y²) dx dy
                 - (1+dx) ∫∫ y√(x²+y²) dx dy
                 + ∫∫ x*y√(x²+y²) dx dy.

Each of the four functions can be integrated with Wolfram Alpha (note the middle two are the
same function with x and y swapped), and then we can compute the definite integral over our
desired bounds. We can compute the top left, bottom left, and bottom right regions similarly
to get e(dx, dy).

Now take two shapes S1 and S2 consisting of unit squares. The expected distance d(S1, S2)
between a point in S1 and a point in S2 is equal to the sum of the expected distances between
all pairs of unit squares (each one equal to e(dx, dy) for some (dx, dy)), divided by the
number of pairs of unit squares |S1|x|S2|.

Now let L be a hollow square lamina consisting of a NxN square S with a rectangular hole H
removed. Then the sum of the expected distances is d(L, L) = d(S, S) - 2d(S, H) + d(H, H). To
compute d(S, S) and d(H, H), we note that they are just scaled versions of e(0, 0), where we
replace 1 with the appropriate width and height. To compute d(S, H), we can precompute d(S,
u) for every unit square u, and then efficiently sum up d(S, u) for all u ∈ H with dynamic
programming. The expected distance is then d(L, L) / |L|².
"""

from __future__ import annotations

from math import asinh, atanh, log, sqrt
from typing import Callable


N = 40


def pow_float(a: float, b: int) -> float:
    """Float power."""
    return a**b


def hypot(a: float, b: float) -> float:
    """Hypotenuse: sqrt(a² + b²)."""
    return sqrt(a * a + b * b)


def asinh_impl(x: float) -> float:
    """Inverse hyperbolic sine."""
    return log(x + hypot(x, 1))


def atanh_impl(x: float) -> float:
    """Inverse hyperbolic tangent."""
    return log((1 + x) / (1 - x)) / 2


def i_hypot(x: float, y: float) -> float:
    """Integral of √(x²+y²)."""
    if x == 0 and y == 0:
        return 0.0
    res = (
        4 * x * pow_float(y, 3) / hypot(x, y)
        + 4 * pow_float(x, 3) * y / hypot(x, y)
        - 2 * pow_float(y, 3) / 3
    )
    if x != 0:
        res += (
            pow_float(x, 4)
            * hypot(y / x, 1)
            * asinh_impl(y / x)
            / hypot(x, y)
            + 3 * pow_float(x, 3) * log(hypot(x, y) + y)
            - 2 * pow_float(x, 3) * atanh_impl(y / hypot(x, y))
        )
    if y != 0:
        res += 2 * pow_float(y, 3) * log(hypot(x, y) + x)
    return res / 12


def i_x_hypot(x: float, y: float) -> float:
    """Integral of x√(x²+y²)."""
    res = 5 * x * x * y + 2 * pow_float(y, 3)
    if x != 0:
        res += 3 * pow_float(x, 3) * asinh_impl(y / x) / hypot(y / x, 1)
    return res * hypot(x, y) / 24


def i_xy_hypot(x: float, y: float) -> float:
    """Integral of xy√(x²+y²)."""
    return pow_float(hypot(x, y), 5) / 15


def definite_integral(
    xl: float,
    xh: float,
    yl: float,
    yh: float,
    f: Callable[[float, float], float],
) -> float:
    """Definite integral over rectangle using fundamental theorem."""
    return f(xh, yh) - f(xh, yl) - f(xl, yh) + f(xl, yl)


def e(dx: int, dy: int, w: int, h: int) -> float:
    """Expected distance between unit squares."""
    res = 0.0
    for sign_w in [-w, w]:
        for sign_h in [-h, h]:
            res += definite_integral(
                dx, dx + sign_w, dy, dy + sign_h, i_xy_hypot
            ) - (dy + sign_h) * definite_integral(
                dx, dx + sign_w, dy, dy + sign_h, i_x_hypot
            ) - (dx + sign_w) * definite_integral(
                dy, dy + sign_h, dx, dx + sign_w, i_x_hypot
            ) + (dx + sign_w) * (dy + sign_h) * definite_integral(
                dx, dx + sign_w, dy, dy + sign_h, i_hypot
            )
    return res


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def solve() -> float:
    """Solve Problem 547."""
    unit_to_unit = [[0.0] * N for _ in range(N)]
    for dx in range(N):
        for dy in range(N):
            unit_to_unit[dx][dy] = e(dx, dy, 1, 1)

    full_to_unit = [[0.0] * N for _ in range(N)]
    for x1 in range(N):
        for y1 in range(N):
            for x2 in range(N):
                for y2 in range(N):
                    full_to_unit[x1][y1] += unit_to_unit[abs(x1 - x2)][
                        abs(y1 - y2)
                    ]

    region_to_itself = [[0.0] * (N + 1) for _ in range(N + 1)]
    for w in range(1, N + 1):
        for h in range(1, N + 1):
            region_to_itself[w][h] = e(0, 0, w, h)

    ans = 0.0
    for xl in range(1, N):
        for yl in range(1, N):
            full_to_region = [[0.0] * N for _ in range(N)]
            for xh in range(xl + 1, N):
                for yh in range(yl + 1, N):
                    full_to_region[xh][yh] = (
                        full_to_unit[xh - 1][yh - 1]
                        + full_to_region[xh][yh - 1]
                        + full_to_region[xh - 1][yh]
                        - full_to_region[xh - 1][yh - 1]
                    )
                    area = sq(N) - (xh - xl) * (yh - yl)
                    ans += (
                        region_to_itself[N][N]
                        - 2 * full_to_region[xh][yh]
                        + region_to_itself[xh - xl][yh - yl]
                    ) / sq(area)

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.4f}")


if __name__ == "__main__":
    main()
