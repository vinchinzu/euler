"""Project Euler Problem 667: Moving Pentagon.

Find the maximum area of a pentagon with equal sides that can be pushed through
an L-shaped corridor of width 1.

We use ternary search over the angle ratio α/β to find the maximum area.
"""

from __future__ import annotations

import math
from dataclasses import dataclass


@dataclass
class Sofa:
    """Sofa configuration."""

    a: float
    b: float

    def __init__(self, a: float) -> None:
        """Initialize sofa with angle a."""
        self.a = a
        self.b = math.acos(1 / (4 * math.cos(a)))


def ternary_search(
    left: float, right: float, f: callable, eps: float = 1e-10
) -> tuple[float, float]:
    """Ternary search for maximum."""
    while right - left > eps:
        m1 = left + (right - left) / 3
        m2 = right - (right - left) / 3
        f1 = f(m1)
        f2 = f(m2)
        if isinstance(f1, tuple):
            f1_val = f1[1]
        else:
            f1_val = f1
        if isinstance(f2, tuple):
            f2_val = f2[1]
        else:
            f2_val = f2
        if f1_val < f2_val:
            left = m1
        else:
            right = m2
    return (left + right) / 2, f((left + right) / 2)


def max_area(sofa: Sofa) -> float:
    """Compute maximum area for sofa configuration."""
    start_ratio = (
        1.0 / 2 * math.tan(sofa.a) + math.sin(sofa.b - sofa.a)
    )
    middle_ratio_func = lambda t: corridor_ratio_at(sofa, t)
    middle_ratio_result = ternary_search(0, math.pi / 2, middle_ratio_func)
    if isinstance(middle_ratio_result, tuple):
        middle_ratio = middle_ratio_result[1]
    else:
        middle_ratio = middle_ratio_result

    s = 1 / max(start_ratio, middle_ratio)
    area = (
        s * s
        * (
            math.tan(sofa.a) / 4
            + math.tan(sofa.b) / (8 * (math.cos(sofa.a) ** 2))
        )
    )
    return area


def corridor_ratio_at(sofa: Sofa, t: float) -> float:
    """Compute corridor ratio at angle t."""
    Dx = math.cos(sofa.a + sofa.b - t)
    Dy = math.sin(t) + math.sin(sofa.a + sofa.b - t)
    Ox = math.cos(sofa.a - t) / (2 * math.cos(sofa.a))
    Oy = math.sin(sofa.a + t) / (2 * math.cos(sofa.a))
    return (Oy / (Dy - Oy) - Ox / (Dx - Ox)) / (
        1 / (Dy - Oy) - 1 / (Dx - Ox)
    )


def solve() -> float:
    """Solve Problem 667."""
    result = ternary_search(0, math.pi / 3, lambda a: max_area(Sofa(a)))
    if isinstance(result, tuple):
        return result[1]
    return result


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()
