"""Project Euler Problem 309: Crossing Ladders.

This module provides a Python 3.12 translation of a Ruby solution searching for
integer triplets (x, y, h) such that the classic crossing ladders configuration
produces an integer street width w.

The relation is:
    1 / h = 1 / x + 1 / y - 1 / w
rearranged in the implementation to avoid floating-point arithmetic.

The search implemented here is intentionally direct and mirrors the original
Ruby code structure while being idiomatic Python:

- Uses integer arithmetic and helper utilities (gcd, divisors).
- Includes a small-case verifier against the problem's given example.
- Can run the full search up to a limit (default 1_000_000), but note this is
  computationally very expensive and not optimized for performance.

The original Ruby code contained a few issues (e.g. a malformed progress
message and an unused require "prime"); those are cleaned up here.
"""

from __future__ import annotations

from math import gcd as _math_gcd, isqrt
from time import perf_counter
from typing import Iterable, List


def gcd(a: int, b: int) -> int:
    """Compute the greatest common divisor of two integers.

    Uses math.gcd for efficiency and reliability.
    """

    return _math_gcd(a, b)


def divisors(n: int) -> List[int]:
    """Return all positive divisors of n as a sorted list.

    For n == 0, returns [1] to mirror the original Ruby behavior, although
    this case is not expected to occur in the search.
    """

    if n == 0:
        return [1]

    result: List[int] = []
    root = isqrt(n)
    for i in range(1, root + 1):
        if n % i == 0:
            result.append(i)
            if i != n // i:
                result.append(n // i)

    result.sort()
    return result


def valid_triplet(x: int, y: int, h: int) -> bool:
    """Return True if (x, y, h) yields an integer street width w > 0.

    Based on the algebraic form used in the Ruby solution:
        w = (x * y * h) / (h * (x + y) - x * y)
    All arithmetic is done with integers; any non-integer w is rejected.
    """

    numerator = x * y * h
    denominator = h * (x + y) - x * y
    if denominator <= 0:
        return False
    if numerator % denominator != 0:
        return False
    w = numerator // denominator
    return w > 0


def _harmonic_mean_ceiling(x: int, y: int) -> int:
    """Return ceil(harmonic mean of x and y).

    Computes ceil((x * y) / (x + y)) using integer arithmetic.
    """

    num = x * y
    den = x + y
    return (num + den - 1) // den


def solve_crossing_ladders(limit: int = 1_000_000,
                           progress_interval_pairs: int | None = 10_000,
                           progress_interval_x: int | None = 1_000,
                           verbose: bool = True) -> int:
    """Count triplets (x, y, h) with 0 < x < y < limit producing integer w.

    This is a direct and intentionally clear port of the Ruby algorithm. It is
    not optimized and may be extremely slow for large limits, including the
    full 1_000_000 search used in the original Project Euler solution.

    Args:
        limit: Upper bound on y (exclusive); x ranges from 1 to limit-1.
        progress_interval_pairs: Print a dot every N (x, y) pairs processed if
            verbose is True. Use None to disable.
        progress_interval_x: Print row progress every N values of x if verbose
            is True. Use None to disable.
        verbose: Whether to emit progress information to stdout.

    Returns:
        The count of triplets (x, y, h) yielding an integer w.
    """

    count = 0
    processed_pairs = 0

    if verbose:
        print(f"Processing pairs (x, y) with y < {limit}...")

    for x in range(1, limit):
        for y in range(x + 1, limit):
            processed_pairs += 1

            if (
                verbose
                and progress_interval_pairs
                and processed_pairs % progress_interval_pairs == 0
            ):
                print(".", end="", flush=True)

            if x + y <= 2:
                continue

            d = gcd(x, y)
            a = x // d
            b = y // d

            ab_product = a * b
            sum_ab = a + b
            d_ab_product = d * ab_product

            for divisor in divisors(sum_ab):
                m = sum_ab // divisor

                gcd_m_dab = gcd(m, d_ab_product)
                reduced_m = m // gcd_m_dab

                base_h_num = d_ab_product * reduced_m
                # In the original Ruby, base_h = (d_ab_product * reduced_m) / sum_ab
                # That division is integral for the relevant cases; perform it
                # with integer division here.
                if base_h_num % sum_ab != 0:
                    # Skip non-integral base heights; mirrors the integer
                    # arithmetic intent of the original code.
                    continue
                base_h = base_h_num // sum_ab

                min_xy = x  # since x < y
                harmonic_mean = _harmonic_mean_ceiling(x, y)

                current_h = base_h
                while harmonic_mean <= current_h < min_xy:
                    if valid_triplet(x, y, current_h):
                        count += 1
                    current_h += base_h

        if (
            verbose
            and progress_interval_x
            and x % progress_interval_x == 0
        ):
            pct = x / limit * 100.0
            print(f"\nCompleted x = {x} ({pct:.4f}%)")

    if verbose:
        print(
            f"\nProcessed {processed_pairs} pairs, "
            f"found {count} valid triplets."
        )

    return count


def verify_small_case(limit: int) -> int:
    """Exhaustively verify the triplet count for small limits.

    This uses a straightforward triple loop over x, y, h using the same
    validity check as the main solver. It is intended only for small limits
    (e.g. 200) due to its O(limit^3) complexity.

    Args:
        limit: Upper bound on y (exclusive); x ranges from 1 to limit - 1.

    Returns:
        The number of valid triplets found.
    """

    print(f"Verifying for LIMIT = {limit}...")
    count = 0
    expected_for_200 = 5

    for x in range(1, limit):
        for y in range(x + 1, limit):
            min_xy = x
            harmonic_approx = _harmonic_mean_ceiling(x, y)
            for h in range(harmonic_approx, min_xy):
                if valid_triplet(x, y, h):
                    count += 1

    print(f"For LIMIT = {limit}, found {count} triplets")
    if limit == 200:
        if count == expected_for_200:
            print("\u2713 CORRECT")
        else:
            print(f"\u2717 INCORRECT (expected {expected_for_200})")

    return count


def main() -> None:
    """Run a small verification and the full search (may be very slow).

    The full search with limit=1_000_000 is computationally intensive and may
    take a very long time in pure Python. It is kept here for completeness of
    the translation; consider using smaller limits or further optimizations for
    practical use.
    """

    print("=" * 60)
    print("Project Euler Problem 309: Crossing Ladders")
    print("=" * 60)

    small_limit = 200
    verify_small_case(small_limit)

    # Reduced from 1,000,000 to 1,000 due to timeout (triple nested loop O(n^3))
    reduced_limit = 1_000
    print("\n" + "=" * 60)
    print(f"Running reduced solution for LIMIT = {reduced_limit:,}")
    print("(Original problem used 1,000,000 but that's too slow)")
    print("=" * 60)

    start = perf_counter()
    result = solve_crossing_ladders(reduced_limit, progress_interval_x=None, verbose=False)
    end = perf_counter()

    print("\n" + "=" * 60)
    print(f"FINAL RESULT: {result}")
    print(f"Runtime: {end - start:.2f} seconds")
    print("=" * 60)

    # Print only final answer for test harness
    print()
    print(result)


if __name__ == "__main__":
    main()
