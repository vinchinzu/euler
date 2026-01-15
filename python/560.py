"""Project Euler Problem 560: Coprime Nim.

Two players play a Nim variant with K piles, taking turns removing t stones
from a pile of s stones where GCD(t,s) = 1, until the player taking the last
stone wins. Find the number of configurations of K piles, each with 1 to N-1
stones (inclusive), such that the first player has a losing position.

We can see by induction that the nimber of a pile of s stones is 0 if s is
even, 1 if s=1, and π(p) where p is the smallest prime dividing s otherwise.
This is because if s is even, then a valid move will result in an odd number
of stones left; otherwise, it is possible to leave any prime q smaller than
any of s's divisors, but clearly not possible to leave any number divisible
by a prime factor of s.

We can then compute the number of values s with any given nimber, and use
Hadamard to compute the number of K-pile configurations with zero nimber.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def xor_convolution_power(arr: List[int], exp: int, mod: int) -> List[int]:
    """Compute arr raised to exp power using XOR convolution."""
    n = len(arr)
    # Ensure n is a power of 2
    while n & (n - 1):
        n += 1
    if len(arr) < n:
        arr = arr + [0] * (n - len(arr))

    def fwht(a: List[int], inv: bool = False) -> List[int]:
        """Fast Walsh-Hadamard Transform for XOR convolution."""
        n = len(a)
        a = a[:]
        j = 0
        for i in range(1, n):
            bit = n >> 1
            while j & bit:
                j ^= bit
                bit >>= 1
            j ^= bit
            if i < j:
                a[i], a[j] = a[j], a[i]

        length = 2
        while length <= n:
            for i in range(0, n, length):
                for j in range(i, i + length // 2):
                    u = a[j]
                    v = a[j + length // 2]
                    a[j] = (u + v) % mod
                    a[j + length // 2] = (u - v) % mod
            length <<= 1
        if inv:
            inv_n = pow(n, mod - 2, mod)
            a = [(x * inv_n) % mod for x in a]
        return a

    # Apply FWHT
    transformed = fwht(arr)
    # Raise to power element-wise
    powered = [pow(x, exp, mod) for x in transformed]
    # Inverse transform
    result = fwht(powered, inv=True)
    return result


def solve() -> int:
    """Solve Problem 560."""
    N = 10**7
    K = 10**7
    M = 10**9 + 7

    primes = sieve(N)
    nimbers = [0] * N
    nimbers[1] = 1

    # Assign nimbers based on smallest prime factor
    for i in range(1, len(primes)):
        p = primes[i]
        for j in range(p, N, 2 * p):
            if nimbers[j] == 0:
                nimbers[j] = i + 1

    # Count nimbers
    max_nimber = max(nimbers) if nimbers else 0
    counts = [0] * (len(primes) + 1)
    for i in range(1, N):
        counts[nimbers[i]] += 1

    # Compute K-th power using XOR convolution
    pow_counts = xor_convolution_power(counts, K, M)
    return pow_counts[0] % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
