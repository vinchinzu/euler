# Project Euler Problem 884
#
# PROBLEM DESCRIPTION:
# <p>
# Starting from a positive integer $n$, at each step we subtract from $n$ the largest perfect cube not exceeding $n$, until $n$ becomes $0$.<br>
# For example, with $n = 100$ the procedure ends in $4$ steps:
# $$100 \xrightarrow{-4^3} 36 \xrightarrow{-3^3} 9 \xrightarrow{-2^3} 1 \xrightarrow{-1^3} 0.$$
# Let $D(n)$ denote the number of steps of the procedure. Thus $D(100) = 4$.</p>
# 
# <p>
# Let $S(N)$ denote the sum of $D(n)$ for all positive integers $n$ <b>strictly less</b> than $N$.<br>
# For example, $S(100) = 512$.</p>
# 
# <p>
# Find $S(10^{17})$.</p>
#

from __future__ import annotations

def integer_cbrt(n: int) -> int:
    """Returns the integer cube root of n."""
    if n < 0:
        return -integer_cbrt(-n)
    if n == 0:
        return 0
    # Floating point approximation
    x = int(n**(1/3))
    # Refine the guess
    if (x + 1)**3 <= n:
        x += 1
    elif x**3 > n:
        x -= 1
    return x

def solve() -> int:
    """Computes S(10^17)."""
    N = 10**17

    if N <= 1:
        return 0

    K_max = integer_cbrt(N - 1)

    # prefix_T[k] will store sum of T(i) for i in 1..k
    # where T(i) is the contribution of the full interval [i^3, (i+1)^3 - 1]
    # T(i) = Length + S(Length) = (3i^2+3i+1) + S(3i^2+3i+1)
    # The array needs to accommodate indices up to K_max.
    prefix_T = [0] * (K_max + 1)

    def recursive_S(n: int) -> int:
        if n <= 1:
            return 0
        k = integer_cbrt(n - 1)

        # Contribution from full intervals 1..k-1
        # Sum of T(i) for i=1 to k-1
        # prefix_T is 1-based index for k, so prefix_T[k-1] gives sum for i=1..k-1
        full_intervals_sum = prefix_T[k-1]

        # Contribution from partial interval [k^3, n-1]
        # Length L = (n-1) - k^3 + 1 = n - k^3
        L = n - k**3
        partial_sum = L + recursive_S(L)

        return full_intervals_sum + partial_sum

    for k in range(1, K_max + 1):
        # Calculate T(k)
        # Interval length for k is (k+1)^3 - k^3 = 3k^2 + 3k + 1
        L_k = 3*k*k + 3*k + 1

        # Calculate S(L_k).
        # We need S(L_k) to compute T(k).
        # recursive_S(L_k) will only access prefix_T for indices < k,
        # because L_k approx 3k^2, so new k' approx (3k^2)^(1/3) < k.
        val = recursive_S(L_k)

        term = L_k + val
        prefix_T[k] = prefix_T[k-1] + term

    return recursive_S(N)

if __name__ == "__main__":
    print(solve())
