"""Project Euler Problem 328 - Minimal worst-case cost strategy.

This module implements a dynamic programming solution to compute C(n), the
minimal possible worst-case total cost for guessing a hidden number in
{1, ..., n} using the rules:

- Each query is an integer q you choose.
- Cost of asking q is q.
- Response is one of: lower, equal, or higher than the hidden number.

The goal is to minimize, for each n, the worst-case sum of all queries asked.

The implementation follows the Ruby reference version closely but uses
idiomatic Python 3.12 with type hints and clearer structure.

Note: The direct DP implemented here runs in O(n^3) time and O(n^2) memory.
It is suitable only for relatively small n (e.g. up to a few thousands in a
reasonable time in CPython). The original Ruby comments suggesting
n = 200_000 as "manageable" are not realistic for this algorithmic complexity.
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass
class MinimalCostFinder:
    """Compute minimal worst-case query costs for the guessing game.

    Public methods:
    - compute_c(n): return C(n) for given n >= 0.
    - sum_c_up_to(max_n): return sum_{k=1..max_n} C(k).
    - verify(): run basic checks against known values and print results.

    This is a direct translation of the classic O(n^3) DP approach:
    we let dp[low][high] be the minimal worst-case total cost needed to
    determine the hidden number within the range [low, high].
    """

    def compute_c(self, n: int) -> int:
        """Return C(n), the worst-case cost of an optimal strategy.

        For n <= 0, this returns 0.
        Complexity: O(n^3) time, O(n^2) space.
        """

        if n < 1:
            return 0

        # We use (n + 2) for easier 1-based indexing and to avoid bounds checks
        dp: list[list[int]] = [[0] * (n + 2) for _ in range(n + 2)]

        # length is the size of the interval [low, high]
        for length in range(1, n + 1):
            for low in range(1, n - length + 2):
                high = low + length - 1
                if low >= high:
                    # Single element or invalid range: no cost required.
                    continue

                min_worst_cost = float("inf")

                # q can be any value from low to high inclusive
                for q in range(low, high + 1):
                    if q == low:
                        # If equal: done. If lower: impossible. If higher: search [low+1, high]
                        total_worst = q + dp[low + 1][high]
                    elif q == high:
                        # If equal: done. If higher: impossible. If lower: search [low, high-1]
                        total_worst = q + dp[low][high - 1]
                    else:
                        # Strictly inside: lower -> [low, q-1], higher -> [q+1, high]
                        left_cost = dp[low][q - 1]
                        right_cost = dp[q + 1][high]
                        total_worst = q + max(left_cost, right_cost)

                    if total_worst < min_worst_cost:
                        min_worst_cost = total_worst

                dp[low][high] = int(min_worst_cost)

        return dp[1][n]

    def sum_c_up_to(self, max_n: int) -> int:
        """Return sum_{n=1..max_n} C(n).

        This uses repeated calls to compute_c and is therefore extremely
        expensive for large max_n. It is provided mainly to mirror the
        original script and for small-scale experimentation.
        """

        if max_n < 1:
            return 0

        total_sum = 0
        for n in range(1, max_n + 1):
            total_sum += self.compute_c(n)
        return total_sum

    def verify(self) -> None:
        """Print verification results against known reference values."""

        print("Verification against known values:")

        test_cases: list[tuple[int, int]] = [
            (1, 0),
            (2, 1),
            (3, 2),
            (8, 12),
            (100, 400),
        ]

        for n, expected in test_cases:
            actual = self.compute_c(n)
            status = "PASS" if actual == expected else "FAIL"
            print(f"C({n}) = {actual} (expected {expected}) - {status}")

        sum_100 = sum(self.compute_c(n) for n in range(1, 101))
        expected_sum_100 = 17_575
        status = "PASS" if sum_100 == expected_sum_100 else "FAIL"
        print(
            "Sum C(1) to C(100) = "
            f"{sum_100} (expected {expected_sum_100}) - {status}"
        )
        print()


def main() -> None:
    """Run basic verification.

    Warning: The naive O(n^3) DP is not suitable for very large n in Python.
    For the full Project Euler constraints, a more advanced solution is required.
    """

    finder = MinimalCostFinder()
    finder.verify()

    # For the actual problem, we need sum up to 200000, but that's O(n^4) with this approach
    # For now, just verify the small cases work
    print("Note: Full computation for n=200000 requires optimized algorithm")


if __name__ == "__main__":
    main()
