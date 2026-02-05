#!/usr/bin/env python3
"""
Project Euler Problem 374: Maximum Integer Partition Product

For partitions into distinct parts, find sum of f(n)*m(n) for 1 <= n <= 10^14 mod 982451653,
where f(n) is max product and m(n) is number of parts achieving that product.

Key insight: For n = T_k + r where T_k = k(k+1)/2 and 0 <= r <= k:

  Case 1: 0 <= r <= k-2
    Partition: {2,...,k+1} minus element (k-r)
    f(n) = (k+1)! / (k-r)
    m(n) = k - 1

  Case 2: r = k-1
    Partition: {3,...,k+2} minus element (k+1)
    f(n) = (k+2)! / (2*(k+1))
    m(n) = k - 1

  Case 3: r = k (n = T_{k+1} - 1)
    Partition: {2,...,k+1}
    f(n) = (k+1)!
    m(n) = k

For k=1,2 the formulas don't apply; handle those cases directly.
"""

import math

MOD = 982451653

def solve():
    """Solve Problem 374."""
    N = 10**14

    # Find K such that T_K <= N < T_{K+1}
    K = int(math.isqrt(2 * N))
    while K * (K + 1) // 2 > N:
        K -= 1
    while (K + 1) * (K + 2) // 2 <= N:
        K += 1

    max_k = K + 10

    # Precompute factorials mod MOD
    fact = [1] * (max_k + 3)
    for i in range(1, max_k + 3):
        fact[i] = fact[i-1] * i % MOD

    # Precompute modular inverses using linear sieve
    inv = [0] * (max_k + 3)
    inv[1] = 1
    for i in range(2, max_k + 3):
        inv[i] = (MOD - MOD // i) * inv[MOD % i] % MOD

    # Precompute harmonic sums: harmonic_sum[k] = sum_{j=2}^{k} 1/j mod MOD
    harmonic_sum = [0] * (max_k + 3)
    for i in range(2, max_k + 3):
        harmonic_sum[i] = (harmonic_sum[i-1] + inv[i]) % MOD

    total = 0

    # Handle k=1: n=1,2
    if N >= 1:
        total = (total + 1) % MOD
    if N >= 2:
        total = (total + 2) % MOD

    # Handle k=2: n=3,4,5
    if N >= 3:
        total = (total + 3) % MOD
    if N >= 4:
        total = (total + 4) % MOD
    if N >= 5:
        total = (total + 12) % MOD

    # For k >= 3, use the formulas
    inv2 = inv[2]

    for k in range(3, K + 1):
        T_k = k * (k + 1) // 2

        # Determine r_max (could be partial for the last k)
        if k < K:
            r_max = k
        else:
            r_max = min(k, N - T_k)

        # Case 1: r from 0 to min(k-2, r_max)
        r1 = min(k - 2, r_max)
        if r1 >= 0:
            # sum_{r=0}^{r1} 1/(k-r) = sum_{j=k-r1}^{k} 1/j
            j_min = k - r1
            if j_min <= 1:
                sum_inv = (1 + harmonic_sum[k]) % MOD
            else:
                sum_inv = (harmonic_sum[k] - harmonic_sum[j_min - 1] + MOD) % MOD
            contrib = fact[k + 1] * (k - 1) % MOD * sum_inv % MOD
            total = (total + contrib) % MOD

        # Case 2: r = k-1
        if r_max >= k - 1:
            contrib = fact[k + 2] * (k - 1) % MOD * inv2 % MOD * inv[k + 1] % MOD
            total = (total + contrib) % MOD

        # Case 3: r = k
        if r_max >= k:
            contrib = fact[k + 1] * k % MOD
            total = (total + contrib) % MOD

    return total


if __name__ == "__main__":
    print(solve())
