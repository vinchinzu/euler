"""Project Euler Problem 417: Reciprocal cycles II.

Let L(n) be the length of the repeating decimal of 1/n. Find sum_{n=3}^N L(n).

If n is divisible by 2, then L(n) = L(n/2). Similarly, if n is divisible by 5,
then L(n) = L(n/5). So we only need to compute L(n) when GCD(n, 10) = 1.

If GCD(n, 10) = 1, then L(n) is the order of 10 modulo n. For n = a*b where
a and b are relatively prime, this is L(n) = LCM(L(a), L(b)). For n a prime
power p^e, we can use the relation L(n) = L(p) or p L(p), and simply check
whether 10^L(p) ≡ 1. Finally, if n is a prime, then the order must divide
n - 1. We start with n - 1 and repeatedly try dividing out factors of n - 1.
The final number must be the order L(n).
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import Dict, List


def sieve_spf(limit: int) -> List[int]:
    """Sieve to find smallest prime factor (SPF) for each number."""
    spf = list(range(limit + 1))
    spf[0] = spf[1] = 0
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:  # i is prime
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


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


def lcm(a: int, b: int) -> int:
    """Compute least common multiple of a and b."""
    if a == 0 or b == 0:
        return 0
    return a * b // gcd(a, b)


def multiplicative_order(base: int, p: int) -> int:
    """Find the multiplicative order of base modulo prime p."""
    if gcd(base, p) != 1:
        return 0
    
    # Factor p - 1
    phi = p - 1
    factors: Dict[int, int] = {}
    temp = phi
    d = 2
    while d * d <= temp:
        if temp % d == 0:
            count = 0
            while temp % d == 0:
                temp //= d
                count += 1
            factors[d] = count
        d += 1
    if temp > 1:
        factors[temp] = 1
    
    order = phi
    for q, exp in factors.items():
        q_pow = q ** exp
        while order % q == 0 and pow(base, order // q, p) == 1:
            order //= q
    
    return order


def compute_l(n: int, orders: Dict[int, int], spf: List[int]) -> int:
    """Compute L(n) using precomputed orders and SPF."""
    # Remove factors of 2 and 5
    temp = n
    while temp % 2 == 0:
        temp //= 2
    while temp % 5 == 0:
        temp //= 5
    
    if temp == 1:
        return 0
    
    # Get distinct primes
    distinct_primes: List[int] = []
    t = temp
    while t > 1:
        p = spf[t]
        if p not in distinct_primes:
            distinct_primes.append(p)
        while t % p == 0:
            t //= p
    
    # LCM of orders of distinct primes
    result = 1
    for p in distinct_primes:
        if p not in orders:
            return 0  # Should not happen if precomputed correctly
        result = lcm(result, orders[p])
    
    return result


def decimal_cycle_lengths(n_max: int, base: int = 10) -> List[int]:
    """Compute decimal cycle lengths for all numbers up to n_max."""
    spf = sieve_spf(n_max)
    primes = sieve_primes(n_max)
    
    # Precompute multiplicative orders for primes (excluding 2 and 5)
    orders: Dict[int, int] = {}
    for p in primes:
        if p != 2 and p != 5:
            orders[p] = multiplicative_order(base, p)
    
    lengths = [0] * (n_max + 1)
    
    # Compute lengths for all numbers
    for n in range(2, n_max + 1):
        if n % 2 == 0:
            lengths[n] = lengths[n // 2]
        elif n % 5 == 0:
            lengths[n] = lengths[n // 5]
        else:
            lengths[n] = compute_l(n, orders, spf)
    
    return lengths


def solve() -> int:
    """Solve Problem 417."""
    N = 10**8
    B = 10
    
    lengths = decimal_cycle_lengths(N, B)
    result = sum(lengths[n] for n in range(3, N + 1))
    return result


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
