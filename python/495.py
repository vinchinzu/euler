"""Project Euler Problem 495: Writing n! as product of k distinct integers.

Find the number of ways that N! can be written as the product of K distinct
positive integers.

Uses inclusion-exclusion over partitions of K, following the Java reference.
"""

from __future__ import annotations
from math import isqrt
from functools import lru_cache


def sieve_primes(limit: int) -> list[int]:
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


def solve() -> int:
    """Solve Problem 495."""
    N = 10_000
    K = 30
    M = 10**9 + 7

    primes = sieve_primes(N)
    exponents = [num_factors_in_factorial(N, p) for p in primes]

    # Precompute factorials and inverse factorials
    fact = [1] * (K + 1)
    for i in range(1, K + 1):
        fact[i] = fact[i - 1] * i % M
    inv_fact = [1] * (K + 1)
    inv_fact[K] = pow(fact[K], M - 2, M)
    for i in range(K - 1, -1, -1):
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % M

    # Precompute inverses of 1..K
    inv = [0] * (K + 1)
    inv[1] = 1
    for i in range(2, K + 1):
        inv[i] = (M - M // i) * inv[M % i] % M

    # Memoized function f(c) returns array where f(c)[e] = number of solutions
    # to c[0]*a_0 + c[1]*a_1 + ... = e, with a_i >= 0
    cache = {}

    def f(c: tuple[int, ...]) -> list[int]:
        if c in cache:
            return cache[c]

        result = [0] * (N + 1)
        if len(c) == 0:
            result[0] = 1
            cache[c] = result
            return result

        # Compute from f(c[:-1])
        prev_f = f(c[:-1])
        last_c = c[-1]

        for i in range(N + 1):
            result[i] = prev_f[i]
            if i >= last_c:
                result[i] = (result[i] + result[i - last_c]) % M

        cache[c] = result
        return result

    # parity: (-1)^(c_i - 1) = -1 if c_i even, 1 if c_i odd
    def parity(c_i: int) -> int:
        return 1 if c_i % 2 == 1 else -1

    ans = 0

    # Generate all partitions of K (non-decreasing sequences summing to K)
    def helper(min_val: int, remaining: int, c: list[int]):
        nonlocal ans

        if remaining == 0:
            c_tuple = tuple(c)
            f_c = f(c_tuple)

            # Product over all exponents
            res = 1
            for e in exponents:
                res = res * f_c[e] % M

            # Multiply by -parity(c_i) * inv(c_i) for each c_i
            # parity(c_i) = (-1)^c_i, so -parity(c_i) = (-1)^(c_i+1)
            for c_i in c:
                sign = 1 if (c_i + 1) % 2 == 0 else M - 1  # (-1)^(c_i+1) mod M
                res = res * sign % M * inv[c_i] % M

            # Multiply by inv_fact of frequency of each value in c
            from collections import Counter
            freq = Counter(c)
            for count in freq.values():
                res = res * inv_fact[count] % M

            ans = (ans + res) % M
            return

        for coeff in range(min_val, remaining + 1):
            c.append(coeff)
            helper(coeff, remaining - coeff, c)
            c.pop()

    helper(1, K, [])

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
