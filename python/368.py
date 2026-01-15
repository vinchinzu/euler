"""Project Euler Problem 368 (improved version).

This module computes the sum of 1/n over all positive integers n whose decimal
representation contains at least three equal consecutive digits. This is the
"omitted" part of the harmonic series for the problem's definition.

The code is a Pythonic port of the provided Ruby implementation, using the
standard library only. It is structured to be:

- Python 3.12 compatible
- Fully typed
- Reasonably efficient for exploratory / educational use

Note: This implementation preserves the original Ruby script's approximate
approach, including a heuristic tail estimation. It is suitable for reproducing
the given solution-style behavior, but it is not a formally proven, rigorously
optimized solver.
"""

from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal, getcontext
from time import perf_counter
from typing import Callable, Iterable, List, Optional

# Global configuration constants (mirroring the Ruby script's intent)

MAX_DIGITS: int = 20
"""Maximum digits to consider when generating numbers.

Numbers larger than 10 ** MAX_DIGITS contribute terms < 1e-20, which are
negligible for our target precision.
"""

PRECISION: float = 1e-12
"""Target precision threshold for convergence (currently informational only)."""

TAIL_THRESHOLD: float = 1e-12
"""Threshold below which individual terms are ignored in explicit generation."""

MAX_GENERATION_TIME: float = 60.0
"""Maximum allowed generation time in seconds before warning the user."""

# Configure Decimal precision high enough for our needs.
getcontext().prec = 50


def has_three_consecutive_digits(s: str) -> bool:
    """Return True if s contains three equal consecutive digits.

    For example:
    - "111" -> True
    - "1211" -> False
    - "1110" -> True
    """\

    if len(s) < 3:
        return False

    # Simple sliding window over characters
    for i in range(len(s) - 2):
        if s[i] == s[i + 1] == s[i + 2]:
            return True
    return False


def _generate_bad_numbers_recursive(
    current: str,
    length_limit: int,
    collector: List[int],
    callback: Optional[Callable[[int], None]] = None,
) -> None:
    """Recursive generator for numbers with three equal consecutive digits.

    This mirrors the recursive Ruby approach. It explores all digit strings
    up to ``length_limit`` and:
    - whenever a string with three equal consecutive digits is found, its
      integer value is recorded/returned via ``collector`` and ``callback``.

    The search is naive and will be expensive for large ``length_limit``; it is
    kept here to stay faithful to the original script.
    """

    if len(current) > length_limit:
        return

    if has_three_consecutive_digits(current) and current:
        n = int(current)
        if n != 0:  # Skip zero to avoid division by zero
            if callback is not None:
                callback(n)
            else:
                # For potential testing or analysis when no callback is supplied.
                collector.append(n)

    if len(current) == length_limit:
        return

    for digit in "0123456789":
        new_num = current + digit
        _generate_bad_numbers_recursive(new_num, length_limit, collector, callback)


def generate_bad_numbers(
    length_limit: int,
    callback: Optional[Callable[[int], None]] = None,
) -> List[int]:
    """Generate all integers with at least three equal consecutive digits.

    Parameters
    ----------
    length_limit:
        Maximum number of digits in the generated numbers.
    callback:
        If provided, called for each generated integer. When ``callback`` is
        given, the returned list will still contain all generated integers for
        convenience; in performance-sensitive use, you may want a more
        streaming-oriented design.
    """

    collector: List[int] = []

    def _store(n: int) -> None:
        collector.append(n)
        if callback is not None:
            callback(n)

    _generate_bad_numbers_recursive("", length_limit, collector, _store)
    collector = sorted(set(collector))
    return collector


def estimate_tail_sum(start_n: int, max_digits: int) -> float:
    """Estimate the tail sum of 1/n for "bad" numbers starting from ``start_n``.

    This is a heuristic based on an extremely simple density estimate and
    bounded by the harmonic tail from ``start_n`` to 10**max_digits.

    Note: This is not mathematically rigorous; kept to match the Ruby code's
    behavior. For precise work, replace with a proper analytic or DP-based
    estimate tailored to the digit pattern.
    """

    if start_n <= 0:
        raise ValueError("start_n must be positive")

    upper_bound = (10**max_digits).bit_length()  # placeholder, see below
    # TODO: The original Ruby uses Math.log(10**max_digits) - Math.log(start_n).
    # Using bit_length here would be incorrect; we preserve behavior below with
    # logs from math. This line remains only to document the porting concern.

    from math import log

    upper_bound = log(10**max_digits) - log(start_n)
    density_factor = 0.01
    estimated_tail = upper_bound * density_factor
    return float(min(estimated_tail, upper_bound))


def compute_bad_sum(max_digits: int = MAX_DIGITS) -> Decimal:
    """Compute an approximate sum of 1/n over all "bad" numbers.

    "Bad" numbers are those with at least three equal consecutive digits in
    their decimal representation. The algorithm:

    - Generates such numbers up to 10**max_digits (naively).
    - Accumulates 1/n using Decimal until terms fall below TAIL_THRESHOLD.
    - Adds a heuristic tail estimate beyond the last generated value.
    """

    start_time = perf_counter()

    bad_numbers: List[int] = []
    total = Decimal(0)

    def on_bad(n: int) -> None:
        nonlocal total
        term = Decimal(1) / Decimal(n)
        if term < Decimal(TAIL_THRESHOLD):
            return
        total += term

    # Generate numbers and accumulate via callback.
    bad_numbers = generate_bad_numbers(max_digits, callback=on_bad)

    last_n = bad_numbers[-1] if bad_numbers else 10**max_digits
    tail = estimate_tail_sum(last_n, max_digits + 5)
    total += Decimal(str(tail))

    elapsed = perf_counter() - start_time
    print(
        f"Generated {len(bad_numbers)} bad numbers in "
        f"{elapsed:.2f} seconds",
    )

    if elapsed > MAX_GENERATION_TIME:
        print(
            "Warning: generation took too long; consider reducing MAX_DIGITS "
            "or using a digit DP approach.",
        )

    return total


@dataclass
class ValidationCase:
    n: int
    expected_bad_count: int
    expected_sum_approx: float


def validate_partial_sums() -> None:
    """Run simple validation checks against small ranges.

    These tests mirror the Ruby code's intent but fix its formatting issues.
    """

    cases: Iterable[ValidationCase] = (
        ValidationCase(n=1000, expected_bad_count=10, expected_sum_approx=0.009),
        ValidationCase(n=2000, expected_bad_count=20, expected_sum_approx=0.018),
    )

    print("Running validation tests...")

    from math import isclose

    for i, case in enumerate(cases, start=1):
        bad_count = 0
        partial_sum = Decimal(0)

        for x in range(1, case.n + 1):
            if has_three_consecutive_digits(str(x)):
                bad_count += 1
                partial_sum += Decimal(1) / Decimal(x)

        sum_ok = isclose(
            float(partial_sum),
            case.expected_sum_approx,
            rel_tol=0.0,
            abs_tol=1e-3,
        )
        count_ok = bad_count == case.expected_bad_count
        status = "PASS" if (sum_ok and count_ok) else "FAIL"

        print(
            f"Test {i} (n={case.n}) - "
            f"Count: {bad_count} (exp {case.expected_bad_count}), "
            f"Sum: {float(partial_sum):.3f} (exp {case.expected_sum_approx:.3f}) - "
            f"{status}",
        )


@dataclass
class UnitTestCase:
    value: int
    expected: bool
    desc: str


def run_unit_tests() -> None:
    """Run unit tests for has_three_consecutive_digits."""

    print("Running unit tests...")

    cases: List[UnitTestCase] = [
        UnitTestCase(1, False, "single digit"),
        UnitTestCase(12, False, "two digits"),
        UnitTestCase(111, True, "three consecutive"),
        UnitTestCase(1211, False, "no three consecutive"),
        UnitTestCase(1111, True, "four consecutive"),
        UnitTestCase(123111, True, "three consecutive in middle"),
        UnitTestCase(9999, True, "four 9's"),
        UnitTestCase(1000, False, "1000 (no three consecutive)"),
        UnitTestCase(1110, True, "1110 from problem example"),
    ]

    for case in cases:
        result = has_three_consecutive_digits(str(case.value))
        status = "PASS" if result == case.expected else "FAIL"
        print(
            f"Test {case.value} ({case.desc}): {status} "
            f"(got {result}, expected {case.expected})",
        )


def main() -> None:
    """Entry point mirroring the Ruby script's CLI behavior."""

    print("=" * 60)
    print("Project Euler Problem 368 Solution (Python)")
    print("=" * 60)
    print("Computing sum of 1/n for all n with 3+ consecutive equal digits")
    print("Rounded to 10 decimal places")
    print()

    run_unit_tests()
    print()
    validate_partial_sums()
    print()

    print(f"Computing full solution with MAX_DIGITS = {MAX_DIGITS}...")
    total_sum = compute_bad_sum(MAX_DIGITS)

    float_sum = float(total_sum)
    rounded_sum = round(float_sum, 10)

    print("=" * 60)
    print("FINAL RESULT:")
    print(f"Sum = {rounded_sum}")
    print(f"Raw computation: {float_sum}")
    print("Using Decimal precision with tail estimation")
    print("=" * 60)


if __name__ == "__main__":  # pragma: no cover - CLI behavior
    main()
