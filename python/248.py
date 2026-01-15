"""Project Euler Problem 248: Numbers for which Euler's totient equals 13!.

Find the Nth smallest number n such that ϕ(n) = K!.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import isqrt
from typing import List


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def factorial(n: int) -> int:
    """Return n!."""
    result = 1
    for i in range(1, n + 1):
        result *= i
    return result


def all_divisors(n: int, primes: List[int]) -> List[int]:
    """Return all divisors of n."""
    divisors = [1]
    temp = n
    for p in primes:
        if temp % p == 0:
            size = len(divisors)
            power = 1
            while temp % p == 0:
                temp //= p
                power *= p
                for i in range(size):
                    divisors.append(divisors[i] * power)
    if temp > 1:
        size = len(divisors)
        for i in range(size):
            divisors.append(divisors[i] * temp)
    return divisors


def is_probable_prime(n: int) -> bool:
    """Check if n is probably prime."""
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


@dataclass
class Num:
    """Represents a number with its totient."""

    prod: int
    phi: int


def solve() -> int:
    """Solve Problem 248."""
    N = 150000
    K = 13
    Kf = factorial(K)

    nums: List[Num] = [Num(1, 1)]

    primes_list = sieve(K)
    for d in all_divisors(Kf, primes_list):
        p = Kf // d + 1
        if is_probable_prime(p):
            new_nums: List[Num] = []
            for num in nums:
                pe = 1
                while Kf % (num.phi * pe * (p - 1)) == 0:
                    pe *= p
                    new_nums.append(Num(num.prod * pe * p, num.phi * pe * (p - 1)))
            nums.extend(new_nums)

    # Filter and sort
    valid_nums = sorted([num.prod for num in nums if num.phi == Kf])
    return valid_nums[N - 1]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
