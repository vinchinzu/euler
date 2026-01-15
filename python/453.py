"""Project Euler Problem 453: Simple quadrilaterals.

Find the number of simple quadrilaterals (no straight angles or
self-intersections) whose vertices are lattice points (x, y) with 0 ≤ x ≤ W
and 0 ≤ y ≤ H.
"""

from __future__ import annotations

from math import gcd, isqrt
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


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp //= 2
    return result


def sum_powers(n: int, k: int, mod: int) -> int:
    """Sum of k-th powers from 1 to n modulo mod."""
    result = 0
    for i in range(1, n + 1):
        result = (result + pow_mod(i, k, mod)) % mod
    return result


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    t, new_t = 0, 1
    r, new_r = m, a
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Inverse does not exist")
    if t < 0:
        t += m
    return t


def solve() -> int:
    """Solve Problem 453."""
    W = 12_345
    H = 6_789
    L = min(W, H)
    M = 135_707_531
    mobius = pre_mobius(L)

    def f(a: int, b: int) -> int:
        """Helper function."""
        return pow_mod(W + 1, a, M) * pow_mod(H + 1, b, M) % M

    def g(a: int, b: int, c: int) -> int:
        """Helper function."""
        res = 0
        for g_val in range(1, L + 1):
            for gp in range(1, L // g_val + 1):
                res = (
                    res
                    + mobius[gp]
                    * pow_mod(g_val, a + b + c, M)
                    * pow_mod(gp, a + b, M)
                    % M
                    * sum_powers(W // g_val // gp, a, M)
                    % M
                    * sum_powers(H // g_val // gp, b, M)
                    % M
                ) % M
        return res % M

    ans = (
        20
        * mod_inv(3, M)
        * (
            (
                f(0, 0) * g(1, 1, 2)
                - f(0, 1) * g(1, 0, 2)
                - f(1, 0) * g(0, 1, 2)
                + f(1, 1) * g(0, 0, 2)
            )
            % M
        )
        + 7
        * (
            f(0, 1) * g(1, 0, 1)
            + f(1, 0) * g(0, 1, 1)
            - f(1, 1) * g(0, 0, 1)
            - f(0, 0) * g(1, 1, 1)
        )
        + 4
        * (
            f(1, 2) * g(1, 0, 1)
            + f(2, 1) * g(0, 1, 1)
            - f(2, 2) * g(0, 0, 1)
            - f(1, 1) * g(1, 1, 1)
        )
        + 7 * mod_inv(12, M) * f(1, 1)
        - 5 * mod_inv(18, M) * (f(1, 2) + f(2, 1))
        - 7 * mod_inv(12, M) * (f(1, 3) + f(3, 1))
        + 269 * mod_inv(432, M) * f(2, 2)
        + 5 * mod_inv(18, M) * (f(1, 4) + f(4, 1))
        - 149 * mod_inv(432, M) * (f(2, 4) + f(4, 2))
        + 7 * mod_inv(12, M) * f(3, 3)
        + 29 * mod_inv(432, M) * f(4, 4)
    ) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
