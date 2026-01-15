"""Project Euler Problem 779: Prime Factor and Order.

Let f_K(n) = (α(n)-1)/p(n)^K, where p(n) is the smallest prime dividing
n and α(n) is its p-adic order. Find Σ_{K=1}^∞ lim_{N->∞} 1/N
Σ_{n=2}^N f_K(n).

Consider any p(n) and a(n). The fraction of integers with that p(n) and
a(n) is 1/p^a (the integers divisible by p^a) times (p-1)/p (the integers
not divisible by p^{a+1}) times Π_{q<p} (q-1)/q (the integers without
any smaller prime factors). So this contributes f_K(n) 1/p^a (p-1)/p
Π_{q<p} (q-1)/q to the sum. Summing this over all a and K gives:

S_p = Σ_a Σ_K (a-1)/p^K 1/p^a (p-1)/p Π_{q<p} (q-1)/q
    = Σ_a (a-1) 1/p^{a+1} Π_{q<p} (q-1)/q
    = 1 / (p(p-1)²) Π_{q<p} (1-1/q).

Summing this for enough p gives the answer.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def solve() -> str:
    """Solve Problem 779."""
    L = 100_000
    primes = sieve_primes(L)
    ans = 0.0

    for i, p in enumerate(primes):
        res = 1.0 / p / ((p - 1) ** 2)
        for j in range(i):
            res *= 1 - 1.0 / primes[j]
        ans += res

    return f"{ans:.12f}"


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
