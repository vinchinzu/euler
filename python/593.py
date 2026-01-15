"""Project Euler Problem 593: Fleeting Medians.

Define S(k) = (p_k)^k (mod M) where p_k is the kth prime number, define
S2(k) = S(k) + S(⌊k/D⌋+1). Find F(N, K), the sum of the medians of all
subsequences of length K from the first N terms of S2.

We compute the sliding median by maintaining two heaps, a max heap with
the smaller K/2 elements, and a min heap with the larger K/2 elements.
"""

from __future__ import annotations

import heapq
from typing import List

from sympy import primerange


def generator(m: int) -> int:
    """Find a generator modulo m."""
    phi = m - 1  # For prime m
    factors = []
    temp = phi
    p = 2
    while p * p <= temp:
        if temp % p == 0:
            factors.append(p)
            while temp % p == 0:
                temp //= p
        p += 1
    if temp > 1:
        factors.append(temp)

    for g in range(2, m):
        is_gen = True
        for f in factors:
            if pow(g, phi // f, m) == 1:
                is_gen = False
                break
        if is_gen:
            return g
    return 1


def solve() -> float:
    """Solve Problem 593."""
    N = 10**7
    K = 100_000
    M = 10007
    D = 10000

    # Precompute primes
    primes = list(primerange(2, N + 100))
    primes = primes[: N + 1]

    # Precompute generator powers
    g = generator(M)
    pows = [1] * M
    for i in range(1, M):
        pows[i] = (pows[i - 1] * g) % M

    # Precompute logs
    logs = [0] * M
    gp = 1
    for i in range(M):
        logs[gp] = i
        gp = (gp * g) % M

    # Compute S
    S = [0] * (N + 2)
    k = 1
    for i in range(2, len(primes)):
        if k > N + 1:
            break
        p = primes[i - 2] if i >= 2 else 2
        if p == M:
            S[k] = 0
        else:
            exp = (k % (M - 1)) * logs[p % M] % (M - 1)
            S[k] = pows[exp]
        k += 1

    # Compute S2
    S2 = [0] * (N + 2)
    for k in range(1, N + 2):
        S2[k] = S[k] + S[k // D + 1]

    # Sliding median using two heaps
    first_half: List[tuple[int, int]] = []  # Max heap (negate values)
    second_half: List[tuple[int, int]] = []  # Min heap

    # Initialize first window
    for i in range(1, K + 1):
        heapq.heappush(first_half, (-S2[i], i))

    # Balance heaps
    while len(first_half) > K // 2:
        val, key = heapq.heappop(first_half)
        heapq.heappush(second_half, (S2[key], key))

    F = 0.0

    for next_val in range(K + 1, N + 2):
        # Add median
        median = (S2[first_half[0][1]] + S2[second_half[0][1]]) / 2.0
        F += median

        # Remove old element
        old_key = next_val - K
        removed = False
        for i, (val, key) in enumerate(first_half):
            if key == old_key:
                first_half.pop(i)
                heapq.heapify(first_half)
                removed = True
                break
        if not removed:
            for i, (val, key) in enumerate(second_half):
                if key == old_key:
                    second_half.pop(i)
                    heapq.heapify(second_half)
                    removed = True
                    break

        # Add new element
        if S2[next_val] <= S2[first_half[0][1]]:
            heapq.heappush(first_half, (-S2[next_val], next_val))
        else:
            heapq.heappush(second_half, (S2[next_val], next_val))

        # Rebalance
        if len(first_half) > len(second_half):
            val, key = heapq.heappop(first_half)
            heapq.heappush(second_half, (S2[key], key))
        elif len(second_half) > len(first_half):
            val, key = heapq.heappop(second_half)
            heapq.heappush(first_half, (-S2[key], key))

    return F


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.1f}")


if __name__ == "__main__":
    main()
