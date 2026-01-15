"""Project Euler Problem 552: Chinese Remainder Theorem.

Let A_n be the smallest positive integer satisfying A_n ≡ i (mod p_i) for 1≤i≤n.
Find the sum of all primes up to N that divide some A_n.

Clearly p_i can only divide A_n for n < i. So for each p_i, we compute A_n in
Garner form: 1 + 2g_1 + 6g_2 + 30g_3 + ..., where each coefficient is product
of the first k primes. If any cumulative sum is zero, then that p_i divides an
A_n. We can iteratively compute each g_k by solving A_k + (Πp_i)g_k ≡ k+1
(mod p_k).
"""

from __future__ import annotations

from math import gcd
from typing import List, Tuple


def extended_gcd(a: int, b: int) -> Tuple[int, int, int]:
    """Extended Euclidean algorithm.

    Returns (gcd, x, y) such that a*x + b*y = gcd(a, b).
    """
    if b == 0:
        return (a, 1, 0)
    g, x1, y1 = extended_gcd(b, a % b)
    return (g, y1, x1 - (a // b) * y1)


def mod_inverse(a: int, m: int) -> int:
    """Return modular inverse of a modulo m."""
    g, x, _y = extended_gcd(a, m)
    if g != 1:
        raise ValueError(f"Modular inverse does not exist: gcd({a}, {m}) = {g}")
    return (x % m + m) % m


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def solve() -> int:
    """Solve Problem 552."""
    N = 300000

    primes = sieve(N)
    L = len(primes)
    garner = [0] * L
    ans = 0

    for i in range(L):
        p = primes[i]
        prod_primes = 1
        A = 0
        good = False

        for j in range(i):
            A = (A + prod_primes * garner[j]) % p
            prod_primes = (prod_primes * primes[j]) % p
            if A == 0:
                good = True

        # Compute garner[i] such that A + prod_primes * garner[i] ≡ i+1 (mod p)
        # So: prod_primes * garner[i] ≡ (i+1 - A) (mod p)
        if prod_primes % p != 0:
            garner[i] = ((i + 1 - A) * mod_inverse(prod_primes, p)) % p
        else:
            garner[i] = 0

        if good:
            ans += p

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
