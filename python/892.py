# Project Euler Problem 892
#
# PROBLEM DESCRIPTION:
# <p>
# Consider a circle where $2n$ distinct points have been marked on its circumference.</p>
# 
# <p>
# A <i>cutting</i> $C$ consists of connecting the $2n$ points with $n$ line segments, so that no two line segments intersect, including on their end points. The $n$ line segments then cut the circle into $n + 1$ pieces.
# Each piece is painted either black or white, so that adjacent pieces are opposite colours.
# Let $d(C)$ be the absolute difference between the numbers of black and white pieces under the cutting $C$.</p>
# 
# <p>
# Let $D(n)$ be the sum of $d(C)$ over all different cuttings $C$.
# For example, there are five different cuttings with $n = 3$.</p>
# 
# <div style="text-align:center;">
# <img src="resources/images/0892_Zebra.png?1714876283" alt="0892_Zebra.png"></div>
# 
# <p>
# The upper three cuttings all have $d = 0$ because there are two black and two white pieces; the lower two cuttings both have $d = 2$ because there are three black and one white pieces.
# Therefore $D(3) = 0 + 0 + 0 + 2 + 2 = 4$. 
# You are also given $D(100) \equiv 1172122931\pmod{1234567891}$.</p>
# 
# <p>
# Find $\displaystyle \sum_{n=1}^{10^7} D(n)$. Give your answer modulo $1234567891$.</p>
#
# SOLUTION:
# Through analysis and pattern matching with small values (computed via DP), we derived the following formulas:
# For even n = 2m: D(2m) = (1/2) * binom(2m, m)^2
# For odd n = 2m + 1: D(2m+1) = (2m / (m + 1)) * binom(2m, m)^2
#
# We compute the sum for n = 1 to 10^7 modulo 1234567891.
# The algorithm iterates m from 1 to 5*10^6, updating the central binomial coefficient and accumulating terms.
# Modular inverse is precomputed for O(1) access.
# Time Complexity: O(N)
# Space Complexity: O(N) for inverses (approx 20MB for N=10^7)

import sys

# Increase recursion depth just in case, though not using deep recursion.
sys.setrecursionlimit(2000)

MOD = 1234567891

def solve():
    N = 10**7
    LIMIT = N // 2 + 2

    # Precompute modular inverses
    inv = [0] * (LIMIT + 1)
    inv[1] = 1
    for i in range(2, LIMIT + 1):
        inv[i] = (MOD - MOD // i) * inv[MOD % i] % MOD

    M = N // 2
    comb = 1 # represents binom(2m, m) for m=0 initially (but loop starts m=1)

    total_sum = 0
    inv2 = inv[2]

    # Iterate m from 1 to M
    for m in range(1, M + 1):
        # Update combination to binom(2m, m)
        # binom(2m, m) = binom(2(m-1), m-1) * (2m(2m-1)) / (m*m)
        #              = prev_comb * 2(2m-1) / m
        comb = comb * 2 * (2 * m - 1) % MOD * inv[m] % MOD

        Tm = comb * comb % MOD

        # Add D(2m)
        term_even = Tm * inv2 % MOD
        total_sum = (total_sum + term_even) % MOD

        # Add D(2m+1) if within range
        if 2 * m + 1 <= N:
            # D(2m+1) = 2m/(m+1) * Tm
            term_odd = Tm * 2 * m % MOD * inv[m+1] % MOD
            total_sum = (total_sum + term_odd) % MOD

    print(total_sum)

if __name__ == "__main__":
    solve()
