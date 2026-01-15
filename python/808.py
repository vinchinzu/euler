"""Project Euler Problem 808: Reversible Prime Squares.

Find the sum of the smallest N positive integers which are squares of a
prime, the reverse is also a square of a prime, but is not a palindrome.
"""

from __future__ import annotations

from math import isqrt
from typing import List, Set


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


def solve() -> int:
    """Solve Problem 808."""
    N = 50
    L = 100000000

    primes = sieve_primes(L)
    squares: Set[int] = {p * p for p in primes}

    nums: List[int] = []
    for p in primes:
        num = p * p
        reversed_str = str(num)[::-1]
        reversed_num = int(reversed_str)
        if reversed_num != num and reversed_num in squares:
            nums.append(num)
            if len(nums) == N:
                break

    return sum(nums)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
