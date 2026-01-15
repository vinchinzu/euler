"""Project Euler Problem 474: Last digits of divisors.

Find the number of divisors of N! whose last digits equal K.
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


def num_factors_in_factorial(n: int, p: int) -> int:
    """Count factors of p in n!."""
    count = 0
    power = p
    while power <= n:
        count += n // power
        power *= p
    return count


def lcm(a: int, b: int) -> int:
    """Least common multiple."""
    return a * b // gcd(a, b)


def iceil_pow(n: int, base: int) -> int:
    """Smallest power of base >= n."""
    result = 1
    while result < n:
        result *= base
    return result


def lphi(n: int) -> int:
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


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    t, new_t = 0, 1
    r, new_r = m, a
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Inverse does not exist")
    if t < 0:
        t += m
    return t


def solve() -> int:
    """Solve Problem 474."""
    N = 10**6
    K = 65_432
    M = 10**16 + 61
    B = 10

    res = 1
    primes = sieve_primes(N)
    for p in primes:
        if gcd(p, B) == 1:
            res = (res * (1 + num_factors_in_factorial(N, p))) % M

    r = lphi(lcm(K, iceil_pow(K, B)) // K)
    ans = (res * mod_inv(r, M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
