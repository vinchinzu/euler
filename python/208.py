"""Project Euler Problem 208: Robot Walks.

Find the number of ways that a robot can move in N (360/K)° arcs starting from
north and return to its starting location, without any sudden turns in the loop.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, List, Tuple


@dataclass(frozen=True)
class Key:
    """Key for memoization: left counts, right counts, and direction."""

    left_counts: Tuple[int, ...]
    right_counts: Tuple[int, ...]
    direction: int


def solve() -> int:
    """Solve Problem 208."""
    N = 70
    K = 5

    def increment(nums: Tuple[int, ...], index: int) -> Tuple[int, ...]:
        """Increment element at index in tuple."""
        lst = list(nums)
        lst[index] += 1
        return tuple(lst)

    initial_counts = tuple([0] * K)
    counts: Dict[Key, int] = {Key(initial_counts, initial_counts, 0): 1}

    for step in range(1, N // 2 + 1):
        new_counts: Dict[Key, int] = {}
        for key, count in counts.items():
            # Counterclockwise move
            new_left = increment(key.left_counts, key.direction)
            new_dir1 = (key.direction + 1) % K
            new_key1 = Key(new_left, key.right_counts, new_dir1)
            new_counts[new_key1] = new_counts.get(new_key1, 0) + count

            # Clockwise move
            new_right = increment(key.right_counts, key.direction)
            new_dir2 = (key.direction + K - 1) % K
            new_key2 = Key(key.left_counts, new_right, new_dir2)
            new_counts[new_key2] = new_counts.get(new_key2, 0) + count

        counts = new_counts

    ans = 0
    for key, count in counts.items():
        for goal in range(N // K + 1):
            remaining_left_counts = tuple(
                goal - key.left_counts[(i + key.direction) % K] for i in range(K)
            )
            remaining_right_counts = tuple(
                N // K - goal - key.right_counts[(i + key.direction) % K]
                for i in range(K)
            )
            remaining_key = Key(
                remaining_left_counts, remaining_right_counts, (K - key.direction) % K
            )
            ans += count * counts.get(remaining_key, 0)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
