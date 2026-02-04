"""Project Euler Problem 801: x^y ≡ y^x (mod n).

Let f(n) be the number of solutions to x^y ≡ y^x (mod n) for 0 < x,y ≤ n²-n.
Find the sum of f(p) for all p from A to A+B.

Let x≡a (mod n) and x≡b (mod n-1), and let y≡c (mod n) and y≡d (mod n-1).
Then the equation becomes a^d ≡ b^c (mod n). For each r (mod n), let c(r)
be the number of solutions to a^d ≡ r (mod n); then f(n) is just
Σ_{r=0}^{n-1} c(r)². For r=0, c(r)=n-1 (the base a must be zero, the
exponent d can be anything). Otherwise, taking the logarithm of both sides
gives d*log(a) ≡ c*log(b), where all elements are now in Z_{n-1}, so this
is just the number of 2x2 matrices [[log a, log b] [c d]] with determinant
zero. For n-1 = p^e, this number is p^{3e} + p^{3e-1} - p^{2e-1} (see
https://oeis.org/A020478), and the function is multiplicative, so can be
computed for arbitrary n-1.

We use a sieve over the range [A, A+B] to compute the above prime
factorizations efficiently.
"""

from __future__ import annotations

from math import isqrt
from typing import Dict, List


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


def imod(a: int, m: int) -> int:
    """Return a mod m, handling negative values correctly."""
    return ((a % m) + m) % m


def prime_factorize(n: int, prime_factors: List[int]) -> Dict[int, int]:
    """Factorize n using given prime factors."""
    factors: Dict[int, int] = {}
    for p in prime_factors:
        e = 0
        while n % p == 0:
            n //= p
            e += 1
        if e > 0:
            factors[p] = e
    if n > 1:
        factors[n] = 1
    return factors


def solve() -> int:
    """Solve Problem 801."""
    A = 10**16
    B = 10**6
    M = 993353399
    L = isqrt(A + B)

    # Sieve to find prime factors for each number in [A, A+B]
    all_factors: List[List[int]] = [[] for _ in range(B + 1)]
    primes = sieve_primes(L)
    for p in primes:
        start_idx = imod(-A, p)
        for i in range(start_idx, B + 1, p):
            if p < A + i:
                all_factors[i].append(p)

    ans = 0
    for i in range(1, B + 1):
        if not all_factors[i]:
            # A + i is prime, so p = A + i and p - 1 = A + i - 1
            n = A + i - 1  # n = p - 1
            res = 1
            factors = prime_factorize(n, all_factors[i - 1])
            for p, e in factors.items():
                term = (
                    pow(p, 3 * e, M)
                    + pow(p, 3 * e - 1, M)
                    - pow(p, 2 * e - 1, M)
                )
                res = (res * term) % M
            ans = (ans + pow(n, 2, M) + res) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
