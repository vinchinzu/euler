"""Project Euler Problem 829: Binary Factor Trees.

Let the binary factor tree T(n) of an integer n consist of a single node
if n is prime, otherwise the left and right subtrees are T(a) and T(b)
where a,b are positive integers with a≤b and b-a smallest. Find
Σ_{n=2}^N M(n), where M(n) is the smallest number with the same binary
factor tree shape as n!! = n(n-2)(n-4)... .

We can compute the shape of T(n) directly. For each n, to find the a,b of
the subtrees, we use bidirectional search by computing all divisors of a
divisor s of n, then for each divisor of n/s we can quickly find the
divisor of s that brings us closest to √n.

Given T(n) of k nodes, we then enumerate all possible products of k
primes, computing the binary factor tree of each to check if it has the
same shape as T(n). The only optimization is to exit if the product is
larger than our current best M(n).
"""

from __future__ import annotations

from functools import lru_cache
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


def prime_factorize(n: int) -> Dict[int, int]:
    """Factorize n."""
    factors: Dict[int, int] = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def get_all_divisors(n: int, prime_factors: Set[int]) -> List[int]:
    """Get all divisors of n."""
    factors = prime_factorize(n)
    divisors = [1]
    for p, e in factors.items():
        new_divs = []
        for d in divisors:
            for i in range(e + 1):
                new_divs.append(d * (p**i))
        divisors = new_divs
    return sorted(divisors)


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


@lru_cache(maxsize=None)
def T(n: int, N: int, primes: tuple) -> str:
    """Compute binary factor tree shape."""
    if n <= N and n in primes:
        return "."
    best_d = best_divisor(n, N, primes)
    return "(" + T(best_d, N, primes) + T(n // best_d, N, primes)


def best_divisor(n: int, N: int, primes: tuple) -> int:
    """Find largest divisor of n that is at most √n."""
    factors = prime_factorize(n)
    num_divisors = 1
    for e in factors.values():
        num_divisors *= e + 1

    s = 1
    num_s_divisors = 1
    for p in sorted(factors.keys()):
        e = factors[p]
        s *= p**e
        num_s_divisors *= e + 1
        if num_s_divisors * num_s_divisors > num_divisors:
            break

    lefts = sorted(get_all_divisors(s, set(factors.keys())))
    sqrt_n = isqrt(n)
    max_d = 1

    for right in get_all_divisors(n // s, set(factors.keys())):
        # Find largest left such that left * right <= sqrt_n
        target = sqrt_n // right if right > 0 else sqrt_n
        for left in reversed(lefts):
            if left <= target:
                max_d = max(max_d, left * right)
                break

    return max_d


def solve() -> int:
    """Solve Problem 829."""
    N = 31
    primes_list = sieve_primes(N)
    primes_tuple = tuple(primes_list)

    ans = 0
    for n in range(2, N + 1):
        # Compute n!!
        ndf = 1
        for i in range(n, 0, -2):
            ndf *= i

        # Count total factors
        factors_ndf = prime_factorize(ndf)
        k = sum(factors_ndf.values())

        # Find smallest number with same tree shape
        target_shape = T(ndf, N, primes_tuple)
        res = ndf

        def helper(k_remaining: int, min_p: int, current_n: int) -> None:
            """Helper to find smallest matching number."""
            nonlocal res
            if k_remaining == 0:
                if T(current_n, N, primes_tuple) == target_shape:
                    res = min(res, current_n)
                return

            for p_idx in range(min_p, len(primes_list)):
                p = primes_list[p_idx]
                if current_n * (p**k_remaining) > res:
                    break
                helper(k_remaining - 1, p_idx, current_n * p)

        helper(k, 0, 1)
        ans += res

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
