"""Project Euler Problem 755: Not Zeckendorf.

Let f(n) be the number of ways of expressing n as the sum of distinct
Fibonacci numbers. Find Σ_{k=0}^N f(k).

We use brute force by iterating over Fibonacci numbers from largest to
smallest, either including or excluding F_n. The only optimization is
that if the sum of all remaining Fibonacci numbers can't exceed our limit,
then any of the 2^k subsets work.
"""

from __future__ import annotations

from functools import lru_cache
from typing import List


def fibonaccis() -> List[int]:
    """Generate Fibonacci numbers."""
    fibs = [1, 1]
    while fibs[-1] < 10**15:  # Enough for N = 10^13
        fibs.append(fibs[-1] + fibs[-2])
    return fibs


@lru_cache(maxsize=None)
def helper(index: int, n: int) -> int:
    """Helper function for recursive computation."""
    if n < 0:
        return 0
    if index < 0:
        return 1
    fibs = fibonaccis()
    if fibs[index + 2] <= n + 2:
        return 2 ** index
    return helper(index - 1, n) + helper(index - 1, n - fibs[index])


def solve() -> int:
    """Solve Problem 755."""
    N = 10**13
    fibs = fibonaccis()
    return helper(len(fibs) - 3, N)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(int(result))
    return result


if __name__ == "__main__":
    main()
