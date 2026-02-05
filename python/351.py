"""Project Euler Problem 351: Hexagonal Orchards

Count hidden lattice points in a hexagonal orchard of order n.

A hexagonal orchard of order n has 3n^2 + 3n + 1 points (including center).
A point is visible from the center iff gcd of its coordinates is 1.
Hidden points = Total points - Visible points - 1 (center).

The number of hidden points can be computed as:
  Hidden = 6 * sum_{k=2}^{n} (k - phi(k))

where phi is Euler's totient function.

This equals: 6 * (sum(k from 2 to n) - sum(phi(k) from 2 to n))
           = 6 * ((n(n+1)/2 - 1) - (sum(phi(k) from 1 to n) - 1))
           = 6 * (n(n+1)/2 - sum(phi(k) from 1 to n))
           = 3*n*(n+1) - 6*sum(phi(k) from 1 to n)
"""

from __future__ import annotations

import numpy as np


def compute_totient_sum(n: int) -> int:
    """Compute sum of phi(k) for k = 1 to n using a sieve.

    Uses a modified sieve approach with numpy vectorization:
    - Initialize phi[k] = k
    - For each prime p, update phi for all multiples using vectorized operations
    """
    # For n = 10^8, we need about 800MB for int64 array
    phi = np.arange(n + 1, dtype=np.int64)

    # Sieve to compute phi
    # For each prime p, we need to multiply phi[m] by (p-1)/p for all multiples m of p
    # This is equivalent to: phi[m] = phi[m] - phi[m]//p

    # We only need to check up to sqrt(n) for marking composites,
    # but for totient we need to process all primes
    for p in range(2, n + 1):
        if phi[p] == p:  # p is prime (hasn't been modified yet)
            # p is prime, so phi[p] = p - 1
            phi[p] = p - 1
            # Update all multiples of p using vectorized numpy slicing
            # phi[2p::p] -= phi[2p::p] // p
            phi[2 * p :: p] -= phi[2 * p :: p] // p

    # Sum all phi values
    return int(np.sum(phi[1:]))


def solve(n: int = 10**8) -> int:
    """Solve PE 351 for hexagonal orchard with order n.

    Returns the number of hidden points (not visible from center).
    """
    # Total lattice points excluding center = 3*n*(n+1)
    # Hidden = 6 * sum_{k=2}^{n} (k - phi(k))
    #        = 6 * (n*(n+1)/2 - 1 - (totient_sum - 1))
    #        = 6 * (n*(n+1)/2 - totient_sum)
    #        = 3*n*(n+1) - 6*totient_sum

    totient_sum = compute_totient_sum(n)
    hidden = 3 * n * (n + 1) - 6 * totient_sum

    return hidden


if __name__ == "__main__":
    print(solve())
