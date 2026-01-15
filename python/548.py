"""Project Euler Problem 548: Gozinta Chains.

Let g(n) be the number of gozinta chains for n, i.e. a sequence {1, a, b, ... n}
where each term properly divides the next. Find the sum of all n such that g(n)=n.

Note that g(1) = 1 and g(n) = Σ_{d|n,d<n} g(d), which depends only on the
exponents of the prime factorization of n.
"""

from __future__ import annotations

from functools import lru_cache
from math import isqrt
from typing import Dict, List, Tuple


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


def ilog2(n: int) -> int:
    """Integer logarithm base 2."""
    if n <= 0:
        return 0
    result = 0
    while n > 1:
        n >>= 1
        result += 1
    return result


def parity(n: int) -> int:
    """Return (-1)^n."""
    return 1 if n % 2 == 0 else -1


def sq(n: int) -> int:
    """Square."""
    return n * n


def pow(n: int, e: int) -> int:
    """Power."""
    return n**e


def solve() -> int:
    """Solve Problem 548."""
    N = 10**16
    primes = sieve_primes(ilog2(N) + 10)
    cache: Dict[Tuple[int, ...], int] = {}

    def g(es: Tuple[int, ...]) -> int:
        """Compute g for exponents."""
        if not es:
            return 1
        
        if es in cache:
            return cache[es]
        
        result = 0
        n_subsets = 1 << len(es)
        for subset in range(1, n_subsets):
            fs = list(es)
            for i in range(len(es)):
                if (subset >> i) & 1:
                    fs[i] = es[i] - 1
            fs = tuple(sorted([f for f in fs if f > 0], reverse=True))
            bit_count = bin(subset).count("1")
            if not fs:
                result -= parity(bit_count)
            else:
                result -= parity(bit_count) * 2 * g(fs)
        
        cache[es] = result
        return result

    def has_exponents(n: int, es: Tuple[int, ...]) -> bool:
        """Check if n has exponents es."""
        es_list = list(es)
        factor = 2
        while factor * factor <= n and es_list:
            if pow(factor, es_list[0]) > n:
                break
            e = 0
            while n % factor == 0:
                n //= factor
                e += 1
            if e > 0:
                if e not in es_list:
                    return False
                es_list.remove(e)
            factor += 1
        
        if n > 1:
            if 1 not in es_list:
                return False
            es_list.remove(1)
        
        return len(es_list) == 0

    ans = 0

    def helper(es: List[int], n: int) -> None:
        """Recursive helper."""
        nonlocal ans
        g_val = g(tuple(sorted(es, reverse=True)))
        if g_val <= N and has_exponents(g_val, tuple(sorted(es, reverse=True))):
            ans += g_val
        
        max_c = es[-1] if es else float("inf")
        for c in range(1, int(max_c) + 1):
            if len(es) >= len(primes):
                break
            new_n = n * pow(primes[len(es)], c)
            if new_n > N:
                break
            es.append(c)
            helper(es, new_n)
            es.pop()

    helper([], 1)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
