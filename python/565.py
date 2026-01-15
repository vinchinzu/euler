"""Project Euler Problem 565: Divisor Sum Divisibility.

Find the sum of all integers n≤N such that the sum of the divisors of n is divisible by K.

Since the sum of the divisors of Πp^e is Π(1+p+p²+...p^e), we first determine all p^e such
that the sum of the divisors of p^e. For e=1, we use a sieve to find i such that K*i-1 is
prime, i.e. p+1 is divisible by K. For e≥2, we compute them in the straightforward manner.

Then we use Inclusion Exclusion. For each valid p^e, we add the sum of all multiples of p^e.
For each valid p1^e1 and p2^e2, we need to subtract the multiples of (p1^e1)(p2^e2). And
for each p^e, we also need to subtract the multiples of p^{e+1}.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import isqrt
from typing import List


N = 10**11
K = 2017


@dataclass(frozen=True)
class Base:
    """Prime power base."""

    p: int
    e: int


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def mod_inv(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        msg = "Modular inverse does not exist"
        raise ValueError(msg)
    if t < 0:
        t += m
    return t


def pow_int(base: int, exp: int) -> int:
    """Integer power."""
    return base**exp


def tr(n: int) -> int:
    """Triangular number: n*(n+1)//2."""
    return n * (n + 1) // 2


ans = 0


def helper(min_index: int, parity: int, n: int, bases: List[Base]) -> None:
    """Recursive inclusion-exclusion."""
    global ans
    if n > 1:
        ans += parity * n * tr(N // n)
    for index in range(min_index, len(bases)):
        base = bases[index]
        p_pow_e = pow_int(base.p, base.e)
        if n * p_pow_e > N:
            break
        helper(index + 1, -parity, n * p_pow_e, bases)
        p_pow_e_plus_1 = p_pow_e * base.p
        if n * p_pow_e_plus_1 <= N:
            helper(index + 1, parity, n * p_pow_e_plus_1, bases)


def solve() -> int:
    """Solve Problem 565."""
    global ans
    ans = 0

    sieve_size = int(N / K) + 1
    sieve = [True] * sieve_size
    primes_list = sieve_primes(isqrt(N))

    for p in primes_list:
        if p == K:
            continue
        inv = mod_inv(K, p)
        i = inv
        while i < sieve_size:
            if p != i * K - 1:
                sieve[i] = False
            i += p

    bases: List[Base] = []
    for i in range(1, sieve_size):
        if sieve[i]:
            bases.append(Base(i * K - 1, 1))

    for p in primes_list:
        sum_divisors = 1 + p
        e = 2
        while pow_int(p, e) <= N:
            sum_divisors = sum_divisors * p + 1
            if sum_divisors % K == 0:
                bases.append(Base(p, e))
            e += 1

    bases.sort(key=lambda b: pow_int(b.p, b.e))
    helper(0, -1, 1, bases)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
