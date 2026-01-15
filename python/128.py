"""Project Euler Problem 128.

Find the 2000th tile n for which PD(n) = 3 on the hexagonal spiral.
"""

import math
from typing import List

TARGET_INDEX = 2000
MAX_RING_ESTIMATE = 70_000
MAX_PRIME = 12 * MAX_RING_ESTIMATE + 5


def build_sieve(max_val: int) -> List[bool]:
    """Simple sieve for constant-time primality within required bounds."""
    sieve = [True] * (max_val + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, int(math.sqrt(max_val)) + 1):
        if sieve[i]:
            for j in range(i * i, max_val + 1, i):
                sieve[j] = False
    return sieve


SIEVE = build_sieve(MAX_PRIME)


def is_prime(value: int) -> bool:
    """Check if value is prime."""
    return value > 1 and value <= MAX_PRIME and SIEVE[value]


def type_a_tile(r: int) -> int:
    """Type A tile."""
    n = 3 * r * r - 3 * r + 2
    if is_prime(6 * r - 1) and is_prime(6 * r + 1) and is_prime(12 * r + 5):
        return n
    return 0


def type_b_tile(r: int) -> int:
    """Type B tile."""
    n = 3 * r * r + 3 * r + 1
    if is_prime(6 * r - 1) and is_prime(6 * r + 5) and is_prime(12 * r - 7):
        return n
    return 0


def find_tile(index: int) -> int:
    """Find the tile at given index."""
    if index == 1:
        return 1
    tiles = [1, 2]
    ring = 2

    while len(tiles) < index:
        candidate = type_a_tile(ring)
        if candidate:
            tiles.append(candidate)
            if len(tiles) == index:
                return candidate

        candidate = type_b_tile(ring)
        if candidate:
            tiles.append(candidate)
            if len(tiles) == index:
                return candidate

        ring += 1

    return tiles[-1]


def main() -> int:
    """Main function."""
    return find_tile(TARGET_INDEX)


if __name__ == "__main__":
    print(main())
