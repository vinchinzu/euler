"""Project Euler Problem 204: Generalised Hamming Numbers.

Find the number of integers up to N with no prime factors larger than K.
"""

from __future__ import annotations

from typing import List


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def solve() -> int:
    """Solve Problem 204."""
    N = 10**9
    K = 100

    nums: List[int] = [1]
    primes_list = sieve(K)

    for p in primes_list:
        size = len(nums)
        for i in range(size):
            prod = nums[i]
            while True:
                prod *= p
                if prod > N:
                    break
                nums.append(prod)

    return len(nums)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
