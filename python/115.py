"""Project Euler Problem 115.

Count tilings of a row of length n where blocks of minimum length m are
separated by at least one black square. Find the smallest n such that
F(m, n) exceeds one million for m = 50.
"""

from typing import List

# Constants for the problem
MIN_RED_LENGTH = 50
TARGET_WAYS = 1_000_000


def count_arrangements(min_length: int, length: int, cache: List[int]) -> int:
    """Count arrangements using dynamic programming."""
    while len(cache) <= length:
        idx = len(cache)

        if idx < min_length:
            cache.append(1)
            continue

        total = 1  # all-black configuration

        for block_length in range(min_length, idx + 1):
            max_start = idx - block_length
            for start in range(max_start + 1):
                remaining = idx - start - block_length
                if remaining == 0:
                    total += 1
                else:
                    total += cache[remaining - 1]

        cache.append(total)

    return cache[length]


def find_smallest_n(min_length: int, target: int) -> int:
    """Find the smallest n such that F(min_length, n) > target."""
    cache: List[int] = [1]
    n = min_length

    while True:
        total = count_arrangements(min_length, n, cache)
        if total > target:
            return n
        n += 1


def main() -> int:
    """Main function."""
    return find_smallest_n(MIN_RED_LENGTH, TARGET_WAYS)


if __name__ == "__main__":
    print(main())
