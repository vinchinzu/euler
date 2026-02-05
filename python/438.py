#!/usr/bin/env python3
"""
Project Euler 438 - Integer part of polynomial equation's solutions

Find sum S(t) for all n-tuples t = (a_1, ..., a_n) such that the polynomial
x^n + a_1*x^(n-1) + ... + a_n has n real roots, and when sorted, the i-th root
is in [i, i+1).

For n=7, find the sum of S(t) = sum of |a_i| over all valid tuples.
"""

import math

def solve():
    N = 7
    EPS = 1 / math.factorial(N)

    # Build inequality coefficients
    # inequalities[i][j][k] stores coefficient of a_k in the j-th inequality at level i
    inequalities = [None] * (N + 1)

    # Base level: evaluate polynomial at N+1-j-eps for j=0..N
    inequalities[N] = [[0.0] * (N + 1) for _ in range(N + 1)]
    for j in range(N + 1):
        x = N + 1 - j - EPS
        for k in range(N + 1):
            inequalities[N][j][k] = x ** (N - k)

    # Take differences to eliminate variables one by one
    for i in range(N - 1, 0, -1):
        inequalities[i] = [[0.0] * (i + 1) for _ in range(i + 1)]
        for j in range(i + 1):
            for k in range(i + 1):
                inequalities[i][j][k] = inequalities[i + 1][j][k] - inequalities[i + 1][j + 1][k]

    ans = [0]  # Use list to allow modification in nested function

    def helper(t):
        """Recursively find all valid tuples and sum their S(t)."""
        index = len(t) + 1

        # Compute bounds for a_{index}
        lower_bound = float('-inf')
        upper_bound = float('inf')

        for j in range(index + 1):
            goal = -inequalities[index][j][0]
            for k in range(index - 1):
                goal -= inequalities[index][j][k + 1] * t[k]
            goal /= inequalities[index][j][index]

            if j % 2 == 0:
                lower_bound = max(math.ceil(goal), lower_bound)
            else:
                upper_bound = min(math.floor(goal), upper_bound)

        lower_bound = int(lower_bound)
        upper_bound = int(upper_bound)

        if index == N:
            # Final variable: sum over all valid a_N values
            num_terms = upper_bound - lower_bound + 1
            if num_terms > 0:
                sum_prefix = sum(abs(a) for a in t)
                # Sum of |a_N| for a_N from lower to upper
                # Need to compute sum_{a=lower}^{upper} |a|
                if lower_bound >= 0:
                    sum_abs = (upper_bound * (upper_bound + 1) - lower_bound * (lower_bound - 1)) // 2
                elif upper_bound <= 0:
                    sum_abs = ((-lower_bound) * (-lower_bound + 1) - (-upper_bound) * (-upper_bound - 1)) // 2
                else:
                    # Split: negative part and non-negative part
                    sum_neg = (-lower_bound) * (-lower_bound + 1) // 2
                    sum_pos = upper_bound * (upper_bound + 1) // 2
                    sum_abs = sum_neg + sum_pos
                ans[0] += num_terms * sum_prefix + sum_abs
            return

        for a_r in range(lower_bound, upper_bound + 1):
            t.append(a_r)
            helper(t)
            t.pop()

    helper([])
    return ans[0]

if __name__ == "__main__":
    print(solve())
