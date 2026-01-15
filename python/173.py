"""Project Euler Problem 173: Using up to one million tiles how many different square laminae can be formed."""

import math

LIMIT = 1_000_000


def count_laminae(limit: int) -> int:
    """Count the number of different square laminae using up to limit tiles."""
    total = 0
    max_k = int(math.sqrt(limit / 4.0))
    for k in range(1, max_k + 1):
        max_m_float = limit / (4.0 * k) - k
        max_m = int(max_m_float)
        if max_m >= 1:
            total += max_m
    return total


def main() -> int:
    """Main function."""
    return count_laminae(LIMIT)


if __name__ == "__main__":
    print(main())
