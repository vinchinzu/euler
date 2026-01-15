"""Project Euler Problem 174: Counting the number of "hollow" square laminae that can form one, two, three, ... distinct arrangements."""

from typing import Dict
import math

LIMIT = 1_000_000
MAX_N = 10


def main() -> int:
    """Main function."""
    # Calculate maximum valid m
    max_m = int(math.sqrt(LIMIT / 4.0)) + 1

    # Precompute all possible t values and their representation counts
    counts: Dict[int, int] = {}

    for m in range(1, max_m + 1):
        # Minimum k: ensures inner square side length (k-2m) >= 1
        min_k = 2 * m + 1

        # Maximum k: ensures t = 4m(k-m) <= LIMIT
        max_k = m + (LIMIT // (4 * m))
        max_k = max(max_k, min_k - 1)  # Ensure non-empty range

        if max_k >= min_k:
            for k in range(min_k, max_k + 1):
                t = 4 * m * (k - m)
                counts[t] = counts.get(t, 0) + 1

    # Compute N(n) for n = 1 to MAX_N
    n_values: list[int] = [0] * (MAX_N + 1)

    for t, count in counts.items():
        if t <= LIMIT and count <= MAX_N:
            n_values[count] += 1

    # Compute the required sum
    result = sum(n_values[n] for n in range(1, MAX_N + 1))
    return result


if __name__ == "__main__":
    print(main())
