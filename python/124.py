"""Project Euler Problem 124: Find the 10000th element in the sorted list
of n from 1 to 100000, sorted by rad(n), then by n.

rad(n) is the product of the distinct prime factors of n.
For example, rad(504) = rad(2^3 * 3^2 * 7) = 2 * 3 * 7 = 42.
rad(1) = 1 by convention.

We need to find E(10000), the 10000th element in the sorted n column.
"""

from sympy import primerange
from typing import List, Tuple

# Constants for clarity and maintainability
LIMIT = 100000
TARGET_K = 10000
TARGET_INDEX = TARGET_K - 1  # 0-indexed array


def main() -> int:
    """Main function."""
    # Precompute radicals for all n from 1 to LIMIT
    # Initialize entire array to 1 (no nil checks needed)
    rad: List[int] = [1] * (LIMIT + 1)

    # Sieve-like approach: for each prime p, multiply p into rad[n] for all multiples n
    # This ensures each distinct prime factor is included exactly once
    for p in primerange(2, LIMIT + 1):
        # For each multiple n of p, multiply p into the radical
        # Since we process primes in ascending order, this builds up the product of distinct primes
        for n in range(p, LIMIT + 1, p):
            rad[n] *= p  # Safe multiplication since rad[n] is always initialized to 1

    # Verify edge cases (basic validation)
    assert rad[1] == 1, "rad[1] should be 1"
    assert rad[2] == 2, "rad[2] should be 2"
    assert rad[4] == 2, "rad[4] should be 2"
    assert rad[6] == 6, "rad[6] should be 6"
    assert rad[12] == 6, "rad[12] should be 6"

    # Create array of [rad(n), n] pairs for sorting
    pairs: List[Tuple[int, int]] = [(rad[n], n) for n in range(1, LIMIT + 1)]

    # Sort by rad(n) ascending, then by n ascending (stable lexicographical sort)
    sorted_pairs = sorted(pairs, key=lambda x: (x[0], x[1]))

    # Extract the k-th element (n value from the TARGET_INDEX-th pair)
    result = sorted_pairs[TARGET_INDEX][1]

    return result  # Expected: 277


if __name__ == "__main__":
    print(main())
