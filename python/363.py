"""Project Euler Problem 363 implementation in Python.

This module approximates a quarter circle using a cubic Bézier curve with control
points P0=(1, 0), P1=(1, v), P2=(v, 1), and P3=(0, 1). The parameter v is
chosen so that the area enclosed by the curve and the axes equals pi/4.

It then computes by how many percent the Bézier curve length differs from the
length of the quarter circle arc.

This is a direct, idiomatic Python 3.12 translation of the provided Ruby code,
with minor structural cleanups and full type hints.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import sqrt
from typing import Callable

PI: float = 3.14159265358979323846264338327950288419716939937510


@dataclass(frozen=True)
class Point:
    """A 2D point supporting basic vector operations."""

    x: float
    y: float

    def __add__(self, other: "Point") -> "Point":  # pragma: no cover - simple op
        return Point(self.x + other.x, self.y + other.y)

    def __sub__(self, other: "Point") -> "Point":  # pragma: no cover - simple op
        return Point(self.x - other.x, self.y - other.y)

    def __mul__(self, scalar: float) -> "Point":  # pragma: no cover - simple op
        return Point(self.x * scalar, self.y * scalar)

    def to_tuple(self) -> tuple[float, float]:  # pragma: no cover - simple op
        return (self.x, self.y)


def bezier_point(t: float, v: float) -> Point:
    """Return the cubic Bézier point at parameter t for a given v.

    Control points:
    - P0 = (1, 0)
    - P1 = (1, v)
    - P2 = (v, 1)
    - P3 = (0, 1)
    """

    p0 = Point(1.0, 0.0)
    p1 = Point(1.0, v)
    p2 = Point(v, 1.0)
    p3 = Point(0.0, 1.0)

    t = float(t)
    one_minus_t = 1.0 - t

    b0 = one_minus_t**3
    b1 = 3.0 * one_minus_t**2 * t
    b2 = 3.0 * one_minus_t * t**2
    b3 = t**3

    x = p0.x * b0 + p1.x * b1 + p2.x * b2 + p3.x * b3
    y = p0.y * b0 + p1.y * b1 + p2.y * b2 + p3.y * b3

    return Point(x, y)


def bezier_derivative_x(t: float, v: float) -> float:
    """Return d/dt of the x-coordinate of the Bézier curve at parameter t."""

    t = float(t)
    one_minus_t = 1.0 - t

    term1 = 6.0 * one_minus_t * t * (v - 1.0)
    term2 = -3.0 * v * t**2
    return term1 + term2


def bezier_derivative_y(t: float, v: float) -> float:
    """Return d/dt of the y-coordinate of the Bézier curve at parameter t."""

    t = float(t)
    one_minus_t = 1.0 - t

    term1 = 3.0 * v * one_minus_t**2
    term2 = 6.0 * (1.0 - v) * one_minus_t * t

    return term1 + term2


def adaptive_simpson(
    f: Callable[[float], float],
    a: float,
    b: float,
    eps: float,
    whole: float,
    fa: float,
    fb: float,
    fc: float,
    depth: int = 0,
    max_depth: int = 25,
) -> float:
    """Adaptive Simpson's rule integration.

    This mirrors the Ruby implementation. The `whole` parameter is not used as an
    externally supplied integral; it is computed internally and threaded through
    recursive calls to match the original structure.
    """

    if depth > max_depth:
        return whole

    c = (a + b) / 2.0
    h = b - a
    d = (a + c) / 2.0
    e = (c + b) / 2.0

    fd = f(d)
    fe = f(e)

    left = (h / 6.0) * (fa + 4.0 * fc + fb)

    right_left = (h / 12.0) * (fa + 4.0 * fd + fc)
    right_right = (h / 12.0) * (fc + 4.0 * fe + fb)
    right = right_left + right_right

    if abs(right - left) < 15.0 * eps:
        return right + (right - left) / 15.0

    return (
        adaptive_simpson(
            f,
            a,
            c,
            eps / 2.0,
            right_left,
            fa,
            fc,
            fd,
            depth + 1,
            max_depth,
        )
        + adaptive_simpson(
            f,
            c,
            b,
            eps / 2.0,
            right_right,
            fc,
            fb,
            fe,
            depth + 1,
            max_depth,
        )
    )


def compute_area(v: float, eps: float = 1e-16) -> float:
    """Compute the signed area under the Bézier curve for parameter v.

    The integral is ∫ x(t) * y'(t) dt over t in [0, 1].
    """

    def integrand(t: float) -> float:
        x = bezier_point(t, v).x
        y_prime = bezier_derivative_y(t, v)
        return x * y_prime

    fa = integrand(0.0)
    fb = integrand(1.0)
    fc = integrand(0.5)

    return adaptive_simpson(integrand, 0.0, 1.0, eps, 0.0, fa, fb, fc)


def arc_length(v: float, eps: float = 1e-16) -> float:
    """Compute the arc length of the Bézier curve for parameter v."""

    def speed(t: float) -> float:
        x_prime = bezier_derivative_x(t, v)
        y_prime = bezier_derivative_y(t, v)
        return sqrt(x_prime * x_prime + y_prime * y_prime)

    fa = speed(0.0)
    fb = speed(1.0)
    fc = speed(0.5)

    return adaptive_simpson(speed, 0.0, 1.0, eps, 0.0, fa, fb, fc)


def find_v(
    eps_v: float = 1e-15,
    max_iter: int = 100,
    low: float = 0.1,
    high: float = 2.0,
) -> float:
    """Find v such that the enclosed area equals pi/4 using binary search."""

    target_area = PI / 4.0

    prev_mid: float | None = None
    convergence_threshold = 1e-13

    for _ in range(max_iter):
        mid = (low + high) / 2.0
        area = compute_area(mid)

        if abs(area - target_area) < eps_v:
            return mid

        if prev_mid is not None and abs(mid - prev_mid) < convergence_threshold:
            # Mirror the Ruby warning behavior but keep library usage quiet.
            return mid

        prev_mid = mid

        if area < target_area:
            low = mid
        else:
            high = mid

    # If we reach here, return the last midpoint as best-effort approximation.
    return (low + high) / 2.0


def percentage_difference(v: float | None = None) -> float:
    """Compute the percentage difference in curve length vs. quarter circle.

    If v is not provided, it is solved for first.
    """

    if v is None:
        v = find_v()

    L = arc_length(v)
    quarter_arc_length = PI / 2.0
    return 100.0 * (L - quarter_arc_length) / quarter_arc_length


def main() -> None:
    """Solve the problem and print only the final numeric answer."""

    v = find_v()
    pct = percentage_difference(v)
    print(f"{pct:.10f}")


if __name__ == "__main__":  # pragma: no cover - script entry
    main()
