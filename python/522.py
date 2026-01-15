"""Project Euler Problem 522: Hilbert's Hotel.

In a Hilbert Hotel graph G, each vertex has a directed edge to some other
vertex. Let f(G) be the minimum number of vertices whose directed edge
needs to be updated to another vertex, such that we obtain a single cycle
over all nodes. Find Σ f(G) over all Hilbert Hotel graphs G with N
vertices.
"""

from __future__ import annotations


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp >>= 1
        base = (base * base) % mod
    return result


def factorial_mod(n: int, mod: int) -> int:
    """Factorial modulo mod."""
    result = 1
    for i in range(1, n + 1):
        result = (result * i) % mod
    return result


def ncr_mod(n: int, k: int, mod: int) -> int:
    """nCr modulo mod."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1

    # Use multiplicative formula
    result = 1
    for i in range(min(k, n - k)):
        result = (result * (n - i)) % mod
        result = (result * pow(i + 1, mod - 2, mod)) % mod

    return result


def solve() -> int:
    """Solve Problem 522."""
    N = 12344321
    M = 135707531

    # Part 1: vertices without incoming edges
    ans = N * (N - 1) % M * pow_mod(N - 2, N - 1, M) % M

    # Part 2: cycles without incoming edges
    for l in range(2, N):
        term = (
            ncr_mod(N, l, M)
            * factorial_mod(l - 1, M)
            % M
            * pow_mod(N - l - 1, N - l, M)
        ) % M
        ans = (ans + term) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
