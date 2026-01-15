"""Project Euler Problem 501: Eight Divisors.

Find the number of integers ≤ N with exactly 8 divisors.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def is_prime_small(n: int, primes: List[int]) -> bool:
    """Check if n is prime using precomputed primes."""
    if n < 2:
        return False
    for p in primes:
        if p * p > n:
            return True
        if n % p == 0:
            return False
    return True


def num_primes_small(limit: int, primes: List[int]) -> List[int]:
    """Count primes up to each number ≤ limit."""
    result = [0] * (limit + 1)
    pi_count = 0
    for i in range(1, limit + 1):
        if i <= len(primes) - 1 and primes[pi_count] == i:
            pi_count += 1
        result[i] = pi_count
    return result


def num_primes_large(n: int, small_primes: List[int]) -> int:
    """Count primes ≤ n using segmented sieve."""
    if n < 2:
        return 0
    if n <= len(small_primes) - 1:
        count = 0
        for p in small_primes:
            if p > n:
                break
            count += 1
        return count

    # Use segmented sieve for large n
    limit = isqrt(n)
    count = len([p for p in small_primes if p <= limit])

    # Count primes in [limit+1, n] using segmented sieve
    segment_size = max(limit, 10000)
    low = limit + 1
    total = count

    while low <= n:
        high = min(low + segment_size - 1, n)
        is_prime_seg = [True] * (high - low + 1)

        for p in small_primes:
            if p * p > high:
                break
            start = max(p * p, ((low + p - 1) // p) * p)
            for j in range(start, high + 1, p):
                is_prime_seg[j - low] = False

        for i in range(high - low + 1):
            if is_prime_seg[i]:
                total += 1

        low = high + 1

    return total


class QuotientValues:
    """Helper class to compute π(n/x) for various x."""

    def __init__(self, n: int, small_primes: List[int]):
        """Initialize with large n and small primes."""
        self.n = n
        self.small_primes = small_primes
        self.cache: dict[int, int] = {}

    def div(self, x: int) -> int:
        """Return π(n/x)."""
        if x == 0:
            return 0
        q = self.n // x
        if q in self.cache:
            return self.cache[q]
        result = num_primes_large(q, self.small_primes)
        self.cache[q] = result
        return result


def sq(n: int) -> int:
    """Square."""
    return n * n


def cb(n: int) -> int:
    """Cube."""
    return n * n * n


def solve() -> int:
    """Solve Problem 501."""
    N = 10**12
    L = int(N ** (2.0 / 3))

    primes = sieve_primes(L)
    num_primes_small_arr = num_primes_small(L, primes)
    num_primes = QuotientValues(N, primes)

    ans = 0

    # Count p*q*r with p < q < r
    for pi in range(len(primes)):
        p = primes[pi]
        if cb(p) > N:
            break
        for qi in range(pi + 1, len(primes)):
            q = primes[qi]
            if p * sq(q) > N:
                break
            ans += num_primes.div(p * q) - num_primes_small_arr[q]

    # Count p³*q
    for pi in range(len(primes)):
        p = primes[pi]
        if cb(p) > N:
            break
        ans += num_primes.div(cb(p))
        if cb(p) * p <= N:
            ans -= 1

    # Count p^7
    max_p7 = int(N ** (1.0 / 7))
    ans += num_primes_small_arr[max_p7]

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
