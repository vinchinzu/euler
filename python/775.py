"""Project Euler Problem 775: Paper Wrapping.

Let g(n) be the difference between the amount of paper to wrap n unit cubes,
and the amount of paper to wrap n unit cubes that are bundled into a
poly-cube. Find Σ_{n=1}^N g(n).

The most compact arrangement of the poly-cube is to bundle as many cubes into
an almost-cube (a rectangular prism where all sides are within 1 of each
other), then bundling as many cubes into an almost-square arrangement on the
largest face, then putting the remaining cubes on that face.

We can simulate adding cubes to the largest face. The very first cube requires
6 units of paper. For each face, the 0th (first) cube requires 4 additional
units of paper. Then, each cube k² and each cube k(k+1) requires 2 additional
units of paper, and the rest don't require any additional units. So for each of
those cubes, we add 4 or 2, which is counted for that cube and every cube
added from then onwards.

This counts the total amount of paper required to wrap the poly-cubes. To
compute Σ g(n), we just subtract from the total amount of paper to wrap unit
codes by themselves, which is obviously just 6tr(N).
"""

from __future__ import annotations

from math import isqrt


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


def nCr(n: int, k: int, mod: int) -> int:
    """Binomial coefficient C(n, k) mod mod."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    result = 1
    for i in range(min(k, n - k)):
        result = (result * (n - i) * pow_mod(i + 1, mod - 2, mod)) % mod
    return result


def sum_powers(n: int, k: int, mod: int) -> int:
    """Sum of k-th powers: 1^k + 2^k + ... + n^k mod mod."""
    if k == 0:
        return n % mod
    if k == 1:
        return tr(n, mod)
    if k == 2:
        return (n * (n + 1) * (2 * n + 1) // 6) % mod
    # For higher powers, use Faulhaber's formula or precomputation
    result = 0
    for i in range(1, n + 1):
        result = (result + pow_mod(i, k, mod)) % mod
    return result


def solve() -> int:
    """Solve Problem 775."""
    N = 10**16
    M = 10**9 + 7

    sides = [1, 1, 1]
    index = 1
    ans = 6 * (tr(N, M) - N) % M

    while True:
        side1 = sides[1]
        side2 = sides[2]
        d1 = min(isqrt(N - index - 1), side2 - 1)
        d2 = min((isqrt(4 * (N - index)) - 1) // 2, side1 - 1)

        ans = (ans - 4 * (N - index)) % M
        ans = (
            ans
            - 2
            * ((N - index) % M * d1 % M - sum_powers(d1, 2, M))
            % M
        ) % M
        ans = (
            ans
            - 2
            * ((N - index) % M * d2 % M - 2 * nCr(d2 + 2, 3, M))
            % M
        ) % M
        ans = ans % M

        index += side1 * side2
        if index >= N:
            break

        # Rotate sides
        sides.append(sides.pop(0) + 1)

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
