"""Project Euler Problem 533: Carmichael Lambda Function.

Let λ(n) be the smallest positive integer m such that a^m ≡ 1 (mod n) for
all (a, n) = 1. Find the smallest m such that λ(k) ≥ N for all k ≥ m.

By Carmichael's Theorem, λ(Π p_i^e_i) = LCM_i λ(p_i^e_i), and
λ(p_i^e_i) = ϕ(p_i^e_i) with the exception of p=2, e≥3, where
λ(p_i^e_i) = ϕ(p_i^e_i) / 2.
"""

from __future__ import annotations

import math
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(math.sqrt(limit)) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def pre_phi(limit: int) -> List[int]:
    """Precompute Euler's totient function."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:
            for j in range(i, limit + 1, i):
                phi[j] = phi[j] // i * (i - 1)
    return phi


def is_prime(n: int, primes: List[int]) -> bool:
    """Check if n is prime."""
    if n < 2:
        return False
    for p in primes:
        if p * p > n:
            return True
        if n % p == 0:
            return False
    return True


def solve() -> int:
    """Solve Problem 533."""
    N = 20_000_000
    M = 10**9

    phi = pre_phi(N)
    primes = sieve_primes(int(math.sqrt(N)) + 100)

    logs = [0.0] * N
    mods = [1] * N

    for d in range(1, N):
        if is_prime(d + 1, primes):
            p = d + 1
            log_p = math.log(p)
            pe = 1
            while d * pe < N:
                for n in range(0, N, d * pe):
                    logs[n] += log_p
                    mods[n] = (mods[n] * p) % M
                pe *= p
            if p == 2:
                for n in range(0, N, 4):
                    logs[n] += log_p
                    mods[n] = (mods[n] * p) % M
            for n in range(0, N, d):
                mods[n] %= M

    max_log = 0.0
    ans = 0
    for n in range(1, N):
        if logs[n] > max_log:
            max_log = logs[n]
            ans = (mods[n] + 1) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
