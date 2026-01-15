"""Project Euler Problem 114 Solution.

A row measuring fifty units in length is to be filled with red blocks of
minimum length 3 units, separated by at least one grey square. The goal is
to find the number of ways to do this, allowing mixing of block sizes.
Example: For a row of 7 units, there are exactly 17 ways.
"""

from typing import Dict


def ways(n: int, memo: Dict[int, int]) -> int:
    """Recursive function with memoization to compute ways to tile length n.
    
    The recurrence relation is:
      ways(n) = ways(n-1) + sum_{k=3 to n} ways(n-k-1)
    where ways(-1) = 1 (for red block at the start with no separator needed)
    and ways(m) = 0 for m < -1. This correctly handles all possible tilings,
    including those starting with grey tiles, all-grey tilings, and red blocks
    of various lengths with proper separation.
    """
    if n < -1:  # Invalid lengths
        return 0
    if n == -1:  # Base case for red block at the start
        return 1
    if n == 0:  # Empty tiling (all grey)
        return 1

    # Return memoized result if available
    if n in memo:
        return memo[n]

    total = ways(n - 1, memo)  # Add one more grey tile to any valid tiling of n-1

    # Add cases for placing a red block of length k (k >= 3) at the end,
    # preceded by exactly one grey separator (extra greys handled by the first term)
    for k in range(3, n + 1):
        remaining = n - k - 1
        total += ways(remaining, memo)

    memo[n] = total
    return total


def main() -> int:
    """Main execution: compute ways for n=50."""
    memo: Dict[int, int] = {-1: 1}  # Base case for no prior tiling (red block at start)
    return ways(50, memo)


if __name__ == "__main__":
    print(main())
