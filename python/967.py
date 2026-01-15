# Project Euler Problem 967
#
# PROBLEM DESCRIPTION:
# A positive integer $n$ is considered $B$-trivisible if the sum of all different prime factors of $n$ which are not larger than $B$ is divisible by $3$.
# For example, $175 = 5^2 \cdot 7$ is $10$-trivisible because $5 + 7 = 12$ which is divisible by $3$. Similarly, $175$ is $4$-trivisible because all primes dividing $175$ are larger than $4$, and the empty summation $0$ is divisible by $3$.
# On the other hand, $175$ is not $6$-trivisible because the sum of relevant primes is $5$ which is not divisible by $3$.
# Let $F(N, B)$ be the number of $B$-trivisible integers not larger than $N$.
# For example, $F(10, 4) = 5$, the $4$-trivisible numbers being $1,3,5,7,9$.
# You are also given $F(10, 10) = 3$ and $F(100, 10) = 41$.
# Find $F(10^{18}, 120)$.
#
# PYTHON IMPLEMENTATION NOTES:
# - Solve the problem described above
# - Implement solve() function
#

from __future__ import annotations
import math
from typing import List

def get_primes_up_to(limit: int) -> List[int]:
    """Get all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []

    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False

    for i in range(2, int(math.sqrt(limit)) + 1):
        if sieve[i]:
            for j in range(i * i, limit + 1, i):
                sieve[j] = False

    return [i for i in range(2, limit + 1) if sieve[i]]

def compute_local_sum(N: int, p: int, k: int) -> complex:
    """
    Compute sum_{m=0}^∞ ζ^{-k * v_p * m} * floor(N / p^m)
    where ζ = e^{2πi/3}, v_p = p % 3
    """
    v_p = p % 3
    zeta = complex(-0.5, math.sqrt(3)/2)  # primitive 3rd root of unity

    # ζ^{-k * v_p} = ζ^{3 - k * v_p mod 3}
    exponent = (3 - (k * v_p) % 3) % 3
    alpha = zeta ** exponent

    if exponent == 0:
        # α = 1, geometric series diverges
        # Need to compute sum_{m=0}^∞ floor(N / p^m)
        # This is approximately N * product_{j=0}^∞ (1 + 1/p^j) but actually
        # sum floor(N/p^m) = sum_{m=0}^∞ floor(N / p^m)
        # This can be computed by iterating until p^m > N
        total = 0
        pm = 1
        m = 0
        while pm <= N:
            total += N // pm
            if pm > N // p:  # prevent overflow
                break
            pm *= p
            m += 1
        return complex(total, 0)
    else:
        # α ≠ 1, sum converges
        total = complex(0, 0)
        pm = 1
        alpha_m = complex(1, 0)
        m = 0
        while pm <= N:
            total += alpha_m * (N // pm)
            alpha_m *= alpha
            if pm > N // p:
                break
            pm *= p
            m += 1
        return total

def compute_contrib(M: int, Q: List[int], index: int = 0, current_prod: int = 1, sign: int = 1) -> int:
    """Recursive function to compute the alternating sum for coprime counting."""
    if index == len(Q):
        return sign * (M // current_prod) if current_prod <= M else 0

    p = Q[index]
    # Not include p
    result = compute_contrib(M, Q, index + 1, current_prod, sign)
    # Include p
    if current_prod <= M // p:
        result += compute_contrib(M, Q, index + 1, current_prod * p, -sign)
    return result

def f_inclusion_exclusion(N: int, B: int) -> int:
    """
    Compute F(N, B) using inclusion-exclusion over subsets of primes ≤ B.
    """
    primes = get_primes_up_to(B)
    n = len(primes)
    total = 0

    for mask in range(1 << n):
        prod_S = 1
        sum_mod = 0
        for i in range(n):
            if mask & (1 << i):
                p = primes[i]
                sum_mod = (sum_mod + p % 3) % 3
                if prod_S > N // p:
                    prod_S = N + 1
                    break
                prod_S *= p

        if prod_S > N or sum_mod != 0:
            continue

        M = N // prod_S

        # Q = indices not in mask
        Q_indices = [i for i in range(n) if not (mask & (1 << i))]
        Q_primes = [primes[i] for i in Q_indices]
        # Sort in descending order to prune earlier
        Q_primes.sort(reverse=True)

        contrib = compute_contrib(M, Q_primes)
        total += contrib

    return total

def solve() -> int:
    """
    Problem 967: A positive integer $n$ is considered $B$-trivisible if the sum of all different prime factors of $n$ which are not larger than $B$ is divisible by $3$.

For example, $175 = 5^2 \cdot 7$ is $10$-trivisible because $5 + 7 = 12$ which is divisible by $3$. Similarly, $175$ is $4$-trivisible because all primes dividing $175$ are larger than $4$, and the empty summation $0$ is divisible by $3$.
On the other hand, $175$ is not $6$-trivisible because the sum of relevant primes is $5$ which is not divisible by $3$.
Let $F(N, B)$ be the number of $B$-trivisible integers not larger than $N$.
For example, $F(10, 4) = 5$, the $4$-trivisible numbers being $1,3,5,7,9$.
You are also given $F(10, 10) = 3$ and $F(100, 10) = 41$.
Find $F(10^{18}, 120)$.

    """
    return f_inclusion_exclusion(10**18, 120)

if __name__ == "__main__":
    print(solve())
