"""Project Euler Problem 303 (Improved)

This module computes the sum of f(n)/n for n from 1 to 10_000, where f(n)
is the least positive multiple of n that, in base 10, uses only the digits 0, 1, and 2.

The implementation follows the intent of the provided Ruby solution but adjusts
its state handling to be both correct and efficient in Python.
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass
from typing import Deque, Dict, Iterable, List, Set, Tuple

MAX_NUM: int = 10**30
N_MAX: int = 10_000
ALLOWED_DIGITS: Tuple[int, ...] = (0, 1, 2)


@dataclass(frozen=True)
class State:
    """Represents a BFS state for constructing candidate numbers.

    Attributes:
        remainder: Remainder of the current number modulo n.
    """

    remainder: int


def min_multiple_with_small_digits(n: int, *, max_num: int = MAX_NUM) -> int:
    """Return the smallest positive multiple of ``n`` using only digits 0, 1, and 2.

    The search is performed with a breadth-first search (BFS) over remainders
    modulo ``n``. Only the remainder is tracked for visited states to guarantee
    both correctness and efficiency: once a remainder has been seen, any future
    visit with the same remainder would correspond to a number that is not
    smaller than the one already discovered.

    Args:
        n: Positive integer for which to find the constrained multiple.
        max_num: Hard limit on the value of the constructed number. Used as a
            safety guard to avoid unbounded growth in pathological cases.

    Returns:
        The smallest integer ``m`` such that ``m`` is a positive multiple of ``n``
        and the decimal expansion of ``m`` uses only digits from {0, 1, 2}.

    Raises:
        ValueError: If ``n`` is not positive.
        RuntimeError: If no such multiple is found up to ``max_num``.
    """

    if n <= 0:
        raise ValueError("n must be a positive integer")

    if n == 1:
        # For n = 1, any positive integer works; the smallest valid is 1.
        return 1

    # BFS over (remainder, current_value). Start from digits 1 and 2 (not 0),
    # because the desired multiple must be positive and leading zeros are
    # irrelevant for minimality.
    queue: Deque[Tuple[int, int]] = deque()
    visited_remainders: Set[int] = set()

    for digit in (1, 2):
        remainder = digit % n
        if remainder == 0:
            return digit
        queue.append((remainder, digit))
        visited_remainders.add(remainder)

    while queue:
        remainder, value = queue.popleft()

        for digit in ALLOWED_DIGITS:
            next_value = value * 10 + digit
            if next_value > max_num:
                # Respect the safety limit; skip overly large candidates.
                continue

            next_remainder = (remainder * 10 + digit) % n
            if next_remainder == 0 and next_value > 0:
                return next_value

            if next_remainder not in visited_remainders:
                visited_remainders.add(next_remainder)
                queue.append((next_remainder, next_value))

    raise RuntimeError(f"No solution found for n={n} within limit {max_num}")


def verify_examples() -> None:
    """Verify known example values for ``min_multiple_with_small_digits``.

    Raises ``AssertionError`` if any example does not match its expected value.
    """

    examples: Dict[int, int] = {
        2: 2,
        3: 12,
        7: 21,
        42: 210,
        89: 1_121_222,
    }

    for n, expected in examples.items():
        result = min_multiple_with_small_digits(n)
        if result != expected:
            raise AssertionError(
                f"Example failed for n={n}: got {result}, expected {expected}"
            )


def compute_f_values(limit: int = N_MAX) -> List[int]:
    """Compute f(n) for 1 <= n <= ``limit``.

    Returns a list ``f_values`` of length ``limit + 1`` where
    ``f_values[n] == min_multiple_with_small_digits(n)`` and index 0 is unused.
    """

    if limit < 1:
        raise ValueError("limit must be at least 1")

    f_values: List[int] = [0] * (limit + 1)
    for n in range(1, limit + 1):
        f_values[n] = min_multiple_with_small_digits(n)
    return f_values


def sum_f_over_n(limit: int = N_MAX) -> int:
    """Return sum_{n=1..limit} f(n) / n using integer division.

    The function first computes all f(n) values up to ``limit`` and then sums
    ``f(n) // n``.
    """

    f_values = compute_f_values(limit)
    return sum(f_values[n] // n for n in range(1, limit + 1))


def main() -> None:
    """Run verification for n <= 100 and print the final result for n <= 10_000."""

    verify_examples()

    sum_100 = sum_f_over_n(100)
    expected_100 = 11_363_107
    if sum_100 != expected_100:
        raise AssertionError(
            f"Verification failed: got {sum_100}, expected {expected_100}"
        )

    total_sum = sum_f_over_n(N_MAX)
    print(total_sum)


if __name__ == "__main__":  # pragma: no cover - CLI entry point
    main()
