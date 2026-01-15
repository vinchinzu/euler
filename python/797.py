"""Project Euler Problem 797: Cyclotomic Polynomials.

Let P_n(x) be the sum of all polynomials that divide x^n - 1 but not any
smaller x^k - 1. Find Σ_{n=1}^N P_n(2).

Let F_n(x) be the cyclotomic polynomial of degree n. This can be computed by
starting with an F_n(x) = x^n - 1, then for each i, dividing each i'th
element by x^i - 1.

We can compute the sum of all factors of x^n - 1 by taking each cyclotomic
polynomial factor and adding 1 (for the cases where the cyclotomic polynomial
is one of the factors and where it's not). Call this G_n(x).

P_n(x) is just G_n(x) with the polynomials that divide smaller x^k - 1 removed.
This can be computed using Inclusion Exclusion. For any term in G_n(x), it is
included in P_n(x), excluded in P_{2n}(x), and in general multiplied by µ(k)
in P_{k*n}(x). This means we just need to multiply G_n(x) by Σ_{k=1}^{N/n}
µ(k) to get the total contribution for a specific G_n(x), and then sum that
over all n.
"""

from __future__ import annotations

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


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    return pow(a, m - 2, m)


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


def solve() -> int:
    """Solve Problem 797."""
    N = 10**7
    M = 10**9 + 7

    mobius = pre_mobius(N)

    # Precompute Mertens function
    mertens = [0] * (N + 1)
    for i in range(1, N + 1):
        mertens[i] = mertens[i - 1] + mobius[i]

    # Compute F_n(2) = cyclotomic polynomials evaluated at 2
    F = [pow_mod(2, i, M) - 1 for i in range(N + 1)]

    # Divide by x^i - 1 for each i
    for i in range(1, N + 1):
        inv = mod_inv(F[i], M)
        for j in range(2 * i, N + 1, i):
            F[j] = (F[j] * inv) % M

    # Compute G_n(2) = product of (F_i(2) + 1) for all i|n
    G = [1] * (N + 1)
    for i in range(1, N + 1):
        for j in range(i, N + 1, i):
            G[j] = (G[j] * (F[i] + 1)) % M

    # Sum contributions
    ans = 0
    for i in range(1, N + 1):
        ans = (ans + mertens[N // i] * G[i]) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
