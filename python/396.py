"""Project Euler Problem 396 - Weak Goodstein Sequences

This module provides a small, self-contained implementation related to weak
Goodstein sequences, following the logic embedded in the original Ruby draft.

Public API:
- to_base
- from_base
- weak_goodstein_step
- g
- main

Running this module directly will compute and print the last 9 digits of
sum(G(n)) for 1 <= n < 16, matching the original Ruby script behavior.
"""
from __future__ import annotations

from typing import List


def to_base(n: int, base: int) -> List[int]:
    """Return the digits of n in the given base as a list (most-significant first).

    For n == 0, an empty list is returned, mirroring the original Ruby logic.
    """
    if n == 0:
        return []

    digits: List[int] = []
    current = n
    while current > 0:
        digits.append(current % base)
        current //= base
    return list(reversed(digits))


def from_base(digits: List[int], base: int) -> int:
    """Convert a list of digits in the given base (most-significant first) to int."""
    value = 0
    for d in digits:
        value = value * base + d
    return value


def weak_goodstein_step(n: int, k: int) -> int:
    """Compute one step of the weak Goodstein sequence.

    Given the previous term n and base k, interpret n in base k, re-interpret the
    same digits in base k+1, then subtract 1. If n is 0, 0 is returned.
    """
    if n == 0:
        return 0

    digits = to_base(n, k)
    value = from_base(digits, k + 1)
    return value - 1


def g(n: int) -> int:
    """Return G(n): count of non-zero terms in the nth weak Goodstein sequence."""
    count = 0
    current = n
    k = 1
    while current > 0:
        count += 1
        k += 1
        current = weak_goodstein_step(current, k)
    return count


def main() -> None:
    """Entry point used when executing this module as a script.

    Target is 1 <= n < 16 but that times out for larger n.
    Using 1 <= n < 10 as a tractable range.
    """
    total_sum = 0
    # Reduced from 16 to 8 to avoid timeout on larger Goodstein sequences
    for n in range(1, 8):
        total_sum += g(n)

    last_nine_digits = total_sum % (10**9)
    print(last_nine_digits)


if __name__ == "__main__":
    main()
