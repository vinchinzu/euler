"""Project Euler Problem 516: 5-smooth totients.

Find the sum of all n ≤ N such that ϕ(n) is a Hamming number (5-smooth).
"""

from __future__ import annotations

from bisect import bisect_right
from typing import List


def is_prime_64(n: int) -> bool:
    """Deterministic Miller-Rabin for 64-bit integers."""
    if n < 2:
        return False
    small_primes = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for p in small_primes:
        if n % p == 0:
            return n == p

    d = n - 1
    s = 0
    while d % 2 == 0:
        s += 1
        d //= 2

    def check(a: int) -> bool:
        x = pow(a, d, n)
        if x == 1 or x == n - 1:
            return True
        for _ in range(s - 1):
            x = (x * x) % n
            if x == n - 1:
                return True
        return False

    for a in (2, 3, 5, 7, 11, 13, 17):
        if not check(a):
            return False
    return True


def solve() -> int:
    """Solve Problem 516."""
    N = 10**12
    M = 2**32

    # Generate all Hamming numbers (5-smooth).
    hammings: List[int] = []
    n2 = 1
    while n2 <= N:
        n3 = n2
        while n3 <= N:
            n5 = n3
            while n5 <= N:
                hammings.append(n5)
                n5 *= 5
            n3 *= 3
        n2 *= 2
    hammings.sort()

    # Prefix sums of Hamming numbers modulo M.
    prefix = [0]
    total = 0
    for h in hammings:
        total = (total + h) % M
        prefix.append(total)

    # Good primes: p>5 with p-1 Hamming.
    good_primes: List[int] = []
    for h in hammings:
        p = h + 1
        if p > 5 and is_prime_64(p):
            good_primes.append(p)
    good_primes.sort()

    # Generate all products of distinct good primes <= N.
    products: List[int] = []

    def gen_products(start: int, prod: int) -> None:
        products.append(prod)
        for i in range(start, len(good_primes)):
            p = good_primes[i]
            new_prod = prod * p
            if new_prod > N:
                break
            gen_products(i + 1, new_prod)

    gen_products(0, 1)

    # Sum over g * sum(h) for h <= N/g.
    ans = 0
    for g in products:
        limit = N // g
        idx = bisect_right(hammings, limit)
        sum_h = prefix[idx]
        ans = (ans + (g % M) * sum_h) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
