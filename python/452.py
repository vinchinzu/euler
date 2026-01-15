"""Project Euler Problem 452: N-tuples with product <= N.

Find the number of N-tuples of positive integers whose product does not exceed N.
"""

from __future__ import annotations

from math import isqrt
from typing import List


class Zp:
    """Modular arithmetic helper class."""

    def __init__(self, limit: int, mod: int) -> None:
        """Initialize with limit and modulus."""
        self.limit = limit
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

    def inv_factorial(self, n: int) -> int:
        """Inverse factorial."""
        return self.inv_fact[n]


def solve() -> int:
    """Solve Problem 452."""
    N = 10**9
    L = N.bit_length() + 1
    M = 1_234_567_891
    zp = Zp(L, M)

    prods = [1] * (L + 1)
    for i in range(1, L + 1):
        prods[i] = (prods[i - 1] * (N + 1 - i)) % M

    ans = 0

    def helper(
        min_val: int, n: int, prev: int, num_elements: int, num_perm: int
    ) -> None:
        """Recursive helper."""
        nonlocal ans
        if prev != 1:
            ans = (ans + num_perm * prods[num_elements]) % M
        if min_val <= N // n:
            ans = (
                ans
                + num_perm * prods[num_elements + 1] % M * (N // n - min_val + 1)
            ) % M
        i = min_val
        while n * i * i <= N:
            count = 1
            new_n = n * i
            while new_n <= N:
                helper(
                    i + 1,
                    new_n,
                    count,
                    num_elements + count,
                    (num_perm * zp.inv_factorial(count)) % M,
                )
                count += 1
                new_n *= i
            i += 1

    helper(2, 1, 0, 0, 1)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
