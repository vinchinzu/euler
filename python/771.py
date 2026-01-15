"""Project Euler Problem 771: Increasing Sequences.

Find the number of sequences of at least 5 strictly increasing integers x_i
such that |(x_i)² - x_{i-1}x_{i+1}| ≤ 2 for all i.

There are the following categories of sequences:
- Sequences of consecutive integers (e.g. 1, 2, 3, 4, 5). There are tr(N-4)
  of these.
- Subsequences of 6 miscellaneous sequences (e.g. 1, 2, 3, 4, 6), hardcoded
  below.
- Subsequences of a few hardcoded linear recurrences of degree 2 (e.g. 1, 2, 3,
  5, 8), encoded below as their first 2 terms x0 and x1, and constants a and
  b such that x_i = ax_{i-2} + bx_{i-1}.
- Subsequences of the linear recurrences beginning with 1, x1≥3 and either
  x_i = x1 * x_{i-1} - x_{i-2}, or x_i = x1 * x_{i-1} + x_{i-2} (e.g. 1, 3,
  8, 21, 55).
- Sequences starting with 1, 2, 6, 18, 54, with each term 3 times the
  previous.
- Perfect geometric sequences, which can be expressed as (k*a^e, k*a^{e-1},
  ..., k*b^e) with a<b and (a,b)=1. We can count these by iterating over e
  and b. Then there are ϕ(b) values of a, and ⌊N/b^e⌋ values of k.
"""

from __future__ import annotations

from math import isqrt
from typing import List

from sympy import primerange


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


def tr(n: int, mod: int) -> int:
    """Triangular number: n*(n+1)//2 mod mod."""
    return (n * (n + 1) // 2) % mod


def pre_phi(limit: int) -> List[int]:
    """Precompute Euler totient function."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] -= phi[j] // i
    return phi


def process_seq(seq: List[int], N: int, mod: int) -> int:
    """Count subsequences of at least 5 elements from seq."""
    count = 0
    for start in range(len(seq)):
        for end in range(start + 5, len(seq) + 1):
            if seq[end - 1] <= N:
                count += 1
    return count % mod


def process_recursive_seq(
    x0: int, x1: int, a: int, b: int, N: int, mod: int
) -> int:
    """Process recursive sequence x_i = a*x_{i-2} + b*x_{i-1}."""
    seq = [x0, x1]
    while a * seq[-2] + b * seq[-1] <= N:
        seq.append(a * seq[-2] + b * seq[-1])
    return process_seq(seq, N, mod)


def solve() -> int:
    """Solve Problem 771."""
    N = 10**18
    M = 10**9 + 7
    phi_limit = int(N ** (1.0 / 4))
    phi = pre_phi(phi_limit)

    ans = tr(N - 4, M)

    # Hardcoded sequences
    ans = (ans + process_seq([1, 2, 3, 4, 6, 9], N, M)) % M
    ans = (ans + process_seq([1, 2, 3, 5, 9, 16], N, M)) % M
    ans = (ans + process_seq([1, 2, 4, 7, 12], N, M)) % M
    ans = (ans + process_seq([1, 2, 4, 9, 20], N, M)) % M
    ans = (ans + process_seq([1, 2, 6, 17, 48], N, M)) % M
    ans = (ans + process_seq([1, 2, 6, 19, 60], N, M)) % M

    # Hardcoded recursive sequences
    ans = (ans + process_recursive_seq(1, 2, 1, 1, N, M)) % M
    ans = (ans + process_recursive_seq(1, 2, 1, 2, N, M)) % M
    ans = (ans + process_recursive_seq(1, 2, -1, 3, N, M)) % M
    ans = (ans + process_recursive_seq(1, 3, 1, 2, N, M)) % M
    ans = (ans + process_recursive_seq(1, 3, -1, 4, N, M)) % M

    # Recursive sequences with x1 >= 3
    x1 = 3
    while pow_mod(x1 - 1, 4, M) <= N:
        ans = (ans + process_recursive_seq(1, x1, -1, x1, N, M)) % M
        ans = (ans + process_recursive_seq(1, x1, 1, x1, N, M)) % M
        x1 += 1

    # Sequences starting with 1, 2, 6, 18, 54, ...
    x1 = 2
    while 27 * x1 <= N:
        ans = (ans + 1) % M
        x1 *= 3

    # Perfect geometric sequences
    e = 4
    while pow_mod(2, e, M) <= N:
        x1 = 2
        while pow_mod(x1, e, M) <= N:
            ans = (ans + (N // pow_mod(x1, e, M)) % M * phi[x1]) % M
            x1 += 1
        e += 1

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
