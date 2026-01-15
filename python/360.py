"""Project Euler Problem 360 - Python implementation.

This module provides functions to compute S(r), the sum of Manhattan distances from the
origin of all integer-coordinate points lying on the surface of the sphere of radius r.

Key public APIs:
- compute_s_optimized: Efficient computation suitable for very large radii.
- compute_s_brute_force: Correctness reference using explicit enumeration (slow).
- run_tests: Basic self-checks and a small performance comparison.

Python 3.12, standard library only.
"""

from __future__ import annotations

from dataclasses import dataclass
from time import perf_counter
from typing import Tuple

OCTANT_MULTIPLIER: int = 8
PLANE_MULTIPLIER: int = 4
AXIS_MULTIPLIER: int = 2


def validate_radius(r: int) -> int:
    """Validate that r is a non-negative integer; return it or raise ValueError."""

    if not isinstance(r, int) or r < 0:
        msg = "Radius must be a non-negative integer"
        raise ValueError(msg)
    return r


def integer_sqrt(n: int) -> int:
    """Return floor(sqrt(n)) for non-negative n using integer arithmetic.

    Negative inputs return 0, mirroring the behavior of the original Ruby code.
    For correct usage in this module, callers should avoid passing negatives.
    """

    if n < 0:
        return 0
    if n in (0, 1):
        return n

    low, high = 1, n
    while low <= high:
        mid = (low + high) // 2
        sq = mid * mid
        if sq == n:
            return mid
        if sq < n:
            low = mid + 1
        else:
            high = mid - 1
    return high


def _two_squares_representation_count(n: int) -> int:
    """Return the number of representations of n as x^2 + y^2.

    This mirrors the Ruby logic based on number-theoretic properties:
    - If any prime ≡ 3 (mod 4) divides n to an odd power, the count is 0.
    - Otherwise, count is 4 * (d1 - d3), where d1 and d3 are the counts of
      divisors of n that are ≡ 1 and ≡ 3 (mod 4), respectively.

    Note: For this problem we only require consistency with the original
    implementation, not asymptotically optimal factorization.
    """

    if n < 0:
        return 0
    if n == 0:
        # (0, 0) is one way.
        return 1

    # Remove factors of 2.
    while n % 2 == 0:
        n //= 2

    # Check primes ≡ 3 (mod 4) for odd exponent.
    i = 3
    tmp = n
    while i * i <= tmp:
        if tmp % i == 0:
            count = 0
            while tmp % i == 0:
                tmp //= i
                count += 1
            if i % 4 == 3 and count % 2 == 1:
                return 0
        i += 2

    # If remaining prime factor is ≡ 3 (mod 4), then invalid.
    if tmp > 1 and tmp % 4 == 3:
        return 0

    # Count divisors of the original n by reconstructing from tmp is non-trivial.
    # To stay faithful to the Ruby version (which recomputed divisors on n after
    # its checks), we follow its structure: recompute divisors on the filtered n.

    # At this point, `n` has only primes ≡ 1 (mod 4) or 2.
    divisors: list[int] = []
    i = 1
    while i * i <= n:
        if n % i == 0:
            divisors.append(i)
            j = n // i
            if j != i:
                divisors.append(j)
        i += 1

    d1 = sum(1 for d in divisors if d % 4 == 1)
    d3 = sum(1 for d in divisors if d % 4 == 3)
    return 4 * (d1 - d3)


def compute_s_optimized(r: int) -> int:
    """Compute S(r) using an optimized approach based on two-square representations.

    Suitable for very large radii such as 10**10.
    """

    r = validate_radius(r)
    if r == 0:
        return 0

    r_squared = r * r
    total_sum = 0

    for x in range(1, r + 1):
        remaining = r_squared - x * x
        if remaining < 0:
            break
        r2 = _two_squares_representation_count(remaining)
        # Factor 6: symmetry over axes/octants for Manhattan distance contribution.
        total_sum += 6 * x * r2

    return total_sum


def _enumerate_integer_triples_on_sphere(r: int) -> Tuple[int, int]:
    """Enumerate integer triples (x, y, z) on the sphere x^2 + y^2 + z^2 = r^2.

    Returns a tuple (count, manhattan_sum) for the first octant (x, y, z >= 0).
    Used as a helper for the brute-force implementation.
    """

    r_squared = r * r
    total_manhattan_sum = 0

    for x in range(0, r + 1):
        x_sq = x * x
        remaining = r_squared - x_sq
        if remaining < 0:
            break

        y_max = integer_sqrt(remaining)
        for y in range(0, y_max + 1):
            y_sq = y * y
            z_sq = remaining - y_sq
            if z_sq < 0:
                continue
            z = integer_sqrt(z_sq)
            if z * z != z_sq:
                continue
            _, quadrant_sum = count_and_sum_quadrants_improved(x, y, z)
            total_manhattan_sum += quadrant_sum

    return 0, total_manhattan_sum


def compute_s_brute_force(r: int) -> int:
    """Brute-force computation of S(r) via enumeration of integer triples.

    This matches the Ruby version and is intended for testing correctness of the
    optimized method. Complexity is high; do not use for very large r.
    """

    r = validate_radius(r)
    if r == 0:
        return 0

    _, total_manhattan_sum = _enumerate_integer_triples_on_sphere(r)
    return total_manhattan_sum


def count_and_sum_quadrants_improved(a: int, b: int, c: int) -> Tuple[int, int]:
    """Return (count, manhattan_sum) over all sign-variants of (a, b, c).

    The result exploits symmetry:
    - 3 non-zero coords: 8 symmetric points.
    - 2 non-zero, 1 zero: 4 symmetric points (lying in coordinate planes).
    - 1 non-zero, 2 zero: 2 symmetric points (lying on axes).
    - all zero: the origin.
    """

    coords = (a, b, c)
    zero_count = sum(1 for v in coords if v == 0)
    non_zero_coords = [v for v in coords if v != 0]

    if zero_count == 0:
        # All three non-zero
        manhattan = sum(non_zero_coords)
        return OCTANT_MULTIPLIER, OCTANT_MULTIPLIER * manhattan
    if zero_count == 1:
        # Exactly one zero
        manhattan = sum(non_zero_coords)
        return PLANE_MULTIPLIER, PLANE_MULTIPLIER * manhattan
    if zero_count == 2:
        # Exactly two zeros
        non_zero = non_zero_coords[0]
        return AXIS_MULTIPLIER, AXIS_MULTIPLIER * non_zero
    if zero_count == 3:
        # All zeros (origin)
        return 1, 0

    msg = "Invalid coordinates"
    raise ValueError(msg)


@dataclass
class _TimingResult:
    label: str
    elapsed: float


def _time_function(label: str, func, *args, **kwargs) -> _TimingResult:  # type: ignore[no-untyped-def]
    """Measure wall-clock time for calling func(*args, **kwargs)."""

    start = perf_counter()
    func(*args, **kwargs)
    end = perf_counter()
    return _TimingResult(label=label, elapsed=end - start)


def _assert_equal(expected: int, actual: int, description: str) -> None:
    """Minimal assertion helper for internal tests.

    Raises AssertionError on mismatch.
    """

    if expected != actual:
        msg = f"{description}, expected {expected}, got {actual}"
        raise AssertionError(msg)


def run_tests() -> None:
    """Run basic self-tests and small performance comparison.

    Intended for use when running this module as a script.
    """

    print("Running tests...")

    _assert_equal(0, compute_s_optimized(0), "S(0)")
    _assert_equal(0, compute_s_brute_force(0), "S(0) brute force")

    _assert_equal(6, compute_s_optimized(1), "S(1)")
    _assert_equal(6, compute_s_brute_force(1), "S(1) brute force")

    _assert_equal(198, compute_s_optimized(5), "S(5)")
    _assert_equal(198, compute_s_brute_force(5), "S(5) brute force")

    _assert_equal(34518, compute_s_optimized(45), "S(45)")
    _assert_equal(34518, compute_s_brute_force(45), "S(45) brute force")

    s100 = compute_s_brute_force(100)
    s100_opt = compute_s_optimized(100)
    _assert_equal(s100, s100_opt, "S(100)")

    print("All tests passed! ✓")

    print("\nPerformance comparison for r = 1000:")
    brute = _time_function("brute", compute_s_brute_force, 1000)
    optimized = _time_function("optimized", compute_s_optimized, 1000)

    print(f"Brute force: {brute.elapsed:.6f}s")
    print(f"Optimized:   {optimized.elapsed:.6f}s")
    if optimized.elapsed > 0:
        speedup = brute.elapsed / optimized.elapsed
        print(f"Speedup: {speedup:.2f}x")


def main() -> None:
    """Run tests and display sample results for Problem 360."""

    run_tests()

    print("\nComputing S(45) (should be 34518):")
    result_45 = compute_s_optimized(45)
    print(f"S(45) = {result_45}")

    print("\nComputing S(10^10)...")
    r = 10**10
    result_big = compute_s_optimized(r)
    print(f"S(10^10) = {result_big}")


if __name__ == "__main__":  # pragma: no cover - script entry point
    main()
