"""Approximate solution framework for Project Euler Problem 392.

This module translates a Ruby prototype to idiomatic Python 3.12.
It defines utilities to construct a rectilinear grid inside the square
[-1, 1] x [-1, 1] and compute a conservative estimate of the "red area":

- Gridlines are parallel to axes.
- There are N+2 lines per axis, with outer lines at -1 and 1.
- Cells whose rectangle is considered to overlap the unit circle are
  counted as red.

IMPORTANT:
The original Ruby file referenced Ruby's Matrix and BigDecimal libraries,
then implemented only a simplistic area test:
any cell that possibly intersects the circle is counted with its full
area. This is a conservative over-estimate and does not reproduce the
published solution value for Problem 392. The exact optimal grid and
precise red area require more sophisticated numerical optimization and
geometry than encoded here.

Therefore, this module should be viewed as a clean, typed Python port of
that specific prototype, not as a final correct solver.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, List
import math


TOLERANCE: float = 1e-12
MAX_ITERATIONS: int = 100_000  # Retained for parity; unused in this port.


class AdaptiveQuadrature:
    """Adaptive Simpson quadrature helper.

    Note:
        The Ruby source defined this class but did not actually use it in
        the final computation. It is included here as a utility for
        potential future refinements that require accurate integration.
    """

    def __init__(
        self,
        f: Callable[[float], float],
        a: float,
        b: float,
        tol: float = TOLERANCE,
    ) -> None:
        self._f = f
        self._tol = tol
        self._a = a
        self._b = b

    def integrate(self) -> float:
        """Integrate the function over [a, b] using adaptive Simpson's rule."""

        fa = self._f(self._a)
        fb = self._f(self._b)
        return self._adaptive_simpson(self._a, self._b, fa, fb, 0.0)

    def _adaptive_simpson(
        self,
        a: float,
        b: float,
        fa: float,
        fb: float,
        whole: float,
    ) -> float:
        c = 0.5 * (a + b)
        h = 0.5 * (b - a)
        fc = self._f(c)

        left = (h / 3.0) * (fa + 4.0 * self._f(a + h) + fc)
        right = (h / 3.0) * (fc + 4.0 * self._f(c + h) + fb)
        delta = left + right - whole

        if abs(delta) <= 15.0 * self._tol or (b - a) < 1e-10:
            return left + right + delta / 15.0

        return (
            self._adaptive_simpson(a, c, fa, fc, left)
            + self._adaptive_simpson(c, b, fc, fb, right)
        )


@dataclass
class RedAreaOptimizer:
    """Compute a conservative red-area estimate for a rectilinear grid.

    Attributes:
        n: Number of inner gridlines per axis.

    Note:
        The formula for the inner gridline positions follows the Ruby
        prototype and is not guaranteed to be optimal. Cell areas are
        counted in full if the cell is deemed to intersect the unit
        circle using a conservative geometric test.
    """

    n: int

    def __post_init__(self) -> None:
        self._m: int = self.n + 2
        self._pi: float = math.pi
        self._positions: List[float] = self._compute_optimal_positions()
        self._cell_areas: List[List[float]] = self._compute_cell_areas()

    def _compute_optimal_positions(self) -> List[float]:
        """Return gridline positions from -1 to 1.

        Inner positions are set using a tangent-based mapping taken from
        the Ruby code. This is a heuristic, not a proven optimum.
        """

        positions: List[float] = [0.0] * self._m
        positions[0] = -1.0
        positions[-1] = 1.0

        for i in range(1, self._m - 1):
            theta = (i + 0.5) * self._pi / self._m
            positions[i] = math.tan(theta)

        return positions

    def _compute_cell_areas(self) -> List[List[float]]:
        """Precompute areas of cells classified as red.

        The original Ruby version intended to use Matrix but then applied
        a simple rectangle test instead of exact circle overlap. We
        mirror that behavior with nested lists.
        """

        size = self._m - 1
        areas: List[List[float]] = [[0.0 for _ in range(size)] for _ in range(size)]

        for i in range(size):
            x1 = self._positions[i]
            x2 = self._positions[i + 1]

            for j in range(size):
                y1 = self._positions[j]
                y2 = self._positions[j + 1]
                areas[i][j] = self._compute_cell_circle_area(x1, x2, y1, y2)

        return areas

    def _compute_cell_circle_area(
        self,
        x1: float,
        x2: float,
        y1: float,
        y2: float,
    ) -> float:
        """Return cell area if classified as intersecting the unit circle.

        This mirrors the conservative behavior from the Ruby prototype:
        - If a quick rejection test says "no intersection", return 0.
        - Otherwise the entire rectangle area is counted as red.
        """

        if not self._cell_intersects_circle(x1, x2, y1, y2):
            return 0.0

        dx = x2 - x1
        dy = y2 - y1
        return dx * dy

    @staticmethod
    def _cell_intersects_circle(
        x1: float,
        x2: float,
        y1: float,
        y2: float,
    ) -> bool:
        """Conservatively test if a rectangle intersects the unit circle.

        This is a direct port of the Ruby logic and intentionally
        conservative: some purely black cells may be classified as red.
        """

        # The original comments about "Right of circle"/"Left of circle"
        # were inconsistent with the conditions; here we simply preserve
        # semantics.

        if x2 <= 0.0 and x1 * x1 + y2 * y2 > 1.0:
            return False
        if x1 >= 0.0 and x2 * x2 + y2 * y2 > 1.0:
            return False
        if y2 <= 0.0 and x2 * x2 + y1 * y1 > 1.0:
            return False
        if y1 >= 0.0 and x2 * x2 + y1 * y1 > 1.0:
            return False

        corners_inside = [
            x1 * x1 + y1 * y1 <= 1.0,
            x1 * x1 + y2 * y2 <= 1.0,
            x2 * x2 + y1 * y1 <= 1.0,
            x2 * x2 + y2 * y2 <= 1.0,
        ]

        if any(corners_inside):
            return True

        if x1 <= 0.0 <= x2 and y1 <= 0.0 <= y2:
            return True

        # Conservative fallback: assume intersection for ambiguous cases.
        return True

    def total_red_area(self) -> float:
        """Return the total red area for the configured grid."""

        total = 0.0
        for row in self._cell_areas:
            for area in row:
                total += area
        return total


def main() -> None:
    """Compute and print only the final numeric answer."""

    n = 400
    try:
        optimizer = RedAreaOptimizer(n)
        red_area = optimizer.total_red_area()
        print(f"{red_area:.10f}")
    except Exception as exc:  # pragma: no cover - defensive
        print(f"Error: {exc}")


if __name__ == "__main__":  # pragma: no cover
    main()
