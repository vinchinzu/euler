"""Project Euler Problem 484: Arithmetic derivative.

Compute Σ_{k=2}^N GCD(k, k'), where k' is the arithmetic derivative of k.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def euler_totient(n: int) -> int:
    """Euler's totient function."""
    result = n
    p = 2
    while p * p <= n:
        if n % p == 0:
            while n % p == 0:
                n //= p
            result = result // p * (p - 1)
        p += 1
    if n > 1:
        result = result // n * (n - 1)
    return result


def solve() -> int:
    """Solve Problem 484."""
    N = 5 * 10**15
    primes = sieve_primes(isqrt(N))
    ans = 0

    def helper(min_index: int, mult: int, n: int, primes_list: List[int]) -> None:
        """Recursive helper."""
        nonlocal ans
        ans += mult * n
        for i in range(min_index, len(primes_list)):
            p = primes_list[i]
            if n * p > N:
                break
            pe = p
            e = 1
            while n * pe <= N:
                if e % p == 0:
                    correction = pe * (p + 1)
                else:
                    correction = pe // p * euler_totient(p)
                helper(i + 1, mult * correction, n * pe, primes_list)
                pe *= p
                e += 1

    helper(0, 1, 1, primes)
    ans -= 1  # Subtract k=1
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
