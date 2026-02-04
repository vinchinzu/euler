"""Project Euler Problem 643: 2-Friendly.

Find the number of positive integer pairs 1 <= p < q <= N such that gcd(p, q)
is a power of 2.

For every t >= 1, the number of pairs with gcd(p, q) = 2^t is exactly the
number of pairs of distinct relatively prime integers up to floor(N / 2^t). This
is equal to ( sum_{k=1}^{floor(N / 2^t)} phi(k) ) - 1.

We compute sum of phi using a sub-linear algorithm with sieve up to n^{2/3}.
"""

from __future__ import annotations

from math import isqrt


def compute_sum_phi_all(n, mod):
    """Compute S(m) = sum_{k=1}^{m} phi(k) mod M for all quotient values m=n//k.

    Uses sieve of phi up to V = n^{2/3} for efficiency, then only needs the
    recursive formula for O(n^{1/3}) large values.
    """
    # Sieve up to V ~ n^{2/3}
    V = int(round(n ** (2.0/3.0))) + 1
    sqrtn = isqrt(n)

    # Sieve phi
    phi = list(range(V + 1))
    for i in range(2, V + 1):
        if phi[i] == i:  # prime
            for j in range(i, V + 1, i):
                phi[j] -= phi[j] // i

    # Prefix sums
    small = [0] * (V + 1)
    for i in range(1, V + 1):
        small[i] = (small[i - 1] + phi[i]) % mod

    inv2 = pow(2, mod - 2, mod)

    # Only need to compute S(m) for m > V, which means m = n//i for i <= n//V ~ n^{1/3}
    large = {}
    limit = n // (V + 1) + 1  # max i such that n//i > V

    for i in range(limit, 0, -1):
        m = n // i
        if m <= V:
            continue
        result = (m % mod) * ((m + 1) % mod) % mod * inv2 % mod

        d = 2
        while d <= m:
            q = m // d
            d_max = m // q
            if q <= V:
                s_q = small[q]
            else:
                s_q = large[n // q]
            result = (result - (d_max - d + 1) % mod * s_q) % mod
            d = d_max + 1

        large[i] = result % mod

    def S(m):
        if m <= V:
            return small[m]
        return large[n // m]

    return S


def solve():
    """Solve Problem 643."""
    N = 10**11
    M = 10**9 + 7

    S = compute_sum_phi_all(N, M)

    ans = 0
    t = 1
    while (1 << t) <= N:
        limit = N >> t
        ans = (ans + S(limit) - 1) % M
        t += 1

    return ans % M


def main():
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
