"""Project Euler Problem 746: A Messy Dinner.

Find Σ_{k=2}^N M(k), where M(k) is the number of ways to arrange k families,
each of 2 men and 2 women, around a circular table of 4k seats such that men
and women alternate and no family has all members sitting next to each other.

Let f_k(r) be the number of ways to arrange k families such that exactly r≥1
of them are sitting next to each other. The number of ways to do this is the
product of:
- nCr(k,r), the number of ways to choose those r families,
- 4k, the number of ways to seat the first family,
- nCr(4k-3r-1,r-1), the number of ways to arrange the remaining r-1 families
  among the 4(N-r) other people,
- (r-1)!, the number of ways to permute those r-1 families,
- 4^r, the number of ways to arrange the family members in each of the r
  families,
- (2(N-i)!)², the number of ways to permute the remaining men among each
  other, and the remaining women among each other.

M(k) can easily be computed by taking the (2k)!² total permutations of people
and then using Inclusion Exclusion on the f_k(r).
"""

from __future__ import annotations

from typing import List


class Zp:
    """Modular arithmetic helper."""

    def __init__(self, max_n: int, mod: int) -> None:
        """Initialize with maximum n and modulus."""
        self.mod = mod
        self.max_n = max_n
        self._factorials: List[int] = []
        self._inv_factorials: List[int] = []
        self._precompute()

    def _precompute(self) -> None:
        """Precompute factorials and inverse factorials."""
        self._factorials = [1] * (self.max_n + 1)
        for i in range(1, self.max_n + 1):
            self._factorials[i] = (self._factorials[i - 1] * i) % self.mod

        self._inv_factorials = [1] * (self.max_n + 1)
        self._inv_factorials[self.max_n] = pow(
            self._factorials[self.max_n], self.mod - 2, self.mod
        )
        for i in range(self.max_n - 1, -1, -1):
            self._inv_factorials[i] = (
                self._inv_factorials[i + 1] * (i + 1)
            ) % self.mod

    def factorial(self, n: int) -> int:
        """Return n! mod mod."""
        if n > self.max_n:
            raise ValueError(f"n={n} exceeds max_n={self.max_n}")
        return self._factorials[n]

    def inv_factorial(self, n: int) -> int:
        """Return 1/(n!) mod mod."""
        if n > self.max_n:
            raise ValueError(f"n={n} exceeds max_n={self.max_n}")
        return self._inv_factorials[n]

    def nCr(self, n: int, r: int) -> int:
        """Return C(n, r) mod mod."""
        if r < 0 or r > n:
            return 0
        return (
            self.factorial(n)
            * self.inv_factorial(r)
            % self.mod
            * self.inv_factorial(n - r)
            % self.mod
        )


def sq(n: int, mod: int) -> int:
    """Square modulo mod."""
    return (n * n) % mod


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod <= 0:
        msg = "mod must be positive"
        raise ValueError(msg)
    if base == 0:
        return 0
    base %= mod
    result = 1
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 746."""
    N = 2021
    M = 10**9 + 7

    zp = Zp(4 * N, M)
    ans = 0

    for k in range(2, N + 1):
        res = sq(zp.factorial(2 * k), M)
        for r in range(1, k + 1):
            f_k = (
                zp.nCr(k, r)
                * (4 * k) % M
                * zp.nCr(4 * k - 3 * r - 1, r - 1) % M
                * zp.factorial(r - 1) % M
                * pow_mod(4, r, M) % M
                * sq(zp.factorial(2 * (k - r)), M) % M
            )
            res = (res + parity(r) * f_k) % M
        ans = (ans + 2 * res) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
