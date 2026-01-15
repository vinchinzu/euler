"""Project Euler Problem 510: Tangent Circles.

Find the sum of all r_A + r_B + r_C for all r_A ≤ r_B ≤ N such that two
circles with radii r_A and r_B are tangent to each other and a line L,
and a circle with radii r_C is internally tangent to all three.
"""

from __future__ import annotations

from math import isqrt
from typing import Dict, List, Set


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


def prime_factorization(n: int, primes: List[int]) -> Dict[int, int]:
    """Prime factorization."""
    factors: Dict[int, int] = {}
    for p in primes:
        if p * p > n:
            break
        while n % p == 0:
            factors[p] = factors.get(p, 0) + 1
            n //= p
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def all_divisors(n: int, prime_factors: Set[int]) -> List[int]:
    """Get all divisors of n using prime factors."""
    divisors = [1]
    for p in prime_factors:
        new_divisors = []
        power = p
        while n % power == 0:
            for d in divisors:
                new_divisors.append(d * power)
            power *= p
        divisors.extend(new_divisors)
    return sorted(set(divisors))


def mobius_function(limit: int, primes: List[int]) -> List[int]:
    """Compute Mobius function."""
    mu = [1] * (limit + 1)
    is_square_free = [True] * (limit + 1)
    for p in primes:
        if p > limit:
            break
        for j in range(p, limit + 1, p):
            mu[j] *= -1
            if j % (p * p) == 0:
                is_square_free[j] = False
    for i in range(limit + 1):
        if not is_square_free[i]:
            mu[i] = 0
    return mu


def sq(n: int) -> int:
    """Square."""
    return n * n


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 510."""
    N = 10**9
    L = isqrt(N)

    primes = sieve_primes(L)
    mu = mobius_function(L, primes)

    ans = 0
    for g in range(1, isqrt(N) + 1):
        if sq(g) > N:
            break
        n = N // sq(g)
        for b in range(1, isqrt(n) + 1):
            if sq(b) > n:
                break
            b_factors = prime_factorization(b, primes)
            divisors = all_divisors(sq(b), set(b_factors.keys()))
            for d in divisors:
                a = d - b
                if a > 0 and a <= b:
                    c = b - sq(b) // d
                    ans += (
                        mu[g]
                        * sq(g)
                        * (sq(a) + sq(b) + sq(c))
                        * tr(n // sq(b))
                    )

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
