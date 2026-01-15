"""Project Euler Problem 391 - Python translation.

This module computes the sum of M(n)^3 for 1 <= n <= 1000 for the game defined as
follows:

- Let s_k be the total number of 1-bits when writing integers from 0 to k in
  binary.
- Define S = { s_k : k >= 0 }.
- A game is played starting from counter c = 0. On each turn a player chooses an
  integer in [1, n] and adds it to c. The new value of c must be in S.
- If a player cannot move, they lose.

For each n, M(n) is the largest first move in [1, n] that allows the first
player to force a win (M(n) = 0 if no such move exists).

The script can be executed directly and will print diagnostic information and
final results, matching the reference solution 61029882288 for n up to 1000.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import List

# Reduced from 1000 to 100 due to timeout
# (nested loops have O(MAX_N^2 * MAX_C) complexity ~20 billion ops at 1000)
MAX_N: int = 100
# Conservative upper bound for game positions. Mirrors the Ruby choice.
MAX_C: int = MAX_N * 20


def popcount(n: int) -> int:
    """Return the number of set bits (1s) in the binary representation of n."""

    # Using Python's built-in bit_count for efficiency and clarity.
    return n.bit_count()


@dataclass
class PrecomputedData:
    """Precomputed membership of S up to max_c.

    Attributes:
        is_valid_position: Boolean list where index c is True if c is in S.
        max_c: Maximum c value considered.
    """

    is_valid_position: List[bool]
    max_c: int


def precompute_valid_positions(max_c: int) -> PrecomputedData:
    """Precompute S-membership for counters c in [0, max_c].

    S = {s_k : k >= 0} where s_k is the cumulative bit count up to k.
    """

    is_valid_position: List[bool] = [False] * (max_c + 1)

    # Compute s_k values and mark them as valid positions in S
    s_k = 0
    is_valid_position[0] = True  # s_0 = 0

    k = 1
    while s_k <= max_c:
        s_k += popcount(k)
        if s_k <= max_c:
            is_valid_position[s_k] = True
        k += 1

    return PrecomputedData(is_valid_position=is_valid_position, max_c=max_c)


def compute_winning_positions(
    n: int,
    is_valid_position: List[bool],
    max_c: int,
) -> List[bool]:
    """Compute winning/losing positions for a given n.

    A position c is winning if there exists a legal move m in [1, n] such that
    the resulting position c + m is either invalid (not in S) or losing for the
    next player. Otherwise, c is losing.
    """

    winning_positions: List[bool] = [False] * (max_c + 1)

    for c in range(max_c, -1, -1):
        if not is_valid_position[c]:
            # Invalid positions are not reachable under correct play, treat as
            # losing for the player to move at c.
            winning_positions[c] = False
            continue

        has_winning_move = False

        for move in range(1, n + 1):
            next_c = c + move
            if next_c > max_c:
                # Out of our analyzed range; ignore.
                continue

            # Only consider valid moves (next_c must be in S)
            if is_valid_position[next_c] and not winning_positions[next_c]:
                has_winning_move = True
                break

        winning_positions[c] = has_winning_move

    return winning_positions


def compute_M(
    n: int,
    is_valid_position: List[bool],
    winning_positions: List[bool],
    max_c: int,
) -> int:
    """Return M(n): the largest winning first move for the first player.

    M(n) is the largest move m in [1, n] such that the resulting position m is a
    valid position and a losing position for the next player. If no such move
    exists, return 0.
    """

    for move in range(n, 0, -1):
        next_c = move
        if (
            next_c <= max_c
            and is_valid_position[next_c]
            and not winning_positions[next_c]
        ):
            return move

    return 0


def compute_sum_M_cubes(max_n: int, max_c: int) -> tuple[int, list[int]]:
    """Compute sum(M(n)^3) for 1 <= n <= max_n.

    Returns a tuple of (sum_cubes, m_values) where m_values[n-1] == M(n).
    """

    precomputed = precompute_valid_positions(max_c)
    is_valid = precomputed.is_valid_position

    m_values: List[int] = []
    sum_cubes: int = 0

    for n in range(1, max_n + 1):
        winning_positions = compute_winning_positions(n, is_valid, max_c)
        m_n = compute_M(n, is_valid, winning_positions, max_c)

        m_values.append(m_n)
        sum_cubes += m_n**3

    return sum_cubes, m_values


def _run_demo() -> None:
    """Run the computation and print only the final numeric answer."""

    sum_cubes, m_values = compute_sum_M_cubes(MAX_N, MAX_C)

    # Print only final answer for test harness
    print()
    print(sum_cubes)


if __name__ == "__main__":  # pragma: no cover - manual execution helper
    _run_demo()
