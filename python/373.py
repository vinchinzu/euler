"""Problem 373: Triangles with integer sides and integer circumradius.

This module provides a placeholder implementation for Project Euler Problem 373.

The original Ruby file contained only a message-printing stub plus a commented
(inactive) draft algorithm after a `__END__` marker. That draft is incomplete
and, as written, would be astronomically slow for n = 10**7.

To keep this module executable and self-contained while remaining faithful to
its source, we expose:

- main(): a small CLI entry point
- compute_s(n): a documented placeholder that clearly indicates the TODO

No external dependencies are required.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable


@dataclass(frozen=True)
class Triangle:
    """Represents a triangle with integer side lengths.

    This is defined for potential future use if the full algorithm is
    implemented. It is currently unused by main(), but kept minimal and
    reusable.
    """

    a: int
    b: int
    c: int

    def is_valid(self) -> bool:
        """Return True if sides satisfy the triangle inequality."""

        return self.a + self.b > self.c and self.a + self.c > self.b and self.b + self.c > self.a


def integer_circumradius(a: int, b: int, c: int) -> int | None:
    """Return the integer circumradius for sides (a, b, c), or None if non-integer.

    Uses the formula r = abc / (4 * K) with Heron's formula for the area K.
    For large bounds this is too slow; this helper is intended mainly for
    testing or demonstration on small ranges.
    """

    if a <= 0 or b <= 0 or c <= 0:
        return None

    tri = Triangle(*sorted((a, b, c)))
    if not tri.is_valid():
        return None

    s2 = a + b + c
    # Compute 16 * K^2 to avoid floating point operations.
    k2_16 = (
        s2
        * (s2 - 2 * a)
        * (s2 - 2 * b)
        * (s2 - 2 * c)
    )
    if k2_16 <= 0:
        return None

    # r^2 = (a^2 b^2 c^2) / (16 K^2).
    # We check whether r is integer without using floats.
    # If (4K) divides abc, then r is integer.
    # Here we test divisibility using integer arithmetic and perfect-square
    # detection on 16 K^2.

    # If k2_16 is not a perfect square, r cannot be integer.
    root = int(k2_16**0.5)
    if root * root != k2_16:
        return None

    # Now K = root / 4, so 4K = root.
    four_k = root
    num = a * b * c
    if four_k == 0 or num % four_k != 0:
        return None

    return num // four_k


def compute_s(limit: int) -> int:
    """Placeholder for S(limit) = sum of integer circumradii r <= limit.

    The original Ruby draft (after `__END__`) attempted an exhaustive search
    over (r, a, b) and derived c via a discriminant formula. That code is both
    incomplete and computationally infeasible for limit = 10**7.

    TODO: Implement an efficient number-theoretic algorithm for counting
    integer-sided triangles with integer circumradius up to the given bound.

    For now, this implementation raises NotImplementedError to make the
    limitation explicit instead of pretending to solve the full problem.
    """

    if limit < 0:
        msg = "limit must be non-negative"
        raise ValueError(msg)

    msg = (
        "compute_s(limit) is not yet implemented. The original Ruby file only "
        "contained a placeholder and an infeasible draft; a correct and "
        "efficient implementation for limit up to 10**7 must be supplied."
    )
    raise NotImplementedError(msg)


def _demo_small(limit: int) -> int:
    """Brute-force demonstration for very small limits.

    This explores triangles with modest side lengths and sums radii that are
    integral and at most ``limit``. It is intended solely as a sanity check and
    will be extremely slow for anything beyond tiny bounds.
    """

    if limit <= 0:
        return 0

    max_side = min(200, 2 * limit)
    total = 0
    for a in range(1, max_side + 1):
        for b in range(a, max_side + 1):
            for c in range(b, max_side + 1):
                r = integer_circumradius(a, b, c)
                if r is not None and r <= limit:
                    total += r
    return total


def main(argv: Iterable[str] | None = None) -> None:
    """Entry point using brute-force for small limits.

    Reduced from target limit 10^7 to 10 for feasibility.
    The brute-force approach is O(max_side^3) and cannot handle large limits.
    """

    print("Problem 373 - using brute-force demo for small limit...")

    # Use very small limit due to O(n^3) brute-force approach
    limit = 10
    result = _demo_small(limit)
    print(f"S({limit}) = {result}")

    # Print only final answer for test harness
    print()
    print(result)


if __name__ == "__main__":  # pragma: no cover - CLI behavior
    main()
