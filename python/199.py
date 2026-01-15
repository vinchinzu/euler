"""Project Euler Problem 199: Iterative Circle Packing."""

import math

ITERATIONS = 10
RADIUS_LARGE = 1.0

K_LARGE = -1.0
K_SMALL = 1.0 + 2.0 / math.sqrt(3.0)
AREA_LARGE = math.pi * RADIUS_LARGE * RADIUS_LARGE

SMALL_AREA = math.pi * (1.0 / K_SMALL) ** 2


def circle_area(k: float) -> float:
    """Circle area from curvature (positive k only)."""
    if k <= 0.0:
        return 0.0
    r = 1.0 / k
    return math.pi * r * r


def recurse_area(k1: float, k2: float, k3: float, depth: int) -> float:
    """Recursive area contribution for Apollonian gasket."""
    if depth == 0:
        return 0.0

    sum_val = k1 + k2 + k3
    root_term = math.sqrt(k1 * k2 + k1 * k3 + k2 * k3)
    k_new = sum_val + 2.0 * root_term

    area_new = circle_area(k_new)
    return (
        area_new
        + recurse_area(k_new, k1, k2, depth - 1)
        + recurse_area(k_new, k1, k3, depth - 1)
        + recurse_area(k_new, k2, k3, depth - 1)
    )


def main() -> str:
    """Main function."""
    peripheral_extra = 3 * recurse_area(K_LARGE, K_SMALL, K_SMALL, ITERATIONS)
    central_extra = recurse_area(K_SMALL, K_SMALL, K_SMALL, ITERATIONS)
    extra_area = peripheral_extra + central_extra
    total_area = 3 * SMALL_AREA + extra_area
    uncovered_fraction = 1.0 - (total_area / AREA_LARGE)
    return f"{uncovered_fraction:.8f}"


if __name__ == "__main__":
    print(main())
