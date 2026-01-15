"""Project Euler Problem 584: Birthday problem.

If people enter a room one by one, find the expected number of people when
there first are K people with birthdays within W days from each other,
assuming years of N days and no leap years.

Call a group of people "good" if there are K people with birthdays within
W days from each other. Let P(predicate) be the probability that if the
nth person makes the group "good", then n satisfies the given predicate.
The expected value is Σ_{n=1}^∞ n*P(=n), and this is equal to
Σ_{n=0}^∞ P(>n). But P(>n) is just the probability that the group of
the first n people are not "good".

To compute this, we use dynamic programming with windows representing
the last W days of birthdays.
"""

from __future__ import annotations

from itertools import product
from math import factorial
from typing import List


def solve() -> float:
    """Solve Problem 584."""
    N = 365
    K = 4
    W = 7
    L = (K - 1).bit_length()  # Number of bits needed

    # Precompute factorials
    factorials = [1] * (K + 1)
    for i in range(1, K + 1):
        factorials[i] = factorials[i - 1] * i

    # Generate all valid windows (bitsets representing last W days)
    window_bitsets: List[int] = []
    window_sums: List[int] = []
    bitset_to_index: dict[int, int] = {}

    # Generate all combinations of K-1 or fewer birthdays in W days
    for combo in product(range(K), repeat=W):
        total = sum(combo)
        if total < K:
            bitset = 0
            for j in range(W):
                bitset += combo[j] << (L * j)
            bitset_to_index[bitset] = len(window_bitsets)
            window_bitsets.append(bitset)
            window_sums.append(total)

    num_windows = len(window_bitsets)

    # DP: dp[w0][w][n] = number of ways with window w0->w and n people
    dp = [[[0.0] * (N + 1) for _ in range(num_windows)] for _ in range(num_windows)]

    # Base case: dp[w][w][0] = 1
    for w in range(num_windows):
        dp[w][w][0] = 1.0

    # Process each day
    for d in range(N):
        new_dp = [[[0.0] * (N + 1) for _ in range(num_windows)] for _ in range(num_windows)]

        for w0 in range(num_windows):
            for w in range(num_windows):
                max_n = d * (K - 1) // (W + 1) + K
                for n in range(max_n + 1):
                    if dp[w0][w][n] == 0:
                        continue
                    for dn in range(K):
                        if window_sums[w] + dn >= K:
                            break
                        if n + dn > N:
                            break

                        # Update window: shift left by L bits, add dn at end
                        new_bitset = (window_bitsets[w] >> L) + (dn << (L * (W - 1)))
                        new_w = bitset_to_index[new_bitset]

                        new_dp[w0][new_w][n + dn] += dp[w0][w][n] / factorials[dn]

        dp = new_dp

    # Compute answer: sum over all valid dp[N][w][w][n] * n! / N^n
    ans = 0.0
    for n in range(N + 1):
        term = 0.0
        for w in range(num_windows):
            term += dp[w][w][n]

        if term == 0:
            break

        term *= factorial(n) / (N ** n)
        ans += term

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")


if __name__ == "__main__":
    main()
