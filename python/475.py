"""Project Euler Problem 475: Music festival.

Find the number of ways that N musicians can be rearranged from N/K quartets
to N/3 trios such that no two members of the same quartet are in the same
trio.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict


@dataclass(frozen=True)
class Key:
    """Key for memoization."""

    m1: int
    m2: int
    m3: int


class Zp:
    """Modular arithmetic helper."""

    def __init__(self, limit: int, mod: int) -> None:
        """Initialize."""
        self.mod = mod
        self.fact = [1] * (limit + 1)
        self.inv_fact = [1] * (limit + 1)
        for i in range(1, limit + 1):
            self.fact[i] = (self.fact[i - 1] * i) % mod
        self.inv_fact[limit] = self._mod_inv(self.fact[limit], mod)
        for i in range(limit - 1, -1, -1):
            self.inv_fact[i] = (self.inv_fact[i + 1] * (i + 1)) % mod

    def _mod_inv(self, a: int, m: int) -> int:
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

    def factorial(self, n: int) -> int:
        """Factorial."""
        return self.fact[n]

    def inv_factorial(self, n: int) -> int:
        """Inverse factorial."""
        return self.inv_fact[n]


def nCr(n: int, k: int, zp: Zp) -> int:
    """Binomial coefficient."""
    if k < 0 or k > n:
        return 0
    return (
        zp.factorial(n)
        * zp.inv_factorial(k)
        % zp.mod
        * zp.inv_factorial(n - k)
        % zp.mod
    )


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


def solve() -> int:
    """Solve Problem 475."""
    N = 600
    K = 4
    M = 10**9 + 7
    zp = Zp(K * N, M)
    cache: Dict[Key, int] = {}

    def f(m1: int, m2: int, m3: int) -> int:
        """Recursive function."""
        key = Key(m1, m2, m3)
        if key in cache:
            return cache[key]

        if m1 + m2 + m3 == 0:
            return 1

        result = 0
        for d1 in range(min(m1, K) + 1):
            for d2 in range(min(2 * m2, K - d1) + 1):
                d3 = K - d1 - d2
                if d3 < 0 or d3 > 3 * m3:
                    continue
                if d2 % 2 != 0 or d3 % 3 != 0:
                    continue
                d2_groups = d2 // 2
                d3_groups = d3 // 3

                ways = 1
                ways = (ways * pow_mod(1, d1, M) * nCr(m1, d1, zp)) % M
                ways = (
                    ways * pow_mod(2, d2_groups, M) * nCr(m2, d2_groups, zp)
                ) % M
                ways = (
                    ways * pow_mod(3, d3_groups, M) * nCr(m3, d3_groups, zp)
                ) % M

                result = (
                    result
                    + ways
                    * f(m1 - d1 + d2_groups, m2 - d2_groups + d3_groups, m3 - d3_groups)
                ) % M

        cache[key] = result
        return result

    ans = (
        f(0, 0, N // 3)
        * pow_mod(zp.factorial(K), N // K, M)
        % M
        * pow_mod(zp.inv_factorial(3), N // 3, M)
        % M
        * zp.inv_factorial(N // 3)
        % M
    )
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
