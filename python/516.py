"""Project Euler Problem 516: 5-smooth totients.

Find the sum of all n ≤ N such that ϕ(n) is a Hamming number (5-smooth).
"""

from __future__ import annotations

from math import isqrt
from typing import List


def is_probable_prime(n: int) -> bool:
    """Check if n is probably prime using trial division."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    for i in range(3, isqrt(n) + 1, 2):
        if n % i == 0:
            return False
    return True


def solve() -> int:
    """Solve Problem 516."""
    N = 10**12
    M = 2**32

    # Generate all Hamming numbers
    hammings: List[int] = []
    n2 = 1
    while n2 <= N:
        n3 = n2
        while n3 <= N:
            n = n3
            while n <= N:
                hammings.append(n)
                n *= 5
            n3 *= 3
        n2 *= 2

    # Find good primes (p = h+1 where h is Hamming and p is prime)
    good_primes: List[int] = []
    for h in hammings:
        if h >= 5 and is_probable_prime(h + 1):
            good_primes.append(h + 1)

    good_primes.sort()

    ans = 0

    def helper(min_index: int, n: int) -> None:
        """Recursive helper."""
        nonlocal ans
        ans = (ans + n) % M
        for idx in range(min_index, len(good_primes)):
            p = good_primes[idx]
            if n * p > N:
                break
            helper(idx + 1, n * p)

    for h in hammings:
        helper(0, h)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
