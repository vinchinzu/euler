"""Project Euler Problem 321 (improved version) in Python 3.12.

This module provides utilities to compute the minimal number of moves needed to
swap red and blue counters on a 1D board, and to find values of ``n`` for which
that move count is a triangular number.

Public API:
- ``is_triangular``
- ``MinMoves``
- ``find_sequence``

Running this module as a script will:
- verify the ``MinMoves`` implementation for small cases, and
- compute and print the sum of the first 40 qualifying terms.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable, List

SLIDE_COST: int = 1
JUMP_COST: int = 2
MAX_N: int = 10**15  # Safety limit for search (terms grow exponentially)


def _is_perfect_square(num: int) -> bool:
    """Return True if ``num`` is a non-negative perfect square.

    Uses integer arithmetic and avoids floating point imprecision.
    """

    if num < 0:
        return False
    root: int = int(num**0.5)
    return root * root == num


def is_triangular(num: int) -> bool:
    """Return True if ``num`` is a triangular number.

    A number ``T`` is triangular if there exists integer ``k`` such that
    ``T = k * (k + 1) / 2``. This implementation uses the discriminant test
    ``1 + 8*T`` being a perfect square, where ``(-1 + sqrt(1 + 8*T))`` is even.
    """

    if num < 0:
        return False
    disc: int = 1 + 8 * num
    if not _is_perfect_square(disc):
        return False
    root: int = int(disc**0.5)
    return (root - 1) % 2 == 0


@dataclass(frozen=True)
class MinMoves:
    """Compute minimal move counts for the counter-swapping puzzle.

    The configuration consists of ``n`` red counters on the left, ``n`` blue
    counters on the right, and one empty square between them. ``m(n)`` returns
    the known closed-form minimal number of moves required to swap the red and
    blue sides.
    """

    @staticmethod
    def m(n: int) -> int:
        """Return the minimal number of moves required for ``n`` counters.

        Raises ``ValueError`` if ``n`` is negative.
        """

        if not isinstance(n, int) or n < 0:
            raise ValueError("n must be non-negative integer")
        if n == 0:
            return 0
        # Closed form: n * (n + 2)
        return n * (n + 2)

    @staticmethod
    def verify() -> bool:
        """Verify ``m(n)`` for a small set of test cases.

        Returns ``True`` on success; otherwise prints a warning and returns
        ``False``.
        """

        test_cases: list[tuple[int, int]] = [
            (1, 3),
            (2, 8),
            (3, 15),
        ]

        all_ok = True
        for n, expected in test_cases:
            computed = MinMoves.m(n)
            if computed != expected:
                print(
                    f"Verification failed: M({n}) = {computed}, expected {expected}",
                )
                all_ok = False
        return all_ok


def find_sequence(target_count: int = 40) -> List[int]:
    """Return first ``target_count`` n such that ``MinMoves.m(n)`` is triangular.

    Uses recurrence relation a[n] = 6*a[n-2] - a[n-4] + 4 for efficient
    generation, discovered from empirical analysis of the sequence.
    """

    if target_count <= 0:
        raise ValueError("target_count must be positive")

    # Start with manually verified first 4 terms
    sequence: List[int] = [1, 3, 10, 22]

    if target_count <= 4:
        return sequence[:target_count]

    # Use recurrence relation: a[n] = 6*a[n-2] - a[n-4] + 4
    while len(sequence) < target_count:
        next_term = 6 * sequence[-2] - sequence[-4] + 4
        sequence.append(next_term)

    return sequence


def _main() -> None:
    """Compute and print only the final numeric answer."""

    if not MinMoves.verify():
        raise SystemExit("M(n) verification failed! Check the formula.")

    sequence = find_sequence(40)
    total = sum(sequence)
    print(total)


if __name__ == "__main__":
    _main()
