"""Project Euler Problem 233: Lattice points on a circle.

Find the number of positive integers k≤N such that there are exactly K lattice
points on the circle through (0, 0), (k, 0), (0, k), and (k, k).
"""

from __future__ import annotations

from math import isqrt, pow
from typing import List, Set, Tuple


def primes_mod(limit: int, k: int, mod: int) -> List[int]:
    """Return primes p <= limit with p ≡ k (mod mod)."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [p for p in range(limit + 1) if is_prime[p] if p % mod == k]


def get_primes_limit(num_primes: int) -> int:
    """Estimate limit needed for num_primes primes."""
    import math

    if num_primes < 6:
        return 15
    return int(num_primes * (math.log(num_primes) + math.log(math.log(num_primes + 1)))) + 2


def ipow(base: int, exp: int) -> int:
    """Return base^exp."""
    return base**exp


def pow_int(base: int, exp: int) -> int:
    """Return base^exp."""
    return base**exp


def solve() -> int:
    """Solve Problem 233."""
    N = 10**11
    K = 420

    # Find factorizations of K/4
    factorizations: Set[Tuple[int, ...]] = set()

    def find_factorizations(min_factor: int, n: int, factorization: List[int]) -> None:
        """Find all factorizations."""
        if n == 1:
            factorizations.add(tuple(factorization))
            return
        for factor in range(min_factor, n + 1, 2):
            if n % factor == 0:
                factorization.append((factor - 1) // 2)
                find_factorizations(factor, n // factor, factorization)
                factorization.pop()

    find_factorizations(3, K // 4, [])

    possible_ps = primes_mod(get_primes_limit(K), 1, 4)
    max_p = 0
    max_q = 0

    for factorization in factorizations:
        p_val = N
        for i in range(len(factorization) - 1):
            p_val //= ipow(possible_ps[i], factorization[i + 1])
        q_val = p_val // pow_int(possible_ps[len(factorization) - 1], factorization[0])
        p_val = round(p_val ** (1.0 / factorization[0]))
        if p_val > max_p:
            max_p = p_val
        if q_val > max_q:
            max_q = int(q_val)

    # Build prod_qs: products of 2s and primes ≡ 3 (mod 4)
    prod_qs = [True] * (max_q + 1)
    prod_qs[0] = False
    for p in primes_mod(max_q, 1, 4):
        for i in range(p, max_q + 1, p):
            prod_qs[i] = False

    ans = [0]

    def helper(index: int, prod: int, factorization: List[int], ps: List[int]) -> None:
        """Recursive helper."""
        if index == len(factorization):
            max_i = min(N // prod, max_q)
            for i in range(1, max_i + 1):
                if prod_qs[i]:
                    ans[0] += prod * i
            return

        e = factorization[index]
        for i, p in enumerate(ps):
            if prod * pow(p, e) > N:
                break
            if prod % p != 0:
                helper(index + 1, prod * pow_int(p, e), factorization, ps)

    for factorization in factorizations:
        helper(0, 1, list(factorization), primes_mod(max_p, 1, 4))

    return ans[0]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
