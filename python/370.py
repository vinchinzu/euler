"""Project Euler Problem 370 - Geometric Triangles

This module counts integer-sided triangles whose side lengths form a geometric
progression ("geometric triangles") with perimeter up to a given limit.

The original Ruby implementation used parameters ``p`` and ``q`` to generate
candidate triangles and logged progress to standard output. This Python 3.12
port preserves the algorithm while providing a more focused and typed API.

Public API:
- count_geometric_triangles(limit): return the number of geometric triangles
  with perimeter <= limit.
- run_tests(): run built-in validation tests and print their results.

The module can be executed directly. When run as ``__main__``, it:
- runs the small validation test suite; then
- performs the main computation for LIMIT = 25_000_000_000_000 and reports the
  result and timing.
"""

from __future__ import annotations

from dataclasses import dataclass
from time import perf_counter


LIMIT: int = 25_000_000_000_000  # 2.5 × 10^13
PROGRESS_INTERVAL: int = 1000  # Log progress every 1000 q values


@dataclass(slots=True, frozen=True)
class TriangleCounts:
    """Result container for geometric triangle counting.

    Attributes:
        triangles: Total number of geometric triangles found.
        valid_pairs: Number of (p, q) pairs that contributed at least one
            triangle.
    """

    triangles: int
    valid_pairs: int


def _coprime(p: int, q: int) -> bool:
    """Return True if ``p`` and ``q`` are coprime.

    ``q == 1`` is treated as trivially coprime to avoid an unnecessary gcd call.
    """

    if q == 1:
        return True
    # Python's math.gcd is efficient; no need for a custom implementation.
    from math import gcd

    return gcd(p, q) == 1


def _valid_triangle_inequality(p: int, q: int) -> bool:
    """Return True if the (p, q)-generated triple can satisfy triangle rules.

    Mirrors the Ruby check ``q * (q + p) > p * p`` which ensures that, for fixed
    ``q``, larger ``p`` values would also fail, allowing an early break.
    """

    return q * (q + p) > p * p


def _max_q(limit: int) -> int:
    """Compute the maximum q value to consider for a given limit.

    The original Ruby code used ``sqrt(limit) + 1``; we keep the same logic.
    """

    from math import isqrt

    return isqrt(limit) + 1


def count_geometric_triangles(limit: int, *, verbose: bool = True) -> int:
    """Count geometric triangles with perimeter <= ``limit``.

    A geometric triangle is an integer-sided triangle (a <= b <= c) whose sides
    form a geometric sequence: b^2 = a * c.

    Args:
        limit: Maximum allowed perimeter.
        verbose: If True, print progress information to stdout.

    Returns:
        The number of geometric triangles with perimeter not exceeding ``limit``.
    """

    if limit < 3:
        return 0

    from math import isqrt

    total = 0
    valid_pairs = 0
    max_q = _max_q(limit)

    if verbose:
        formatted_limit = f"{limit:,}"
        print(
            "Counting geometric triangles with perimeter c= "
            f"{formatted_limit}..."
        )
        print(f"Searching q from 1 to {max_q}...")

    upper_q = min(max_q, limit)

    for q in range(1, upper_q + 1):
        # Rough upper bound for p from the Ruby implementation.
        # If ``limit - 2 * q * q`` becomes negative, no valid p exist for this q.
        remaining = limit - 2 * q * q
        if remaining < 0:
            break

        max_p_for_perimeter = isqrt(remaining) + q

        for p in range(q, max_p_for_perimeter + 1):
            if not _coprime(p, q):
                continue

            if not _valid_triangle_inequality(p, q):
                # For fixed q, larger p will also fail the inequality.
                break

            denominator = p * p + p * q + q * q
            if denominator > limit:
                continue

            m_max = limit // denominator
            if m_max >= 1:
                total += m_max
                valid_pairs += 1

        if verbose and q % PROGRESS_INTERVAL == 0:
            percent = 100.0 * q / max_q
            print(
                f"Processed q = {q}/{max_q} "
                f"({percent:.1f}%) - Found {valid_pairs} valid (p,q) pairs so far"
            )

    if verbose:
        print(
            "Completed! Found "
            f"{valid_pairs} valid (p,q) pairs yielding {total} geometric triangles."
        )

    return total


def run_tests() -> None:
    """Run basic validation tests against small limits.

    Prints the results to stdout. Intended for quick regression checks.
    """

    print("\n=== VALIDATION TESTS ===")

    test_cases: list[tuple[int, int]] = [
        (2, 0),  # Below minimum perimeter
        (3, 1),  # Just (1,1,1)
        (6, 2),  # (1,1,1), (2,2,2)
        (19, 7),  # Equilaterals up to (6,6,6) + (4,6,9)
        (100, 56),  # More comprehensive test
    ]

    for limit, expected in test_cases:
        result = count_geometric_triangles(limit, verbose=False)
        status = "PASS" if result == expected else "FAIL"
        print(f"Test limit={limit}, got={result} [{status}]")


def _main() -> None:
    """Entry point when executing this module as a script."""

    run_tests()

    print("\n=== MAIN COMPUTATION ===")
    start = perf_counter()

    result = count_geometric_triangles(LIMIT)

    duration = perf_counter() - start

    print("\n=== RESULTS ===")
    print(
        "Number of geometric triangles with perimeter cb= "
        f"{LIMIT:,}"
    )
    print(f"Computation completed in {duration:.2f} seconds")
    print(f"Answer: {result}")


if __name__ == "__main__":
    _main()
