"""Project Euler Problem 627: Counting Products.

Find the number of integers that can be expressed as the product of N integers
up to K.

Let p_1, p_2, ..., p_L be the primes up to K. We consider the Ehrhart polynomial
of the convex hull of lattice points. We brute-force for small dilations, then
use polynomial interpolation to extrapolate.
"""

from __future__ import annotations

from sympy import primerange


def solve() -> int:
    """Solve Problem 627."""
    N = 10001
    K = 30
    M = 10**9 + 7
    L = len(list(primerange(2, K + 1)))  # number of primes up to K
    D = L - int(L**0.5)  # L - floor(sqrt(L))

    # Compute F[n] for small n by brute force
    # F[n] = number of distinct products of n integers from 1..K
    F = [0] * (L + 1 - D)
    F[0] = 1
    products = {1}

    for n in range(1, len(F)):
        new_products = set()
        for product in products:
            for i in range(1, K + 1):
                new_products.add(product * i)
        products = new_products
        F[n] = len(products)

    # Build the values for interpolation.
    # The Java code uses: polynomialExtrapolation(i -> i < D ? 0 : F[i - D], L, M).apply(N + D)
    # This means we have a degree-L polynomial P where:
    #   P(0) = 0, P(1) = 0, ..., P(D-1) = 0, P(D) = F[0], P(D+1) = F[1], ...
    # We have L+1 points (indices 0..L) and want P(N+D)

    # vals[i] = P(i) for i = 0, 1, ..., L
    vals = []
    for i in range(L + 1):
        if i < D:
            vals.append(0)
        else:
            vals.append(F[i - D] % M)

    # Lagrange interpolation at x = N + D, modulo M
    # P(x) = sum_{i=0}^{L} vals[i] * prod_{j!=i} (x - j) / (i - j)
    x = N + D
    n_pts = L + 1

    # Precompute prefix and suffix products of (x - j) mod M
    # prefix[i] = prod_{j=0}^{i-1} (x - j) mod M
    # suffix[i] = prod_{j=i+1}^{n_pts-1} (x - j) mod M
    prefix = [1] * (n_pts + 1)
    for j in range(n_pts):
        prefix[j + 1] = prefix[j] * ((x - j) % M) % M

    suffix = [1] * (n_pts + 1)
    for j in range(n_pts - 1, -1, -1):
        suffix[j] = suffix[j + 1] * ((x - j) % M) % M

    # Precompute factorial and inverse factorial for denominators
    # denominator for i = prod_{j!=i} (i - j) = (-1)^(n_pts-1-i) * i! * (n_pts-1-i)!
    fact = [1] * n_pts
    for i in range(1, n_pts):
        fact[i] = fact[i - 1] * i % M

    inv_fact = [1] * n_pts
    inv_fact[n_pts - 1] = pow(fact[n_pts - 1], M - 2, M)
    for i in range(n_pts - 2, -1, -1):
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % M

    ans = 0
    for i in range(n_pts):
        if vals[i] == 0:
            continue
        numer = prefix[i] * suffix[i + 1] % M
        denom_inv = inv_fact[i] * inv_fact[n_pts - 1 - i] % M
        if (n_pts - 1 - i) % 2 == 1:
            denom_inv = (-denom_inv) % M
        ans = (ans + vals[i] * numer % M * denom_inv) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
