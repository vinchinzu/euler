# Project Euler Problem 859
#
# PROBLEM DESCRIPTION:
# <p>
# Odd and Even are playing a game with $N$ cookies.</p>
# 
# <p>
# The game begins with the $N$ cookies divided into one or more piles, not necessarily of the same size. They then make moves in turn, starting with Odd.<br>
# Odd's turn: Odd may choose any pile with an <b>odd</b> number of cookies, eat one and divide the remaining (if any) into two equal piles.<br>
# Even's turn: Even may choose any pile with an <b>even</b> number of cookies, eat two of them and divide the remaining (if any) into two equal piles.<br>
# The player that does not have a valid move loses the game.</p>
# 
# <p>
# Let $C(N)$ be the number of ways that $N$ cookies can be divided so that Even has a winning strategy.<br>
# For example, $C(5) = 2$ because there are two winning configurations for Even: a single pile containing all five cookies; three piles containing one, two and two cookies.<br>
# You are also given $C(16) = 64$.</p>
# 
# <p>
# Find $C(300)$.</p>
#
# ANALYSIS:
# The game is a sum of games played on individual piles.
# Each pile has a value G(k).
# If the sum of values > 0, Odd wins.
# If the sum of values < 0, Even wins.
# If the sum of values = 0, Second player (Even) wins.
# So Even wins if sum <= 0.
#
# G(k) calculation (Conway numbers):
# G(0) = 0
# For odd k: Odd moves to {m, m}, value 2*G(m). Even has no moves.
#   L = {2*G(m)}, R = {}. G(k) = simplest number > 2*G(m).
#   If 2*G(m) < 0: simplest > val is 0.
#   If 2*G(m) >= 0: simplest > val is 2*G(m) + 1.
# For even k: Even moves to {m, m}, value 2*G(m). Odd has no moves.
#   L = {}, R = {2*G(m)}. G(k) = simplest number < 2*G(m).
#   If 2*G(m) > 0: simplest < val is 0.
#   If 2*G(m) <= 0: simplest < val is 2*G(m) - 1.

from __future__ import annotations
import sys

# Increase recursion depth just in case, though we iterate
sys.setrecursionlimit(2000)

def solve() -> int:
    N = 300
    
    # 1. Compute G values
    G = [0] * (N + 1)
    for k in range(1, N + 1):
        if k % 2 == 1: # Odd
            m = (k - 1) // 2
            val = 2 * G[m]
            if val < 0:
                G[k] = 0
            else:
                G[k] = val + 1
        else: # Even
            m = (k - 2) // 2
            val = 2 * G[m]
            if val > 0:
                G[k] = 0
            else:
                G[k] = val - 1
            
    # 2. DP for partitions
    # dp[n][v] = number of partitions of n with value sum v
    # Use array with offset to handle negative indices
    # Safe bounds: Max G ~ 255, Min G ~ -127.
    # Sums bounded by [-150, 300]. 
    # Use generous offset.
    OFFSET = 2000
    MAX_VAL = 4000 
    
    dp = [[0] * MAX_VAL for _ in range(N + 1)]
    dp[0][OFFSET] = 1 
    
    for k in range(1, N + 1):
        g_val = G[k]
        # Iterate n from k to N
        for n in range(k, N + 1):
            prev_row = dp[n - k]
            curr_row = dp[n]
            
            if g_val >= 0:
                # Shift right
                for i in range(MAX_VAL - g_val):
                    if prev_row[i] > 0:
                        curr_row[i + g_val] += prev_row[i]
            else:
                # Shift left
                for i in range(-g_val, MAX_VAL):
                    if prev_row[i] > 0:
                        curr_row[i + g_val] += prev_row[i]
                        
    # 3. Sum counts for value <= 0 (index <= OFFSET)
    total_count = sum(dp[N][:OFFSET+1])
            
    return total_count

if __name__ == "__main__":
    print(solve())
