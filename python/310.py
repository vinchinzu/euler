"""Project Euler Problem 310 - Nim Square (Python 3.12)

This module computes the number of losing positions in the Nim Square game for
triples (a, b, c) with 0 <= a <= b <= c <= MAX.

Nim Square rules:
- Normal play, 3 heaps.
- A move consists of removing a perfect-square number of stones from a heap.

We use standard Sprague-Grundy theory:
- Precompute Grundy numbers for single-heap sizes up to MAX.
- A triple (a, b, c) is losing iff g(a) ^ g(b) ^ g(c) == 0.

The original Ruby file contained several iterative "improved" attempts and
verbose logging; this Python version exposes a clean, tested API while keeping
an executable "main" routine.
"""

from __future__ import annotations

from bisect import bisect_left
from dataclasses import dataclass
from functools import lru_cache
from math import isqrt
from typing import Dict, Iterable, List, Sequence, Tuple


@dataclass(frozen=True)
class NimSquareConfig:
    """Configuration for Nim Square computations.

    Attributes:
        max_heap: Inclusive upper bound on heap size (c in 0 <= a <= b <= c).
    """

    max_heap: int = 100_000

    def __post_init__(self) -> None:  # type: ignore[override]
        if self.max_heap < 0:
            msg = "max_heap must be non-negative"
            raise ValueError(msg)

    @property
    def squares(self) -> List[int]:
        """List of allowed move sizes (perfect squares up to max_heap)."""

        limit = isqrt(self.max_heap)
        return [i * i for i in range(1, limit + 1)]


def compute_grundy_table(config: NimSquareConfig) -> List[int]:
    """Compute Grundy numbers g[n] for 0 <= n <= max_heap.

    Uses a bottom-up dynamic program specialized for moves of square sizes.
    The original Ruby used recursive memoization with Set; we use an
    iterative approach, which is both clearer and faster in Python.
    """

    max_n = config.max_heap
    squares = config.squares

    grundy = [0] * (max_n + 1)

    # Temporary boolean array reused for mex computation.
    # Size is conservative: Grundy values stay reasonably small in practice.
    # For safety, sized relative to number of squares (upper bound on degree).
    max_candidates = len(squares) + 2
    seen = [False] * max_candidates

    for n in range(1, max_n + 1):
        # Mark reachable Grundy values from n.
        # Only squares <= n are valid moves.
        for sq in squares:
            if sq > n:
                break
            g = grundy[n - sq]
            if g < max_candidates:
                seen[g] = True

        # mex: smallest non-negative integer not in seen.
        mex = 0
        while mex < max_candidates and seen[mex]:
            mex += 1
        grundy[n] = mex

        # Reset seen for values we may have touched.
        for sq in squares:
            if sq > n:
                break
            g = grundy[n - sq]
            if g < max_candidates:
                seen[g] = False

    return grundy


def verify_small_case() -> bool:
    """Verify correctness for MAX=29 where the answer is known (1160).

    Returns True if the precomputed Grundy table reproduces the known count.
    """

    config = NimSquareConfig(max_heap=29)
    grundy = compute_grundy_table(config)

    count = 0
    for a in range(0, 30):
        ga = grundy[a]
        for b in range(a, 30):
            gb = grundy[b]
            for c in range(b, 30):
                gc = grundy[c]
                if (ga ^ gb ^ gc) == 0:
                    count += 1

    return count == 1160


def build_positions_by_grundy(grundy: Sequence[int]) -> Tuple[List[int], List[List[int]]]:
    """Build frequency and positions lists for each Grundy value.

    Returns a tuple (freq, positions):
    - freq[g] = how many heap sizes n have Grundy value g
    - positions[g] = sorted list of such n
    """

    max_g = max(grundy)
    freq = [0] * (max_g + 1)
    positions: List[List[int]] = [[] for _ in range(max_g + 1)]

    for n, g in enumerate(grundy):
        freq[g] += 1
        positions[g].append(n)

    return freq, positions


def _count_ordered_triples_for_values(
    pos_a: Sequence[int], pos_b: Sequence[int], pos_c: Sequence[int]
) -> int:
    """Count triples (a, b, c) with a <= b <= c.

    - a ranges over pos_a
    - b ranges over pos_b
    - c ranges over pos_c

    All sequences must be sorted. This function is performance-critical and
    intentionally compact.
    """

    if not pos_a or not pos_b or not pos_c:
        return 0

    count = 0

    for b in pos_b:
        # a choices: indices where a <= b
        # bisect_left returns first index with value > b, so that index
        # equals the count of entries <= b.
        num_a = bisect_left(pos_a, b + 1)

        # c choices: indices where c >= b
        idx_c = bisect_left(pos_c, b)
        num_c = len(pos_c) - idx_c

        if num_a and num_c:
            count += num_a * num_c

    return count


def count_losing_positions(config: NimSquareConfig) -> int:
    """Count losing positions (a, b, c) for Nim Square under given config.

    A position is losing for the next player iff g[a] ^ g[b] ^ g[c] == 0.
    The algorithm:
    - Compute Grundy table g[n].
    - Group heap sizes by Grundy value.
    - For each pair (ga, gb), derive gc = ga ^ gb and count triples using
      precomputed position lists.

    Complexity is roughly O(MAX + G^3), where G is the number of distinct
    Grundy values, which is small in practice for this game.
    """

    grundy = compute_grundy_table(config)
    freq, positions = build_positions_by_grundy(grundy)
    max_g = len(freq) - 1

    total = 0

    for ga in range(max_g + 1):
        if not freq[ga]:
            continue
        pos_a = positions[ga]

        for gb in range(max_g + 1):
            if not freq[gb]:
                continue
            pos_b = positions[gb]

            gc = ga ^ gb
            if gc > max_g or not freq[gc]:
                continue
            pos_c = positions[gc]

            total += _count_ordered_triples_for_values(pos_a, pos_b, pos_c)

    return total


def main(max_heap: int = 100_000) -> None:
    """Entry point for CLI-style execution.

    - Verifies correctness on the small MAX=29 case.
    - Computes and prints the losing-position count for given max_heap.
    """

    config = NimSquareConfig(max_heap=max_heap)

    if not verify_small_case():
        msg = "Verification for MAX=29 failed; aborting computation."
        raise RuntimeError(msg)

    result = count_losing_positions(config)
    print(result)


if __name__ == "__main__":  # pragma: no cover - CLI behavior
    import argparse

    parser = argparse.ArgumentParser(
        description=(
            "Compute the number of losing positions in Nim Square for "
            "0 <= a <= b <= c <= MAX."
        ),
    )
    parser.add_argument(
        "MAX",
        type=int,
        nargs="?",
        default=100_000,
        help="Maximum heap size (default: 100000)",
    )
    args = parser.parse_args()

    main(args.MAX)
