"""Project Euler Problem 135.

Count n < 1_000_000 with exactly ten solutions to x² − y² − z² = n
where x, y, z form an arithmetic progression with positive integers.
"""

import math
from typing import List

LIMIT = 1_000_000
TARGET_SOLUTIONS = 10
MAX_D = LIMIT // 4  # ensures 4d - 1 < LIMIT so at least one solution per d


def accumulate_solutions(counts: List[int]) -> None:
    """Accumulate solutions."""
    for d in range(1, MAX_D + 1):
        max_k = 3 * d - 1
        threshold = 4 * d * d

        if threshold <= LIMIT:
            for k in range(1, max_k + 1):
                n = k * (4 * d - k)
                if n < LIMIT:
                    counts[n] += 1
            continue

        s = math.sqrt(threshold - LIMIT)
        k1 = math.ceil(2 * d - s)
        k2 = math.floor(2 * d + s)

        if k1 > 1:
            for k in range(1, k1):
                n = k * (4 * d - k)
                if n < LIMIT:
                    counts[n] += 1

        if k2 < max_k:
            for k in range(k2 + 1, max_k + 1):
                n = k * (4 * d - k)
                if n < LIMIT:
                    counts[n] += 1


def main() -> int:
    """Main function."""
    solution_counts = [0] * LIMIT
    accumulate_solutions(solution_counts)
    return sum(1 for value in solution_counts if value == TARGET_SOLUTIONS)


if __name__ == "__main__":
    print(main())
