"""Project Euler Problem 631: Constrained Permutations.

Find the number of permutations of (1...n) for n ≤ N elements such that
there are at most K occurrences of (2 1) in the permutation (an element
followed by a smaller element) and no occurrences of (1 2 4 3).

Let numPermutations(n, k, min, rise) be the number of permutations of (1...n)
such that there are at most k occurrences of (2 1), the minimum element before
it is min, and the minimum element that is larger than an element before it
is rise.
"""

from __future__ import annotations

from functools import lru_cache


def solve() -> int:
    """Solve Problem 631."""
    N = 10**18
    K = 40
    M = 10**9 + 7

    @lru_cache(maxsize=None)
    def num_permutations(n: int, k: int, min_val: int, rise: int) -> int:
        """Compute number of valid permutations."""
        if n == 0:
            return 1

        result = 0
        for next_val in range(1, min(n, rise) + 1):
            if next_val - 1 > k:
                continue

            next_rise = rise
            if next_val < next_rise:
                if next_val >= min_val:
                    next_rise = next_val
                else:
                    next_rise -= 1

            result = (
                result
                + num_permutations(
                    n - 1,
                    k - (next_val - 1),
                    min(next_val, min_val),
                    next_rise,
                )
            ) % M

        return result % M

    ans = 0
    for n in range(K + 2):
        ans = (ans + num_permutations(n, K, n, n)) % M

    if N > K + 1:
        base_count = num_permutations(K + 2, K, K + 2, K + 2)
        ans = (ans + (N - K - 1) % M * base_count) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
