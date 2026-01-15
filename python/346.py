"""Project Euler Problem 346 - Strong Repunits

This module provides a function to compute the sum of all strong repunits below a
specified limit.

A strong repunit is a positive integer that is a repunit (all digits 1) in at
least two bases greater than 1.

The implementation here is an idiomatic Python 3.12 translation of the given
Ruby solution. It uses base iteration and counts how many bases produce a given
repunit value.
"""

from __future__ import annotations

from collections import defaultdict
from typing import DefaultDict, Dict


DEFAULT_LIMIT: int = 10**12
MAX_BASE: int = 1_000_000


def generate_repunit_counts(limit: int = DEFAULT_LIMIT) -> tuple[Dict[int, int], Dict[int, int | None]]:
    """Generate counts of repunit representations for numbers below ``limit``.

    For each candidate value ``r`` formed as a repunit in base ``b`` (for
    ``b >= 2``), track:

    - how many bases represent ``r`` as a repunit (``count``)
    - a "unique length" when ``r`` is seen in exactly one base so far

    The "unique length" mirrors the original Ruby logic: it records the repunit
    length (number of 1-digits) if and only if ``r`` has been encountered in
    exactly one base. If a second base is found, the entry is cleared (set to
    ``None``).

    Parameters
    ----------
    limit:
        Upper bound (exclusive) for repunit values.

    Returns
    -------
    tuple[dict[int, int], dict[int, int | None]]
        A pair ``(count, unique_length)``.
    """

    count: DefaultDict[int, int] = defaultdict(int)
    unique_length: Dict[int, int | None] = {}

    for base in range(2, MAX_BASE + 1):
        repunit = base + 1
        length = 2

        if repunit >= limit:
            continue

        count[repunit] += 1
        if count[repunit] == 1:
            unique_length[repunit] = length
        elif count[repunit] == 2:
            unique_length[repunit] = None

        current = repunit
        length += 1

        while True:
            next_repunit = current * base + 1
            if next_repunit >= limit:
                break

            current = next_repunit
            count[current] += 1

            if count[current] == 1:
                unique_length[current] = length
            elif count[current] == 2:
                unique_length[current] = None

            length += 1

    return dict(count), unique_length


def is_strong_repunit(
    value: int,
    count: Dict[int, int],
    unique_length: Dict[int, int | None],
) -> bool:
    """Return True if ``value`` is a strong repunit.

    A value ``n`` is considered a strong repunit if:

    - it is representable as a repunit in at least two bases (``count[n] >= 2``)
      or
    - it is representable as a repunit in exactly one base but with length
      ``>= 3`` (this mirrors the subtle condition from the original Ruby code,
      which accounts for base ``n-1`` always representing ``n`` as ``11`` and
      requires an additional, longer representation).
    """

    if value == 1:
        return True

    c = count.get(value, 0)
    ul = unique_length.get(value)

    if c >= 2:
        return True

    if c == 1 and ul is not None and ul >= 3:
        return True

    return False


def sum_strong_repunits(limit: int = DEFAULT_LIMIT) -> int:
    """Compute the sum of all strong repunits below ``limit``.

    Uses the same logic as the original Ruby solution, including special
    treatment of ``1``.
    """

    count, unique_length = generate_repunit_counts(limit)

    total = 1  # ``1`` is included as a strong repunit.
    for value in count.keys():
        if value >= limit or value == 1:
            continue
        if is_strong_repunit(value, count, unique_length):
            total += value

    return total


def main() -> None:
    """Entry point: compute and print the sum for the default limit."""

    result = sum_strong_repunits(DEFAULT_LIMIT)
    print(result)


if __name__ == "__main__":
    main()
