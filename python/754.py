"""Project Euler Problem 754: Product of Gauss Factorials.

Let g(n) be the product of all positive integers up to n relatively prime
to n. Find Π_{i=1}^N g(i).

We can use the usual trick to remove the relatively prime condition:

Π_{i=1}^N g(i) = Π_{g=1}^N ( Π_{i=1}^⌊N/g⌋ Π_{j=1}^i g*j )^µ(g)
               = Π_{g=1}^N ( g^{tr(⌊N/g⌋)} Π_{i=1}^⌊N/g⌋ i! )^µ(g).

To compute Π_g g^{tr(⌊N/g⌋)}, we compute the terms directly for g≤√N,
but for g>√N we take the product of g's with the same exponent, and
perform the exponentiation only once for efficiency. Meanwhile, the
product of factorials i! can be pre-computed, so that we can perform O(1)
operations for each g. Finally, we keep two separate cumulative products,
one for µ(g)=1 and another for µ(g)=-1, and only invert the latter at
the end for performance.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def pre_mobius(limit: int) -> List[int]:
    """Precompute Möbius function."""
    mu = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]
    return mu


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inverse(a: int, m: int) -> int:
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
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def factorials(n: int, mod: int) -> List[int]:
    """Precompute factorials modulo mod."""
    fact = [1] * (n + 1)
    for i in range(1, n + 1):
        fact[i] = (fact[i - 1] * i) % mod
    return fact


def solve() -> int:
    """Solve Problem 754."""
    N = 10**8
    M = 10**9 + 7
    L = isqrt(N)

    mobius = pre_mobius(N)

    # res[0] for mu=-1, res[1] for mu=0 (unused), res[2] for mu=1
    res = [1, 0, 1]

    # For g ≤ N/L
    for g in range(1, N // L + 1):
        idx = mobius[g] + 1
        res[idx] = (res[idx] * pow_mod(g, tr(N // g), M)) % M

    # For g > N/L, group by exponent
    for q in range(1, L):
        subres = [1, 0, 1]
        for g in range(N // (q + 1) + 1, N // q + 1):
            idx = mobius[g] + 1
            subres[idx] = (subres[idx] * g) % M
        res[0] = (res[0] * pow_mod(subres[0], tr(q), M)) % M
        res[2] = (res[2] * pow_mod(subres[2], tr(q), M)) % M

    # Multiply by product of factorials
    fact = factorials(N, M)
    prod_factorials = [1] * (N + 1)
    for i in range(1, N + 1):
        prod_factorials[i] = (prod_factorials[i - 1] * fact[i]) % M

    for g in range(1, N + 1):
        idx = mobius[g] + 1
        res[idx] = (res[idx] * prod_factorials[N // g]) % M

    ans = (res[2] * mod_inverse(res[0], M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
