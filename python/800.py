"""Project Euler Problem 800: Hybrid Integers.

Find the number of integers of the form p^q q^p ≤ N^N where p and q are
distinct primes.

For each prime p, we just use binary search to find the maximum prime q
such that p^q q^p ≤ N^N. Since the numbers are large, we compare the
logarithms of the values.
"""

from __future__ import annotations

import math
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


def ilog2(n: int) -> int:
    """Integer logarithm base 2."""
    return n.bit_length() - 1


def solve() -> int:
    """Solve Problem 800."""
    N = 800_800
    max_prime = N * (ilog2(N) + 1)
    primes = sieve_primes(max_prime)

    ans = 0
    for i, p in enumerate(primes):
        low = i
        high = len(primes)
        while low + 1 < high:
            mid = (low + high) // 2
            q = primes[mid]
            if p * math.log(q) + q * math.log(p) < N * math.log(N):
                low = mid
            else:
                high = mid
        ans += low - i

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
