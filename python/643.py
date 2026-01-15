"""Project Euler Problem 643: 2-Friendly.

Find the number of positive integer pairs 1 ≤ p < q ≤ N such that gcd(p, q)
is a power of 2.

For every t ≥ 1, the number of pairs with gcd(p, q) = 2^t is exactly the
number of pairs of distinct relatively prime integers up to ⌊N / 2^t⌋. This
is equal to ( sum_{k=1}^{⌊N / 2^t⌋} φ(k) ) - 1, where we subtract one to
remove the pair where p = q.
"""

from __future__ import annotations

from sympy import primerange


def sieve_phi(limit: int) -> list[int]:
    """Euler totient function sieve."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] -= phi[j] // i
    return phi


def sum_phis(n: int, mod: int) -> int:
    """Sum of Euler totient function values."""
    phi = sieve_phi(n)
    return sum(phi[1:]) % mod


def solve() -> int:
    """Solve Problem 643."""
    N = 10**11
    M = 10**9 + 7

    ans = 0
    t = 1
    while 2**t <= N:
        limit = N // (2**t)
        ans = (ans + sum_phis(limit, M) - 1) % M
        t += 1
    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
