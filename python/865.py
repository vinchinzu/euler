# Project Euler Problem 865
#
# PROBLEM DESCRIPTION:
# <p>
# A <dfn>triplicate number</dfn> is a positive integer such that, after repeatedly removing three consecutive identical digits from it, all its digits can be removed.</p>
# 
# <p>
# For example, the integer $122555211$ is a triplicate number:
# $$122{\color{red}555}211 \rightarrow 1{\color{red}222}11\rightarrow{\color{red}111}\rightarrow.$$
# On the other hand, neither $663633$ nor $9990$ are triplicate numbers.</p>
# 
# <p>
# Let $T(n)$ be how many triplicate numbers are less than $10^n$.</p>
# 
# <p>
# For example, $T(6) = 261$ and $T(30) = 5576195181577716$.</p>
# 
# <p>
# Find $T(10^4)$. Give your answer modulo $998244353$.</p>
#
# SOLUTION LOGIC:
# The problem asks for the number of valid triplicate strings of length L <= N.
# A string is triplicate if it reduces to the empty string via the rule ddd -> epsilon.
# This structure is analogous to ternary Dyck paths but with 10 types of digits.
# Key recurrences:
# 1. Let dp[L] be the number of reducible strings of length L using digits 0-9 (allowing leading zeros).
#    A reducible string is non-empty only if L is a multiple of 3.
#    Any non-empty reducible string S can be uniquely decomposed based on the *first* digit d and its matching partners:
#    S = d A d B d C
#    where A, B, C are reducible strings (possibly empty), and A and B satisfy a "first match" constraint to ensure uniqueness.
#    Specifically, A and B must not have any "accessible" occurrences of d (no prefix reduces to empty exposing d).
#
# 2. Let V[L] be the number of reducible strings of length L that do NOT expose a specific digit d.
#    By symmetry, this count is the same for any digit d.
#    The recurrence for dp[L] becomes:
#    dp[L] = 10 * sum_{i+j+k = L-3} V[i] * V[j] * dp[k]
#    (The factor 10 accounts for the choice of the first digit d).
#
# 3. Let Prim[L] be the number of "primitive" reducible strings (cannot be split into two non-empty reducible strings U V).
#    dp[L] = sum_{k} Prim[k] * dp[L-k].
#    So Prim[L] can be derived from dp.
#
# 4. V[L] consists of strings formed by primitives that do not start with d.
#    Since digits are symmetric, 9/10 of primitives do not start with d.
#    V[L] = sum_{k} (9/10 * Prim[k]) * V[L-k].
#
# 5. T(n) counts positive integers < 10^n (lengths 1 to n).
#    Leading zeros are not allowed.
#    Since 1/10 of all reducible strings start with '0' (invalid), the valid count for length L is (9/10) * dp[L].
#    Total T(n) = sum_{L=1..n} (9/10 * dp[L]).
#
# Complexity: O(N^2) due to convolution, where N=10000/3. With N=3333, this is ~10^7 ops, well within limits.

import sys

# Increase recursion depth just in case
sys.setrecursionlimit(20000)

def solve():
    N = 10000
    MOD = 998244353
    
    # We work with indices m = L/3
    limit = N // 3
    
    # Arrays to store values for lengths 0, 3, 6, ...
    # dp[m] corresponds to length 3*m
    dp = [0] * (limit + 1)
    prim = [0] * (limit + 1)
    v = [0] * (limit + 1)
    
    dp[0] = 1
    v[0] = 1
    prim[0] = 0 
    
    inv10 = pow(10, -1, MOD)
    
    for m in range(1, limit + 1):
        # 1. Compute dp[m]
        # dp[m] = 10 * sum_{c=0 to m-1} dp[c] * (sum_{a+b = m-1-c} V[a] * V[b])
        
        sum_dp = 0
        for c in range(m):
            rem = (m - 1) - c
            # Compute convolution of V*V at index rem
            conv_v_v = 0
            for a in range(rem + 1):
                b = rem - a
                term = v[a] * v[b]
                conv_v_v = (conv_v_v + term) % MOD
            
            term_dp = dp[c] * conv_v_v
            sum_dp = (sum_dp + term_dp) % MOD
            
        dp[m] = (10 * sum_dp) % MOD
            
        # 2. Compute Prim[m]
        # dp[m] = Prim[m] + sum_{k=1 to m-1} Prim[k] dp[m-k]
        # Prim[m] = dp[m] - sum...
        
        sum_prim_dp = 0
        for k in range(1, m):
            term = prim[k] * dp[m-k]
            sum_prim_dp = (sum_prim_dp + term) % MOD
            
        prim[m] = (dp[m] - sum_prim_dp + MOD) % MOD
            
        # 3. Compute V[m]
        # V[m] = sum_{k=1 to m} (9/10 * Prim[k]) * V[m-k]
        
        sum_v = 0
        for k in range(1, m + 1):
            p_val = (prim[k] * 9 * inv10) % MOD
            term = p_val * v[m-k]
            sum_v = (sum_v + term) % MOD
            
        v[m] = sum_v
        
    # Calculate T(n)
    # Sum of (9/10 * dp[len]) for len = 3, 6, ..., n
    
    total = 0
    for m in range(1, limit + 1):
        term = (dp[m] * 9 * inv10) % MOD
        total = (total + term) % MOD
            
    return total

if __name__ == "__main__":
    print(solve())
