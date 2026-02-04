"""Project Euler Problem 201: Subsets with a Unique Sum.

Find the sum of all numbers that are the sum of exactly one K-element subset
of {1^2, 2^2, ..., 100^2}.

Uses DP with the key optimization from the Java reference: cap the DP table
at L/2 where L = sum of all squares, and use numpy for the inner loops.
Values are capped at min(2, count) to save memory and time.
"""

import numpy as np

def solve():
    N = 100
    K = 50
    L = sum(i * i for i in range(1, N + 1))  # 338350

    half = L // 2  # 169175

    # dp[i][j] = min(2, number of i-element subsets summing to j)
    # Use uint8 since values are only 0, 1, or 2
    dp = np.zeros((K + 1, half + 1), dtype=np.uint8)
    dp[0][0] = 1

    for n in range(1, N + 1):
        sq = n * n
        # Upper bound on j: min(sum of first n squares, half)
        max_j = min(n * (n + 1) * (2 * n + 1) // 6, half)
        max_i = min(K, n)

        for i in range(max_i, 0, -1):
            # dp[i][sq:max_j+1] = min(2, dp[i][sq:max_j+1] + dp[i-1][0:max_j+1-sq])
            end = max_j + 1
            if end <= sq:
                continue
            old = dp[i][sq:end].copy()
            contrib = dp[i - 1][0:end - sq]
            # Add and cap at 2
            result = old.astype(np.uint16) + contrib.astype(np.uint16)
            np.minimum(result, 2, out=result)
            dp[i][sq:end] = result.astype(np.uint8)

    # Sum all j where dp[K][j] == 1
    # By symmetry: if sum S has exactly 1 subset of size K, then L-S also
    # has exactly 1 subset of size K (the complement). So we count pairs.
    # The Java uses: for j in [half..0], if dp[K][j] == 1: ans += L
    # This works because j and L-j are complementary and each contributes L total.
    ans = 0
    row = dp[K]
    for j in range(half + 1):
        if row[j] == 1:
            ans += L

    return ans

if __name__ == "__main__":
    print(solve())
