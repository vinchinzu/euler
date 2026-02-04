"""Project Euler Problem 756: Approximating a Sum.

For a random K-tuple 0 < X_1 < X_2 < ... < X_K ≤ N, let S = Σ_{i=1}^N ϕ(i)
and let S* = Σ_{i=1}^K ϕ(X_i)(X_i - X_{i-1}). Find the expected value of
S - S*.

Let f(i) = ϕ(r) where r is the smallest integer that is one of the X_i,
or 0 if none exists. Then S* = Σ_{i=1}^N f(i). So by linearity of
expectation, we can sum the expected values of each f(i).

With probability K/N, f(i) = ϕ(i). With probability (K/N) (N-K)/(N-1),
f(i) = ϕ(i+1), and so on. These coefficients are the same for all i. So
we have:

E(S*) = K/N Σ_{i=1}^N ϕ(i) + (K/N) (N-K)/(N-1) Σ_{i=2}^N ϕ(i) + ...

We can precompute the cumulative sums Σ ϕ(i), and iteratively build up
the coefficients to compute the entire sum in linear time. As a further
optimization, we can stop when the terms become negligible. The answer is
then Σ ϕ(i) - E(S*).
"""

from __future__ import annotations

from typing import List

import mpmath
mpmath.mp.dps = 50


def pre_phi(limit: int) -> List[int]:
    """Precompute Euler's totient function."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] = phi[j] // i * (i - 1)
    return phi


def solve() -> str:
    """Solve Problem 756."""
    N = 12_345_678
    K = 12_345

    phi = pre_phi(N)
    sum_phis = [0] * (N + 1)
    for i in range(1, N + 1):
        sum_phis[i] = sum_phis[i - 1] + phi[i]

    d = mpmath.mpf(K) / mpmath.mpf(N)
    ans = mpmath.mpf(sum_phis[N])

    for i in range(1, N + 1):
        diff = d * mpmath.mpf(sum_phis[N] - sum_phis[i - 1])
        if abs(float(diff)) == 0.0:
            break
        d = d * mpmath.mpf(N - K - i + 1) / mpmath.mpf(N - i)
        ans = ans - diff

    # Format to 6 decimal places
    s = mpmath.nstr(ans, 15, strip_zeros=False)
    # Find the decimal point and take 6 digits after it
    dot = s.index('.')
    return s[:dot + 7]


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
