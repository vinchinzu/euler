"""Project Euler Problem 316 (translated from Ruby to Python).

This module computes the sum of expected waiting times g(m), where g(m) is the
expected position of the first occurrence of the digit string of m in a random
infinite sequence of base-10 digits.

The original Ruby file used exact Gaussian elimination per string and timed out
for the required 10^6 invocations. We now exploit the KMP automaton structure:
transitions from state i touch only states <= i + 1, so the expectation system
is banded. We solve it symbolically in O(d^2) integer arithmetic (no Fractions)
by expressing each E[i] as a linear form a[i] * E[0] + b[i] and enforcing the
absorbing condition E[d] = 0. This makes the million evaluations tractable.

Public APIs:
- expected_waiting_time(m: int) -> int
- main() -> None

The script is executable as a standalone program.
"""

from __future__ import annotations

from functools import lru_cache
from typing import List

BASE: int = 10
MAX_POWER: int = 16
POWER_VALUE: int = BASE ** MAX_POWER
MAX_N: int = 999_999
def build_prefix_function(digits: List[int]) -> List[int]:
    """Return KMP prefix function values for the provided digit sequence."""

    d = len(digits)
    prefix: List[int] = [0] * d
    j = 0
    for i in range(1, d):
        while j > 0 and digits[i] != digits[j]:
            j = prefix[j - 1]
        if digits[i] == digits[j]:
            j += 1
        prefix[i] = j
    return prefix


def build_transition_counts(digits: List[int], prefix: List[int]) -> List[List[int]]:
    """Return digit-transition counts for each transient automaton state."""

    d = len(digits)
    if d == 0:
        return []

    failure: List[int] = [0] * (d + 1)
    for i in range(1, d + 1):
        failure[i] = prefix[i - 1]

    transitions: List[List[int]] = [[0] * BASE for _ in range(d)]
    first_digit = digits[0]
    row0 = transitions[0]
    for digit in range(BASE):
        row0[digit] = 1 if digit == first_digit else 0

    for state in range(1, d):
        base_row = transitions[failure[state]][:]
        base_row[digits[state]] = state + 1
        transitions[state] = base_row

    counts: List[List[int]] = [[0] * (d + 1) for _ in range(d)]
    for state in range(d):
        row = counts[state]
        trans_row = transitions[state]
        for digit in range(BASE):
            row[trans_row[digit]] += 1
    return counts


@lru_cache(maxsize=None)
def expected_waiting_time(m: int) -> int:
    """Return g(m): expected position of the first occurrence of m's digits."""

    if m < 0:
        msg = "m must be non-negative"
        raise ValueError(msg)

    digits = [ord(ch) - 48 for ch in str(m)]
    d = len(digits)
    if d == 0:
        return 0
    if d == 1:
        return BASE

    prefix = build_prefix_function(digits)
    counts = build_transition_counts(digits, prefix)

    coeffs: List[int] = [0] * (d + 1)
    constants: List[int] = [0] * (d + 1)
    coeffs[0] = 1

    for state in range(d):
        row = counts[state]
        coeff = (BASE - row[state]) * coeffs[state]
        const = (BASE - row[state]) * constants[state] - BASE

        for prev in range(state):
            count = row[prev]
            if count:
                coeff -= count * coeffs[prev]
                const -= count * constants[prev]

        next_state = state + 1
        coeffs[next_state] = coeff
        constants[next_state] = const

    final_coeff = coeffs[d]
    final_const = constants[d]
    if final_coeff == 0:
        msg = "Unexpected zero coefficient while solving expectation."
        raise ValueError(msg)
    if final_const % final_coeff:
        msg = (
            f"Non-integer expected waiting time for {m}: "
            f"a={final_coeff}, b={final_const}"
        )
        raise ValueError(msg)

    return -final_const // final_coeff


def main() -> None:
    """Compute and print the requested Project Euler 316 sum.

    This is a direct translation and cleanup of the original Ruby logic. It may
    be computationally intensive for the full range up to MAX_N. The algorithm
    now reuses the KMP automaton structure to evaluate expectations quickly.
    """

    total = 0

    for n in range(2, MAX_N + 1):
        m = POWER_VALUE // n
        total += expected_waiting_time(m)

    print(total)


if __name__ == "__main__":  # pragma: no cover - manual execution entrypoint
    main()
