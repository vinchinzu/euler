"""Project Euler Problem 245: Coresilience.

Find the number of composite integers n ≤ N such that (n - ϕ(n)) / (n - 1) is
a unit fraction.
"""

from __future__ import annotations

from math import isqrt
from typing import List, Set


def build_spf(limit: int) -> List[int]:
    """Build smallest prime factor array up to limit."""
    spf = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


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


def is_sq(n: int, p: int) -> bool:
    """Check if n is a perfect square modulo p."""
    return pow(n, (p - 1) // 2, p) <= 1


def sqrt_mod(n: int, p: int) -> int:
    """Compute sqrt(n) mod p."""
    if p % 4 == 3:
        return pow(n, (p + 1) // 4, p)
    for x in range(1, p):
        if pow(x, 2, p) == n:
            return x
    return 0


def imod(n: int, mod: int) -> int:
    """Return n mod mod (always positive)."""
    return ((n % mod) + mod) % mod


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def all_divisors(n: int, prime_factors: List[int]) -> List[int]:
    """Return all divisors of n given prime factors."""
    divisors = [1]
    temp = n
    for p in prime_factors:
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


def is_prime(n: int, spf: List[int]) -> bool:
    """Check if n is prime."""
    return n > 1 and (n >= len(spf) or spf[n] == n)


def solve() -> int:
    """Solve Problem 245."""
    N = 2 * (10**11)
    L = isqrt(N)

    limit = int(N ** (2.0 / 3))
    spf = build_spf(limit)
    primes_list = sieve(L)

    # Build prime factors for p*(p-1)+1
    prime_factors: List[List[int]] = [[] for _ in range(L + 1)]
    for q in primes_list:
        if q >= 3 and is_sq(q - 3, q):
            r1 = sqrt_mod(q - 3, q)
            for p in range(imod((1 + r1) * (q + 1) // 2, q), L + 1, q):
                prime_factors[p].append(q)
            r2 = q - r1
            for p in range(imod((1 - r1) * (q + 1) // 2, q), L + 1, q):
                prime_factors[p].append(q)

    ans = 0

    # Handle two primes case
    for p in primes_list:
        if p < 3:
            continue
        for d in all_divisors(p * (p - 1) + 1, prime_factors[p]):
            if d >= p:
                q = d - (p - 1)
                if p < q and p * q <= N and is_prime(q, spf):
                    ans += p * q

    # Handle more than two primes
    def helper(index: int, P: int, phi: int, factors: List[int]) -> None:
        """Recursive helper."""
        if len(factors) >= 2:
            for k in range(2, factors[0], 2):
                num = phi * k + 1
                den = P - (P - phi) * k
                if den != 0 and num % den == 0:
                    q = num // den
                    if (not factors or factors[-1] < q) and P * q <= N and is_prime(q, spf):
                        ans += P * q

        while index < len(primes_list):
            q = primes_list[index]
            if P * sq(q) > N:
                break
            factors.append(q)
            helper(index + 1, P * q, phi * (q - 1), factors)
            factors.pop()
            index += 1

    helper(0, 1, 1, [])
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
