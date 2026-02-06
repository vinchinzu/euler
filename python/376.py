#!/usr/bin/env python3
"""
Project Euler Problem 376: Nontransitive sets of dice

Find the number of nontransitive sets of 3 dice, where each die has K=6 sides,
each side has at most N=30 pips, and each pair beats the next cyclically
(probability > 1/2).

Algorithm:
Recurse over pip values in increasing order. For each pip value, choose how many
sides of each die get that value. Track the number of "losing rolls" between each
pair. At the base case (all sides assigned), multiply by C(N, maxPip) since we can
replace the M distinct pip values with any M values from [1..N]. Divide by 3 to
remove cyclic triple-counting.
"""
from math import comb
from functools import lru_cache


def solve():
    N = 30
    K = 6
    half_k_sq = K * K // 2  # 18

    @lru_cache(maxsize=None)
    def helper(max_pip, rem1, rem2, rem3, loss1, loss2, loss3):
        # loss1 = losses for die1 vs die2, loss2 = die2 vs die3, loss3 = die3 vs die1
        if loss1 >= half_k_sq or loss2 >= half_k_sq or loss3 >= half_k_sq:
            return 0
        if rem1 == 0 and rem2 == 0 and rem3 == 0:
            return comb(N, max_pip)
        total = 0
        for s1 in range(rem1 + 1):
            for s2 in range(rem2 + 1):
                for s3 in range(rem3 + 1):
                    if s1 + s2 + s3 > 0:
                        total += helper(
                            max_pip + 1,
                            rem1 - s1, rem2 - s2, rem3 - s3,
                            loss1 + rem1 * s2,
                            loss2 + rem2 * s3,
                            loss3 + rem3 * s1,
                        )
        return total

    return helper(0, K, K, K, 0, 0, 0) // 3


if __name__ == "__main__":
    print(solve())
