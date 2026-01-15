"""Project Euler Problem 415: Titanic sets.

A titanic set is a set of lattice points such that some line passes through
exactly 2 points. Find the number of titanic sets such that each point (x, y)
satisfies 0 ≤ x,y ≤ N.

There are 2 ^ (N+1)² subsets of points 0 ≤ x,y ≤ N, and we count the number
of subsets that are not Titanic sets:
- The empty set is not a titanic set.
- None of the (N+1)² sets of a single point are titanic sets.
- By Sylvester's Theorem, the only sets with at least two points that are
  not titanic sets, are sets that consist of only 3 or more collinear points.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def sq(n: int, mod: int) -> int:
    """Return n² mod mod."""
    return (n * n) % mod


def tr(n: int, mod: int) -> int:
    """Return n(n+1)/2 mod mod."""
    return (n * (n + 1) // 2) % mod


def sum_powers(n: int, exp: int, mod: int) -> int:
    """Return sum_{k=1}^n k^exp mod mod."""
    if exp == 0:
        return n % mod
    elif exp == 1:
        return tr(n, mod)
    elif exp == 2:
        return (n * (n + 1) * (2 * n + 1) // 6) % mod
    else:
        result = 0
        for k in range(1, n + 1):
            result = (result + pow_mod(k, exp, mod)) % mod
        return result


def sum_ag(L: int, e: int, mod: int) -> int:
    """Sum of arithmetico-geometric series: sum_{g=1}^L g^e * 2^g."""
    if e == 0:
        return (pow_mod(2, L + 1, mod) - 2) % mod
    elif e == 1:
        return (((L - 1) % mod * pow_mod(2, L + 1, mod) + 2) % mod) % mod
    elif e == 2:
        return (
            (
                (sq(L, mod) - 2 * L + 3) % mod * pow_mod(2, L + 1, mod) - 6
            )
            % mod
        ) % mod
    else:
        raise ValueError(f"e={e} not supported")


def euler_totient(n: int) -> int:
    """Compute Euler's totient function phi(n)."""
    result = n
    p = 2
    while p * p <= n:
        if n % p == 0:
            while n % p == 0:
                n //= p
            result -= result // p
        p += 1
    if n > 1:
        result -= result // n
    return result


def sum_phis(n: int, e: int, mod: int) -> dict[int, int]:
    """Compute sum_{x=1}^floor(n/q) x^e * phi(x) for each q.

    Returns a dictionary mapping q to the sum.
    """
    result: dict[int, int] = {}
    L = isqrt(n)

    # Precompute phi values
    phi = [0] * (L + 1)
    for i in range(1, L + 1):
        phi[i] = euler_totient(i)

    # For q <= L, compute directly
    for q in range(1, L + 1):
        s = 0
        for x in range(1, n // q + 1):
            if x <= L:
                s = (s + pow_mod(x, e, mod) * phi[x]) % mod
            else:
                s = (s + pow_mod(x, e, mod) * euler_totient(x)) % mod
        result[q] = s % mod

    # For q > L, use quotient method
    for q in range(L + 1, n // L + 1):
        # Simplified: use direct computation for now
        s = 0
        for x in range(1, n // q + 1):
            s = (s + pow_mod(x, e, mod) * euler_totient(x)) % mod
        result[q] = s % mod

    return result


def solve() -> int:
    """Solve Problem 415."""
    N = 10**11
    L = isqrt(N)
    M = 10**8

    # Compute sum_phis for e = 0, 1, 2
    sum_phis_0 = sum_phis(N, 0, M)
    sum_phis_1 = sum_phis(N, 1, M)
    sum_phis_2 = sum_phis(N, 2, M)

    # Total subsets
    ans = pow_mod(pow_mod(2, N + 1, M), N + 1, M)

    # Subtract empty set
    ans = (ans - 1) % M

    # Subtract single points
    ans = (ans - sq(N + 1, M)) % M

    # Subtract horizontal/vertical lines
    term = (
        pow_mod(2, N + 1, M)
        - 1
        - (N + 1) % M
        - tr(N, M)
    ) % M
    ans = (ans - 2 * (N + 1) % M * term) % M

    # Subtract diagonal lines for g <= L
    for g in range(1, L + 1):
        T = (
            sq(g, M) * sum_phis_2.get(g, 0)
            - 3 * (N + 1) % M * g % M * sum_phis_1.get(g, 0)
            + 2 * sq(N + 1, M) * sum_phis_0.get(g, 0)
            - (N + 1 - g) % M * ((N + 1) % M)
        ) % M
        ans = (ans - (pow_mod(2, g, M) - 2) * T) % M

    # Subtract diagonal lines for q > L
    for q in range(1, N // L):
        if q not in sum_phis_2:
            continue
        term1 = sum_phis_2[q] * (
            sum_ag(N // q, 2, M) - sum_ag(N // (q + 1), 2, M)
        )
        term2 = (N + 1) % M * (3 * sum_phis_1.get(q, 0) - 1) % M * (
            sum_ag(N // q, 1, M) - sum_ag(N // (q + 1), 1, M)
        )
        term3 = sq(N + 1, M) * (2 * sum_phis_0.get(q, 0) - 1) % M * (
            sum_ag(N // q, 0, M) - sum_ag(N // (q + 1), 0, M)
        )
        term4 = 2 * sum_phis_2[q] * (
            sum_powers(N // q, 2, M) - sum_powers(N // (q + 1), 2, M)
        )
        term5 = 2 * (N + 1) % M * (3 * sum_phis_1.get(q, 0) - 1) % M * (
            sum_powers(N // q, 1, M) - sum_powers(N // (q + 1), 1, M)
        )
        term6 = 2 * sq(N + 1, M) * (2 * sum_phis_0.get(q, 0) - 1) % M * (
            sum_powers(N // q, 0, M) - sum_powers(N // (q + 1), 0, M)
        )
        ans = (
            ans
            - term1
            + term2
            - term3
            - term4
            + term5
            - term6
        ) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
