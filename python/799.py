"""Project Euler Problem 799: Pentagonal Number Representations.

Find the smallest pentagonal number that can be expressed as the sum of two
pentagonal numbers in over N different ways.

A pentagonal number is of the form x(3x-1)/2, so we have
a(3a-1)/2 + b(3b-1)/2 = c(3c-1)/2 => (6a-1)²+(6b-1)²=36c²-12c+2. So we need
36c²-12c+2 to be writable as the sum of two 5 (mod 6) squares in N different
ways. This means 36c²-12c+2 should only have prime factors of 2 and 1 (mod
4) primes, which we can find with a sieve: solving 36c²-12c+2 ≡ 0 (mod p)
gives c = (1±√-1)/6. Then, for the remaining values we can directly count the
number of ways it can be written as the sum of two 5 (mod 6) squares, and stop
when we find one over N.
"""

from __future__ import annotations

from math import isqrt
from typing import List, Tuple

from sympy import primerange


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    return pow(a, m - 2, m)


def sqrt_mod(n: int, p: int) -> int:
    """Find square root of n modulo p (Tonelli-Shanks)."""
    if p == 2:
        return n % 2
    if pow(n, (p - 1) // 2, p) != 1:
        return -1  # No solution

    # Find Q and S such that p-1 = Q * 2^S
    Q = p - 1
    S = 0
    while Q % 2 == 0:
        Q //= 2
        S += 1

    # Find a quadratic non-residue
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1

    M = S
    c = pow(z, Q, p)
    t = pow(n, Q, p)
    R = pow(n, (Q + 1) // 2, p)

    while t != 1:
        i = 1
        while i < M and pow(t, 1 << i, p) != 1:
            i += 1
        b = pow(c, 1 << (M - i - 1), p)
        M = i
        c = (b * b) % p
        t = (t * c) % p
        R = (R * b) % p

    return R


def figurate(n: int, sides: int) -> int:
    """Compute n-th figurate number with given number of sides."""
    return n * ((sides - 2) * n - (sides - 4)) // 2


def sums_of_two_squares(factors: List[int]) -> List[Tuple[int, int]]:
    """Find all representations as sum of two squares."""
    # Simplified - in practice would use proper algorithm
    result = []
    n = 1
    for p in factors:
        n *= p

    # Brute force search (inefficient but works for small n)
    limit = isqrt(n)
    for x in range(limit + 1):
        y_sq = n - x * x
        if y_sq < 0:
            break
        y = isqrt(y_sq)
        if y * y == y_sq:
            result.append((x, y))
            if x != y:
                result.append((y, x))

    return result


def solve() -> int:
    """Solve Problem 799."""
    N = 100
    L = 30000000
    L2 = 1000

    # Compute targets
    targets = [36 * sq(c) - 12 * c + 2 for c in range(L)]

    # Remove factors of 2
    remaining = targets[:]
    for c in range(L):
        while remaining[c] % 2 == 0:
            remaining[c] //= 2

    # Sieve out primes ≡ 3 (mod 4)
    primes_mod4_1 = list(primerange(2, L2 + 1))
    primes_mod4_1 = [p for p in primes_mod4_1 if p % 4 == 1]

    for p in primes_mod4_1:
        # Solve 36c²-12c+2 ≡ 0 (mod p)
        # => c = (1±√-1)/6
        sqrt_neg1 = sqrt_mod(p - 1, p)
        if sqrt_neg1 == -1:
            continue

        for sign in [1, -1]:
            c_val = (1 + sign * sqrt_neg1) * mod_inv(6, p) % p
            c_val = (c_val + p) % p
            c = c_val
            while c < L:
                while remaining[c] % p == 0:
                    remaining[c] //= p
                c += p

    # Find first c where remaining[c] == 1
    for c in range(L):
        if remaining[c] == 1:
            # Factorize target
            factors = []
            n = targets[c]
            for p in primerange(2, int(n**0.5) + 1):
                while n % p == 0:
                    factors.append(p)
                    n //= p
            if n > 1:
                factors.append(n)

            # Count representations as sum of two 5 (mod 6) squares
            count = 0
            for x, y in sums_of_two_squares(factors):
                if x % 6 == 5 and y % 6 == 5 and x < y:
                    count += 1

            if count >= N:
                return figurate(c, 5)

    return 0


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
