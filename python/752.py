"""Project Euler Problem 752: Powers of 1+√7.

Let g(x) be the smallest integer n such that α(n) ≡ 1 (mod x) and β(n) ≡ 0
(mod x), where (1+√7)^n = α(n) + β(n)√7, or 0 if no such n exists.
Find Σ_{x=2}^N g(x).

We can see that [α(n), β(n)] = [[1, 7], [1, 1]]^n [1, 0]. This has no
solutions if GCD(x, 6) = 1, and if x=7, we have g(x)=7. Otherwise, n is
the order of the 2x2 matrix [[1, 7], [1, 1]] (mod x). For prime x, this
order must divide (x-1)(x+1), so we can try all divisors. Combining this
with g(p^e) = g(p)*p^{e-1} and g(m*n) = LCM(g(m), g(n)) for GCD(m, n) = 1,
we can efficiently compute all g(x).
"""

from __future__ import annotations

from functools import lru_cache
from math import gcd, isqrt
from typing import Dict, List, Set


def preff(limit: int) -> List[int]:
    """Precompute largest prime factor."""
    ff = [0] * (limit + 1)
    for i in range(2, limit + 1):
        if ff[i] == 0:
            for j in range(i, limit + 1, i):
                ff[j] = i
    return ff


def lcm(a: int, b: int) -> int:
    """Least common multiple."""
    return a * b // gcd(a, b)


def pow2x2(matrix: List[int], exp: int, mod: int) -> List[int]:
    """Raise 2x2 matrix to power exp modulo mod.

    Matrix is represented as [a, b, c, d] for [[a, b], [c, d]].
    """
    a, b, c, d = matrix
    result = [1, 0, 0, 1]  # Identity matrix

    base = [a % mod, b % mod, c % mod, d % mod]
    while exp > 0:
        if exp & 1:
            # Multiply result by base
            new_result = [
                (result[0] * base[0] + result[1] * base[2]) % mod,
                (result[0] * base[1] + result[1] * base[3]) % mod,
                (result[2] * base[0] + result[3] * base[2]) % mod,
                (result[2] * base[1] + result[3] * base[3]) % mod,
            ]
            result = new_result
        # Square base
        new_base = [
            (base[0] * base[0] + base[1] * base[2]) % mod,
            (base[0] * base[1] + base[1] * base[3]) % mod,
            (base[2] * base[0] + base[3] * base[2]) % mod,
            (base[2] * base[1] + base[3] * base[3]) % mod,
        ]
        base = new_base
        exp >>= 1

    return result


def prime_factorization(n: int, ff: List[int]) -> Dict[int, int]:
    """Prime factorization using largest prime factor array."""
    factors: Dict[int, int] = {}
    while n > 1:
        p = ff[n] if n < len(ff) else n
        factors[p] = factors.get(p, 0) + 1
        n //= p
    return factors


def all_divisors(n: int, prime_factors: Set[int]) -> List[int]:
    """Get all divisors of n using prime factors."""
    divisors = [1]
    temp = n
    for p in sorted(prime_factors):
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


def solve() -> int:
    """Solve Problem 752."""
    N = 1_000_000
    ff = preff(N)
    cache: Dict[int, int] = {}
    identity = [1, 0, 0, 1]
    A = [1, 7, 1, 1]

    def g(x: int) -> int:
        """Compute g(x)."""
        if gcd(x, 6) > 1:
            return 0
        if x == 7:
            return 7

        d = ff[x]
        xx = x
        while xx % d == 0:
            xx //= d

        if xx > 1:
            return lcm(g(xx), g(x // xx))
        elif d != x:
            return g(d) * (x // d)

        if x in cache:
            return cache[x]

        # For prime x, find order dividing (x-1)(x+1)
        factors1 = prime_factorization(x - 1, ff)
        factors2 = prime_factorization(x + 1, ff)
        all_primes: Set[int] = set(factors1.keys()) | set(factors2.keys())
        divisors = all_divisors((x - 1) * (x + 1), all_primes)

        for e in sorted(divisors):
            if pow2x2(A, e, x) == identity:
                cache[x] = e
                return e

        raise ValueError(f"No solution found for x={x}")

    ans = 0
    for x in range(2, N + 1):
        ans += g(x)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
