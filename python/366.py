"""Approximate/experimental solver for Project Euler Problem 366.

This is a direct, idiomatic Python translation of the provided Ruby draft
(`366.py`). It preserves structure and intent, but it does NOT
implement a mathematically proven O(log n) or similar solution for n up to
1e18. Instead, it:

- Implements the exact game logic for small n via recursion with memoization.
- Includes helper functions intended for pattern analysis and large-n
  approximation.
- Provides a placeholder `compute_large_sum_efficient` that mimics the Ruby
  draft's heuristic block-approximation approach.

IMPORTANT:
    The current large-n strategy is heuristic and is not guaranteed to produce
    the correct sum for n up to 1e18. It remains here as a faithful translation
    of the draft logic. A mathematically correct solution would require deeper
    game-theoretic analysis that is beyond the scope of this converter.
"""

from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from math import log
from typing import Dict, Iterable, List, Tuple

MOD: int = 100_000_000
MAX_PRECOMPUTE: int = 1_000  # Reduced for faster pattern analysis


@dataclass(frozen=True)
class GameState:
    """Represents a state in the game.

    Attributes:
        n: Remaining stones in the pile.
        prev: Number of stones taken in the previous move.
    """

    n: int
    prev: int


@lru_cache(maxsize=None)
def is_losing_position(n: int, prev: int) -> bool:
    """Return True if the current player is in a losing position.

    This implements the recursive game logic:
    - If no stones remain (n <= 0), the current player cannot move and loses.
    - The player may take k stones where 1 <= k <= min(2 * prev, n).
    - If any such move leads to either
        * taking the last stone, or
        * a losing position for the opponent,
      then the current position is winning; otherwise losing.
    """

    if n <= 0:
        # No legal moves: losing position.
        return True

    max_move = min(prev * 2, n)
    if max_move < 1:
        # No move allowed under the constraint.
        return True

    for k in range(1, max_move + 1):
        remaining = n - k
        if remaining == 0:
            # Current player can take the last stone -> winning.
            return False
        if is_losing_position(remaining, k):
            # There exists a move that forces opponent into a losing position.
            return False

    # All moves lead to winning positions for the opponent.
    return True


def compute_m(n: int) -> int:
    """Compute M(n) as defined in the problem draft.

    For position n:
    - Consider all legal first moves k (1 <= k < n).
    - Opponent's next move is capped at 2 * k.
    - We only accept k if the remaining position is losing for the opponent,
      under those rules.
    - Return the maximum such k, or 0 if none exists (i.e., losing position).
    """

    if n <= 1:
        return 0

    max_take = 0
    for k in range(1, n):
        remaining = n - k

        # If the entire remainder could be taken by the opponent in one move,
        # this k cannot be winning for the first player.
        if 0 < remaining <= 2 * k:
            continue

        if is_losing_position(remaining, k):
            if k > max_take:
                max_take = k

    return max_take


def run_tests() -> None:
    """Run basic unit tests against example values from the problem statement."""

    tests: List[Tuple[int, int]] = [
        (1, 0),  # Can't move
        (2, 0),  # First takes 1, second takes 1 and wins
        (5, 0),  # Problem example - losing position
        (17, 4),  # Problem example - winning with max take of 4
    ]

    print("Running unit tests...")
    for n, expected in tests:
        result = compute_m(n)
        status = "PASS" if result == expected else "FAIL"
        print(f"M({n}) = {result} (expected {expected}) - {status}")


def verify_small_sum() -> None:
    """Verify that sum M(n) for n <= 100 matches the known value (728)."""

    sum_100 = sum(compute_m(n) for n in range(1, 101))

    print(f"Verification: sum for n<=100 = {sum_100}")
    if sum_100 == 728:
        print("\u2713 Verification PASSED!")
    else:
        print(f"\u2717 Verification FAILED! Expected 728, got {sum_100}")
        raise SystemExit(1)


def analyze_patterns() -> Tuple[Dict[int, int], List[int]]:
    """Compute M(n) for a range and collect losing positions.

    Returns:
        A tuple of:
        - A mapping n -> M(n) for n in [1, MAX_PRECOMPUTE].
        - A list of losing positions n where M(n) == 0.
    """

    print("Analyzing game patterns for large n optimization...")

    m_values: Dict[int, int] = {}
    for n in range(1, MAX_PRECOMPUTE + 1):
        m_values[n] = compute_m(n)
        if n % 100 == 0:
            print(f"  Progress: computed M(n) for n <= {n}")

    losing_positions: List[int] = [
        n for n in range(1, MAX_PRECOMPUTE + 1) if m_values[n] == 0
    ]

    print(
        f"Found {len(losing_positions)} losing positions up to {MAX_PRECOMPUTE}"
    )

    return m_values, losing_positions


def compute_large_sum_efficient(limit: int) -> int:
    """Heuristic large-n computation of sum(M(n)) modulo MOD.

    NOTE:
        This function is a placeholder translation of the Ruby draft's
        "efficient" method. It uses a crude block-wise approximation and is NOT
        mathematically verified for very large limits such as 1e18.

    Args:
        limit: Upper bound for n.

    Returns:
        An integer representing the heuristic sum of M(n) for 1 <= n <= limit,
        taken modulo MOD.
    """

    if limit <= 100:
        return sum(compute_m(n) for n in range(1, limit + 1)) % MOD

    # Only compute exact values for small n (verified: sum for n<=100 is 728)
    small_sum = sum(compute_m(n) for n in range(1, 101))
    print(f"Verified sum for n <= 100: {small_sum}")

    total_sum = small_sum
    current_base = 101

    # Use heuristic for the rest
    while current_base <= limit:
        block_size = calculate_block_size(current_base)
        block_start = current_base
        block_end = min(current_base + block_size - 1, limit)

        if block_end >= block_start:
            block_sum = sum_block_m(block_start, block_end)
            total_sum += block_sum
            total_sum %= MOD

        current_base = block_end + 1

        # Progress reporting for very large ranges
        if current_base % (10**12) < block_size:
            print(f"  Progress: processed up to n ~ {current_base:.2e}")

    return total_sum % MOD


def calculate_block_size(n: int) -> int:
    """Estimate a block size for large-n approximation.

    This mirrors the Ruby draft's `calculate_block_size` and has no theoretical
    guarantee. It attempts to grow block sizes roughly with n.
    """

    base = 3.0
    if n <= 1:
        return 1

    level = int(log(n, base))
    return int((base**level) * 1.5)


def sum_block_m(start_n: int, end_n: int) -> int:
    """Approximate sum of M(n) on [start_n, end_n] via a simple estimator.

    Optimized to use arithmetic sum formula instead of iterating.
    Since estimate_m_for_large_n(n) = n // 3, we approximate as:
    sum(n // 3) ≈ (1/3) * sum(n) = (1/3) * (end_n*(end_n+1)/2 - (start_n-1)*start_n/2)
    """

    # Use arithmetic series formula for sum of n // 3
    # sum(n//3 for n in range(start_n, end_n+1)) ≈ (sum of n from start to end) / 3
    count = end_n - start_n + 1
    sum_n = (count * (start_n + end_n)) // 2
    return sum_n // 3


def estimate_m_for_large_n(n: int) -> int:
    """Estimate M(n) for large n.

    Placeholder based on the Ruby draft: uses floor(n / 3).
    This is NOT derived from proven analysis of the game.
    """

    return n // 3


def clear_memo() -> None:
    """Clear the memoization cache for `is_losing_position`.

    Useful for experiments where the game definition might be adjusted.
    """

    is_losing_position.cache_clear()


def solve() -> int:
    """Solve PE 366."""
    return compute_large_sum_efficient(10**18)


if __name__ == "__main__":
    print(solve())
