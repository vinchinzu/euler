"""Project Euler Problem 467: Superstring of prime and composite digital roots.

Let P_D and C_D be the sequences of digital roots of the k'th prime and k'th
composite numbers, respectively. Find the smallest common super-sequence of
the first N digits of P_D and the first N digits of C_D.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[bool]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return is_prime


def digital_root(n: int) -> int:
    """Compute digital root."""
    while n >= 10:
        n = sum(int(d) for d in str(n))
    return n


def solve() -> int:
    """Solve Problem 467."""
    N = 10_000
    M = 10**9 + 7
    B = 10

    # Generate primes and composites
    is_prime = sieve_primes(2 * N)
    P: List[int] = []
    n = 2
    while len(P) < N:
        if is_prime[n]:
            P.append(digital_root(n))
        n += 1

    C: List[int] = []
    n = 2
    while len(C) < N:
        if not is_prime[n]:
            C.append(digital_root(n))
        n += 1

    # DP for shortest common supersequence
    dp = [[0] * (N + 1) for _ in range(N + 1)]
    move_i = [[False] * (N + 1) for _ in range(N + 1)]

    for i in range(N, -1, -1):
        for j in range(N, -1, -1):
            if i < N and j < N and P[i] == C[j]:
                dp[i][j] = 1 + dp[i + 1][j + 1]
            elif i < N or j < N:
                val_i = dp[i + 1][j] if i < N else float("inf")
                val_j = dp[i][j + 1] if j < N else float("inf")
                dp[i][j] = 1 + min(val_i, val_j)
                move_i[i][j] = val_i < val_j or (
                    val_i == val_j and i < N and (j >= N or P[i] < C[j])
                )

    # Reconstruct answer
    ans = 0
    i, j = 0, 0
    while i < N or j < N:
        if i < N and j < N and P[i] == C[j]:
            digit = P[i]
            i += 1
            j += 1
        elif move_i[i][j]:
            digit = P[i]
            i += 1
        else:
            digit = C[j]
            j += 1
        ans = (B * ans + digit) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
