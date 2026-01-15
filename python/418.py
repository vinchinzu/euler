"""Project Euler Problem 418: Factorisation triples.

Find the minimum possible value of a+b+c if a*b*c = 43!.

We find all factors of 43! close to ³√(43!), and brute force over all
factorization triplets.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes to find all primes up to limit."""
    if limit < 2:
        return []
    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, isqrt(limit) + 1):
        if sieve[i]:
            for j in range(i * i, limit + 1, i):
                sieve[j] = False
    return [i for i in range(2, limit + 1) if sieve[i]]


def num_factors_in_factorial(n: int, p: int) -> int:
    """Count factors of p in n!."""
    count = 0
    power = p
    while power <= n:
        count += n // power
        power *= p
    return count


def factorial(n: int) -> int:
    """Compute n!."""
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result


def helper(
    index: int,
    primes: List[int],
    exponents: List[int],
    n: int,
    factors: List[int],
    L: float,
    R: float,
) -> None:
    """Recursively generate factors close to cube root."""
    if index == len(primes):
        if n > L * R:
            factors.append(n)
        return
    
    p = primes[index]
    # Don't include this prime
    helper(index + 1, primes, exponents, n, factors, L, R)
    
    # Include this prime with various exponents
    for e in range(exponents[index]):
        if float(n) * p > L / R:
            return
        n *= p
        helper(index + 1, primes, exponents, n, factors, L, R)


def solve() -> int:
    """Solve Problem 418."""
    N = 43
    L = factorial(N) ** (1.0 / 3.0)  # Cube root
    R = 0.99999
    
    primes = sieve_primes(N)
    exponents = [num_factors_in_factorial(N, p) for p in primes]
    
    factors: List[int] = []
    helper(0, primes, exponents, 1, factors, L, R)
    
    total = factorial(N)
    ans = float('inf')
    
    for f1 in factors:
        for f2 in factors:
            prod = f1 * f2
            if total % prod == 0:
                f3 = total // prod
                if f1 + f2 + f3 < ans:
                    ans = f1 + f2 + f3
    
    return int(ans)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
